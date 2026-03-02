use serde::{Deserialize, Serialize};

use crate::v2009::{AccessType, EndianessType};
use crate::v2009::types::memory_map::MemoryMapEntry;

/// Address space type - defines an address space in a component.
///
/// Maps to XML schema `addressSpaceType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressSpace {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Address unit bits (default is 8)
    #[serde(rename = "addressUnitBits", skip_serializing_if = "Option::is_none")]
    pub address_unit_bits: Option<u32>,

    /// Range (required)
    #[serde(rename = "range")]
    pub range: u64,

    /// Width (required)
    #[serde(rename = "width")]
    pub width: u32,

    /// Endianness
    #[serde(rename = "endianess", skip_serializing_if = "Option::is_none")]
    pub endianess: Option<EndianessType>,

    /// Local memory map
    #[serde(rename = "localMemoryMap", skip_serializing_if = "Option::is_none")]
    pub local_memory_map: Option<LocalMemoryMap>,

    /// Banked address space
    #[serde(rename = "bankedAddressSpace", skip_serializing_if = "Option::is_none")]
    pub banked_address_space: Option<BankedAddressSpace>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Local memory map within an address space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalMemoryMap {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Memory entries (address blocks and register files)
    #[serde(default, rename = "memoryMap")]
    pub memory_map: Vec<MemoryMapEntry>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Banked address space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankedAddressSpace {
    /// Master address space
    #[serde(rename = "master")]
    pub master: AddressSpaceRef,

    /// Bank list
    #[serde(default, rename = "bank")]
    pub bank: Vec<Bank>,
}

/// Address space reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressSpaceRef {
    /// Reference value
    #[serde(rename = "$value")]
    pub value: Option<String>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Bank in a banked address space
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Base address
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,

    /// Range
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<u64>,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Access
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessType>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl AddressSpace {
    pub fn new(name: String, range: u64, width: u32) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            address_unit_bits: None,
            range,
            width,
            endianess: None,
            local_memory_map: None,
            banked_address_space: None,
            parameters: None,
            vendor_extensions: None,
            id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_space_new() {
        let aspace = AddressSpace::new("mem_space".to_string(), 4096, 32);
        assert_eq!(aspace.name, "mem_space");
        assert_eq!(aspace.range, 4096);
        assert_eq!(aspace.width, 32);
    }
}
