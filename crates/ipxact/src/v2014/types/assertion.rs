//! Assertion type for IP-XACT 2014

use serde::{Deserialize, Serialize};

use super::component::BitExpression;

/// Assertion - a boolean expression that must be true
///
/// Maps to XML schema `assertionType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:assertion", deserialize = "assertion"))]
pub struct Assertion {
    /// Unique name (required)
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    /// Display name
    #[serde(
        rename(serialize = "ipxact:displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    /// Description
    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// The assertion expression (required)
    #[serde(rename(serialize = "ipxact:assert", deserialize = "assert"))]
    pub assert: BitExpression,

    /// ID attribute
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Assertion {
    pub fn new(name: impl Into<String>, assert: impl Into<BitExpression>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            assert: assert.into(),
            id: None,
        }
    }
}
