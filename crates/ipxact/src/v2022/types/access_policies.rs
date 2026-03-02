//! Access policies type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Access policies container
///
/// Maps to XML schema `accessPoliciesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicies {
    #[serde(rename = "accessPolicy", default)]
    pub access_policy: Vec<AccessPolicy>,
}

/// Individual access policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Access properties
    #[serde(rename = "accessProperties", skip_serializing_if = "Option::is_none")]
    pub access_properties: Option<super::AccessPropertiesType>,
}

impl AccessPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
