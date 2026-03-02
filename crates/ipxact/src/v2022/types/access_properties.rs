//! Access properties type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Access properties type
///
/// Maps to XML schema `accessPropertiesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessPropertiesType {
    /// Read access
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,

    /// Write access
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<String>,

    /// Read-action
    #[serde(rename = "readAction", skip_serializing_if = "Option::is_none")]
    pub read_action: Option<String>,

    /// Test access
    #[serde(rename = "test", skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
}
