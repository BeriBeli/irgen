//! Bit stride type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Bit stride - defines stride in bits for vector ports
///
/// Maps to XML schema `bitStrideType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitStride {
    /// Stride value (required)
    #[serde(rename = "$value")]
    pub value: String,
}

impl BitStride {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
