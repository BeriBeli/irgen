pub(crate) fn extract_access_value(attr: &str) -> Result<String, String> {
    match attr.to_ascii_uppercase().as_str() {
        "RO" => Ok("read-only".into()),
        "RW" | "RC" | "RS" | "WRC" | "WRS" | "WSRC" | "WCRS" | "W1C" | "W1S" | "W1T" | "W0C"
        | "W0S" | "W0T" | "W1SRC" | "W1CRS" | "W0SRC" | "W0CRS" => Ok("read-write".into()),
        "WO" | "WC" | "WS" | "WOC" | "WOS" => Ok("write-only".into()),
        "W1" | "WO1" => Ok("writeOnce".into()),
        _ => Err(format!("invalid attribute: {attr}")),
    }
}
