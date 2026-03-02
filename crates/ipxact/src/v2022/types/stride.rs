//! Stride type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Stride - defines stride for register arrays
///
/// Maps to XML schema `strideType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stride {
    /// Stride value (required)
    #[serde(rename = "$value")]
    pub value: String,
}

impl Stride {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
