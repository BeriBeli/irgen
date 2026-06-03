use crate::ast::Access;
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

pub(crate) fn access_from_attr(value: &str) -> Result<Access, Error> {
    match value.to_ascii_uppercase().as_str() {
        "RW" => Ok(Access::Rw),
        "RO" => Ok(Access::Ro),
        "WO" => Ok(Access::Wo),
        "W1" => Ok(Access::W1),
        "W1C" => Ok(Access::W1c),
        "RC" => Ok(Access::Rc),
        "RS" => Ok(Access::Rs),
        "WRC" => Ok(Access::Wrc),
        "WRS" => Ok(Access::Wrs),
        "WC" => Ok(Access::Wc),
        "WS" => Ok(Access::Ws),
        "WSRC" => Ok(Access::Wsrc),
        "WCRS" => Ok(Access::Wcrs),
        "W1S" => Ok(Access::W1s),
        "W1T" => Ok(Access::W1t),
        "W0C" => Ok(Access::W0c),
        "W0S" => Ok(Access::W0s),
        "W0T" => Ok(Access::W0t),
        "W1SRC" => Ok(Access::W1src),
        "W1CRS" => Ok(Access::W1crs),
        "W0SRC" => Ok(Access::W0src),
        "W0CRS" => Ok(Access::W0crs),
        "WOC" => Ok(Access::Woc),
        "WOS" => Ok(Access::Wos),
        "WO1" => Ok(Access::Wo1),
        _ => Err(Error::UnsupportedAccess(value.into())),
    }
}

pub(crate) fn ralf_number(kind: &'static str, value: &str) -> Result<String, Error> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        let number = u64::from_str_radix(hex, 16).map_err(|_| Error::InvalidNumber {
            kind,
            value: value.into(),
        })?;
        return Ok(format!("'h{number:x}"));
    }

    let number = parse_unsigned(kind, trimmed)?;
    Ok(number.to_string())
}

pub(crate) fn quote_attr_value(value: &str) -> String {
    if value.chars().any(char::is_whitespace) {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.into()
    }
}

pub(crate) fn sanitize_doc(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(['{', '}'], "()")
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
