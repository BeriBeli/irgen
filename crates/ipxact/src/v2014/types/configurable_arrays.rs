//! Configurable arrays type for IP-XACT 2014

use serde::{Deserialize, Serialize};

use super::vendor_extensions::VendorExtensions;

/// Configurable arrays - container for arrays of configurable elements
///
/// Maps to XML schema `configurableArraysType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableArrays {
    /// The array entries
    #[serde(rename = "configurableArray", default)]
    pub configurable_array: Vec<ConfigurableArrayEntry>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Default for ConfigurableArrays {
    fn default() -> Self {
        Self {
            configurable_array: Vec::new(),
            vendor_extensions: None,
        }
    }
}

/// Individual configurable array entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableArrayEntry {
    /// Entry name
    #[serde(rename = "entryName")]
    pub entry_name: String,

    /// Attribute name
    #[serde(rename = "attributeName")]
    pub attribute_name: String,

    /// Dimension
    #[serde(rename = "dimension")]
    pub dimension: String,
}

impl ConfigurableArrays {
    pub fn new() -> Self {
        Self::default()
    }
}
