use std::collections::HashMap;

use irgen_model::attr::extract_access_value;
use irgen_model::base::{Field, Register};

use crate::error::Error;
use crate::excel::{Row, Table};
use crate::number::{format_address, parse_literal, parse_u64};

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
const REGISTER_WIDTH_BITS: u64 = 32;
const DEFAULT_ARRAY_STEP_BYTES: u64 = 0x4;
const MAX_ARRAY_ELEMENTS: usize = 1_000_000;

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

pub(crate) fn parse_registers(
    table: &Table,
    block: &str,
    block_range: u64,
) -> Result<Vec<Register>, Error> {
    table.require_columns(REGISTER_COLUMNS)?;

    let mut current_addr = None;
    let mut current_spec = None;
    let mut groups = Vec::<RegisterGroup>::new();

    for row in table.rows() {
        if let Some(value) = row.get("ADDR") {
            current_addr = Some(parse_u64(table, row, "ADDR", value, Some(block), None)?);
        }
        if let Some(value) = row.get("REG") {
            current_spec = Some(value.to_owned());
        }

        let offset = current_addr.ok_or_else(|| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("ADDR"),
                Some(block),
                None,
                "register address is missing and cannot be inherited",
            )
        })?;
        let spec = current_spec.as_deref().ok_or_else(|| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("REG"),
                Some(block),
                None,
                "register name is missing and cannot be inherited",
            )
        })?;

        let field = parse_field(table, row, block, spec)?;
        let group_index = groups
            .iter()
            .position(|group| group.spec == spec && group.offset == offset);

        if let Some(index) = group_index {
            add_field(table, row, block, &mut groups[index], field)?;
        } else {
            let mut group = RegisterGroup {
                source_row: row.number(),
                spec: spec.into(),
                offset,
                fields: Vec::new(),
                ranges: Vec::new(),
                width: 0,
            };
            add_field(table, row, block, &mut group, field)?;
            groups.push(group);
        }
    }

    let mut registers = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    let mut occupied = Vec::<(u64, u64, String, usize)>::new();

    for group in groups {
        if group.width != REGISTER_WIDTH_BITS {
            return Err(Error::validation(
                table.sheet(),
                Some(group.source_row),
                Some("WIDTH"),
                Some(block),
                Some(&group.spec),
                format!(
                    "register fields occupy {} bits; expected {REGISTER_WIDTH_BITS}",
                    group.width
                ),
            ));
        }
        if group.width % 8 != 0 {
            return Err(Error::validation(
                table.sheet(),
                Some(group.source_row),
                Some("WIDTH"),
                Some(block),
                Some(&group.spec),
                "register width must be byte-aligned",
            ));
        }

        for (name, offset) in expand_register(table, &group, block)? {
            if let Some(previous_row) = names.insert(name.clone(), group.source_row) {
                return Err(Error::validation(
                    table.sheet(),
                    Some(group.source_row),
                    Some("REG"),
                    Some(block),
                    Some(&name),
                    format!("register name collides with the definition on row {previous_row}"),
                ));
            }

            let byte_width = group.width / 8;
            let end = offset.checked_add(byte_width).ok_or_else(|| {
                Error::validation(
                    table.sheet(),
                    Some(group.source_row),
                    Some("ADDR"),
                    Some(block),
                    Some(&name),
                    "register address overflows u64",
                )
            })?;
            if end > block_range {
                return Err(Error::validation(
                    table.sheet(),
                    Some(group.source_row),
                    Some("ADDR"),
                    Some(block),
                    Some(&name),
                    format!(
                        "register range {}..{} exceeds address block range {}",
                        format_address(offset),
                        format_address(end),
                        format_address(block_range)
                    ),
                ));
            }
            if let Some((_, _, other_name, other_row)) = occupied
                .iter()
                .find(|(other_start, other_end, _, _)| offset < *other_end && *other_start < end)
            {
                return Err(Error::validation(
                    table.sheet(),
                    Some(group.source_row),
                    Some("ADDR"),
                    Some(block),
                    Some(&name),
                    format!("address overlaps register `{other_name}` from row {other_row}"),
                ));
            }
            occupied.push((offset, end, name.clone(), group.source_row));
            let fields = fields_for_register(table, block, &group, &name)?;
            registers.push(Register::new(
                name,
                format_address(offset),
                group.width.to_string(),
                fields,
            ));
        }
    }

    Ok(registers)
}

fn parse_field(
    table: &Table,
    row: &Row,
    block: &str,
    register: &str,
) -> Result<ParsedField, Error> {
    let (name, uses_register_name) = row
        .get("FIELD")
        .map_or((register, true), |name| (name, false));
    let bit = table.require(row, "BIT", Some(block), Some(register))?;
    let width_text = table.require(row, "WIDTH", Some(block), Some(register))?;
    let attribute = table.require(row, "ATTRIBUTE", Some(block), Some(register))?;
    let reset = table.require(row, "DEFAULT", Some(block), Some(register))?;
    let description = row.get("DESCRIPTION").unwrap_or("No Description");

    let width = parse_u64(table, row, "WIDTH", width_text, Some(block), Some(register))?;
    if width == 0 {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("WIDTH"),
            Some(block),
            Some(register),
            "field width must be greater than zero",
        ));
    }

    let (msb, lsb) = parse_bit_range(table, row, block, register, bit)?;
    let bit_width = msb - lsb + 1;
    if bit_width != width {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("BIT"),
            Some(block),
            Some(register),
            format!("bit range width is {bit_width}, but WIDTH is {width}"),
        ));
    }
    if msb >= REGISTER_WIDTH_BITS {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("BIT"),
            Some(block),
            Some(register),
            format!("field exceeds {REGISTER_WIDTH_BITS}-bit register width"),
        ));
    }

    extract_access_value(attribute).map_err(|error| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("ATTRIBUTE"),
            Some(block),
            Some(register),
            error.to_string(),
        )
    })?;

    let reset_value = parse_u64(table, row, "DEFAULT", reset, Some(block), Some(register))?;
    if width < u64::BITS.into() && reset_value >= (1_u64 << width) {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("DEFAULT"),
            Some(block),
            Some(register),
            format!("reset value does not fit in {width} bits"),
        ));
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
    let end = start + width;

    if group
        .fields
        .iter()
        .any(|existing| existing.name() == field.name())
    {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("FIELD"),
            Some(block),
            Some(&group.spec),
            format!("field `{}` is duplicated", field.name()),
        ));
    }
    if let Some((_, _, other_name)) = group
        .ranges
        .iter()
        .find(|(other_start, other_end, _)| start < *other_end && *other_start < end)
    {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("BIT"),
            Some(block),
            Some(&group.spec),
            format!("field `{}` overlaps field `{other_name}`", field.name()),
        ));
    }

    group.width = group.width.checked_add(width).ok_or_else(|| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("WIDTH"),
            Some(block),
            Some(&group.spec),
            "register width overflows u64",
        )
    })?;
    group.ranges.push((start, end, field.name().into()));
    group.fields.push(field);
    Ok(())
}

fn fields_for_register(
    table: &Table,
    block: &str,
    group: &RegisterGroup,
    register: &str,
) -> Result<Vec<Field>, Error> {
    let mut fields = Vec::with_capacity(group.fields.len());
    let mut names = HashMap::<String, usize>::new();

    for parsed in &group.fields {
        let field = parsed.for_register(register);
        if let Some(previous_row) = names.insert(field.name().into(), parsed.source_row) {
            return Err(Error::validation(
                table.sheet(),
                Some(parsed.source_row),
                Some("FIELD"),
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
                Some("BIT"),
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
            Some("BIT"),
            Some(block),
            Some(register),
            message,
        )
    })?;
    let lsb = parse_literal(lsb).map_err(|message| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("BIT"),
            Some(block),
            Some(register),
            message,
        )
    })?;

    if msb < lsb {
        return Err(Error::validation(
            table.sheet(),
            Some(row.number()),
            Some("BIT"),
            Some(block),
            Some(register),
            "bit range MSB must be greater than or equal to LSB",
        ));
    }
    Ok((msb, lsb))
}

fn expand_register(
    table: &Table,
    group: &RegisterGroup,
    block: &str,
) -> Result<Vec<(String, u64)>, Error> {
    let Some((base, indexes, step)) = parse_array_spec(table, group, block)? else {
        return Ok(vec![(group.spec.clone(), group.offset)]);
    };

    indexes
        .into_iter()
        .enumerate()
        .map(|(position, index)| {
            let byte_offset = (position as u64)
                .checked_mul(step)
                .and_then(|increment| group.offset.checked_add(increment))
                .ok_or_else(|| {
                    Error::validation(
                        table.sheet(),
                        Some(group.source_row),
                        Some("ADDR"),
                        Some(block),
                        Some(&group.spec),
                        "expanded register address overflows u64",
                    )
                })?;
            Ok((format!("{base}_{index}"), byte_offset))
        })
        .collect()
}

fn parse_array_spec(
    table: &Table,
    group: &RegisterGroup,
    block: &str,
) -> Result<Option<(String, Vec<u64>, u64)>, Error> {
    let spec = group.spec.trim();
    if !spec.contains("{n}") && !spec.contains("range(") {
        return Ok(None);
    }

    let invalid = |message: String| {
        Error::validation(
            table.sheet(),
            Some(group.source_row),
            Some("REG"),
            Some(block),
            Some(&group.spec),
            message,
        )
    };
    let (base, suffix) = spec
        .split_once("{n}")
        .ok_or_else(|| invalid("array register must include the `{n}` placeholder".into()))?;
    if base.trim().is_empty() || suffix.contains("{n}") {
        return Err(invalid(
            "array register has an invalid `{n}` placeholder".into(),
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
    let (start, end, step) = match args.as_slice() {
        [end] => (0, *end, DEFAULT_ARRAY_STEP_BYTES),
        [start, end] => (*start, *end, DEFAULT_ARRAY_STEP_BYTES),
        [start, end, step] => (*start, *end, *step),
        _ => return Err(invalid("range(...) expects one to three arguments".into())),
    };
    if step == 0 {
        return Err(invalid("range(...) step must be greater than zero".into()));
    }
    if start >= end {
        return Err(invalid(
            "range(...) must produce at least one non-negative index".into(),
        ));
    }

    let mut indexes = Vec::new();
    let mut index = start;
    while index < end {
        if indexes.len() >= MAX_ARRAY_ELEMENTS {
            return Err(invalid(format!(
                "range(...) exceeds the {MAX_ARRAY_ELEMENTS}-element safety limit"
            )));
        }
        indexes.push(index);
        index = index
            .checked_add(1)
            .ok_or_else(|| invalid("range(...) index overflows u64".into()))?;
    }

    Ok(Some((base.trim().into(), indexes, step)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(rows: &[&[&str]]) -> Table {
        Table::for_test("regs", REGISTER_COLUMNS, rows)
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

        let registers = parse_registers(&table, "regs", 0x1_0000_0100).unwrap();

        assert_eq!(registers[0].name(), "reg_1");
        assert_eq!(registers[0].offset(), "0x100000000");
        assert_eq!(registers[1].name(), "reg_2");
        assert_eq!(registers[1].offset(), "0x100000004");
    }

    #[test]
    fn uses_array_step_as_register_offset() {
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

        let registers = parse_registers(&table, "regs", 0x200).unwrap();

        assert_eq!(registers[0].name(), "reg_1");
        assert_eq!(registers[0].offset(), "0x100");
        assert_eq!(registers[1].name(), "reg_2");
        assert_eq!(registers[1].offset(), "0x110");
    }

    #[test]
    fn defaults_empty_field_name_to_register_name() {
        let table = table(&[&["0", "reg", "", "[31:0]", "32", "RW", "0", ""]]);

        let registers = parse_registers(&table, "regs", 4).unwrap();

        assert_eq!(registers[0].fields()[0].name(), "reg");
    }

    #[test]
    fn defaults_empty_field_name_to_expanded_register_name() {
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

        let registers = parse_registers(&table, "regs", 12).unwrap();

        assert_eq!(registers[0].fields()[0].name(), "reg_1");
        assert_eq!(registers[1].fields()[0].name(), "reg_2");
    }

    #[test]
    fn rejects_field_name_collisions_after_register_expansion() {
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
            &["", "", "reg_1", "[15:0]", "16", "RW", "0", ""],
        ]);

        let error = parse_registers(&table, "regs", 12).unwrap_err();

        assert!(error.to_string().contains("field `reg_1` is duplicated"));
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
    fn rejects_array_steps_smaller_than_register_width() {
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
                .contains("address overlaps register `reg_0`")
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
            &["8", "reg_1", "value", "[31:0]", "32", "RW", "0", ""],
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
