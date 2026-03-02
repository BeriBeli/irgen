use serde::{Deserialize, Serialize};

/// Vendor extensions container - for tool-specific extensions.
///
/// Maps to XML schema `vendorExtensionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VendorExtensions {
    /// Any XML content - stored as string for flexibility
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content: String,
}

impl VendorExtensions {
    pub fn new() -> Self {
        Self::default()
    }
}
