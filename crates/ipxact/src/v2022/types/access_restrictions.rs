//! Access restrictions type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Access restrictions container
///
/// Maps to XML schema `accessRestrictionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessRestrictions {
    #[serde(rename = "accessRestriction", default)]
    pub access_restriction: Vec<AccessRestriction>,
}

/// Individual access restriction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessRestriction {
    /// Restriction name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Lower bound
    #[serde(rename = "限定词", skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<String>,

    /// Upper bound
    #[serde(rename = "upperBound", skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<String>,
}

impl AccessRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
