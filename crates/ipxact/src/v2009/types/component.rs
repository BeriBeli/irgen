use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::v2009::Parameters;

use super::address_space::AddressSpaceRef;
use super::component_containers::{AddressSpaces, BusInterfaces, Channels, Model, RemapStates};
use super::file_set::{Choices, ComponentGenerators, FileSets};
use super::vendor_extensions::VendorExtensions;
use crate::v2009::enums::ip_xact_version::IpXactVersion;

pub const NAMESPACE: &str = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009";
pub const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";
pub const SCHEMA_LOCATION: &str = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009 http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009/index.xsd";

fn namespace() -> String {
    NAMESPACE.into()
}

fn xsi_namespace() -> String {
    XSI_NAMESPACE.into()
}

fn schema_location() -> String {
    SCHEMA_LOCATION.into()
}

/// Component type - the root element of an IP-XACT component file.
///
/// Maps to XML schema `componentType` complex type.
/// This is the main entry point for parsing IP-XACT component definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "spirit:component", deserialize = "component"))]
pub struct Component {
    /// SPIRIT XML namespace.
    #[serde(rename = "@xmlns:spirit", default = "namespace")]
    pub xmlns_spirit: String,

    /// XML schema instance namespace.
    #[serde(rename = "@xmlns:xsi", default = "xsi_namespace")]
    pub xmlns_xsi: String,

    /// Schema location for official validation.
    #[serde(
        rename(serialize = "@xsi:schemaLocation", deserialize = "@schemaLocation"),
        default = "schema_location"
    )]
    pub schema_location: String,

    /// Vendor name (required)
    #[serde(rename(serialize = "spirit:vendor", deserialize = "vendor"))]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename(serialize = "spirit:library", deserialize = "library"))]
    pub library: String,

    /// Component name (required)
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    /// Version (required)
    #[serde(rename(serialize = "spirit:version", deserialize = "version"))]
    pub version: String,

    /// Bus interfaces defined in this component
    #[serde(rename = "busInterfaces", skip_serializing_if = "Option::is_none")]
    pub bus_interfaces: Option<BusInterfaces>,

    /// Channel definitions
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Channels>,

    /// Remap states
    #[serde(rename = "remapStates", skip_serializing_if = "Option::is_none")]
    pub remap_states: Option<RemapStates>,

    /// Address spaces (for bus masters)
    #[serde(rename = "addressSpaces", skip_serializing_if = "Option::is_none")]
    pub address_spaces: Option<AddressSpaces>,

    /// Memory maps (for bus slaves)
    #[serde(
        rename(serialize = "spirit:memoryMaps", deserialize = "memoryMaps"),
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maps: Option<MemoryMaps>,

    /// Model information
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,

    /// Component generators
    #[serde(
        rename = "componentGenerators",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_generators: Option<ComponentGenerators>,

    /// Choices for configurable elements
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Choices>,

    /// File sets
    #[serde(rename = "fileSets", skip_serializing_if = "Option::is_none")]
    pub file_sets: Option<FileSets>,

    /// Whitebox elements
    #[serde(rename = "whiteboxElements", skip_serializing_if = "Option::is_none")]
    pub whitebox_elements: Option<WhiteboxElements>,

    /// CPUs
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Cpus>,

    /// Other clock drivers
    #[serde(rename = "otherClockDrivers", skip_serializing_if = "Option::is_none")]
    pub other_clock_drivers: Option<OtherClocks>,

    /// Description
    #[serde(
        rename(serialize = "spirit:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,

    /// XML namespace (for version detection)
    #[serde(rename = "xmlns", skip)]
    pub xmlns: Option<String>,
}

/// Container for memory maps.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryMaps {
    /// List of memory maps.
    #[serde(
        default,
        rename(serialize = "spirit:memoryMap", deserialize = "memoryMap")
    )]
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

/// Memory map subset used for register-oriented component output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMap {
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "spirit:addressBlock", deserialize = "addressBlock"),
        default
    )]
    pub address_block: Vec<AddressBlock>,
}

impl MemoryMap {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            address_block: Vec::new(),
        }
    }

    pub fn add_address_block(&mut self, address_block: AddressBlock) {
        self.address_block.push(address_block);
    }
}

/// Address block subset used for register-oriented component output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBlock {
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "spirit:baseAddress", deserialize = "baseAddress"))]
    pub base_address: String,

    #[serde(rename(serialize = "spirit:range", deserialize = "range"))]
    pub range: String,

    #[serde(rename(serialize = "spirit:width", deserialize = "width"))]
    pub width: String,

    #[serde(
        rename(serialize = "spirit:register", deserialize = "register"),
        default
    )]
    pub register: Vec<Register>,

    #[serde(
        rename(serialize = "spirit:registerFile", deserialize = "registerFile"),
        default
    )]
    pub register_file: Vec<RegisterFile>,
}

impl AddressBlock {
    pub fn new(
        name: impl Into<String>,
        base_address: impl Into<String>,
        range: impl Into<String>,
        width: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            base_address: base_address.into(),
            range: range.into(),
            width: width.into(),
            register: Vec::new(),
            register_file: Vec::new(),
        }
    }

    pub fn add_register(&mut self, register: Register) {
        self.register.push(register);
    }

    pub fn add_register_file(&mut self, register_file: RegisterFile) {
        self.register_file.push(register_file);
    }
}

/// Register file subset used for register-oriented component output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterFile {
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "spirit:dim", deserialize = "dim"), default)]
    pub dim: Vec<String>,

    #[serde(rename(serialize = "spirit:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: String,

    #[serde(rename(serialize = "spirit:range", deserialize = "range"))]
    pub range: String,

    #[serde(
        rename(serialize = "spirit:register", deserialize = "register"),
        default
    )]
    pub register: Vec<Register>,
}

impl RegisterFile {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<String>,
        range: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            dim: Vec::new(),
            address_offset: address_offset.into(),
            range: range.into(),
            register: Vec::new(),
        }
    }

    pub fn add_register(&mut self, register: Register) {
        self.register.push(register);
    }
}

/// Register subset used for register-oriented component output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "spirit:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: String,

    #[serde(rename(serialize = "spirit:size", deserialize = "size"))]
    pub size: String,

    #[serde(rename(serialize = "spirit:field", deserialize = "field"), default)]
    pub field: Vec<Field>,
}

impl Register {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            address_offset: address_offset.into(),
            size: size.into(),
            field: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.field.push(field);
    }
}

/// Field subset used for register-oriented component output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename(serialize = "spirit:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "spirit:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename(serialize = "spirit:bitOffset", deserialize = "bitOffset"))]
    pub bit_offset: String,

    #[serde(rename(serialize = "spirit:bitWidth", deserialize = "bitWidth"))]
    pub bit_width: String,

    #[serde(
        rename(serialize = "spirit:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<String>,

    #[serde(
        rename(
            serialize = "spirit:modifiedWriteValue",
            deserialize = "modifiedWriteValue"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_write_value: Option<String>,

    #[serde(
        rename(serialize = "spirit:readAction", deserialize = "readAction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub read_action: Option<String>,
}

impl Field {
    pub fn new(
        name: impl Into<String>,
        bit_offset: impl Into<String>,
        bit_width: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: None,
            bit_offset: bit_offset.into(),
            bit_width: bit_width.into(),
            access: None,
            modified_write_value: None,
            read_action: None,
        }
    }
}

/// Container for whitebox elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElements {
    /// Whitebox element list.
    #[serde(default, rename = "whiteboxElement")]
    pub whitebox_element: Vec<WhiteboxElementType>,
}

/// Whitebox element definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElementType {
    /// Unique name.
    #[serde(rename = "name")]
    pub name: String,

    /// Optional display name.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Element type: register/signal/pin/interface.
    #[serde(rename = "whiteboxType")]
    pub whitebox_type: String,

    /// Drive capability flag.
    #[serde(rename = "driveable", skip_serializing_if = "Option::is_none")]
    pub driveable: Option<bool>,

    /// Register reference.
    #[serde(rename = "registerRef", skip_serializing_if = "Option::is_none")]
    pub register_ref: Option<String>,

    /// Optional parameters.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions.
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

/// Container for CPUs.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Cpus {
    /// CPU list.
    #[serde(default, rename = "cpu")]
    pub cpu: Vec<Cpu>,
}

/// CPU description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cpu {
    /// Unique name.
    #[serde(rename = "name")]
    pub name: String,

    /// Optional display name.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Address space references.
    #[serde(default, rename = "addressSpaceRef")]
    pub address_space_ref: Vec<AddressSpaceRef>,

    /// Optional parameters.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions.
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

/// Container for component-level non-port clocks.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OtherClocks {
    /// Other clock drivers.
    #[serde(default, rename = "otherClockDriver")]
    pub other_clock_driver: Vec<OtherClockDriver>,
}

/// Simplified other clock driver type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherClockDriver {
    /// Clock period.
    #[serde(rename = "clockPeriod", skip_serializing_if = "Option::is_none")]
    pub clock_period: Option<String>,

    /// Time until first pulse.
    #[serde(rename = "clockPulseOffset", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_offset: Option<String>,

    /// First pulse value.
    #[serde(rename = "clockPulseValue", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_value: Option<String>,

    /// First pulse duration.
    #[serde(rename = "clockPulseDuration", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_duration: Option<String>,

    /// Clock name.
    #[serde(rename = "clockName", skip_serializing_if = "Option::is_none")]
    pub clock_name: Option<String>,

    /// Clock source name.
    #[serde(rename = "clockSource", skip_serializing_if = "Option::is_none")]
    pub clock_source: Option<String>,
}

impl Component {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            xmlns_spirit: namespace(),
            xmlns_xsi: xsi_namespace(),
            schema_location: schema_location(),
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
            bus_interfaces: None,
            channels: None,
            remap_states: None,
            address_spaces: None,
            memory_maps: None,
            model: None,
            component_generators: None,
            choices: None,
            file_sets: None,
            whitebox_elements: None,
            cpus: None,
            other_clock_drivers: None,
            description: None,
            parameters: None,
            vendor_extensions: None,
            xmlns: None,
        }
    }

    /// Get the IP-XACT version from the namespace
    pub fn version(&self) -> Option<IpXactVersion> {
        self.xmlns
            .as_ref()
            .and_then(|ns| IpXactVersion::from_namespace(ns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_new() {
        let component = Component::new(
            "vendor".to_string(),
            "library".to_string(),
            "component_name".to_string(),
            "1.0".to_string(),
        );

        assert_eq!(component.vendor, "vendor");
        assert_eq!(component.library, "library");
        assert_eq!(component.name, "component_name");
        assert_eq!(component.version, "1.0");
    }

    #[test]
    fn test_component_with_description() {
        let mut component = Component::new(
            "vendor".to_string(),
            "library".to_string(),
            "component_name".to_string(),
            "1.0".to_string(),
        );
        component.description = Some("A test component".to_string());

        assert_eq!(component.description, Some("A test component".to_string()));
    }

    #[test]
    fn test_component_with_parameters() {
        let mut component = Component::new(
            "vendor".to_string(),
            "library".to_string(),
            "component_name".to_string(),
            "1.0".to_string(),
        );
        component.parameters = Some(Parameters::new());

        assert!(component.parameters.is_some());
    }

    #[test]
    fn test_component_version_from_namespace() {
        let mut component = Component::new(
            "vendor".to_string(),
            "library".to_string(),
            "component_name".to_string(),
            "1.0".to_string(),
        );
        component.xmlns =
            Some("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009".to_string());

        assert_eq!(component.version(), Some(IpXactVersion::Ieee1685_2009));
    }
}
