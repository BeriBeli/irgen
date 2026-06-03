//! Register definitions type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Register definitions container
///
/// Maps to XML schema `registerDefinitionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RegisterDefinitions {
    #[serde(rename = "registerDefinition", default)]
    pub register_definition: Vec<RegisterDefinition>,
}

/// Individual register definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterDefinition {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,
}

impl RegisterDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
