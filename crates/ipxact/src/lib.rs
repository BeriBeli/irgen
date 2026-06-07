//! IEEE IP-XACT 1685 schema types.

#![recursion_limit = "512"]

pub mod error;
pub mod export;
pub mod v1_4;
pub mod v1_5;
pub mod v2009;
pub mod v2014;
pub mod v2022;

pub use error::{Error, Result};
pub use export::{serialize_1_4, serialize_1_5, serialize_2009, serialize_2014, serialize_2022};
