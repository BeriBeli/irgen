//! Configurable array bounds for IP-XACT 2014.

use serde::{Deserialize, Serialize};

use super::bus_definition::UnsignedIntExpression;

/// Multi-dimensional array bounds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableArrays {
    #[serde(rename(serialize = "ipxact:array", deserialize = "array"), default)]
    pub array: Vec<ConfigurableArray>,
}

/// Bounds of one array dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableArray {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:left", deserialize = "left"))]
    pub left: UnsignedIntExpression,

    #[serde(rename(serialize = "ipxact:right", deserialize = "right"))]
    pub right: UnsignedIntExpression,
}

impl ConfigurableArrays {
    pub fn add(&mut self, array: ConfigurableArray) {
        self.array.push(array);
    }
}

impl ConfigurableArray {
    pub fn new(
        left: impl Into<UnsignedIntExpression>,
        right: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            id: None,
            left: left.into(),
            right: right.into(),
        }
    }
}
