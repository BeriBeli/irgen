//! Memory map definitions type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Memory map definitions container
///
/// Maps to XML schema `memoryMapDefinitionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryMapDefinitions {
    #[serde(rename = "memoryMapDefinition", default)]
    pub memory_map_definition: Vec<MemoryMapDefinition>,
}

/// Individual memory map definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMapDefinition {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,
}

impl MemoryMapDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
