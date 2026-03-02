//! Library reference type (VLNV)

use serde::{Deserialize, Serialize};

/// Library reference type - Vendor, Library, Name, Version
///
/// Maps to XML schema `libraryRefType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryRefType {
    /// Vendor name (required)
    #[serde(rename = "vendor")]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename = "library")]
    pub library: String,

    /// Component name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version (required)
    #[serde(rename = "version")]
    pub version: String,
}

impl LibraryRefType {
    pub fn new(vendor: impl Into<String>, library: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
        }
    }
}
