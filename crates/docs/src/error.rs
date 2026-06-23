use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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
    #[error(transparent)]
    Ipxact(#[from] irgen_ipxact_parser::Error),
}
