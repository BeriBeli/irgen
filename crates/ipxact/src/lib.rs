//! IEEE IP-XACT 1685 schema types.

#![recursion_limit = "512"]

pub mod error;
pub mod export;
pub mod v2022;

mod attr;

pub use error::{Error, Result};
pub use export::{ExportOptions, Standard, serialize, serialize_2022, serialize_with_options};
