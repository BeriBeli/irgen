//! Assertion type for IP-XACT 2014

use serde::{Deserialize, Serialize};

/// Assertion - a boolean expression that must be true
///
/// Maps to XML schema `assertionType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "assertion")]
pub struct Assertion {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The assertion expression (required)
    #[serde(rename = "assert")]
    pub assert: String,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Assertion {
    pub fn new(name: String, assert: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            assert,
            id: None,
        }
    }
}
