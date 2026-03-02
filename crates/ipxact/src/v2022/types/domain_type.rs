//! Domain type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Domain types container
///
/// Maps to XML schema `domainTypeDefsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DomainTypes {
    #[serde(rename = "domainType", default)]
    pub domain_type: Vec<DomainType>,
}

/// Individual domain type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainType {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Clock port reference
    #[serde(rename = "clockPortReference", skip_serializing_if = "Option::is_none")]
    pub clock_port_reference: Option<String>,

    /// Reset port reference
    #[serde(rename = "resetPortReference", skip_serializing_if = "Option::is_none")]
    pub reset_port_reference: Option<String>,
}

impl DomainTypes {
    pub fn new() -> Self {
        Self::default()
    }
}
