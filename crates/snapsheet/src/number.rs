use crate::error::Error;
use crate::excel::{Row, Table};

pub(crate) fn parse_u64(
    table: &Table,
    row: &Row,
    column: &str,
    value: &str,
    block: Option<&str>,
    register: Option<&str>,
) -> Result<u64, Error> {
    parse_literal(value).map_err(|message| {
        Error::validation(
            table.sheet(),
            Some(row.number()),
            Some(column),
            block,
            register,
            message,
        )
    })
}

pub(crate) fn parse_literal(value: &str) -> Result<u64, String> {
    let value = value.trim();
    let (digits, radix) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .map_or((value, 10), |digits| (digits, 16));

    if digits.is_empty() {
        return Err(format!("invalid unsigned integer `{value}`"));
    }

    u64::from_str_radix(digits, radix)
        .map_err(|_| format!("invalid unsigned integer `{value}`; use decimal or 0x-prefixed hex"))
}

pub(crate) fn format_address(value: u64) -> String {
    format!("0x{value:X}")
}
