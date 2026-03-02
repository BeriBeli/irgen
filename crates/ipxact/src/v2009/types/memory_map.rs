use serde::{Deserialize, Serialize};

use crate::v2009::types::address_block::BaseAddress;

/// Memory map type - defines a memory map in a component.
///
/// Maps to XML schema `memoryMapType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMap {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name for UI
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Memory map entries (address blocks, banks, subspace maps)
    #[serde(default, rename = "memoryMap")]
    pub memory_map: Vec<MemoryMapEntry>,

    /// Address unit bits (default is byte addressable = 8)
    #[serde(rename = "addressUnitBits", skip_serializing_if = "Option::is_none")]
    pub address_unit_bits: Option<u32>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::vendor_extensions::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Memory map entry - can be an address block, bank, or subspace reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MemoryMapEntry {
    /// Address block entry
    AddressBlock(super::address_block::AddressBlock),
    /// Bank entry
    Bank(Bank),
    /// Subspace reference
    SubspaceMap(SubspaceMap),
}

/// Bank type - grouped address blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name for UI
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Base address
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<BaseAddress>,

    /// Range
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<u64>,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::vendor_extensions::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Subspace map reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubspaceMap {
    /// Reference to address space
    #[serde(rename = "addressSpaceRef")]
    pub address_space_ref: String,

    /// Base address
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<BaseAddress>,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::vendor_extensions::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl MemoryMap {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            memory_map: Vec::new(),
            address_unit_bits: None,
            vendor_extensions: None,
            id: None,
        }
    }

    pub fn with_address_unit_bits(mut self, bits: u32) -> Self {
        self.address_unit_bits = Some(bits);
        self
    }

    pub fn add_entry(&mut self, entry: MemoryMapEntry) {
        self.memory_map.push(entry);
    }
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            base_address: None,
            range: None,
            width: None,
            vendor_extensions: None,
            id: None,
        }
    }
}

impl SubspaceMap {
    pub fn new(address_space_ref: String) -> Self {
        Self {
            address_space_ref,
            base_address: None,
            display_name: None,
            description: None,
            vendor_extensions: None,
            id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_map_new() {
        let map = MemoryMap::new("test_map".to_string());
        assert_eq!(map.name, "test_map");
        assert!(map.memory_map.is_empty());
    }

    #[test]
    fn test_memory_map_with_bits() {
        let map = MemoryMap::new("test_map".to_string())
            .with_address_unit_bits(8);
        assert_eq!(map.address_unit_bits, Some(8));
    }

    #[test]
    fn test_memory_map_entry() {
        let block = super::super::address_block::AddressBlock::new(
            "regs".to_string(),
            BaseAddress::new("0x1000"),
            256,
            32,
        );
        let entry = MemoryMapEntry::AddressBlock(block);
        assert!(matches!(entry, MemoryMapEntry::AddressBlock(_)));
    }
}
