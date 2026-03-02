//! IP-XACT File type

use serde::{Deserialize, Serialize};

use super::library_ref::LibraryRefType;
use super::string_expression::StringURIExpression;
use super::vendor_extensions::VendorExtensions;

/// Individual IP-XACT file reference
///
/// Maps to XML schema `ipxactFileType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpxactFile {
    /// VLNV of the IP-XACT file being cataloged (required)
    #[serde(rename = "vlnv")]
    pub vlnv: LibraryRefType,

    /// Name of the IP-XACT file being cataloged (required)
    #[serde(rename = "name")]
    pub name: StringURIExpression,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl IpxactFile {
    pub fn new(vlnv: LibraryRefType, name: StringURIExpression) -> Self {
        Self {
            vlnv,
            name,
            description: None,
            vendor_extensions: None,
        }
    }
}
