//! IP-XACT Files container type

use serde::{Deserialize, Serialize};

use super::ipxact_file::IpxactFile;

/// Container for a list of IP-XACT files
///
/// Maps to XML schema `ipxactFilesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IpxactFiles {
    /// List of IP-XACT files
    #[serde(rename = "ipxactFile", default)]
    pub ipxact_file: Vec<IpxactFile>,
}

impl IpxactFiles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, file: IpxactFile) {
        self.ipxact_file.push(file);
    }
}
