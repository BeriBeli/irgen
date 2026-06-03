//! Port slices type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Port slices container
///
/// Maps to XML schema `portSlicesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortSlices {
    #[serde(rename = "portSlice", default)]
    pub port_slice: Vec<PortSlice>,
}

/// Individual port slice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortSlice {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Left index
    #[serde(rename = "left", skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,

    /// Right index
    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}

impl PortSlices {
    pub fn new() -> Self {
        Self::default()
    }
}
