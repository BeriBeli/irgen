//! Error types for IP-XACT operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("XML parsing error: {0}")]
    Parse(String),

    #[error("XML serialization error: {0}")]
    Serialize(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value: {field} = {value}")]
    InvalidValue { field: String, value: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("Serde error: {0}")]
    Serde(String),

    #[error("Unknown IP-XACT version: {0}")]
    UnknownVersion(String),
}
