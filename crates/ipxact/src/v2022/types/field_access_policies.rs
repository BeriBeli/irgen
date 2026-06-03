//! Field access policies type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Field access policies container
///
/// Maps to XML schema `fieldAccessPoliciesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessPolicies {
    #[serde(rename = "fieldAccessPolicy", default)]
    pub field_access_policy: Vec<FieldAccessPolicy>,
}

/// Individual field access policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessPolicy {
    /// Policy mode reference
    #[serde(rename = "modeRef", default)]
    pub mode_ref: Vec<String>,

    /// Access properties
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,

    /// Modified write value
    #[serde(rename = "modifiedWriteValue", skip_serializing_if = "Option::is_none")]
    pub modified_write_value: Option<String>,

    /// Read action
    #[serde(rename = "readAction", skip_serializing_if = "Option::is_none")]
    pub read_action: Option<String>,
}

impl FieldAccessPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
