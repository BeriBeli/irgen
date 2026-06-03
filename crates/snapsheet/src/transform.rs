use std::collections::HashMap;

use irgen_model::base::{Block, Component, Register, RegisterFile};

use crate::config::SnapsheetConfig;
use crate::error::{Error, ValidationIssue};
use crate::excel::Table;
use crate::number::{format_address, parse_u64};

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
        if config.validation.reject_overlapping_blocks
            && let Some((_, _, other_name, other_row)) = occupied
                .iter()
                .find(|(other_start, other_end, _, _)| offset < *other_end && *other_start < end)
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

        let (registers, register_files) = match registers_for(name, range) {
            Ok(registers) => registers,
            Err(error) => {
                collect_validation(&mut issues, error)?;
                continue;
            }
        };
        occupied.push((offset, end, name.into(), row.number()));
        blocks.push(Block::new_with_register_files(
            name.into(),
            format_address(offset),
            format_address(range),
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

        let error = parse_blocks(&SnapsheetConfig::default(), &table, |_, _| {
            Ok((vec![], vec![]))
        })
        .unwrap_err();

        assert!(error.to_string().contains("address block overlaps `first`"));
    }
}
