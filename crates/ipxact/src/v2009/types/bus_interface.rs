use serde::{Deserialize, Serialize};

use crate::v2009::EndianessType;

/// Bus interface type - defines a bus interface on a component.
///
/// Maps to XML schema `busInterfaceType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusInterface {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Bus type reference (required)
    #[serde(rename = "busType")]
    pub bus_type: LibraryRef,

    /// Abstraction type reference
    #[serde(rename = "abstractionType", skip_serializing_if = "Option::is_none")]
    pub abstraction_type: Option<LibraryRef>,

    /// Master mode details.
    #[serde(rename = "master", skip_serializing_if = "Option::is_none")]
    pub master: Option<MasterDetails>,

    /// Slave mode details.
    #[serde(rename = "slave", skip_serializing_if = "Option::is_none")]
    pub slave: Option<SlaveDetails>,

    /// System mode details.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemDetails>,

    /// Mirrored slave mode details.
    #[serde(rename = "mirroredSlave", skip_serializing_if = "Option::is_none")]
    pub mirrored_slave: Option<MirroredSlave>,

    /// Mirrored master mode details.
    #[serde(rename = "mirroredMaster", skip_serializing_if = "Option::is_none")]
    pub mirrored_master: Option<MirroredMaster>,

    /// Mirrored system mode details.
    #[serde(rename = "mirroredSystem", skip_serializing_if = "Option::is_none")]
    pub mirrored_system: Option<MirroredSystem>,

    /// Monitor mode details.
    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<MonitorMode>,

    /// Connection required flag
    #[serde(rename = "connectionRequired", skip_serializing_if = "Option::is_none")]
    pub connection_required: Option<bool>,

    /// Port maps
    #[serde(rename = "portMaps", skip_serializing_if = "Option::is_none")]
    pub port_maps: Option<PortMaps>,

    /// Number of bits in least addressable unit.
    #[serde(rename = "bitsInLau", skip_serializing_if = "Option::is_none")]
    pub bits_in_lau: Option<u64>,

    /// Bit steering policy.
    #[serde(rename = "bitSteering", skip_serializing_if = "Option::is_none")]
    pub bit_steering: Option<BitSteering>,

    /// Endianness.
    #[serde(rename = "endianness", skip_serializing_if = "Option::is_none")]
    pub endianness: Option<EndianessType>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID field.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Library reference type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryRef {
    /// Vendor name
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Library name
    #[serde(rename = "library")]
    pub library: String,

    /// Name
    #[serde(rename = "name")]
    pub name: String,

    /// Version
    #[serde(rename = "version")]
    pub version: String,
}

/// Interface mode helper for programmatic construction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterfaceMode {
    /// Master mode
    Master(MasterDetails),
    /// Slave mode
    Slave(SlaveDetails),
    /// System mode
    System(SystemDetails),
    /// Mirrored master mode
    MirroredMaster(MirroredMaster),
    /// Mirrored slave mode
    MirroredSlave(MirroredSlave),
    /// Mirrored system mode
    MirroredSystem(MirroredSystem),
    /// Monitor mode
    Monitor(MonitorMode),
}

/// Master mode wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterMode {
    #[serde(rename = "master")]
    pub master: MasterDetails,
}

/// Slave mode wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlaveMode {
    #[serde(rename = "slave")]
    pub slave: SlaveDetails,
}

/// System mode wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMode {
    #[serde(rename = "system")]
    pub system: SystemDetails,
}

/// Mirrored slave details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirroredSlave {
    /// Optional memory map reference.
    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

    /// Optional base address.
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
}

/// Mirrored master details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirroredMaster {
    /// Optional address space reference.
    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<String>,

    /// Optional base address.
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
}

/// Mirrored system details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirroredSystem {
    /// System group reference.
    #[serde(rename = "group")]
    pub group: String,
}

/// Monitor mode details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorMode {
    /// Monitored interface mode.
    #[serde(rename = "interfaceMode")]
    pub interface_mode: String,

    /// Optional group for system monitoring.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Bridge entry in slave mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bridge {
    /// Referenced master bus interface.
    #[serde(rename = "masterRef")]
    pub master_ref: String,

    /// Whether the bridge is opaque.
    #[serde(rename = "opaque")]
    pub opaque: bool,
}

/// File set references grouped by function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSetRefGroup {
    /// Group name.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Referenced file sets.
    #[serde(default, rename = "fileSetRef")]
    pub file_set_ref: Vec<String>,
}

/// Master mode details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterDetails {
    /// Address space reference
    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<String>,

    /// Base address
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
}

/// Slave mode details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlaveDetails {
    /// Memory map reference
    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

    /// Optional bridges.
    #[serde(default, rename = "bridge")]
    pub bridge: Vec<Bridge>,

    /// Optional file set ref groups.
    #[serde(default, rename = "fileSetRefGroup")]
    pub file_set_ref_group: Vec<FileSetRefGroup>,

    /// Base address
    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
}

/// System mode details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemDetails {
    /// System group reference.
    #[serde(rename = "group")]
    pub group: String,
}

/// Bit steering mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitSteering {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Port maps container
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortMaps {
    /// Port map entries
    #[serde(default, rename = "portMap")]
    pub port_map: Vec<PortMap>,
}

/// Single port map entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortMap {
    /// Logical port
    #[serde(rename = "logicalPort")]
    pub logical_port: LogicalPort,

    /// Physical port
    #[serde(rename = "physicalPort")]
    pub physical_port: PhysicalPort,
}

/// Logical port definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalPort {
    /// Port name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Vector range
    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vector>,
}

/// Physical port definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalPort {
    /// Port name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Vector range
    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vector>,
}

/// Vector range
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    /// Left bound
    #[serde(rename = "left")]
    pub left: String,

    /// Right bound
    #[serde(rename = "right")]
    pub right: String,
}

impl BusInterface {
    pub fn new(name: String, bus_type: LibraryRef, mode: InterfaceMode) -> Self {
        let mut value = Self {
            name,
            display_name: None,
            description: None,
            bus_type,
            abstraction_type: None,
            master: None,
            slave: None,
            system: None,
            mirrored_slave: None,
            mirrored_master: None,
            mirrored_system: None,
            monitor: None,
            connection_required: None,
            port_maps: None,
            bits_in_lau: None,
            bit_steering: None,
            endianness: None,
            parameters: None,
            vendor_extensions: None,
            id: None,
        };

        match mode {
            InterfaceMode::Master(v) => value.master = Some(v),
            InterfaceMode::Slave(v) => value.slave = Some(v),
            InterfaceMode::System(v) => value.system = Some(v),
            InterfaceMode::MirroredMaster(v) => value.mirrored_master = Some(v),
            InterfaceMode::MirroredSlave(v) => value.mirrored_slave = Some(v),
            InterfaceMode::MirroredSystem(v) => value.mirrored_system = Some(v),
            InterfaceMode::Monitor(v) => value.monitor = Some(v),
        }

        value
    }
}

impl LibraryRef {
    pub fn new(library: String, name: String, version: String) -> Self {
        Self {
            vendor: None,
            library,
            name,
            version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_interface_new() {
        let bus_type = LibraryRef::new("amba".to_string(), "apb".to_string(), "1.0".to_string());
        let bus_iface = BusInterface::new(
            "apb_if".to_string(),
            bus_type,
            InterfaceMode::Slave(SlaveDetails {
                memory_map_ref: Some("mem_map".to_string()),
                bridge: Vec::new(),
                file_set_ref_group: Vec::new(),
                base_address: Some("0x1000".to_string()),
            }),
        );
        assert_eq!(bus_iface.name, "apb_if");
        assert!(bus_iface.slave.is_some());
    }

    #[test]
    fn test_library_ref() {
        let lib_ref = LibraryRef::new("lib".to_string(), "name".to_string(), "1.0".to_string());
        assert_eq!(lib_ref.library, "lib");
    }
}
