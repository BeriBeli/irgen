use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{kind} `{value}` is not a supported unsigned integer literal")]
    InvalidNumber { kind: &'static str, value: String },
    #[error("register `{name}` has bit size {bits}, which is not byte aligned")]
    UnalignedRegisterSize { name: String, bits: u64 },
    #[error("unsupported RALF access attribute `{0}`")]
    UnsupportedAccess(String),
}
