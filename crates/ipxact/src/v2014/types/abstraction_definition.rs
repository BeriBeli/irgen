//! IEEE 1685-2014 abstraction-definition root document.

use serde::{Deserialize, Serialize};

use super::additional_types::{UnsignedBitVectorExpression, UnsignedPositiveIntExpression};
use super::assertions::Assertions;
use super::component::{
    BitExpression, DriveConstraint, LoadConstraint, NAMESPACE, Parameters, PortKind,
    SCHEMA_LOCATION, TimingConstraint, XSI_NAMESPACE,
};
use super::library_ref::LibraryRefType;
use super::vendor_extensions::{VendorExtensions, protect_qnames};

fn namespace() -> String {
    NAMESPACE.into()
}

fn xsi_namespace() -> String {
    XSI_NAMESPACE.into()
}

fn schema_location() -> String {
    SCHEMA_LOCATION.into()
}

/// Root element for an IEEE 1685-2014 abstraction-definition document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(
    serialize = "ipxact:abstractionDefinition",
    deserialize = "abstractionDefinition"
))]
pub struct AbstractionDefinition {
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

    #[serde(rename(serialize = "ipxact:busType", deserialize = "busType"))]
    pub bus_type: LibraryRefType,

    #[serde(
        rename(serialize = "ipxact:extends", deserialize = "extends"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extends: Option<LibraryRefType>,

    #[serde(rename(serialize = "ipxact:ports", deserialize = "ports"))]
    pub ports: AbstractionPorts,

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

impl AbstractionDefinition {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        bus_type: LibraryRefType,
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
            bus_type,
            extends: None,
            ports: AbstractionPorts::default(),
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    /// Parse an abstraction definition while preserving qualified names inside
    /// vendor extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// Logical ports defined by a bus abstraction.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractionPorts {
    #[serde(rename(serialize = "ipxact:port", deserialize = "port"), default)]
    pub port: Vec<AbstractionPort>,
}

impl AbstractionPorts {
    pub fn add(&mut self, port: AbstractionPort) {
        self.port.push(port);
    }
}

/// One logical wire or transactional abstraction port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractionPort {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:logicalName", deserialize = "logicalName"))]
    pub logical_name: String,

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

    #[serde(rename = "$value")]
    pub style: AbstractionPortStyle,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl AbstractionPort {
    pub fn wire(logical_name: impl Into<String>, wire: WireAbstraction) -> Self {
        Self {
            id: None,
            is_present: None,
            logical_name: logical_name.into(),
            display_name: None,
            description: None,
            style: AbstractionPortStyle::Wire(Box::new(wire)),
            vendor_extensions: None,
        }
    }

    pub fn transactional(
        logical_name: impl Into<String>,
        transactional: TransactionalAbstraction,
    ) -> Self {
        Self {
            id: None,
            is_present: None,
            logical_name: logical_name.into(),
            display_name: None,
            description: None,
            style: AbstractionPortStyle::Transactional(Box::new(transactional)),
            vendor_extensions: None,
        }
    }
}

/// Logical abstraction-port style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbstractionPortStyle {
    #[serde(rename(serialize = "ipxact:wire", deserialize = "wire"))]
    Wire(Box<WireAbstraction>),

    #[serde(rename(serialize = "ipxact:transactional", deserialize = "transactional"))]
    Transactional(Box<TransactionalAbstraction>),
}

/// Wire-port constraints by interface mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WireAbstraction {
    #[serde(
        rename(serialize = "ipxact:qualifier", deserialize = "qualifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub qualifier: Option<Qualifier>,

    #[serde(
        rename(serialize = "ipxact:onSystem", deserialize = "onSystem"),
        default
    )]
    pub on_system: Vec<OnSystem>,

    #[serde(
        rename(serialize = "ipxact:onMaster", deserialize = "onMaster"),
        skip_serializing_if = "Option::is_none"
    )]
    pub on_master: Option<WirePortMode>,

    #[serde(
        rename(serialize = "ipxact:onSlave", deserialize = "onSlave"),
        skip_serializing_if = "Option::is_none"
    )]
    pub on_slave: Option<WirePortMode>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub driver: Option<WirePortDriver>,
}

/// Wire qualifier flags. The XSD enforces valid flag combinations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Qualifier {
    #[serde(
        rename(serialize = "ipxact:isAddress", deserialize = "isAddress"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_address: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:isData", deserialize = "isData"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_data: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:isClock", deserialize = "isClock"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_clock: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:isReset", deserialize = "isReset"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_reset: Option<bool>,
}

/// Wire constraints for a named system-interface group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnSystem {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:group", deserialize = "group"))]
    pub group: String,

    #[serde(flatten)]
    pub mode: WirePortMode,
}

impl OnSystem {
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            id: None,
            group: group.into(),
            mode: WirePortMode::default(),
        }
    }
}

/// Wire constraints for one interface mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WirePortMode {
    #[serde(
        rename(serialize = "ipxact:presence", deserialize = "presence"),
        skip_serializing_if = "Option::is_none"
    )]
    pub presence: Option<Presence>,

    #[serde(
        rename(serialize = "ipxact:width", deserialize = "width"),
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<UnsignedPositiveIntExpression>,

    #[serde(
        rename(serialize = "ipxact:direction", deserialize = "direction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub direction: Option<Direction>,

    #[serde(
        rename(serialize = "ipxact:modeConstraints", deserialize = "modeConstraints"),
        skip_serializing_if = "Option::is_none"
    )]
    pub mode_constraints: Option<AbstractionDefPortConstraints>,

    #[serde(
        rename(
            serialize = "ipxact:mirroredModeConstraints",
            deserialize = "mirroredModeConstraints"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub mirrored_mode_constraints: Option<AbstractionDefPortConstraints>,
}

/// Constraints attached directly to an abstraction-definition wire mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractionDefPortConstraints {
    #[serde(
        rename(
            serialize = "ipxact:timingConstraint",
            deserialize = "timingConstraint"
        ),
        default
    )]
    pub timing_constraint: Vec<TimingConstraint>,

    #[serde(
        rename(serialize = "ipxact:driveConstraint", deserialize = "driveConstraint"),
        skip_serializing_if = "Option::is_none"
    )]
    pub drive_constraint: Option<DriveConstraint>,

    #[serde(
        rename(serialize = "ipxact:loadConstraint", deserialize = "loadConstraint"),
        skip_serializing_if = "Option::is_none"
    )]
    pub load_constraint: Option<LoadConstraint>,
}

/// Optional wire default value or driver requirement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WirePortDriver {
    #[serde(rename(serialize = "ipxact:defaultValue", deserialize = "defaultValue"))]
    DefaultValue(UnsignedBitVectorExpression),

    #[serde(rename(serialize = "ipxact:requiresDriver", deserialize = "requiresDriver"))]
    RequiresDriver(RequiresDriver),
}

/// Driver requirement for an input or inout wire port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiresDriver {
    #[serde(rename = "@driverType", skip_serializing_if = "Option::is_none")]
    pub driver_type: Option<DriverType>,

    #[serde(rename = "$text")]
    pub value: bool,
}

impl RequiresDriver {
    pub fn new(value: bool) -> Self {
        Self {
            driver_type: None,
            value,
        }
    }
}

/// Required wire-driver category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverType {
    #[serde(rename = "clock")]
    Clock,

    #[serde(rename = "singleShot")]
    SingleShot,

    #[serde(rename = "any")]
    Any,
}

/// Transactional-port constraints by interface mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionalAbstraction {
    #[serde(
        rename(serialize = "ipxact:qualifier", deserialize = "qualifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub qualifier: Option<TransactionalQualifier>,

    #[serde(
        rename(serialize = "ipxact:onSystem", deserialize = "onSystem"),
        default
    )]
    pub on_system: Vec<TransactionalOnSystem>,

    #[serde(
        rename(serialize = "ipxact:onMaster", deserialize = "onMaster"),
        skip_serializing_if = "Option::is_none"
    )]
    pub on_master: Option<TransactionalPortMode>,

    #[serde(
        rename(serialize = "ipxact:onSlave", deserialize = "onSlave"),
        skip_serializing_if = "Option::is_none"
    )]
    pub on_slave: Option<TransactionalPortMode>,
}

/// Transactional qualifier flags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionalQualifier {
    #[serde(
        rename(serialize = "ipxact:isAddress", deserialize = "isAddress"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_address: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:isData", deserialize = "isData"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_data: Option<bool>,
}

/// Transactional constraints for a named system-interface group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionalOnSystem {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:group", deserialize = "group"))]
    pub group: String,

    #[serde(flatten)]
    pub mode: TransactionalPortMode,
}

impl TransactionalOnSystem {
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            id: None,
            group: group.into(),
            mode: TransactionalPortMode::default(),
        }
    }
}

/// Transactional constraints for one interface mode.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionalPortMode {
    #[serde(
        rename(serialize = "ipxact:presence", deserialize = "presence"),
        skip_serializing_if = "Option::is_none"
    )]
    pub presence: Option<Presence>,

    #[serde(
        rename(serialize = "ipxact:initiative", deserialize = "initiative"),
        skip_serializing_if = "Option::is_none"
    )]
    pub initiative: Option<Initiative>,

    #[serde(
        rename(serialize = "ipxact:kind", deserialize = "kind"),
        skip_serializing_if = "Option::is_none"
    )]
    pub kind: Option<PortKind>,

    #[serde(
        rename(serialize = "ipxact:busWidth", deserialize = "busWidth"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bus_width: Option<UnsignedPositiveIntExpression>,

    #[serde(
        rename(serialize = "ipxact:protocol", deserialize = "protocol"),
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<Protocol>,
}

/// Logical-port presence requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presence {
    #[serde(rename = "$text")]
    pub value: PresenceValue,
}

impl Presence {
    pub fn new(value: PresenceValue) -> Self {
        Self { value }
    }
}

/// Legal logical-port presence values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceValue {
    #[serde(rename = "required")]
    Required,

    #[serde(rename = "illegal")]
    Illegal,

    #[serde(rename = "optional")]
    Optional,
}

/// Wire direction relative to a non-mirrored interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Direction {
    #[serde(rename = "$text")]
    pub value: DirectionValue,
}

impl Direction {
    pub fn new(value: DirectionValue) -> Self {
        Self { value }
    }
}

/// Legal wire directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectionValue {
    #[serde(rename = "in")]
    In,

    #[serde(rename = "out")]
    Out,

    #[serde(rename = "inout")]
    Inout,
}

/// Initiative of a transactional abstraction port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Initiative {
    #[serde(rename = "$text")]
    pub value: InitiativeValue,
}

impl Initiative {
    pub fn new(value: InitiativeValue) -> Self {
        Self { value }
    }
}

/// Legal transactional abstraction-port initiatives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InitiativeValue {
    #[serde(rename = "requires")]
    Requires,

    #[serde(rename = "provides")]
    Provides,

    #[serde(rename = "both")]
    Both,
}

/// Protocol attached to a transactional abstraction port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Protocol {
    #[serde(rename(serialize = "ipxact:protocolType", deserialize = "protocolType"))]
    pub protocol_type: ProtocolType,

    #[serde(
        rename(serialize = "ipxact:payload", deserialize = "payload"),
        skip_serializing_if = "Option::is_none"
    )]
    pub payload: Option<Payload>,
}

impl Protocol {
    pub fn new(protocol_type: ProtocolType) -> Self {
        Self {
            protocol_type,
            payload: None,
        }
    }
}

/// Standard or custom protocol category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolType {
    #[serde(rename = "@custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    #[serde(rename = "$text")]
    pub value: ProtocolTypeValue,
}

impl ProtocolType {
    pub fn new(value: ProtocolTypeValue) -> Self {
        Self {
            custom: None,
            value,
        }
    }
}

/// Standard protocol category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolTypeValue {
    #[serde(rename = "tlm")]
    Tlm,

    #[serde(rename = "custom")]
    Custom,
}

/// Optional transactional payload metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    #[serde(
        rename(serialize = "ipxact:name", deserialize = "name"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(rename(serialize = "ipxact:type", deserialize = "type"))]
    pub kind: PayloadType,

    #[serde(
        rename(serialize = "ipxact:extension", deserialize = "extension"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extension: Option<PayloadExtension>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Payload {
    pub fn new(kind: PayloadType) -> Self {
        Self {
            name: None,
            kind,
            extension: None,
            vendor_extensions: None,
        }
    }
}

/// Transactional payload category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadType {
    #[serde(rename = "generic")]
    Generic,

    #[serde(rename = "specific")]
    Specific,
}

/// Transactional payload extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadExtension {
    #[serde(rename = "@mandatory", skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl PayloadExtension {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            mandatory: None,
            value: value.into(),
        }
    }
}
