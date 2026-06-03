//! Field definitions type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Field definitions container
///
/// Maps to XML schema `fieldDefinitionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldDefinitions {
    #[serde(rename = "fieldDefinition", default)]
    pub field_definition: Vec<FieldDefinition>,
}

/// Individual field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,
}

impl FieldDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
