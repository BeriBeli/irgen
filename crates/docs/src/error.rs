use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("XML text decoding error: {0}")]
    Text(#[from] quick_xml::encoding::EncodingError),
    #[error("XML attribute error: {0}")]
    Attribute(#[from] quick_xml::events::attributes::AttrError),
    #[error("unexpected XML end event `{0}`")]
    UnexpectedEnd(String),
    #[error("missing required IP-XACT element `{0}`")]
    MissingElement(&'static str),
    #[error("{kind} `{value}` is not a supported unsigned integer")]
    InvalidNumber { kind: &'static str, value: String },
    #[error("failed to render HTML template: {0}")]
    Template(#[from] askama::Error),
    #[error("failed to write HTML page: {0}")]
    WritePage(String),
    #[error("{kind} for `{name}` overflows u64")]
    AddressOverflow { kind: &'static str, name: String },
    #[error("field `{field}` bit range {msb}:{lsb} exceeds register `{register}` size {size}")]
    FieldOutOfRange {
        register: String,
        field: String,
        msb: u64,
        lsb: u64,
        size: u64,
    },
}
