use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::v2009::Parameters;

use crate::v2009::enums::ip_xact_version::IpXactVersion;
use super::component_containers::{
    BusInterfaces, Channels, RemapStates, AddressSpaces, MemoryMaps, Model,
};
use super::address_space::AddressSpaceRef;
use super::file_set::{ComponentGenerators, Choices, FileSets};
use super::vendor_extensions::VendorExtensions;

/// Component type - the root element of an IP-XACT component file.
///
/// Maps to XML schema `componentType` complex type.
/// This is the main entry point for parsing IP-XACT component definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "component")]
pub struct Component {
    /// Vendor name (required)
    #[serde(rename = "vendor")]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename = "library")]
    pub library: String,

    /// Component name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version (required)
    #[serde(rename = "version")]
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
    #[serde(rename = "memoryMaps", skip_serializing_if = "Option::is_none")]
    pub memory_maps: Option<MemoryMaps>,

    /// Model information
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,

    /// Component generators
    #[serde(rename = "componentGenerators", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
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
    pub fn new(vendor: String, library: String, name: String, version: String) -> Self {
        Self {
            vendor,
            library,
            name,
            version,
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
        self.xmlns.as_ref().and_then(|ns| {
            IpXactVersion::from_namespace(ns)
        })
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
        component.xmlns = Some("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009".to_string());

        assert_eq!(component.version(), Some(IpXactVersion::Ieee1685_2009));
    }
}
