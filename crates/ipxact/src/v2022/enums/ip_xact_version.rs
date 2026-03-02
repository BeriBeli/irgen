//! IP-XACT 2022 version enum

use serde::{Deserialize, Serialize};

/// IP-XACT version enumeration for 2022 standard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IpXactVersion {
    /// IEEE 1685-2022
    Ieee1685_2022,
}

impl IpXactVersion {
    /// Get the namespace URI for this version
    pub fn namespace(&self) -> &'static str {
        match self {
            IpXactVersion::Ieee1685_2022 => {
                "http://www.accellera.org/XMLSchema/IPXACT/1685-2022"
            }
        }
    }

    /// Parse version from namespace URI
    pub fn from_namespace(ns: &str) -> Option<Self> {
        if ns.contains("1685-2022") {
            Some(IpXactVersion::Ieee1685_2022)
        } else {
            None
        }
    }
}

impl Default for IpXactVersion {
    fn default() -> Self {
        IpXactVersion::Ieee1685_2022
    }
}
