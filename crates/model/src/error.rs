/// Errors produced while converting the register model into output schemas.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML serialization error: {0}")]
    XmlSe(#[from] quick_xml::SeError),

    #[error("invalid attribute: {attribute}")]
    InvalidAttribute { attribute: String },
}
