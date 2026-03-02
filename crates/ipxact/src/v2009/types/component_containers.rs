use serde::{Deserialize, Serialize};

use crate::v2009::types::memory_map::MemoryMap;
use crate::v2009::types::{AddressSpace, BusInterface};

/// Container for bus interfaces
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusInterfaces {
    /// List of bus interfaces
    #[serde(default, rename = "busInterface")]
    pub bus_interface: Vec<BusInterface>,
}

impl BusInterfaces {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, bus_interface: BusInterface) {
        self.bus_interface.push(bus_interface);
    }
}

/// Container for memory maps
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryMaps {
    /// List of memory maps
    #[serde(default, rename = "memoryMap")]
    pub memory_map: Vec<MemoryMap>,
}

impl MemoryMaps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, memory_map: MemoryMap) {
        self.memory_map.push(memory_map);
    }
}

/// Container for address spaces
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AddressSpaces {
    /// List of address spaces
    #[serde(default, rename = "addressSpace")]
    pub address_space: Vec<AddressSpace>,
}

impl AddressSpaces {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, address_space: AddressSpace) {
        self.address_space.push(address_space);
    }
}

/// Container for channels
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Channels {
    /// List of channels
    #[serde(default, rename = "channel")]
    pub channel: Vec<Channel>,
}

/// Single channel definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Container for remap states
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemapStates {
    /// List of remap states
    #[serde(default, rename = "remapState")]
    pub remap_state: Vec<RemapState>,
}

/// Single remap state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemapState {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Model type - contains view, port, and bus definitions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Model {
    /// Views
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Views>,

    /// Ports
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Ports>,

    /// Bus interfaces
    #[serde(rename = "busInterfaces", skip_serializing_if = "Option::is_none")]
    pub bus_interfaces: Option<BusInterfaces>,

    /// Model connections
    #[serde(rename = "modelConnections", skip_serializing_if = "Option::is_none")]
    pub model_connections: Option<ModelConnections>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Container for views
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Views {
    /// List of views
    #[serde(default, rename = "view")]
    pub view: Vec<View>,
}

/// View definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Environment identifier
    #[serde(rename = "envIdentifier", skip_serializing_if = "Option::is_none")]
    pub env_identifier: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Container for ports
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    /// List of ports
    #[serde(default, rename = "port")]
    pub port: Vec<Port>,
}

/// Port definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Wire ports
    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<WirePort>,

    /// Transactional ports
    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<TransactionalPort>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Wire port definition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WirePort {
    /// Vector
    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<VectorDef>,

    /// Wire type references
    #[serde(rename = "wireTypeDefs", skip_serializing_if = "Option::is_none")]
    pub wire_type_defs: Option<WireTypeDefs>,

    /// Driver
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<Driver>,
}

/// Vector definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorDef {
    /// Left bound
    #[serde(rename = "left")]
    pub left: String,

    /// Right bound
    #[serde(rename = "right")]
    pub right: String,
}

/// Wire type definitions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WireTypeDefs {
    /// List of wire type refs
    #[serde(default, rename = "wireTypeDef")]
    pub wire_type_def: Vec<WireTypeDef>,
}

/// Wire type definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WireTypeDef {
    /// Type name (required)
    #[serde(rename = "typeName")]
    pub type_name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Driver definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Driver {
    /// Queue
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,

    /// Value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Transactional port definition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionalPort {
    /// Transaction type
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionalType>,

    /// Protocol
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

/// Transactional type
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionalType {
    /// Type name
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Protocol definition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Protocol {
    /// Protocol type
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<ProtocolType>,
}

/// Protocol type
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProtocolType {
    /// Type name
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// Model connections
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelConnections {
    /// Active interface connections
    #[serde(rename = "activeInterfaceConnection", skip_serializing_if = "Option::is_none")]
    pub active_interface_connection: Option<Vec<InterfaceConnection>>,
}

/// Interface connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceConnection {
    /// Interface ref (required)
    #[serde(rename = "interfaceRef")]
    pub interface_ref: String,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_interfaces() {
        let bis = BusInterfaces::new();
        assert!(bis.bus_interface.is_empty());
    }

    #[test]
    fn test_memory_maps() {
        let mms = MemoryMaps::new();
        assert!(mms.memory_map.is_empty());
    }

    #[test]
    fn test_address_spaces() {
        let ass = AddressSpaces::new();
        assert!(ass.address_space.is_empty());
    }
}
