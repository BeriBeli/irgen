//! IEEE 1685-2014 design root document and sub-instance connections.

use serde::{Deserialize, Serialize};

use super::assertions::Assertions;
use super::bus_definition::UnsignedIntExpression;
use super::component::{
    BitExpression, ConfigurableLibraryRef, Indices, NAMESPACE, Parameters, PortRange,
    SCHEMA_LOCATION, XSI_NAMESPACE,
};
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

/// Root element for an IEEE 1685-2014 design document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:design", deserialize = "design"))]
pub struct Design {
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

    #[serde(
        rename(
            serialize = "ipxact:componentInstances",
            deserialize = "componentInstances"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub component_instances: Option<ComponentInstances>,

    #[serde(
        rename(
            serialize = "ipxact:interconnections",
            deserialize = "interconnections"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub interconnections: Option<Interconnections>,

    #[serde(
        rename(
            serialize = "ipxact:adHocConnections",
            deserialize = "adHocConnections"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_hoc_connections: Option<AdHocConnections>,

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

impl Design {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
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
            component_instances: None,
            interconnections: None,
            ad_hoc_connections: None,
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    /// Parse a design while preserving qualified names inside vendor extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// Instances contained by a design.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstances {
    #[serde(
        rename(
            serialize = "ipxact:componentInstance",
            deserialize = "componentInstance"
        ),
        default
    )]
    pub component_instance: Vec<ComponentInstance>,
}

impl ComponentInstances {
    pub fn add(&mut self, instance: ComponentInstance) {
        self.component_instance.push(instance);
    }
}

/// One component instance in a design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:instanceName", deserialize = "instanceName"))]
    pub instance_name: String,

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
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:componentRef", deserialize = "componentRef"))]
    pub component_ref: ConfigurableLibraryRef,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ComponentInstance {
    pub fn new(instance_name: impl Into<String>, component_ref: ConfigurableLibraryRef) -> Self {
        Self {
            id: None,
            instance_name: instance_name.into(),
            display_name: None,
            description: None,
            is_present: None,
            component_ref,
            vendor_extensions: None,
        }
    }
}

/// Connections between component instances.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Interconnections {
    #[serde(rename = "$value", default)]
    pub connection: Vec<InterconnectionEntry>,
}

impl Interconnections {
    pub fn add(&mut self, connection: impl Into<InterconnectionEntry>) {
        self.connection.push(connection.into());
    }
}

/// One normal or monitor interconnection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterconnectionEntry {
    #[serde(rename(serialize = "ipxact:interconnection", deserialize = "interconnection"))]
    Interconnection(Interconnection),

    #[serde(rename(
        serialize = "ipxact:monitorInterconnection",
        deserialize = "monitorInterconnection"
    ))]
    MonitorInterconnection(MonitorInterconnection),
}

impl From<Interconnection> for InterconnectionEntry {
    fn from(value: Interconnection) -> Self {
        Self::Interconnection(value)
    }
}

impl From<MonitorInterconnection> for InterconnectionEntry {
    fn from(value: MonitorInterconnection) -> Self {
        Self::MonitorInterconnection(value)
    }
}

/// Connection between active and hierarchical interfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interconnection {
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
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:activeInterface", deserialize = "activeInterface"),
        default
    )]
    pub active_interface: Vec<ActiveInterface>,

    #[serde(
        rename(serialize = "ipxact:hierInterface", deserialize = "hierInterface"),
        default
    )]
    pub hier_interface: Vec<HierInterface>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Interconnection {
    pub fn new(name: impl Into<String>, first: ActiveInterface, second: ActiveInterface) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            active_interface: vec![first, second],
            hier_interface: Vec::new(),
            vendor_extensions: None,
        }
    }
}

/// A component bus-interface reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveInterface {
    #[serde(rename = "@componentRef")]
    pub component_ref: String,

    #[serde(rename = "@busRef")]
    pub bus_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(serialize = "ipxact:excludePorts", deserialize = "excludePorts"),
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_ports: Option<ExcludePorts>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ActiveInterface {
    pub fn new(component_ref: impl Into<String>, bus_ref: impl Into<String>) -> Self {
        Self {
            component_ref: component_ref.into(),
            bus_ref: bus_ref.into(),
            id: None,
            is_present: None,
            description: None,
            exclude_ports: None,
            vendor_extensions: None,
        }
    }
}

/// Physical ports excluded from an active interface connection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExcludePorts {
    #[serde(
        rename(serialize = "ipxact:excludePort", deserialize = "excludePort"),
        default
    )]
    pub exclude_port: Vec<ExcludePort>,
}

/// One excluded physical port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludePort {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ExcludePort {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Exported interface on the containing component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HierInterface {
    #[serde(rename = "@busRef")]
    pub bus_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl HierInterface {
    pub fn new(bus_ref: impl Into<String>) -> Self {
        Self {
            bus_ref: bus_ref.into(),
            id: None,
            is_present: None,
            description: None,
            vendor_extensions: None,
        }
    }
}

/// A fan-out connection from one active interface to monitor interfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorInterconnection {
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
    pub is_present: Option<BitExpression>,

    #[serde(rename(
        serialize = "ipxact:monitoredActiveInterface",
        deserialize = "monitoredActiveInterface"
    ))]
    pub monitored_active_interface: MonitorInterface,

    #[serde(
        rename(
            serialize = "ipxact:monitorInterface",
            deserialize = "monitorInterface"
        ),
        default
    )]
    pub monitor_interface: Vec<MonitorInterface>,
}

impl MonitorInterconnection {
    pub fn new(name: impl Into<String>, monitored_active_interface: MonitorInterface) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            monitored_active_interface,
            monitor_interface: Vec::new(),
        }
    }
}

/// Interface participating in a monitor interconnection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorInterface {
    #[serde(rename = "@componentRef")]
    pub component_ref: String,

    #[serde(rename = "@busRef")]
    pub bus_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl MonitorInterface {
    pub fn new(component_ref: impl Into<String>, bus_ref: impl Into<String>) -> Self {
        Self {
            component_ref: component_ref.into(),
            bus_ref: bus_ref.into(),
            id: None,
            path: None,
            description: None,
            vendor_extensions: None,
            is_present: None,
        }
    }
}

/// Explicit physical-port connections.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdHocConnections {
    #[serde(
        rename(serialize = "ipxact:adHocConnection", deserialize = "adHocConnection"),
        default
    )]
    pub ad_hoc_connection: Vec<AdHocConnection>,
}

impl AdHocConnections {
    pub fn add(&mut self, connection: AdHocConnection) {
        self.ad_hoc_connection.push(connection);
    }
}

/// One explicit physical-port connection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdHocConnection {
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
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:tiedValue", deserialize = "tiedValue"),
        skip_serializing_if = "Option::is_none"
    )]
    pub tied_value: Option<TiedValue>,

    #[serde(rename(serialize = "ipxact:portReferences", deserialize = "portReferences"))]
    pub port_references: PortReferences,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl AdHocConnection {
    pub fn new(name: impl Into<String>, port_references: PortReferences) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            tied_value: None,
            port_references,
            vendor_extensions: None,
        }
    }
}

/// Optional constant tied to an ad-hoc connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TiedValue {
    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "$text")]
    pub value: String,
}

impl TiedValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            minimum: None,
            maximum: None,
            extension_attributes: ExtensionAttributes::default(),
            value: value.into(),
        }
    }
}

/// Internal references precede external references in schema order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortReferences {
    #[serde(
        rename(
            serialize = "ipxact:internalPortReference",
            deserialize = "internalPortReference"
        ),
        default
    )]
    pub internal_port_reference: Vec<InternalPortReference>,

    #[serde(
        rename(
            serialize = "ipxact:externalPortReference",
            deserialize = "externalPortReference"
        ),
        default
    )]
    pub external_port_reference: Vec<ExternalPortReference>,
}

/// Port reference to a contained component instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalPortReference {
    #[serde(rename = "@componentRef")]
    pub component_ref: String,

    #[serde(rename = "@portRef")]
    pub port_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:partSelect", deserialize = "partSelect"),
        skip_serializing_if = "Option::is_none"
    )]
    pub part_select: Option<PartSelect>,
}

impl InternalPortReference {
    pub fn new(component_ref: impl Into<String>, port_ref: impl Into<String>) -> Self {
        Self {
            component_ref: component_ref.into(),
            port_ref: port_ref.into(),
            id: None,
            is_present: None,
            part_select: None,
        }
    }
}

/// Port reference exported by the containing component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalPortReference {
    #[serde(rename = "@portRef")]
    pub port_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:partSelect", deserialize = "partSelect"),
        skip_serializing_if = "Option::is_none"
    )]
    pub part_select: Option<PartSelect>,
}

impl ExternalPortReference {
    pub fn new(port_ref: impl Into<String>) -> Self {
        Self {
            port_ref: port_ref.into(),
            id: None,
            is_present: None,
            part_select: None,
        }
    }
}

/// Selection of a vector range, optionally after array indices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartSelect {
    #[serde(
        rename(serialize = "ipxact:indices", deserialize = "indices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub indices: Option<Indices>,

    #[serde(
        rename(serialize = "ipxact:range", deserialize = "range"),
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<PortRange>,
}

impl PartSelect {
    pub fn range(
        left: impl Into<UnsignedIntExpression>,
        right: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            indices: None,
            range: Some(PortRange::new(left, right)),
        }
    }
}
