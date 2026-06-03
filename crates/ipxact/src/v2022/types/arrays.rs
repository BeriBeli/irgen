//! Arrays type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Arrays container
///
/// Maps to XML schema `arraysType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Arrays {
    #[serde(rename = "array", default)]
    pub array: Vec<Array>,
}

/// Individual array
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Array {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Dimension
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
}

impl Arrays {
    pub fn new() -> Self {
        Self::default()
    }
}
