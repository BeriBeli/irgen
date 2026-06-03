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
    let (digits, radix) = split_literal(value);

    if digits.is_empty() {
        return Err(format!("invalid unsigned integer `{value}`"));
    }

    u64::from_str_radix(digits, radix)
        .map_err(|_| format!("invalid unsigned integer `{value}`; use decimal or 0x-prefixed hex"))
}

pub(crate) fn literal_fits_bits(value: &str, width: u64) -> Result<bool, String> {
    let value = value.trim();
    let (digits, radix) = split_literal(value);

    if digits.is_empty() {
        return Err(format!("invalid unsigned integer `{value}`"));
    }

    let bit_len = match radix {
        16 => hex_bit_len(value, digits)?,
        10 => decimal_bit_len(value, digits)?,
        _ => unreachable!("split_literal only returns decimal or hex"),
    };

    Ok(bit_len <= width)
}

fn split_literal(value: &str) -> (&str, u32) {
    value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .map_or((value, 10), |digits| (digits, 16))
}

fn hex_bit_len(value: &str, digits: &str) -> Result<u64, String> {
    if !digits.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!(
            "invalid unsigned integer `{value}`; use decimal or 0x-prefixed hex"
        ));
    }

    let Some(significant) = digits.trim_start_matches('0').chars().next() else {
        return Ok(0);
    };
    let significant_digits = digits.trim_start_matches('0').len() as u64;
    let leading_bits = 32 - significant.to_digit(16).unwrap().leading_zeros() as u64;

    Ok((significant_digits - 1) * 4 + leading_bits)
}

fn decimal_bit_len(value: &str, digits: &str) -> Result<u64, String> {
    if !digits.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!(
            "invalid unsigned integer `{value}`; use decimal or 0x-prefixed hex"
        ));
    }

    let significant = digits.trim_start_matches('0');
    if significant.is_empty() {
        return Ok(0);
    }

    let mut decimal = significant
        .bytes()
        .map(|digit| digit - b'0')
        .collect::<Vec<_>>();
    let mut bit_len = 0_u64;

    while !decimal.is_empty() {
        bit_len += 1;
        let mut carry = 0_u8;
        let mut quotient = Vec::with_capacity(decimal.len());

        for digit in decimal {
            let value = carry * 10 + digit;
            let next_digit = value / 2;
            carry = value % 2;
            if next_digit != 0 || !quotient.is_empty() {
                quotient.push(next_digit);
            }
        }

        decimal = quotient;
    }

    Ok(bit_len)
}

pub(crate) fn format_address(value: u64) -> String {
    format!("0x{value:X}")
}
