use crate::{Error, Result};

pub(crate) fn parse_u64_expr(field: &'static str, value: &str) -> Result<u64> {
    irgen_ipxact_model::parse_u64_expr(field, value).map_err(|_| Error::InvalidNumber {
        field,
        value: value.into(),
    })
}
