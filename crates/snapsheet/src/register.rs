use std::collections::HashMap;

use irgen_model::attr::extract_access_value;
use irgen_model::base::{Field, Register, RegisterFile};

use crate::config::SnapsheetConfig;
use crate::error::{Error, ValidationIssue};
use crate::excel::{Row, Table};
use crate::number::{format_address, literal_fits_bits, parse_literal, parse_u64};

#[cfg(test)]
const REGISTER_COLUMNS: &[&str] = &[
    "ADDR",
    "REG",
    "FIELD",
    "BIT",
    "WIDTH",
    "ATTRIBUTE",
    "DEFAULT",
    "DESCRIPTION",
];

#[derive(Debug)]
struct RegisterGroup {
    source_row: usize,
    spec: String,
    offset: u64,
    fields: Vec<ParsedField>,
    ranges: Vec<(u64, u64, String)>,
    width: u64,
}

#[derive(Debug, Clone)]
struct ArraySpec {
    name: String,
    start: u64,
    end: u64,
    stride: u64,
}

impl ArraySpec {
    fn dim(&self) -> u64 {
        self.end - self.start
    }

    fn has_same_layout(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end && self.stride == other.stride
    }
}

#[derive(Debug, Clone)]
struct ParsedField {
    source_row: usize,
    field: Field,
    uses_register_name: bool,
}

impl ParsedField {
    fn name(&self) -> &str {
        self.field.name()
    }

    fn for_register(&self, register: &str) -> Field {
        if !self.uses_register_name {
            return self.field.clone();
        }

        Field::new(
            register.into(),
            self.field.offset().into(),
            self.field.width().into(),
            self.field.attr().into(),
            self.field.reset().into(),
            self.field.desc().into(),
        )
    }
}

#[derive(Debug)]
struct RegisterFileGroup {
    name: String,
    source_row: usize,
    spec: String,
    offset: u64,
    array: ArraySpec,
    registers: Vec<Register>,
    ranges: Vec<(u64, u64, String, usize)>,
}

impl RegisterFileGroup {
    fn contains(&self, offset: u64) -> bool {
        self.offset
            .checked_add(self.array.stride)
            .is_some_and(|end| offset >= self.offset && offset < end)
    }

    fn child_range(&self) -> u64 {
        self.ranges
            .iter()
            .map(|(_, end, _, _)| *end)
            .max()
            .unwrap_or(0)
    }
}

pub(crate) fn parse_registers(
    config: &SnapsheetConfig,
    table: &Table,
    block: &str,
    block_range: u64,
) -> Result<(Vec<Register>, Vec<RegisterFile>), Error> {
    let columns = &config.columns.register;
    table.require_columns(&columns.required())?;

    let mut current_addr = None;
    let mut current_spec = None;
    let mut groups = Vec::<RegisterGroup>::new();
    let mut issues = Vec::<ValidationIssue>::new();

    for row in table.rows() {
        let mut row_has_error = false;
        if let Some(value) = row.get(&columns.address) {
            match parse_u64(table, row, &columns.address, value, Some(block), None) {
                Ok(value) => current_addr = Some(value),
                Err(error) => {
                    collect_validation(&mut issues, error)?;
                    current_addr = None;
                    row_has_error = true;
                }
            }
        } else if !config.register.inherit_address {
            current_addr = None;
        }
        if let Some(value) = row.get(&columns.register) {
            current_spec = Some(value.to_owned());
        } else if !config.register.inherit_register {
            current_spec = None;
        }

        let Some(offset) = current_addr else {
            if !row_has_error {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(row.number()),
                        Some(&columns.address),
                        Some(block),
                        None,
                        if config.register.inherit_address {
                            "register address is missing and cannot be inherited"
                        } else {
                            "register address is missing"
                        },
                    ),
                )?;
            }
            continue;
        };
        let Some(spec) = current_spec.as_deref() else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.register),
                    Some(block),
                    None,
                    if config.register.inherit_register {
                        "register name is missing and cannot be inherited"
                    } else {
                        "register name is missing"
                    },
                ),
            )?;
            continue;
        };

        let field = match parse_field(config, table, row, block, spec) {
            Ok(field) => field,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        let group_index = groups
            .iter()
            .position(|group| group.spec == spec && group.offset == offset);

        if let Some(index) = group_index {
            if let Err(error) = add_field(config, table, row, block, &mut groups[index], field) {
                collect_validation(&mut issues, error)?;
            }
        } else {
            let mut group = RegisterGroup {
                source_row: row.number(),
                spec: spec.into(),
                offset,
                fields: Vec::new(),
                ranges: Vec::new(),
                width: 0,
            };
            match add_field(config, table, row, block, &mut group, field) {
                Ok(()) => groups.push(group),
                Err(error) => collect_validation(&mut issues, error)?,
            }
        }
    }

    let mut registers = Vec::new();
    let mut register_file_groups = Vec::<RegisterFileGroup>::new();
    let mut names = HashMap::<String, usize>::new();
    let mut occupied_registers = Vec::<(u64, u64, String, usize)>::new();

    for group in groups {
        if config.register.require_byte_aligned && group.width % 8 != 0 {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(group.source_row),
                    Some(&columns.width),
                    Some(block),
                    Some(&group.spec),
                    "register width must be byte-aligned",
                ),
            )?;
            continue;
        }

        if let Some(array) = match parse_array_spec(config, table, &group, block) {
            Ok(array) => array,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        } {
            let byte_width = group.width / 8;
            let file_index = register_file_groups
                .iter()
                .position(|file| file.array.has_same_layout(&array) && file.contains(group.offset));

            let file_index = match file_index {
                Some(file_index) => file_index,
                None => {
                    if array.stride < byte_width {
                        collect_validation(
                            &mut issues,
                            Error::validation(
                                table.sheet(),
                                Some(group.source_row),
                                Some(&columns.register),
                                Some(block),
                                Some(&group.spec),
                                format!(
                                    "register file range {} is smaller than register byte width {}",
                                    format_address(array.stride),
                                    format_address(byte_width)
                                ),
                            ),
                        )?;
                        continue;
                    }

                    let name = array.name.clone();
                    if let Some(previous_row) = duplicate_name_row(
                        config.validation.reject_duplicate_registers,
                        &names,
                        &name,
                    ) {
                        collect_validation(
                            &mut issues,
                            Error::validation(
                                table.sheet(),
                                Some(group.source_row),
                                Some(&columns.register),
                                Some(block),
                                Some(&name),
                                format!(
                                    "register name collides with the definition on row {previous_row}"
                                ),
                            ),
                        )?;
                        continue;
                    }
                    names.insert(name.clone(), group.source_row);

                    register_file_groups.push(RegisterFileGroup {
                        name,
                        source_row: group.source_row,
                        spec: group.spec.clone(),
                        offset: group.offset,
                        array: array.clone(),
                        registers: Vec::new(),
                        ranges: Vec::new(),
                    });
                    register_file_groups.len() - 1
                }
            };

            if let Err(error) = add_register_to_file(
                config,
                table,
                block,
                &mut register_file_groups[file_index],
                &group,
            ) {
                collect_validation(&mut issues, error)?;
            }
        } else {
            let name = group.spec.clone();
            if let Some(previous_row) =
                duplicate_name_row(config.validation.reject_duplicate_registers, &names, &name)
            {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(group.source_row),
                        Some(&columns.register),
                        Some(block),
                        Some(&name),
                        format!("register name collides with the definition on row {previous_row}"),
                    ),
                )?;
                continue;
            }
            names.insert(name.clone(), group.source_row);

            let byte_width = group.width / 8;
            let Some(end) = group.offset.checked_add(byte_width) else {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(group.source_row),
                        Some(&columns.address),
                        Some(block),
                        Some(&name),
                        "register address overflows u64",
                    ),
                )?;
                continue;
            };
            if end > block_range {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(group.source_row),
                        Some(&columns.address),
                        Some(block),
                        Some(&name),
                        format!(
                            "register range {}..{} exceeds address block range {}",
                            format_address(group.offset),
                            format_address(end),
                            format_address(block_range)
                        ),
                    ),
                )?;
                continue;
            }
            if config.validation.reject_overlapping_registers
                && let Some((_, _, other_name, other_row)) =
                    occupied_registers
                        .iter()
                        .find(|(other_start, other_end, _, _)| {
                            group.offset < *other_end && *other_start < end
                        })
            {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(group.source_row),
                        Some(&columns.address),
                        Some(block),
                        Some(&name),
                        format!("address overlaps register `{other_name}` from row {other_row}"),
                    ),
                )?;
                continue;
            }
            occupied_registers.push((group.offset, end, name.clone(), group.source_row));
            let fields = match fields_for_register(config, table, block, &group, &name) {
                Ok(fields) => fields,
                Err(error) => {
                    collect_validation(&mut issues, error)?;
                    continue;
                }
            };
            registers.push(Register::new(
                name,
                format_address(group.offset),
                group.width.to_string(),
                fields,
            ));
        }
    }

    let mut occupied = occupied_registers;
    let mut register_files = Vec::new();

    for file in register_file_groups {
        let Some(last_element_offset) = file
            .array
            .dim()
            .checked_sub(1)
            .and_then(|last_index| last_index.checked_mul(file.array.stride))
        else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(file.source_row),
                    Some(&columns.register),
                    Some(block),
                    Some(&file.spec),
                    "register file range overflows u64",
                ),
            )?;
            continue;
        };
        let Some(total_range) = last_element_offset.checked_add(file.child_range()) else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(file.source_row),
                    Some(&columns.register),
                    Some(block),
                    Some(&file.spec),
                    "register file range overflows u64",
                ),
            )?;
            continue;
        };
        let Some(end) = file.offset.checked_add(total_range) else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(file.source_row),
                    Some(&columns.address),
                    Some(block),
                    Some(&file.name),
                    "register address overflows u64",
                ),
            )?;
            continue;
        };
        if end > block_range {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(file.source_row),
                    Some(&columns.address),
                    Some(block),
                    Some(&file.name),
                    format!(
                        "register range {}..{} exceeds address block range {}",
                        format_address(file.offset),
                        format_address(end),
                        format_address(block_range)
                    ),
                ),
            )?;
            continue;
        }
        if config.validation.reject_overlapping_registers
            && let Some((_, _, other_name, other_row)) =
                occupied.iter().find(|(other_start, other_end, _, _)| {
                    file.offset < *other_end && *other_start < end
                })
        {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(file.source_row),
                    Some(&columns.address),
                    Some(block),
                    Some(&file.name),
                    format!("address overlaps register `{other_name}` from row {other_row}"),
                ),
            )?;
            continue;
        }
        occupied.push((file.offset, end, file.name.clone(), file.source_row));
        register_files.push(RegisterFile::new(
            file.name,
            format_address(file.offset),
            format_address(file.array.stride),
            file.array.dim().to_string(),
            file.registers,
        ));
    }

    if !issues.is_empty() {
        return Err(Error::validation_issues(issues));
    }

    Ok((registers, register_files))
}

fn duplicate_name_row(
    reject_duplicates: bool,
    names: &HashMap<String, usize>,
    name: &str,
) -> Option<usize> {
    reject_duplicates
        .then(|| names.get(name).copied())
        .flatten()
}

fn collect_validation(issues: &mut Vec<ValidationIssue>, error: Error) -> Result<(), Error> {
    match error.into_validation_issues() {
        Ok(mut new_issues) => {
            issues.append(&mut new_issues);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn add_register_to_file(
    config: &SnapsheetConfig,
    table: &Table,
    block: &str,
    file: &mut RegisterFileGroup,
    group: &RegisterGroup,
) -> Result<(), Error> {
    let columns = &config.columns.register;
    let relative_offset = group.offset.checked_sub(file.offset).ok_or_else(|| {
        Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.address),
            Some(block),
            Some(&group.spec),
            "register file child offset underflows",
        )
    })?;
    let byte_width = group.width / 8;
    let relative_end = relative_offset.checked_add(byte_width).ok_or_else(|| {
        Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.address),
            Some(block),
            Some(&group.spec),
            "register file child range overflows u64",
        )
    })?;

    if relative_end > file.array.stride {
        return Err(Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.address),
            Some(block),
            Some(&group.spec),
            format!(
                "register file child range {}..{} exceeds register file range {}",
                format_address(relative_offset),
                format_address(relative_end),
                format_address(file.array.stride)
            ),
        ));
    }
    if config.validation.reject_overlapping_registers
        && let Some((_, _, other_name, other_row)) =
            file.ranges.iter().find(|(other_start, other_end, _, _)| {
                relative_offset < *other_end && *other_start < relative_end
            })
    {
        return Err(Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.address),
            Some(block),
            Some(&group.spec),
            format!("address overlaps register `{other_name}` from row {other_row}"),
        ));
    }
    if config.validation.reject_duplicate_registers
        && file
            .registers
            .iter()
            .any(|register| register.name() == group.spec)
    {
        return Err(Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.register),
            Some(block),
            Some(&group.spec),
            "register name collides inside registerFile",
        ));
    }

    let Some(array) = parse_array_spec(config, table, group, block)? else {
        unreachable!("caller only passes array register groups")
    };
    let name = array.name;
    if config.validation.reject_duplicate_registers
        && file
            .registers
            .iter()
            .any(|register| register.name() == name)
    {
        return Err(Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&columns.register),
            Some(block),
            Some(&name),
            "register name collides inside registerFile",
        ));
    }

    let fields = fields_for_register(config, table, block, group, &name)?;
    file.ranges.push((
        relative_offset,
        relative_end,
        name.clone(),
        group.source_row,
    ));
    file.registers.push(Register::new(
        name,
        format_address(relative_offset),
        group.width.to_string(),
        fields,
    ));

    Ok(())
}

fn parse_field(
    config: &SnapsheetConfig,
    table: &Table,
    row: &Row,
    block: &str,
    register: &str,
) -> Result<ParsedField, Error> {
    let columns = &config.columns.register;
    let (name, uses_register_name) = if let Some(name) = row.get(&columns.field) {
        (name, false)
    } else if config.register.blank_field_name_uses_register() {
        (register, true)
    } else {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&columns.field),
            Some(block),
            Some(register),
            "required value is missing",
        ));
    };
    if !config.reserved.validate_field_name(name) {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&columns.field),
            Some(block),
            Some(register),
            "reserved fields must match `reserved[0-9]+` or `rsvd[0-9]+`",
        ));
    }
    let bit = table.require(row, &columns.bit, Some(block), Some(register))?;
    let width_text = table.require(row, &columns.width, Some(block), Some(register))?;
    let attribute = table.require(row, &columns.access, Some(block), Some(register))?;
    let reset = table.require(row, &columns.reset, Some(block), Some(register))?;
    let description = row
        .get(&columns.description)
        .unwrap_or(&config.register.default_description);

    let width = parse_u64(
        table,
        row,
        &columns.width,
        width_text,
        Some(block),
        Some(register),
    )?;
    if width == 0 {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&columns.width),
            Some(block),
            Some(register),
            "field width must be greater than zero",
        ));
    }

    let (msb, lsb) = parse_bit_range(config, table, row, block, register, bit)?;
    let bit_width = msb
        .checked_sub(lsb)
        .and_then(|width| width.checked_add(1))
        .ok_or_else(|| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some(&columns.bit),
                Some(block),
                Some(register),
                "bit range width overflows u64",
            )
        })?;
    if config.validation.check_bit_range_matches_width && bit_width != width {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&columns.bit),
            Some(block),
            Some(register),
            format!("bit range width is {bit_width}, but WIDTH is {width}"),
        ));
    }

    extract_access_value(attribute).map_err(|error| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&columns.access),
            Some(block),
            Some(register),
            error.to_string(),
        )
    })?;

    if config.validation.check_reset_fits_width {
        let reset_fits = literal_fits_bits(reset, width).map_err(|message| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some(&columns.reset),
                Some(block),
                Some(register),
                message,
            )
        })?;
        if !reset_fits {
            return Err(Error::validation(
                table.sheet(),
                Some(row.number()),
                Some(&columns.reset),
                Some(block),
                Some(register),
                format!("reset value does not fit in {width} bits"),
            ));
        }
    }

    Ok(ParsedField {
        source_row: row.number(),
        field: Field::new(
            name.into(),
            lsb.to_string(),
            width.to_string(),
            attribute.into(),
            reset.into(),
            description.into(),
        ),
        uses_register_name,
    })
}

fn add_field(
    config: &SnapsheetConfig,
    table: &Table,
    row: &Row,
    block: &str,
    group: &mut RegisterGroup,
    field: ParsedField,
) -> Result<(), Error> {
    let start = field
        .field
        .offset()
        .parse::<u64>()
        .expect("validated bit offset");
    let width = field
        .field
        .width()
        .parse::<u64>()
        .expect("validated field width");
    let end = start.checked_add(width).ok_or_else(|| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.bit),
            Some(block),
            Some(&group.spec),
            "field bit range overflows u64",
        )
    })?;

    if config.validation.reject_duplicate_fields
        && group
            .fields
            .iter()
            .any(|existing| existing.name() == field.name())
    {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.field),
            Some(block),
            Some(&group.spec),
            format!("field `{}` is duplicated", field.name()),
        ));
    }
    if config.validation.reject_overlapping_fields
        && let Some((_, _, other_name)) = group
            .ranges
            .iter()
            .find(|(other_start, other_end, _)| start < *other_end && *other_start < end)
    {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.bit),
            Some(block),
            Some(&group.spec),
            format!("field `{}` overlaps field `{other_name}`", field.name()),
        ));
    }

    group.width = group.width.max(end);
    group.ranges.push((start, end, field.name().into()));
    group.fields.push(field);
    Ok(())
}

fn fields_for_register(
    config: &SnapsheetConfig,
    table: &Table,
    block: &str,
    group: &RegisterGroup,
    register: &str,
) -> Result<Vec<Field>, Error> {
    let mut fields = Vec::with_capacity(group.fields.len());
    let mut names = HashMap::<String, usize>::new();

    for parsed in &group.fields {
        let field = parsed.for_register(register);
        if config.validation.reject_duplicate_fields
            && let Some(previous_row) = names.insert(field.name().into(), parsed.source_row)
        {
            return Err(Error::validation(
                table.sheet(),
                Some(parsed.source_row),
                Some(&config.columns.register.field),
                Some(block),
                Some(register),
                format!(
                    "field `{}` is duplicated by the definition on row {previous_row}",
                    field.name()
                ),
            ));
        }
        fields.push(field);
    }

    Ok(fields)
}

fn parse_bit_range(
    config: &SnapsheetConfig,
    table: &Table,
    row: &Row,
    block: &str,
    register: &str,
    value: &str,
) -> Result<(u64, u64), Error> {
    let inner = value
        .trim()
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some(&config.columns.register.bit),
                Some(block),
                Some(register),
                format!("invalid bit range `{value}`; expected `[msb:lsb]` or `[bit]`"),
            )
        })?;

    let (msb, lsb) = inner.split_once(':').map_or((inner, inner), |parts| parts);
    let msb = parse_literal(msb).map_err(|message| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.bit),
            Some(block),
            Some(register),
            message,
        )
    })?;
    let lsb = parse_literal(lsb).map_err(|message| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.bit),
            Some(block),
            Some(register),
            message,
        )
    })?;

    if msb < lsb {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(&config.columns.register.bit),
            Some(block),
            Some(register),
            "bit range MSB must be greater than or equal to LSB",
        ));
    }
    Ok((msb, lsb))
}

fn parse_array_spec(
    config: &SnapsheetConfig,
    table: &Table,
    group: &RegisterGroup,
    block: &str,
) -> Result<Option<ArraySpec>, Error> {
    let spec = group.spec.trim();
    if !spec.contains("{n}") && !spec.contains("range(") {
        return Ok(None);
    }

    let invalid = |message: String| {
        Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some(&config.columns.register.register),
            Some(block),
            Some(&group.spec),
            message,
        )
    };
    if !config.register.array.enabled {
        return Err(invalid(
            "registerFile arrays require register.array.enabled = true in snapsheet.toml".into(),
        ));
    }
    let (base, suffix) = spec
        .split_once("{n}")
        .ok_or_else(|| invalid("registerFile array must include the `{n}` placeholder".into()))?;
    if base.trim().is_empty() || suffix.contains("{n}") {
        return Err(invalid(
            "registerFile array has an invalid `{n}` placeholder".into(),
        ));
    }

    let expression = suffix
        .trim()
        .strip_prefix(',')
        .map(str::trim)
        .and_then(|value| value.strip_prefix('n'))
        .map(str::trim)
        .and_then(|value| value.strip_prefix('='))
        .map(str::trim)
        .and_then(|value| value.strip_prefix("range("))
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(|| invalid("expected `{name}{n}, n=range(...)`".into()))?;

    let args = expression
        .split(',')
        .map(str::trim)
        .map(|value| parse_literal(value).map_err(&invalid))
        .collect::<Result<Vec<_>, _>>()?;
    let default_stride = config
        .register
        .parse_default_array_step_bytes()
        .map_err(|message| {
            invalid(format!(
                "invalid register.default_array_step_bytes: {message}"
            ))
        })?;
    let (start, end, stride) = match args.as_slice() {
        [end] => (0, *end, default_stride),
        [start, end] => (*start, *end, default_stride),
        [start, end, step] => (*start, *end, *step),
        _ => return Err(invalid("range(...) expects one to three arguments".into())),
    };
    if stride == 0 {
        return Err(invalid("range(...) step must be greater than zero".into()));
    }
    if start >= end {
        return Err(invalid(
            "range(...) must produce at least one non-negative index".into(),
        ));
    }

    let dim = end
        .checked_sub(start)
        .ok_or_else(|| invalid("range(...) dimension overflows u64".into()))?;
    if dim > config.register.max_array_elements as u64 {
        return Err(invalid(format!(
            "range(...) exceeds the {}-element safety limit",
            config.register.max_array_elements
        )));
    }

    Ok(Some(ArraySpec {
        name: base.trim().into(),
        start,
        end,
        stride,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SnapsheetConfig;

    fn table(rows: &[&[&str]]) -> Table {
        Table::for_test("regs", REGISTER_COLUMNS, rows)
    }

    fn parse_registers(
        table: &Table,
        block: &str,
        block_range: u64,
    ) -> Result<(Vec<Register>, Vec<RegisterFile>), Error> {
        super::parse_registers(&complex_config(), table, block, block_range)
    }

    fn complex_config() -> SnapsheetConfig {
        let mut config = SnapsheetConfig::default();
        config.register.inherit_address = true;
        config.register.inherit_register = true;
        config.register.blank_field_name = "register_name".into();
        config.register.array.enabled = true;
        config.reserved.enabled = true;
        config
    }

    #[test]
    fn default_config_requires_explicit_register_cells() {
        let table = table(&[
            &["0", "reg", "high", "[31:16]", "16", "RW", "0", ""],
            &["", "", "low", "[15:0]", "16", "RW", "0", ""],
        ]);

        let error =
            super::parse_registers(&SnapsheetConfig::default(), &table, "regs", 4).unwrap_err();

        assert!(error.to_string().contains("register address is missing"));
    }

    #[test]
    fn default_config_rejects_array_syntax() {
        let table = table(&[&[
            "0",
            "reg{n}, n=range(2)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let error =
            super::parse_registers(&SnapsheetConfig::default(), &table, "regs", 8).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("registerFile arrays require register.array.enabled = true")
        );
    }

    #[test]
    fn supports_64_bit_addresses_and_array_indexes() {
        let table = table(&[&[
            "0x100000000",
            "reg{n}, n=range(1, 3)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let (registers, register_files) = parse_registers(&table, "regs", 0x1_0000_0100).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files[0].name(), "reg");
        assert_eq!(register_files[0].offset(), "0x100000000");
        assert_eq!(register_files[0].range(), "0x4");
        assert_eq!(register_files[0].dim(), "2");
        assert_eq!(register_files[0].regs()[0].offset(), "0x0");
    }

    #[test]
    fn uses_array_step_as_register_file_range() {
        let table = table(&[&[
            "0x100",
            "reg{n}, n=range(1, 3, 0x10)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let (registers, register_files) = parse_registers(&table, "regs", 0x200).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files[0].name(), "reg");
        assert_eq!(register_files[0].offset(), "0x100");
        assert_eq!(register_files[0].range(), "0x10");
        assert_eq!(register_files[0].dim(), "2");
    }

    #[test]
    fn validates_sparse_array_by_last_child_range() {
        let table = table(&[&[
            "0x10",
            "reg{n}, n=range(0, 512, 0x100)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let (registers, register_files) = parse_registers(&table, "regs", 0x20000).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files[0].offset(), "0x10");
        assert_eq!(register_files[0].range(), "0x100");
        assert_eq!(register_files[0].dim(), "512");
    }

    #[test]
    fn groups_matching_array_registers_into_one_register_file() {
        let table = table(&[
            &[
                "0xD00",
                "MATRIX_CTRL_ADDR{n}, n=range(0,10,16)",
                "timeout_en",
                "[31:0]",
                "32",
                "RW",
                "0",
                "",
            ],
            &[
                "0xD04",
                "MATRIX_INFO0_ADDR{n}, n=range(0,10,16)",
                "last_active_master",
                "[31:0]",
                "32",
                "RO",
                "0",
                "",
            ],
        ]);

        let (registers, register_files) = parse_registers(&table, "regs", 0x1000).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files.len(), 1);
        assert_eq!(register_files[0].name(), "MATRIX_CTRL_ADDR");
        assert_eq!(register_files[0].offset(), "0xD00");
        assert_eq!(register_files[0].range(), "0x10");
        assert_eq!(register_files[0].dim(), "10");
        assert_eq!(register_files[0].regs()[0].name(), "MATRIX_CTRL_ADDR");
        assert_eq!(register_files[0].regs()[0].offset(), "0x0");
        assert_eq!(register_files[0].regs()[1].name(), "MATRIX_INFO0_ADDR");
        assert_eq!(register_files[0].regs()[1].offset(), "0x4");
    }

    #[test]
    fn infers_wide_register_size_from_field_bits() {
        let table = table(&[&[
            "0",
            "reg",
            "value",
            "[127:0]",
            "128",
            "RW",
            "0xFFFF0000000000000000000000000000",
            "",
        ]]);

        let (registers, register_files) = parse_registers(&table, "regs", 16).unwrap();

        assert!(register_files.is_empty());
        assert_eq!(registers[0].size(), "128");
        assert_eq!(registers[0].fields()[0].offset(), "0");
        assert_eq!(registers[0].fields()[0].width(), "128");
    }

    #[test]
    fn infers_register_size_from_sparse_field_extent() {
        let table = table(&[
            &["0", "reg", "high", "[63:32]", "32", "RW", "0", ""],
            &["", "", "low", "[15:0]", "16", "RW", "0", ""],
        ]);

        let (registers, register_files) = parse_registers(&table, "regs", 8).unwrap();

        assert!(register_files.is_empty());
        assert_eq!(registers[0].size(), "64");
    }

    #[test]
    fn defaults_empty_field_name_to_register_name() {
        let table = table(&[&["0", "reg", "", "[31:0]", "32", "RW", "0", ""]]);

        let (registers, register_files) = parse_registers(&table, "regs", 4).unwrap();

        assert!(register_files.is_empty());
        assert_eq!(registers[0].fields()[0].name(), "reg");
    }

    #[test]
    fn defaults_empty_field_name_to_register_file_child_register_name() {
        let table = table(&[&[
            "0",
            "reg{n}, n=range(1, 3)",
            "",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let (registers, register_files) = parse_registers(&table, "regs", 12).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files[0].regs()[0].fields()[0].name(), "reg");
    }

    #[test]
    fn supports_configured_register_columns_and_defaults() {
        let mut config = SnapsheetConfig::default();
        config.columns.register.address = "Address".into();
        config.columns.register.register = "Register".into();
        config.columns.register.field = "Field".into();
        config.columns.register.bit = "Bits".into();
        config.columns.register.width = "BitWidth".into();
        config.columns.register.access = "Access".into();
        config.columns.register.reset = "Reset".into();
        config.columns.register.description = "Desc".into();
        config.register.inherit_address = true;
        config.register.inherit_register = true;
        config.register.blank_field_name = "register_name".into();
        config.register.array.enabled = true;
        config.register.default_description = "N/A".into();
        config.register.default_array_step_bytes = "0x8".into();

        let table = Table::for_test(
            "regs",
            &[
                "Address", "Register", "Field", "Bits", "BitWidth", "Access", "Reset", "Desc",
            ],
            &[&[
                "0",
                "reg{n}, n=range(2)",
                "value",
                "[31:0]",
                "32",
                "RW",
                "0",
                "",
            ]],
        );

        let (registers, register_files) =
            super::parse_registers(&config, &table, "regs", 0x20).unwrap();

        assert!(registers.is_empty());
        assert_eq!(register_files[0].range(), "0x8");
        assert_eq!(register_files[0].regs()[0].fields()[0].desc(), "N/A");
    }

    #[test]
    fn rejects_field_name_collisions_after_register_file_conversion() {
        let table = table(&[
            &[
                "0",
                "reg{n}, n=range(1, 3)",
                "",
                "[31:16]",
                "16",
                "RW",
                "0",
                "",
            ],
            &["", "", "reg", "[15:0]", "16", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 12).unwrap_err();

        assert!(error.to_string().contains("field `reg` is duplicated"));
    }

    #[test]
    fn rejects_duplicate_fields() {
        let table = table(&[
            &["0", "reg", "value", "[31:16]", "16", "RW", "0", ""],
            &["", "", "value", "[15:0]", "16", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(error.to_string().contains("field `value` is duplicated"));
    }

    #[test]
    fn reports_multiple_row_errors() {
        let table = table(&[
            &["0", "bad_attr", "value", "[31:0]", "32", "BAD", "0", ""],
            &["4", "bad_reset", "value", "[7:0]", "8", "RW", "0x100", ""],
        ]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();
        let message = error.to_string();

        assert!(message.contains("2 validation errors"));
        assert!(message.contains("invalid attribute: BAD"));
        assert!(message.contains("reset value does not fit in 8 bits"));
    }

    #[test]
    fn rejects_malformed_reserved_field_names_when_enabled() {
        let table = table(&[&["0", "reg", "reserved", "[31:0]", "32", "RO", "0", ""]]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("reserved fields must match `reserved[0-9]+` or `rsvd[0-9]+`")
        );
    }

    #[test]
    fn allows_malformed_reserved_field_names_when_disabled() {
        let mut config = SnapsheetConfig::default();
        config.register.inherit_address = true;
        config.register.inherit_register = true;
        config.reserved.enabled = false;
        let table = table(&[&["0", "reg", "reserved", "[31:0]", "32", "RO", "0", ""]]);

        let (registers, register_files) =
            super::parse_registers(&config, &table, "regs", 4).unwrap();

        assert!(register_files.is_empty());
        assert_eq!(registers[0].fields()[0].name(), "reserved");
    }

    #[test]
    fn rejects_overlapping_fields() {
        let table = table(&[
            &["0", "reg", "high", "[31:8]", "24", "RW", "0", ""],
            &["", "", "low", "[15:0]", "16", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(error.to_string().contains("overlaps field `high`"));
    }

    #[test]
    fn rejects_zero_array_step() {
        let table = table(&[&[
            "0",
            "reg{n}, n=range(0, 2, 0)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();

        assert!(error.to_string().contains("step must be greater than zero"));
    }

    #[test]
    fn rejects_register_file_ranges_smaller_than_register_width() {
        let table = table(&[&[
            "0",
            "reg{n}, n=range(0, 2, 1)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("register file range 0x1 is smaller than register byte width 0x4")
        );
    }

    #[test]
    fn rejects_array_name_collisions() {
        let table = table(&[
            &[
                "0",
                "reg{n}, n=range(2)",
                "value",
                "[31:0]",
                "32",
                "RW",
                "0",
                "",
            ],
            &["8", "reg", "value", "[31:0]", "32", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 12).unwrap_err();

        assert!(error.to_string().contains("register name collides"));
    }

    #[test]
    fn rejects_conflicting_register_addresses() {
        let table = table(&[
            &["0", "reg", "value", "[31:0]", "32", "RW", "0", ""],
            &["4", "reg", "value", "[31:0]", "32", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();

        assert!(error.to_string().contains("register name collides"));
    }

    #[test]
    fn rejects_mismatched_bit_width() {
        let table = table(&[&["0", "reg", "value", "[31:0]", "16", "RW", "0", ""]]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("bit range width is 32, but WIDTH is 16")
        );
    }

    #[test]
    fn rejects_invalid_attributes() {
        let table = table(&[&["0", "reg", "value", "[31:0]", "32", "BAD", "0", ""]]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(error.to_string().contains("invalid attribute: BAD"));
    }

    #[test]
    fn rejects_reset_values_that_do_not_fit() {
        let table = table(&[
            &["0", "reg", "high", "[31:1]", "31", "RW", "0", ""],
            &["", "", "low", "[0]", "1", "RW", "2", ""],
        ]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("reset value does not fit in 1 bits")
        );
    }

    #[test]
    fn rejects_wide_reset_values_that_do_not_fit() {
        let table = table(&[&[
            "0",
            "reg",
            "value",
            "[127:0]",
            "128",
            "RW",
            "0x100000000000000000000000000000000",
            "",
        ]]);

        let error = parse_registers(&table, "regs", 16).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("reset value does not fit in 128 bits")
        );
    }

    #[test]
    fn rejects_register_widths_that_are_not_byte_aligned() {
        let table = table(&[&["0", "reg", "value", "[32:0]", "33", "RW", "0", ""]]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("register width must be byte-aligned")
        );
    }

    #[test]
    fn rejects_malformed_ranges() {
        let table = table(&[&[
            "0",
            "reg{n}, n=range(0, nope)",
            "value",
            "[31:0]",
            "32",
            "RW",
            "0",
            "",
        ]]);

        let error = parse_registers(&table, "regs", 8).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("invalid unsigned integer `nope`")
        );
    }

    #[test]
    fn rejects_registers_outside_the_address_block() {
        let table = table(&[&["4", "reg", "value", "[31:0]", "32", "RW", "0", ""]]);

        let error = parse_registers(&table, "regs", 4).unwrap_err();

        assert!(error.to_string().contains("exceeds address block range"));
    }
}
