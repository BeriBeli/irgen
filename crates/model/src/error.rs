/// Errors produced while converting the register model into output schemas.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid attribute: {attribute}")]
    InvalidAttribute { attribute: String },
}
