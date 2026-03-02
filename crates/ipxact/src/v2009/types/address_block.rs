use serde::{Deserialize, Serialize};

use crate::v2009::AccessType;
use super::vendor_extensions::VendorExtensions;

/// Address block type - defines a block of addresses in memory map.
///
/// Maps to XML schema `addressBlockType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBlock {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name for UI
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Base address (required)
    #[serde(rename = "baseAddress")]
    pub base_address: BaseAddress,

    /// Type identifier for shared definitions
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    /// Address range (required)
    #[serde(rename = "range")]
    pub range: u64,

    /// Bit width (required)
    #[serde(rename = "width")]
    pub width: u32,

    /// Usage type: memory, register, or reserved
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageType>,

    /// Volatile flag
    #[serde(rename = "volatile", skip_serializing_if = "Option::is_none")]
    pub is_volatile: Option<bool>,

    /// Access type
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessType>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Registers directly inside this block.
    #[serde(default, rename = "register")]
    pub register: Vec<super::register::Register>,

    /// Nested register files.
    #[serde(default, rename = "registerFile")]
    pub register_file: Vec<RegisterFile>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Base address value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseAddress {
    /// The address value
    #[serde(rename = "$value")]
    pub value: Option<String>,

    /// Resolve attribute
    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Dependency for dependent resolution
    #[serde(rename = "dependency", skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
}

/// Usage type for address blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UsageType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "reserved")]
    Reserved,
}

/// Register file structure nested in an address block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterFile {
    /// Unique name.
    #[serde(rename = "name")]
    pub name: String,

    /// Optional display name.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional array dimensions.
    #[serde(default, rename = "dim")]
    pub dim: Vec<u64>,

    /// Address offset from containing scope.
    #[serde(rename = "addressOffset")]
    pub address_offset: String,

    /// Type identifier.
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    /// Address range.
    #[serde(rename = "range")]
    pub range: u64,

    /// Registers in this file.
    #[serde(default, rename = "register")]
    pub register: Vec<super::register::Register>,

    /// Nested register files.
    #[serde(default, rename = "registerFile")]
    pub register_file: Vec<RegisterFile>,

    /// Parameters.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions.
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,

    /// ID field.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl AddressBlock {
    pub fn new(name: String, base_address: BaseAddress, range: u64, width: u32) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            base_address,
            type_identifier: None,
            range,
            width,
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            register: Vec::new(),
            register_file: Vec::new(),
            vendor_extensions: None,
            id: None,
        }
    }
}

impl BaseAddress {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            resolve: None,
            id: None,
            dependency: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_block_new() {
        let block = AddressBlock::new(
            "regs".to_string(),
            BaseAddress::new("0x1000"),
            256,
            32,
        );
        assert_eq!(block.name, "regs");
        assert_eq!(block.range, 256);
        assert_eq!(block.width, 32);
    }

    #[test]
    fn test_base_address_new() {
        let addr = BaseAddress::new("0x1000");
        assert_eq!(addr.value, Some("0x1000".to_string()));
    }
}
