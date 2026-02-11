use crate::error::Error;

pub fn extract_access_value(attr: &str) -> Result<String, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" => Ok("read-only".into()),
        "RW" | "RC" | "RS" | "WRC" | "WRS" | "WSRC" | "WCRS" | "W1C" | "W1S" | "W1T" | "W0C"
        | "W0S" | "W0T" | "W1SRC" | "W1CRS" | "W0SRC" | "W0CRS" => Ok("read-write".into()),
        "WO" | "WC" | "WS" | "WOC" | "WOS" => Ok("write-only".into()),
        "W1" | "WO1" => Ok("writeOnce".into()),
        _ => Err(Error::InvalidAttribute {
            attribute: attr.into(),
        }),
    }
}

pub fn extract_modified_write_value(attr: &str) -> Result<Option<String>, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" | "RW" | "RC" | "RS" | "WO" | "W1" | "WO1" => Ok(None),
        "WRC" | "W1C" | "WCRS" | "W1CRS" => Ok(Some("oneToClear".into())),
        "WRS" | "W1S" | "WSRC" | "W1SRC" => Ok(Some("oneToSet".into())),
        "W1T" => Ok(Some("oneToToggle".into())),
        "W0C" | "W0CRS" => Ok(Some("zeroToClear".into())),
        "W0S" | "W0SRC" => Ok(Some("zeroToSet".into())),
        "W0T" => Ok(Some("zeroToToggle".into())),
        "WC" | "WOC" => Ok(Some("clear".into())),
        "WS" | "WOS" => Ok(Some("set".into())),
        _ => Err(Error::InvalidAttribute {
            attribute: attr.into(),
        }),
    }
}

pub fn extract_read_action_value(attr: &str) -> Result<Option<String>, Error> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" | "RW" | "WC" | "WS" | "W1C" | "W1S" | "W1T" | "W0C" | "W0S" | "W0T" | "WO"
        | "WOC" | "WOS" | "W1" | "WO1" => Ok(None),
        "RC" | "WRC" | "WSRC" | "W1SRC" | "W0SRC" => Ok(Some("clear".into())),
        "RS" | "WRS" | "WCRS" | "W1CRS" | "W0CRS" => Ok(Some("set".into())),
        _ => Err(Error::InvalidAttribute {
            attribute: attr.into(),
        }),
    }
}
