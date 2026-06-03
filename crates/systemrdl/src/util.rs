use crate::ast::{Expression, HardwareAccess, PropertyAssignment, SoftwareAccess};
use crate::error::Error;

pub(crate) fn bytes_from_bits(name: &str, value: &str) -> Result<u64, Error> {
    let bits = parse_unsigned("register size", value)?;
    if bits % 8 != 0 {
        return Err(Error::UnalignedRegisterSize {
            name: name.into(),
            bits,
        });
    }
    Ok(bits / 8)
}

pub(crate) fn rdl_number(kind: &'static str, value: &str) -> Result<Expression, Error> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        let number = u64::from_str_radix(hex, 16).map_err(|_| Error::InvalidNumber {
            kind,
            value: value.into(),
        })?;
        return Ok(Expression::Number(format!("0x{number:x}")));
    }

    let number = parse_unsigned(kind, trimmed)?;
    Ok(Expression::Number(number.to_string()))
}

pub(crate) fn access_properties(attr: &str) -> Result<Vec<PropertyAssignment>, Error> {
    let (sw, onread, onwrite) = match attr.to_ascii_uppercase().as_str() {
        "RO" => (SoftwareAccess::R, None, None),
        "RW" => (SoftwareAccess::Rw, None, None),
        "WO" => (SoftwareAccess::W, None, None),
        "W1" | "WO1" => (SoftwareAccess::W1, None, None),
        "RC" => (SoftwareAccess::Rw, Some("rclr"), None),
        "RS" => (SoftwareAccess::Rw, Some("rset"), None),
        "WRC" => (SoftwareAccess::Rw, Some("rclr"), None),
        "WRS" => (SoftwareAccess::Rw, Some("rset"), None),
        "WC" | "WOC" => (SoftwareAccess::W, None, Some("wclr")),
        "WS" | "WOS" => (SoftwareAccess::W, None, Some("wset")),
        "W1C" => (SoftwareAccess::Rw, None, Some("woclr")),
        "W1S" => (SoftwareAccess::Rw, None, Some("woset")),
        "W1T" => (SoftwareAccess::Rw, None, Some("wot")),
        "W0C" => (SoftwareAccess::Rw, None, Some("wzc")),
        "W0S" => (SoftwareAccess::Rw, None, Some("wzs")),
        "W0T" => (SoftwareAccess::Rw, None, Some("wzt")),
        "WSRC" | "W1SRC" | "W0SRC" => (SoftwareAccess::Rw, Some("rclr"), None),
        "WCRS" | "W1CRS" | "W0CRS" => (SoftwareAccess::Rw, Some("rset"), None),
        _ => return Err(Error::UnsupportedAccess(attr.into())),
    };

    let mut props = vec![
        PropertyAssignment::value("sw", Expression::Identifier(sw.as_str().into())),
        PropertyAssignment::value(
            "hw",
            Expression::Identifier(HardwareAccess::R.as_str().into()),
        ),
    ];
    if let Some(onread) = onread {
        props.push(PropertyAssignment::value(
            "onread",
            Expression::Identifier(onread.into()),
        ));
    }
    if let Some(onwrite) = onwrite {
        props.push(PropertyAssignment::value(
            "onwrite",
            Expression::Identifier(onwrite.into()),
        ));
    }
    Ok(props)
}

pub(crate) fn sanitize_string(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn parse_unsigned(kind: &'static str, value: &str) -> Result<u64, Error> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|_| Error::InvalidNumber {
            kind,
            value: value.into(),
        })
}
