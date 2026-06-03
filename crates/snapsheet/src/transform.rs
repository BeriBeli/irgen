use std::collections::HashMap;

use irgen_model::base::{Block, Component, Register};

use crate::error::Error;
use crate::excel::Table;
use crate::number::{format_address, parse_u64};

const VERSION_COLUMNS: &[&str] = &["VENDOR", "LIBRARY", "NAME", "VERSION"];
const BLOCK_COLUMNS: &[&str] = &["BLOCK", "OFFSET", "RANGE"];

pub(crate) fn parse_component(table: &Table, blocks: Vec<Block>) -> Result<Component, Error> {
    table.require_columns(VERSION_COLUMNS)?;
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
        table.require(row, "VENDOR", None, None)?.into(),
        table.require(row, "LIBRARY", None, None)?.into(),
        table.require(row, "NAME", None, None)?.into(),
        table.require(row, "VERSION", None, None)?.into(),
        blocks,
    ))
}

pub(crate) fn parse_blocks<F>(table: &Table, mut registers_for: F) -> Result<Vec<Block>, Error>
where
    F: FnMut(&str, u64) -> Result<Vec<Register>, Error>,
{
    table.require_columns(BLOCK_COLUMNS)?;

    let mut blocks = Vec::new();
    let mut names = HashMap::<String, usize>::new();
    let mut occupied = Vec::<(u64, u64, String, usize)>::new();

    for row in table.rows() {
        let name = table.require(row, "BLOCK", None, None)?;
        let offset_text = table.require(row, "OFFSET", Some(name), None)?;
        let range_text = table.require(row, "RANGE", Some(name), None)?;
        let offset = parse_u64(table, row, "OFFSET", offset_text, Some(name), None)?;
        let range = parse_u64(table, row, "RANGE", range_text, Some(name), None)?;

        if let Some(previous_row) = names.insert(name.into(), row.number()) {
            return Err(Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("BLOCK"),
                Some(name),
                None,
                format!("address block name collides with the definition on row {previous_row}"),
            ));
        }
        if range == 0 {
            return Err(Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("RANGE"),
                Some(name),
                None,
                "address block range must be greater than zero",
            ));
        }

        let end = offset.checked_add(range).ok_or_else(|| {
            Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("RANGE"),
                Some(name),
                None,
                "address block range overflows u64",
            )
        })?;
        if let Some((_, _, other_name, other_row)) = occupied
            .iter()
            .find(|(other_start, other_end, _, _)| offset < *other_end && *other_start < end)
        {
            return Err(Error::validation(
                table.sheet(),
                Some(row.number()),
                Some("OFFSET"),
                Some(name),
                None,
                format!("address block overlaps `{other_name}` from row {other_row}"),
            ));
        }

        let registers = registers_for(name, range)?;
        occupied.push((offset, end, name.into(), row.number()));
        blocks.push(Block::new(
            name.into(),
            format_address(offset),
            format_address(range),
            "32".into(),
            registers,
        ));
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let error = parse_blocks(&table, |_, _| Ok(vec![])).unwrap_err();

        assert!(error.to_string().contains("address block overlaps `first`"));
    }
}
