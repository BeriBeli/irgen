use std::collections::HashMap;

use irgen_model::base::{Block, Component, Register, RegisterFile};

use crate::config::SnapsheetConfig;
use crate::error::{Error, ValidationIssue};
use crate::excel::Table;
use crate::number::{format_address, parse_literal, parse_u64};

pub(crate) fn parse_component(
    config: &SnapsheetConfig,
    table: &Table,
    blocks: Vec<Block>,
) -> Result<Component, Error> {
    let columns = &config.columns.version;
    table.require_columns(&columns.required())?;
    let row = table.rows().first().ok_or_else(|| {
        Error::validation(
            table.sheet(),
            None,
            None,
            None,
            None,
            "worksheet has no component row",
        )
    })?;

    Ok(Component::new(
        table.require(row, &columns.vendor, None, None)?.into(),
        table.require(row, &columns.library, None, None)?.into(),
        table.require(row, &columns.name, None, None)?.into(),
        table.require(row, &columns.version, None, None)?.into(),
        blocks,
    ))
}

pub(crate) fn parse_blocks<F>(
    config: &SnapsheetConfig,
    table: &Table,
    mut registers_for: F,
) -> Result<Vec<Block>, Error>
where
    F: FnMut(&str, u64) -> Result<(Vec<Register>, Vec<RegisterFile>), Error>,
{
    let columns = &config.columns.address_block;
    table.require_columns(&columns.required())?;

    let mut blocks = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    let mut occupied = Vec::<(u64, u64, String, usize)>::new();
    let mut issues = Vec::<ValidationIssue>::new();

    for row in table.rows() {
        let name = match table.require(row, &columns.name, None, None) {
            Ok(name) => name,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        let offset_text = match table.require(row, &columns.offset, Some(name), None) {
            Ok(offset_text) => offset_text,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        let range_text = match table.require(row, &columns.range, Some(name), None) {
            Ok(range_text) => range_text,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        let offset = match parse_u64(table, row, &columns.offset, offset_text, Some(name), None) {
            Ok(offset) => offset,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        let range = match parse_u64(table, row, &columns.range, range_text, Some(name), None) {
            Ok(range) => range,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };

        if config.validation.reject_duplicate_blocks
            && let Some(previous_row) = names.get(name).copied()
        {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.name),
                    Some(name),
                    None,
                    format!(
                        "address block name collides with the definition on row {previous_row}"
                    ),
                ),
            )?;
            continue;
        }
        names.insert(name.into(), row.number());
        if range == 0 {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.range),
                    Some(name),
                    None,
                    "address block range must be greater than zero",
                ),
            )?;
            continue;
        }

        let Some(end) = offset.checked_add(range) else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.range),
                    Some(name),
                    None,
                    "address block range overflows u64",
                ),
            )?;
            continue;
        };
        let (registers, register_files) = match registers_for(name, range) {
            Ok(registers) => registers,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        if registers.is_empty() && register_files.is_empty() {
            continue;
        }
        let actual_range = match actual_block_range(&registers, &register_files) {
            Ok(actual_range) => actual_range,
            Err(message) => {
                collect_validation(
                    &mut issues,
                    Error::validation(
                        table.sheet(),
                        Some(row.number()),
                        Some(&columns.range),
                        Some(name),
                        None,
                        message,
                    ),
                )?;
                continue;
            }
        };
        let Some(actual_end) = offset.checked_add(actual_range) else {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.range),
                    Some(name),
                    None,
                    "address block range overflows u64",
                ),
            )?;
            continue;
        };
        if actual_end > end {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.range),
                    Some(name),
                    None,
                    format!(
                        "actual address block range {} exceeds declared range {}",
                        format_address(actual_range),
                        format_address(range)
                    ),
                ),
            )?;
            continue;
        }
        if config.validation.reject_overlapping_blocks
            && let Some((_, _, other_name, other_row)) =
                occupied.iter().find(|(other_start, other_end, _, _)| {
                    offset < *other_end && *other_start < actual_end
                })
        {
            collect_validation(
                &mut issues,
                Error::validation(
                    table.sheet(),
                    Some(row.number()),
                    Some(&columns.offset),
                    Some(name),
                    None,
                    format!("address block overlaps `{other_name}` from row {other_row}"),
                ),
            )?;
            continue;
        }
        occupied.push((offset, actual_end, name.into(), row.number()));
        blocks.push(Block::new_with_register_files(
            name.into(),
            format_address(offset),
            format_address(actual_range),
            "32".into(),
            registers,
            register_files,
        ));
    }

    if !issues.is_empty() {
        return Err(Error::validation_issues(issues));
    }

    Ok(blocks)
}

fn actual_block_range(
    registers: &[Register],
    register_files: &[RegisterFile],
) -> Result<u64, String> {
    let mut end = 0_u64;

    for register in registers {
        end = end.max(register_end(register)?);
    }

    for register_file in register_files {
        end = end.max(register_file_end(register_file)?);
    }

    Ok(end)
}

fn register_file_end(register_file: &RegisterFile) -> Result<u64, String> {
    let offset = parse_literal(register_file.offset())?;
    let stride = parse_literal(register_file.range())?;
    let dim = parse_literal(register_file.dim())?;
    let mut child_range = 0_u64;
    for register in register_file.regs() {
        child_range = child_range.max(register_end(register)?);
    }
    let last_element_offset = dim
        .checked_sub(1)
        .and_then(|last_index| last_index.checked_mul(stride))
        .ok_or_else(|| "register file range overflows u64".to_string())?;
    let total_range = last_element_offset
        .checked_add(child_range)
        .ok_or_else(|| "register file range overflows u64".to_string())?;

    offset
        .checked_add(total_range)
        .ok_or_else(|| "register file range overflows u64".to_string())
}

fn register_end(register: &Register) -> Result<u64, String> {
    let offset = parse_literal(register.offset())?;
    let bits = parse_literal(register.size())?;
    let bytes = bits / 8;

    offset
        .checked_add(bytes)
        .ok_or_else(|| "register address overflows u64".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SnapsheetConfig;
    use irgen_model::base::Field;

    const BLOCK_COLUMNS: &[&str] = &["BLOCK", "OFFSET", "RANGE"];

    #[test]
    fn rejects_overlapping_address_blocks() {
        let table = Table::for_test(
            "address_map",
            BLOCK_COLUMNS,
            &[
                &["first", "0x1000", "0x100"],
                &["second", "0x1080", "0x100"],
            ],
        );

        let error = parse_blocks(&SnapsheetConfig::default(), &table, |block, _| {
            if block == "first" {
                Ok((vec![test_register_at(block, "0x90", "32")], vec![]))
            } else {
                Ok((vec![test_register(block)], vec![]))
            }
        })
        .unwrap_err();

        assert!(error.to_string().contains("address block overlaps `first`"));
    }

    #[test]
    fn omits_empty_address_blocks() {
        let table = Table::for_test(
            "address_map",
            BLOCK_COLUMNS,
            &[&["empty", "0x0", "0x100"], &["regs", "0x100", "0x100"]],
        );

        let blocks = parse_blocks(&SnapsheetConfig::default(), &table, |block, _| {
            if block == "empty" {
                Ok((vec![], vec![]))
            } else {
                Ok((vec![test_register("status")], vec![]))
            }
        })
        .unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].name(), "regs");
    }

    #[test]
    fn contracts_address_block_range_to_used_registers() {
        let table = Table::for_test("address_map", BLOCK_COLUMNS, &[&["regs", "0x0", "0x100"]]);

        let blocks = parse_blocks(&SnapsheetConfig::default(), &table, |_, _| {
            Ok((vec![test_register_at("status", "0x10", "32")], vec![]))
        })
        .unwrap();

        assert_eq!(blocks[0].range(), "0x14");
    }

    #[test]
    fn contracts_address_block_range_to_used_register_file_tail() {
        let table = Table::for_test("address_map", BLOCK_COLUMNS, &[&["regs", "0x0", "0x20000"]]);

        let blocks = parse_blocks(&SnapsheetConfig::default(), &table, |_, _| {
            Ok((
                vec![],
                vec![RegisterFile::new(
                    "regfile_0".into(),
                    "0x10".into(),
                    "0x100".into(),
                    "512".into(),
                    vec![test_register_at("last", "0xE8", "32")],
                )],
            ))
        })
        .unwrap();

        assert_eq!(blocks[0].range(), "0x1FFFC");
        assert_eq!(blocks[0].register_files()[0].range(), "0x100");
    }

    #[test]
    fn checks_address_block_overlap_against_used_range() {
        let table = Table::for_test(
            "address_map",
            BLOCK_COLUMNS,
            &[&["first", "0x0", "0x100"], &["second", "0x20", "0x100"]],
        );

        let blocks = parse_blocks(&SnapsheetConfig::default(), &table, |block, _| {
            if block == "first" {
                Ok((vec![test_register_at("first", "0x0", "32")], vec![]))
            } else {
                Ok((vec![test_register_at("second", "0x0", "32")], vec![]))
            }
        })
        .unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].range(), "0x4");
    }

    fn test_register(name: &str) -> Register {
        test_register_at(name, "0x0", "32")
    }

    fn test_register_at(name: &str, offset: &str, size: &str) -> Register {
        Register::new(
            name.into(),
            offset.into(),
            size.into(),
            vec![Field::new(
                "ready".into(),
                "0".into(),
                "1".into(),
                "RO".into(),
                "0".into(),
                String::new(),
            )],
        )
    }
}
