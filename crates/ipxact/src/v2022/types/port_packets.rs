//! Port packets type for IP-XACT 2022

use serde::{Deserialize, Serialize};

/// Port packets container
///
/// Maps to XML schema `portPacketsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortPackets {
    #[serde(rename = "portPacket", default)]
    pub port_packet: Vec<PortPacket>,
}

/// Individual port packet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortPacket {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

impl PortPackets {
    pub fn new() -> Self {
        Self::default()
    }
}
