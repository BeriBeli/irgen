//! IEEE 1685-2014 abstractor root document.

use serde::{Deserialize, Serialize};

use super::assertions::Assertions;
use super::component::{
    AbstractionTypes, Choices, ComponentGenerator, Drivers, FileSets, NAMESPACE, Parameters,
    PortAccess, PortDirection, PortVectors, SCHEMA_LOCATION, TransactionalPort, WireTypeDefs,
    XSI_NAMESPACE,
};
use super::component_instantiation::ComponentInstantiation;
use super::library_ref::LibraryRefType;
use super::vendor_extensions::{ExtensionAttributes, VendorExtensions, protect_qnames};

fn namespace() -> String {
    NAMESPACE.into()
}

fn xsi_namespace() -> String {
    XSI_NAMESPACE.into()
}

fn schema_location() -> String {
    SCHEMA_LOCATION.into()
}

/// Root element for an IEEE 1685-2014 abstractor document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:abstractor", deserialize = "abstractor"))]
pub struct Abstractor {
    #[serde(rename = "@xmlns:ipxact", default = "namespace")]
    pub xmlns_ipxact: String,

    #[serde(rename = "@xmlns:xsi", default = "xsi_namespace")]
    pub xmlns_xsi: String,

    #[serde(
        rename(serialize = "@xsi:schemaLocation", deserialize = "@schemaLocation"),
        default = "schema_location"
    )]
    pub schema_location: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:vendor", deserialize = "vendor"))]
    pub vendor: String,

    #[serde(rename(serialize = "ipxact:library", deserialize = "library"))]
    pub library: String,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "ipxact:version", deserialize = "version"))]
    pub version: String,

    #[serde(rename(serialize = "ipxact:abstractorMode", deserialize = "abstractorMode"))]
    pub abstractor_mode: AbstractorMode,

    #[serde(rename(serialize = "ipxact:busType", deserialize = "busType"))]
    pub bus_type: LibraryRefType,

    #[serde(rename(
        serialize = "ipxact:abstractorInterfaces",
        deserialize = "abstractorInterfaces"
    ))]
    pub abstractor_interfaces: AbstractorInterfaces,

    #[serde(
        rename(serialize = "ipxact:model", deserialize = "model"),
        skip_serializing_if = "Option::is_none"
    )]
    pub model: Option<AbstractorModel>,

    #[serde(
        rename(
            serialize = "ipxact:abstractorGenerators",
            deserialize = "abstractorGenerators"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub abstractor_generators: Option<AbstractorGenerators>,

    #[serde(
        rename(serialize = "ipxact:choices", deserialize = "choices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub choices: Option<Choices>,

    #[serde(
        rename(serialize = "ipxact:fileSets", deserialize = "fileSets"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_sets: Option<FileSets>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(
        rename(serialize = "ipxact:assertions", deserialize = "assertions"),
        skip_serializing_if = "Option::is_none"
    )]
    pub assertions: Option<Assertions>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Abstractor {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        abstractor_mode: AbstractorMode,
        bus_type: LibraryRefType,
        abstractor_interfaces: AbstractorInterfaces,
    ) -> Self {
        Self {
            xmlns_ipxact: namespace(),
            xmlns_xsi: xsi_namespace(),
            schema_location: schema_location(),
            id: None,
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
            abstractor_mode,
            bus_type,
            abstractor_interfaces,
            model: None,
            abstractor_generators: None,
            choices: None,
            file_sets: None,
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    /// Parse an abstractor while preserving qualified names inside vendor
    /// extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// Mode shared by the two interfaces exposed by an abstractor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractorMode {
    #[serde(rename = "@group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    #[serde(rename = "$text")]
    pub value: AbstractorModeValue,
}

impl AbstractorMode {
    pub fn new(value: AbstractorModeValue) -> Self {
        Self { group: None, value }
    }

    pub fn system(group: impl Into<String>) -> Self {
        Self {
            group: Some(group.into()),
            value: AbstractorModeValue::System,
        }
    }
}

/// Legal abstractor interface pair modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbstractorModeValue {
    #[serde(rename = "master")]
    Master,

    #[serde(rename = "slave")]
    Slave,

    #[serde(rename = "direct")]
    Direct,

    #[serde(rename = "system")]
    System,
}

/// The two interfaces supported by an abstractor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorInterfaces {
    #[serde(rename(
        serialize = "ipxact:abstractorInterface",
        deserialize = "abstractorInterface"
    ))]
    pub abstractor_interface: [AbstractorInterface; 2],
}

impl AbstractorInterfaces {
    pub fn new(first: AbstractorInterface, second: AbstractorInterface) -> Self {
        Self {
            abstractor_interface: [first, second],
        }
    }
}

/// One side of an abstractor interface pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorInterface {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:abstractionTypes",
            deserialize = "abstractionTypes"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub abstraction_types: Option<AbstractionTypes>,

    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,
}

impl AbstractorInterface {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            abstraction_types: None,
            parameters: None,
            vendor_extensions: None,
            extension_attributes: ExtensionAttributes::default(),
        }
    }
}

/// Abstractor implementation model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorModel {
    #[serde(
        rename(serialize = "ipxact:views", deserialize = "views"),
        skip_serializing_if = "Option::is_none"
    )]
    pub views: Option<AbstractorViews>,

    #[serde(
        rename(serialize = "ipxact:instantiations", deserialize = "instantiations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub instantiations: Option<AbstractorInstantiations>,

    #[serde(
        rename(serialize = "ipxact:ports", deserialize = "ports"),
        skip_serializing_if = "Option::is_none"
    )]
    pub ports: Option<AbstractorPorts>,
}

/// Container for abstractor model views.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorViews {
    #[serde(rename(serialize = "ipxact:view", deserialize = "view"), default)]
    pub view: Vec<AbstractorView>,
}

impl AbstractorViews {
    pub fn add(&mut self, view: AbstractorView) {
        self.view.push(view);
    }
}

/// One abstractor model view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorView {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(serialize = "ipxact:envIdentifier", deserialize = "envIdentifier"),
        default
    )]
    pub env_identifier: Vec<super::component::EnvironmentIdentifier>,

    #[serde(
        rename(
            serialize = "ipxact:componentInstantiationRef",
            deserialize = "componentInstantiationRef"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub component_instantiation_ref: Option<String>,
}

impl AbstractorView {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            env_identifier: Vec::new(),
            component_instantiation_ref: None,
        }
    }
}

/// Container for component instantiations used by an abstractor.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorInstantiations {
    #[serde(
        rename(
            serialize = "ipxact:componentInstantiation",
            deserialize = "componentInstantiation"
        ),
        default
    )]
    pub component_instantiation: Vec<ComponentInstantiation>,
}

impl AbstractorInstantiations {
    pub fn add(&mut self, instantiation: ComponentInstantiation) {
        self.component_instantiation.push(instantiation);
    }
}

/// Container for abstractor physical ports.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorPorts {
    #[serde(rename(serialize = "ipxact:port", deserialize = "port"), default)]
    pub port: Vec<AbstractorPort>,
}

impl AbstractorPorts {
    pub fn add(&mut self, port: AbstractorPort) {
        self.port.push(port);
    }
}

/// Physical port used by an abstractor implementation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorPort {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<super::component::BitExpression>,

    #[serde(rename = "$value")]
    pub style: AbstractorPortStyle,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<PortAccess>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl AbstractorPort {
    pub fn new(name: impl Into<String>, style: AbstractorPortStyle) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            style,
            access: None,
            vendor_extensions: None,
        }
    }
}

/// Abstractor physical-port style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbstractorPortStyle {
    #[serde(rename(serialize = "ipxact:wire", deserialize = "wire"))]
    Wire(AbstractorWirePort),

    #[serde(rename(serialize = "ipxact:transactional", deserialize = "transactional"))]
    Transactional(Box<TransactionalPort>),
}

/// Restricted wire style used by abstractor physical ports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorWirePort {
    #[serde(rename(serialize = "ipxact:direction", deserialize = "direction"))]
    pub direction: PortDirection,

    #[serde(
        rename(serialize = "ipxact:vectors", deserialize = "vectors"),
        skip_serializing_if = "Option::is_none"
    )]
    pub vectors: Option<PortVectors>,

    #[serde(
        rename(serialize = "ipxact:wireTypeDefs", deserialize = "wireTypeDefs"),
        skip_serializing_if = "Option::is_none"
    )]
    pub wire_type_defs: Option<WireTypeDefs>,

    #[serde(
        rename(serialize = "ipxact:drivers", deserialize = "drivers"),
        skip_serializing_if = "Option::is_none"
    )]
    pub drivers: Option<Drivers>,
}

impl AbstractorWirePort {
    pub fn new(direction: PortDirection) -> Self {
        Self {
            direction,
            vectors: None,
            wire_type_defs: None,
            drivers: None,
        }
    }
}

/// Container for abstractor-local generators.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorGenerators {
    #[serde(
        rename(
            serialize = "ipxact:abstractorGenerator",
            deserialize = "abstractorGenerator"
        ),
        default
    )]
    pub abstractor_generator: Vec<ComponentGenerator>,
}

impl AbstractorGenerators {
    pub fn add(&mut self, abstractor_generator: ComponentGenerator) {
        self.abstractor_generator.push(abstractor_generator);
    }
}
