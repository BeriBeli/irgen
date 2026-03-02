//! IP-XACT 2014 version enum

use serde::{Deserialize, Serialize};

/// IP-XACT version enumeration for 2014 standard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IpXactVersion {
    /// IEEE 1685-2014
    Ieee1685_2014,
}

impl IpXactVersion {
    /// Get the namespace URI for this version
    pub fn namespace(&self) -> &'static str {
        match self {
            IpXactVersion::Ieee1685_2014 => {
                "http://www.accellera.org/XMLSchema/IPXACT/1685-2014"
            }
        }
    }

    /// Parse version from namespace URI
    pub fn from_namespace(ns: &str) -> Option<Self> {
        if ns.contains("1685-2014") {
            Some(IpXactVersion::Ieee1685_2014)
        } else {
            None
        }
    }
}

impl Default for IpXactVersion {
    fn default() -> Self {
        IpXactVersion::Ieee1685_2014
    }
}
