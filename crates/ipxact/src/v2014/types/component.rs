//! IEEE 1685-2014 component types for register-oriented memory maps.

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::abstraction_definition::Protocol;
use super::additional_types::{
    SignedLongintExpression, UnsignedBitVectorExpression, UnsignedLongintExpression,
    UnsignedPositiveIntExpression, UnsignedPositiveLongintExpression,
};
use super::assertions::Assertions;
use super::bus_definition::UnsignedIntExpression;
use super::component_instantiation::{ComponentInstantiation, ModuleParameter};
use super::configurable_arrays::ConfigurableArrays;
use super::string_expression::{StringExpression, StringURIExpression};
use super::vendor_extensions::{ExtensionAttributes, VendorExtensions, protect_qnames};

pub const NAMESPACE: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014";
pub const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";
pub const SCHEMA_LOCATION: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014 http://www.accellera.org/XMLSchema/IPXACT/1685-2014/index.xsd";

/// Transactional port type parameter, backed by the schema's moduleParameterType.
pub type TypeParameter = ModuleParameter;

fn namespace() -> String {
    NAMESPACE.into()
}

fn xsi_namespace() -> String {
    XSI_NAMESPACE.into()
}

fn schema_location() -> String {
    SCHEMA_LOCATION.into()
}

/// Root element for an IEEE 1685-2014 component document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:component", deserialize = "component"))]
pub struct Component {
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
        rename(serialize = "ipxact:busInterfaces", deserialize = "busInterfaces"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bus_interfaces: Option<BusInterfaces>,

    #[serde(
        rename(
            serialize = "ipxact:indirectInterfaces",
            deserialize = "indirectInterfaces"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub indirect_interfaces: Option<IndirectInterfaces>,

    #[serde(
        rename(serialize = "ipxact:channels", deserialize = "channels"),
        skip_serializing_if = "Option::is_none"
    )]
    pub channels: Option<Channels>,

    #[serde(
        rename(serialize = "ipxact:remapStates", deserialize = "remapStates"),
        skip_serializing_if = "Option::is_none"
    )]
    pub remap_states: Option<RemapStates>,

    #[serde(
        rename(serialize = "ipxact:addressSpaces", deserialize = "addressSpaces"),
        skip_serializing_if = "Option::is_none"
    )]
    pub address_spaces: Option<AddressSpaces>,

    #[serde(
        rename(serialize = "ipxact:memoryMaps", deserialize = "memoryMaps"),
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maps: Option<MemoryMaps>,

    #[serde(
        rename(serialize = "ipxact:model", deserialize = "model"),
        skip_serializing_if = "Option::is_none"
    )]
    pub model: Option<Model>,

    #[serde(
        rename(
            serialize = "ipxact:componentGenerators",
            deserialize = "componentGenerators"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub component_generators: Option<ComponentGenerators>,

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
        rename(
            serialize = "ipxact:whiteboxElements",
            deserialize = "whiteboxElements"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub whitebox_elements: Option<WhiteboxElements>,

    #[serde(
        rename(serialize = "ipxact:cpus", deserialize = "cpus"),
        skip_serializing_if = "Option::is_none"
    )]
    pub cpus: Option<Cpus>,

    #[serde(
        rename(
            serialize = "ipxact:otherClockDrivers",
            deserialize = "otherClockDrivers"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub other_clock_drivers: Option<OtherClockDrivers>,

    #[serde(
        rename(serialize = "ipxact:resetTypes", deserialize = "resetTypes"),
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_types: Option<ResetTypes>,

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

impl Component {
    /// Parse a component while preserving qualified names inside vendor
    /// extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let bus_interface_extension_attributes = collect_bus_interface_extension_attributes(xml)?;
        let xml = protect_qnames(xml)?;
        let mut component: Self = quick_xml::de::from_str(&xml)
            .map_err(|error| crate::Error::Parse(error.to_string()))?;
        if let Some(bus_interfaces) = &mut component.bus_interfaces {
            for (bus_interface, extension_attributes) in bus_interfaces
                .bus_interface
                .iter_mut()
                .zip(bus_interface_extension_attributes)
            {
                bus_interface.extension_attributes = extension_attributes;
            }
        }
        Ok(component)
    }

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
            bus_interfaces: None,
            indirect_interfaces: None,
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
            reset_types: None,
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }
}

fn collect_bus_interface_extension_attributes(
    xml: &str,
) -> crate::Result<Vec<ExtensionAttributes>> {
    let mut reader = Reader::from_str(xml);
    let mut attributes = Vec::new();

    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start)
                if start.local_name().as_ref() == b"busInterface" =>
            {
                attributes.push(extension_attributes_from_start(&start)?);
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(attributes)
}

fn extension_attributes_from_start(start: &BytesStart<'_>) -> crate::Result<ExtensionAttributes> {
    let decoder = start.decoder();
    let mut extension_attributes = ExtensionAttributes::default();

    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| crate::Error::Parse(error.to_string()))?;
        let name = decoder
            .decode(attribute.key.as_ref())
            .map_err(|error| crate::Error::Parse(error.to_string()))?
            .into_owned();
        if !is_any_attribute_name(&name) {
            continue;
        }
        let value = attribute
            .decode_and_unescape_value(decoder)
            .map_err(|error| crate::Error::Parse(error.to_string()))?
            .into_owned();
        extension_attributes.insert(name, value);
    }

    Ok(extension_attributes)
}

fn is_any_attribute_name(name: &str) -> bool {
    name == "xmlns"
        || name.starts_with("xmlns:")
        || (name.contains(':')
            && !name.starts_with("xml:")
            && !name.starts_with("xsi:")
            && !name.starts_with("ipxact:"))
}

/// Numeric expression used by memory-map elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumericExpression {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
}

impl NumericExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            minimum: None,
            maximum: None,
        }
    }
}

impl From<&str> for NumericExpression {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NumericExpression {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Container for legal values referenced by configurable parameters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Choices {
    #[serde(rename(serialize = "ipxact:choice", deserialize = "choice"), default)]
    pub choice: Vec<Choice>,
}

impl Choices {
    pub fn add(&mut self, choice: Choice) {
        self.choice.push(choice);
    }
}

/// A named set of legal parameter values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:enumeration", deserialize = "enumeration"),
        default
    )]
    pub enumeration: Vec<ChoiceEnumeration>,
}

impl Choice {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            enumeration: Vec::new(),
        }
    }

    pub fn add_enumeration(&mut self, enumeration: ChoiceEnumeration) {
        self.enumeration.push(enumeration);
    }
}

/// One legal value in a configurable choice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceEnumeration {
    #[serde(rename = "@text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "@help", skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ChoiceEnumeration {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            text: None,
            help: None,
            id: None,
            extension_attributes: ExtensionAttributes::default(),
            value: value.into(),
        }
    }
}

/// Container for component-level configurable parameters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(
        rename(serialize = "ipxact:parameter", deserialize = "parameter"),
        default
    )]
    pub parameter: Vec<Parameter>,
}

impl Parameters {
    pub fn add(&mut self, parameter: Parameter) {
        self.parameter.push(parameter);
    }
}

/// Configurable component parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,

    #[serde(rename = "@prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(rename = "@choiceRef", skip_serializing_if = "Option::is_none")]
    pub choice_ref: Option<String>,

    #[serde(rename = "@order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,

    #[serde(rename = "@configGroups", skip_serializing_if = "Option::is_none")]
    pub config_groups: Option<String>,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub format: Option<ParameterFormat>,

    #[serde(rename = "@sign", skip_serializing_if = "Option::is_none")]
    pub sign: Option<ParameterSign>,

    #[serde(rename = "@prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<ParameterPrefix>,

    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<ParameterUnit>,

    #[serde(rename = "@resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<ParameterResolve>,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

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
        rename(serialize = "ipxact:vectors", deserialize = "vectors"),
        skip_serializing_if = "Option::is_none"
    )]
    pub vectors: Option<PortVectors>,

    #[serde(
        rename(serialize = "ipxact:arrays", deserialize = "arrays"),
        skip_serializing_if = "Option::is_none"
    )]
    pub arrays: Option<ConfigurableArrays>,

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: ParameterExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Parameter {
    pub fn new(name: impl Into<String>, value: impl Into<ParameterExpression>) -> Self {
        Self {
            parameter_id: None,
            prompt: None,
            choice_ref: None,
            order: None,
            config_groups: None,
            minimum: None,
            maximum: None,
            format: None,
            sign: None,
            prefix: None,
            unit: None,
            resolve: None,
            extension_attributes: ExtensionAttributes::default(),
            name: name.into(),
            display_name: None,
            description: None,
            vectors: None,
            arrays: None,
            value: value.into(),
            vendor_extensions: None,
        }
    }
}

/// Expression used as a configurable parameter value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterExpression {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,
}

impl ParameterExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            extension_attributes: ExtensionAttributes::default(),
        }
    }
}

impl From<&str> for ParameterExpression {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ParameterExpression {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Parameter resolution policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterResolve {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "generated")]
    Generated,
}

/// Data format for a configurable parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterFormat {
    #[serde(rename = "bit")]
    Bit,
    #[serde(rename = "byte")]
    Byte,
    #[serde(rename = "shortint")]
    Shortint,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "longint")]
    Longint,
    #[serde(rename = "shortreal")]
    Shortreal,
    #[serde(rename = "real")]
    Real,
    #[serde(rename = "string")]
    String,
}

/// Signedness override for a configurable parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterSign {
    #[serde(rename = "signed")]
    Signed,
    #[serde(rename = "unsigned")]
    Unsigned,
}

/// SI prefix associated with a configurable parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterPrefix {
    #[serde(rename = "deca")]
    Deca,
    #[serde(rename = "hecto")]
    Hecto,
    #[serde(rename = "kilo")]
    Kilo,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "giga")]
    Giga,
    #[serde(rename = "tera")]
    Tera,
    #[serde(rename = "peta")]
    Peta,
    #[serde(rename = "exa")]
    Exa,
    #[serde(rename = "zetta")]
    Zetta,
    #[serde(rename = "yotta")]
    Yotta,
    #[serde(rename = "deci")]
    Deci,
    #[serde(rename = "centi")]
    Centi,
    #[serde(rename = "milli")]
    Milli,
    #[serde(rename = "micro")]
    Micro,
    #[serde(rename = "nano")]
    Nano,
    #[serde(rename = "pico")]
    Pico,
    #[serde(rename = "femto")]
    Femto,
    #[serde(rename = "atto")]
    Atto,
    #[serde(rename = "zepto")]
    Zepto,
    #[serde(rename = "yocto")]
    Yocto,
}

/// Physical unit associated with a configurable parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "ampere")]
    Ampere,
    #[serde(rename = "kelvin")]
    Kelvin,
    #[serde(rename = "hertz")]
    Hertz,
    #[serde(rename = "joule")]
    Joule,
    #[serde(rename = "watt")]
    Watt,
    #[serde(rename = "coulomb")]
    Coulomb,
    #[serde(rename = "volt")]
    Volt,
    #[serde(rename = "farad")]
    Farad,
    #[serde(rename = "ohm")]
    Ohm,
    #[serde(rename = "siemens")]
    Siemens,
    #[serde(rename = "henry")]
    Henry,
    #[serde(rename = "Celsius")]
    Celsius,
}

/// Container for file sets associated with a component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSets {
    #[serde(rename(serialize = "ipxact:fileSet", deserialize = "fileSet"), default)]
    pub file_set: Vec<FileSet>,
}

impl FileSets {
    pub fn add(&mut self, file_set: FileSet) {
        self.file_set.push(file_set);
    }
}

/// Ordered set of source files and directories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSet {
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

    #[serde(rename(serialize = "ipxact:group", deserialize = "group"), default)]
    pub group: Vec<FileSetGroup>,

    #[serde(rename(serialize = "ipxact:file", deserialize = "file"), default)]
    pub file: Vec<File>,

    #[serde(
        rename(
            serialize = "ipxact:defaultFileBuilder",
            deserialize = "defaultFileBuilder"
        ),
        default
    )]
    pub default_file_builder: Vec<FileBuilder>,

    #[serde(
        rename(serialize = "ipxact:dependency", deserialize = "dependency"),
        default
    )]
    pub dependency: Vec<Dependency>,

    #[serde(
        rename(serialize = "ipxact:function", deserialize = "function"),
        default
    )]
    pub function: Vec<FileSetFunction>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl FileSet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            group: Vec::new(),
            file: Vec::new(),
            default_file_builder: Vec::new(),
            dependency: Vec::new(),
            function: Vec::new(),
            vendor_extensions: None,
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.file.push(file);
    }
}

/// Group label attached to a file set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSetGroup {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl FileSetGroup {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// File or directory included in a component file set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "@fileId", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: StringURIExpression,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:fileType", deserialize = "fileType"),
        default
    )]
    pub file_type: Vec<FileType>,

    #[serde(
        rename(serialize = "ipxact:isStructural", deserialize = "isStructural"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_structural: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:isIncludeFile", deserialize = "isIncludeFile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_include_file: Option<IncludeFile>,

    #[serde(
        rename(serialize = "ipxact:logicalName", deserialize = "logicalName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_name: Option<LogicalName>,

    #[serde(
        rename(serialize = "ipxact:exportedName", deserialize = "exportedName"),
        default
    )]
    pub exported_name: Vec<ExportedName>,

    #[serde(
        rename(serialize = "ipxact:buildCommand", deserialize = "buildCommand"),
        skip_serializing_if = "Option::is_none"
    )]
    pub build_command: Option<BuildCommand>,

    #[serde(
        rename(serialize = "ipxact:dependency", deserialize = "dependency"),
        default
    )]
    pub dependency: Vec<Dependency>,

    #[serde(rename(serialize = "ipxact:define", deserialize = "define"), default)]
    pub define: Vec<NameValuePair>,

    #[serde(
        rename(serialize = "ipxact:imageType", deserialize = "imageType"),
        default
    )]
    pub image_type: Vec<ImageType>,

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

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,
}

impl File {
    pub fn new(name: impl Into<String>, file_type: FileType) -> Self {
        Self {
            file_id: None,
            id: None,
            name: StringURIExpression::new(name),
            is_present: None,
            file_type: vec![file_type],
            is_structural: None,
            is_include_file: None,
            logical_name: None,
            exported_name: Vec::new(),
            build_command: None,
            dependency: Vec::new(),
            define: Vec::new(),
            image_type: Vec::new(),
            description: None,
            vendor_extensions: None,
            extension_attributes: ExtensionAttributes::default(),
        }
    }
}

/// Include-file marker with its declaration behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncludeFile {
    #[serde(
        rename = "@externalDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_declarations: Option<bool>,

    #[serde(rename = "$text")]
    pub value: bool,
}

impl IncludeFile {
    pub fn new(value: bool) -> Self {
        Self {
            external_declarations: None,
            value,
        }
    }
}

/// Logical library name assigned to a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicalName {
    #[serde(rename = "@default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl LogicalName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            default: None,
            value: value.into(),
        }
    }
}

/// Name exported by a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedName {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ExportedName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// File-level build command.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildCommand {
    #[serde(
        rename(serialize = "ipxact:command", deserialize = "command"),
        skip_serializing_if = "Option::is_none"
    )]
    pub command: Option<StringExpression>,

    #[serde(
        rename(serialize = "ipxact:flags", deserialize = "flags"),
        skip_serializing_if = "Option::is_none"
    )]
    pub flags: Option<BuildFlags>,

    #[serde(
        rename(
            serialize = "ipxact:replaceDefaultFlags",
            deserialize = "replaceDefaultFlags"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_default_flags: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:targetName", deserialize = "targetName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub target_name: Option<StringURIExpression>,
}

/// File-level compiler flags with optional append behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildFlags {
    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "@append", skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl BuildFlags {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            extension_attributes: ExtensionAttributes::default(),
            append: None,
            value: value.into(),
        }
    }
}

/// Single-bit expression used by build metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitExpression {
    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "$text")]
    pub value: String,
}

impl BitExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            extension_attributes: ExtensionAttributes::default(),
            value: value.into(),
        }
    }
}

impl From<&str> for BitExpression {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for BitExpression {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Dependency URI attached to a file or file set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl Dependency {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Executable-image category attached to a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageType {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ImageType {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Named string value used by file defines and related structures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameValuePair {
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

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: StringExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl NameValuePair {
    pub fn new(name: impl Into<String>, value: impl Into<StringExpression>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            value: value.into(),
            vendor_extensions: None,
        }
    }
}

/// Standard IP-XACT file type with an optional user-defined subtype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileType {
    #[serde(rename = "@user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: FileTypeValue,
}

impl FileType {
    pub fn new(value: FileTypeValue) -> Self {
        Self {
            user: None,
            id: None,
            value,
        }
    }
}

/// File-type values defined by IEEE 1685-2014.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileTypeValue {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "cSource")]
    CSource,
    #[serde(rename = "cppSource")]
    CppSource,
    #[serde(rename = "asmSource")]
    AsmSource,
    #[serde(rename = "vhdlSource")]
    VhdlSource,
    #[serde(rename = "vhdlSource-87")]
    VhdlSource87,
    #[serde(rename = "vhdlSource-93")]
    VhdlSource93,
    #[serde(rename = "verilogSource")]
    VerilogSource,
    #[serde(rename = "verilogSource-95")]
    VerilogSource95,
    #[serde(rename = "verilogSource-2001")]
    VerilogSource2001,
    #[serde(rename = "swObject")]
    SwObject,
    #[serde(rename = "swObjectLibrary")]
    SwObjectLibrary,
    #[serde(rename = "vhdlBinaryLibrary")]
    VhdlBinaryLibrary,
    #[serde(rename = "verilogBinaryLibrary")]
    VerilogBinaryLibrary,
    #[serde(rename = "unelaboratedHdl")]
    UnelaboratedHdl,
    #[serde(rename = "executableHdl")]
    ExecutableHdl,
    #[serde(rename = "systemVerilogSource")]
    SystemVerilogSource,
    #[serde(rename = "systemVerilogSource-3.0")]
    SystemVerilogSource30,
    #[serde(rename = "systemVerilogSource-3.1")]
    SystemVerilogSource31,
    #[serde(rename = "systemCSource")]
    SystemCSource,
    #[serde(rename = "systemCSource-2.0")]
    SystemCSource20,
    #[serde(rename = "systemCSource-2.0.1")]
    SystemCSource201,
    #[serde(rename = "systemCSource-2.1")]
    SystemCSource21,
    #[serde(rename = "systemCSource-2.2")]
    SystemCSource22,
    #[serde(rename = "veraSource")]
    VeraSource,
    #[serde(rename = "eSource")]
    ESource,
    #[serde(rename = "perlSource")]
    PerlSource,
    #[serde(rename = "tclSource")]
    TclSource,
    #[serde(rename = "OVASource")]
    OvaSource,
    #[serde(rename = "SVASource")]
    SvaSource,
    #[serde(rename = "pslSource")]
    PslSource,
    #[serde(rename = "systemVerilogSource-3.1a")]
    SystemVerilogSource31a,
    #[serde(rename = "SDC")]
    Sdc,
    #[serde(rename = "vhdlAmsSource")]
    VhdlAmsSource,
    #[serde(rename = "verilogAmsSource")]
    VerilogAmsSource,
    #[serde(rename = "systemCAmsSource")]
    SystemCAmsSource,
    #[serde(rename = "libertySource")]
    LibertySource,
    #[serde(rename = "user")]
    User,
}

/// Default builder for files of one type in a file set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileBuilder {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:fileType", deserialize = "fileType"))]
    pub file_type: FileType,

    #[serde(
        rename(serialize = "ipxact:command", deserialize = "command"),
        skip_serializing_if = "Option::is_none"
    )]
    pub command: Option<StringExpression>,

    #[serde(
        rename(serialize = "ipxact:flags", deserialize = "flags"),
        skip_serializing_if = "Option::is_none"
    )]
    pub flags: Option<StringExpression>,

    #[serde(
        rename(
            serialize = "ipxact:replaceDefaultFlags",
            deserialize = "replaceDefaultFlags"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_default_flags: Option<BitExpression>,
}

impl FileBuilder {
    pub fn new(file_type: FileType) -> Self {
        Self {
            id: None,
            file_type,
            command: None,
            flags: None,
            replace_default_flags: None,
        }
    }
}

/// Software function described by a file set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSetFunction {
    #[serde(rename = "@replicate", skip_serializing_if = "Option::is_none")]
    pub replicate: Option<bool>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:entryPoint", deserialize = "entryPoint"),
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_point: Option<String>,

    #[serde(rename(serialize = "ipxact:fileRef", deserialize = "fileRef"))]
    pub file_ref: String,

    #[serde(
        rename(serialize = "ipxact:returnType", deserialize = "returnType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub return_type: Option<FunctionReturnType>,

    #[serde(
        rename(serialize = "ipxact:argument", deserialize = "argument"),
        default
    )]
    pub argument: Vec<FunctionArgument>,

    #[serde(
        rename(serialize = "ipxact:disabled", deserialize = "disabled"),
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:sourceFile", deserialize = "sourceFile"),
        default
    )]
    pub source_file: Vec<FunctionSourceFile>,
}

impl FileSetFunction {
    pub fn new(file_ref: impl Into<String>) -> Self {
        Self {
            replicate: None,
            id: None,
            entry_point: None,
            file_ref: file_ref.into(),
            return_type: None,
            argument: Vec::new(),
            disabled: None,
            source_file: Vec::new(),
        }
    }
}

/// Return type of a software function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionReturnType {
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "int")]
    Int,
}

/// Typed argument passed to a file-set function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionArgument {
    #[serde(rename = "@dataType")]
    pub data_type: FunctionDataType,

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

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: StringExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl FunctionArgument {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<StringExpression>,
        data_type: FunctionDataType,
    ) -> Self {
        Self {
            data_type,
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            value: value.into(),
            vendor_extensions: None,
        }
    }
}

/// C-compatible data type for a file-set function argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionDataType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "unsigned int")]
    UnsignedInt,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "unsigned long")]
    UnsignedLong,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "char *")]
    CharPointer,
    #[serde(rename = "void *")]
    VoidPointer,
}

/// Additional source location for a file-set function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionSourceFile {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:sourceName", deserialize = "sourceName"))]
    pub source_name: String,

    #[serde(rename(serialize = "ipxact:fileType", deserialize = "fileType"))]
    pub file_type: FileType,
}

impl FunctionSourceFile {
    pub fn new(source_name: impl Into<String>, file_type: FileType) -> Self {
        Self {
            id: None,
            source_name: source_name.into(),
            file_type,
        }
    }
}

/// Reference from an instantiation to a component-local file set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSetRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:localName", deserialize = "localName"))]
    pub local_name: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl FileSetRef {
    pub fn new(local_name: impl Into<String>) -> Self {
        Self {
            id: None,
            local_name: local_name.into(),
            is_present: None,
        }
    }
}

/// Container for component-local whitebox reference points.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElements {
    #[serde(
        rename(serialize = "ipxact:whiteboxElement", deserialize = "whiteboxElement"),
        default
    )]
    pub whitebox_element: Vec<WhiteboxElement>,
}

impl WhiteboxElements {
    pub fn add(&mut self, whitebox_element: WhiteboxElement) {
        self.whitebox_element.push(whitebox_element);
    }
}

/// Internal component reference point visible to model instantiations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElement {
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

    #[serde(rename(serialize = "ipxact:whiteboxType", deserialize = "whiteboxType"))]
    pub whitebox_type: WhiteboxType,

    #[serde(
        rename(serialize = "ipxact:driveable", deserialize = "driveable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub driveable: Option<bool>,

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
}

impl WhiteboxElement {
    pub fn new(name: impl Into<String>, whitebox_type: WhiteboxType) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            whitebox_type,
            driveable: None,
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Kind of component-local whitebox reference point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhiteboxType {
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "interface")]
    Interface,
}

/// Reference from an instantiation to a wire-port constraint set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintSetRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:localName", deserialize = "localName"))]
    pub local_name: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl ConstraintSetRef {
    pub fn new(local_name: impl Into<String>) -> Self {
        Self {
            id: None,
            local_name: local_name.into(),
            is_present: None,
        }
    }
}

/// Container for whitebox references exposed by an instantiation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElementRefs {
    #[serde(
        rename(
            serialize = "ipxact:whiteboxElementRef",
            deserialize = "whiteboxElementRef"
        ),
        default
    )]
    pub whitebox_element_ref: Vec<WhiteboxElementRef>,
}

/// Whitebox reference and its HDL access locations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteboxElementRef {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:location", deserialize = "location"),
        default
    )]
    pub location: Vec<Slices>,
}

impl WhiteboxElementRef {
    pub fn new(name: impl Into<String>, location: Slices) -> Self {
        Self {
            name: name.into(),
            id: None,
            is_present: None,
            location: vec![location],
        }
    }
}

/// HDL path slices for a whitebox location.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Slices {
    #[serde(rename(serialize = "ipxact:slice", deserialize = "slice"), default)]
    pub slice: Vec<Slice>,
}

/// One slice of an HDL path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slice {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:pathSegments", deserialize = "pathSegments"))]
    pub path_segments: PathSegments,

    #[serde(
        rename(serialize = "ipxact:range", deserialize = "range"),
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<PortRange>,
}

impl Slice {
    pub fn new(path_segment: PathSegment) -> Self {
        Self {
            id: None,
            path_segments: PathSegments {
                path_segment: vec![path_segment],
            },
            range: None,
        }
    }
}

/// Ordered HDL path segments.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PathSegments {
    #[serde(
        rename(serialize = "ipxact:pathSegment", deserialize = "pathSegment"),
        default
    )]
    pub path_segment: Vec<PathSegment>,
}

/// One named level in an HDL hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathSegment {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:pathSegmentName", deserialize = "pathSegmentName"))]
    pub path_segment_name: String,

    #[serde(
        rename(serialize = "ipxact:indices", deserialize = "indices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub indices: Option<Indices>,
}

impl PathSegment {
    pub fn new(path_segment_name: impl Into<String>) -> Self {
        Self {
            id: None,
            path_segment_name: path_segment_name.into(),
            indices: None,
        }
    }

    pub fn with_index(mut self, index: impl Into<UnsignedIntExpression>) -> Self {
        self.indices
            .get_or_insert_with(Indices::default)
            .index
            .push(index.into());
        self
    }
}

/// Multi-dimensional indices applied to one HDL path segment.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Indices {
    #[serde(rename(serialize = "ipxact:index", deserialize = "index"), default)]
    pub index: Vec<UnsignedIntExpression>,
}

/// Container for processors described by a component.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Cpus {
    #[serde(rename(serialize = "ipxact:cpu", deserialize = "cpu"), default)]
    pub cpu: Vec<Cpu>,
}

impl Cpus {
    pub fn add(&mut self, cpu: Cpu) {
        self.cpu.push(cpu);
    }
}

/// Processor with one or more mapped component address spaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cpu {
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
        rename(serialize = "ipxact:addressSpaceRef", deserialize = "addressSpaceRef"),
        default
    )]
    pub address_space_ref: Vec<AddressSpaceRef>,

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
}

impl Cpu {
    pub fn new(name: impl Into<String>, address_space_ref: AddressSpaceRef) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            address_space_ref: vec![address_space_ref],
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Container for component bus interfaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusInterfaces {
    #[serde(
        rename(serialize = "ipxact:busInterface", deserialize = "busInterface"),
        default
    )]
    pub bus_interface: Vec<BusInterface>,
}

impl BusInterfaces {
    pub fn add(&mut self, bus_interface: BusInterface) {
        self.bus_interface.push(bus_interface);
    }
}

/// Container for connections between mirrored bus interfaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Channels {
    #[serde(rename(serialize = "ipxact:channel", deserialize = "channel"), default)]
    pub channel: Vec<Channel>,
}

impl Channels {
    pub fn add(&mut self, channel: Channel) {
        self.channel.push(channel);
    }
}

/// Connection between two or more mirrored bus interfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
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
        rename(serialize = "ipxact:busInterfaceRef", deserialize = "busInterfaceRef"),
        default
    )]
    pub bus_interface_ref: Vec<ChannelBusInterfaceRef>,
}

impl Channel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            bus_interface_ref: Vec::new(),
        }
    }

    pub fn add_bus_interface_ref(&mut self, bus_interface_ref: ChannelBusInterfaceRef) {
        self.bus_interface_ref.push(bus_interface_ref);
    }
}

/// Reference to a bus interface participating in a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelBusInterfaceRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:localName", deserialize = "localName"))]
    pub local_name: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl ChannelBusInterfaceRef {
    pub fn new(local_name: impl Into<String>) -> Self {
        Self {
            id: None,
            local_name: local_name.into(),
            is_present: None,
        }
    }
}

/// Container for indirectly accessible memory-map interfaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IndirectInterfaces {
    #[serde(
        rename(
            serialize = "ipxact:indirectInterface",
            deserialize = "indirectInterface"
        ),
        default
    )]
    pub indirect_interface: Vec<IndirectInterface>,
}

impl IndirectInterfaces {
    pub fn add(&mut self, indirect_interface: IndirectInterface) {
        self.indirect_interface.push(indirect_interface);
    }
}

/// Interface for accessing a memory map through address and data fields.
#[derive(Debug, Clone, PartialEq)]
pub struct IndirectInterface {
    pub id: Option<String>,

    pub name: String,

    pub display_name: Option<String>,

    pub description: Option<String>,

    pub indirect_address_ref: String,

    pub indirect_data_ref: String,

    pub target: IndirectInterfaceTarget,

    pub bits_in_lau: Option<UnsignedPositiveLongintExpression>,

    pub endianness: Option<Endianness>,

    pub parameters: Option<Parameters>,

    pub vendor_extensions: Option<VendorExtensions>,
}

impl IndirectInterface {
    pub fn new(
        name: impl Into<String>,
        indirect_address_ref: impl Into<String>,
        indirect_data_ref: impl Into<String>,
        target: impl Into<IndirectInterfaceTarget>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            indirect_address_ref: indirect_address_ref.into(),
            indirect_data_ref: indirect_data_ref.into(),
            target: target.into(),
            bits_in_lau: None,
            endianness: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn set_target(&mut self, target: impl Into<IndirectInterfaceTarget>) {
        self.target = target.into();
    }
}

impl Serialize for IndirectInterface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
            id: Option<String>,

            #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
            name: String,

            #[serde(
                rename(serialize = "ipxact:displayName", deserialize = "displayName"),
                skip_serializing_if = "Option::is_none"
            )]
            display_name: Option<String>,

            #[serde(
                rename(serialize = "ipxact:description", deserialize = "description"),
                skip_serializing_if = "Option::is_none"
            )]
            description: Option<String>,

            #[serde(rename(
                serialize = "ipxact:indirectAddressRef",
                deserialize = "indirectAddressRef"
            ))]
            indirect_address_ref: String,

            #[serde(rename(
                serialize = "ipxact:indirectDataRef",
                deserialize = "indirectDataRef"
            ))]
            indirect_data_ref: String,

            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                skip_serializing_if = "Option::is_none"
            )]
            memory_map_ref: Option<String>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default,
                skip_serializing_if = "Vec::is_empty"
            )]
            transparent_bridge: Vec<TransparentBridge>,

            #[serde(
                rename(serialize = "ipxact:bitsInLau", deserialize = "bitsInLau"),
                skip_serializing_if = "Option::is_none"
            )]
            bits_in_lau: Option<UnsignedPositiveLongintExpression>,

            #[serde(
                rename(serialize = "ipxact:endianness", deserialize = "endianness"),
                skip_serializing_if = "Option::is_none"
            )]
            endianness: Option<Endianness>,

            #[serde(
                rename(serialize = "ipxact:parameters", deserialize = "parameters"),
                skip_serializing_if = "Option::is_none"
            )]
            parameters: Option<Parameters>,

            #[serde(
                rename(
                    serialize = "ipxact:vendorExtensions",
                    deserialize = "vendorExtensions"
                ),
                skip_serializing_if = "Option::is_none"
            )]
            vendor_extensions: Option<VendorExtensions>,
        }

        let (memory_map_ref, transparent_bridge) = match &self.target {
            IndirectInterfaceTarget::MemoryMapRef(memory_map_ref) => {
                (Some(memory_map_ref.clone()), Vec::new())
            }
            IndirectInterfaceTarget::TransparentBridges(transparent_bridge) => {
                if transparent_bridge.is_empty() {
                    return Err(S::Error::custom(
                        "indirectInterface transparentBridge choice requires at least one bridge",
                    ));
                }
                (None, transparent_bridge.clone())
            }
        };

        Helper {
            id: self.id.clone(),
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            description: self.description.clone(),
            indirect_address_ref: self.indirect_address_ref.clone(),
            indirect_data_ref: self.indirect_data_ref.clone(),
            memory_map_ref,
            transparent_bridge,
            bits_in_lau: self.bits_in_lau.clone(),
            endianness: self.endianness,
            parameters: self.parameters.clone(),
            vendor_extensions: self.vendor_extensions.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IndirectInterface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "@xml:id", default)]
            id: Option<String>,

            #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
            name: String,

            #[serde(
                rename(serialize = "ipxact:displayName", deserialize = "displayName"),
                default
            )]
            display_name: Option<String>,

            #[serde(
                rename(serialize = "ipxact:description", deserialize = "description"),
                default
            )]
            description: Option<String>,

            #[serde(rename(
                serialize = "ipxact:indirectAddressRef",
                deserialize = "indirectAddressRef"
            ))]
            indirect_address_ref: String,

            #[serde(rename(
                serialize = "ipxact:indirectDataRef",
                deserialize = "indirectDataRef"
            ))]
            indirect_data_ref: String,

            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                default
            )]
            memory_map_ref: Option<String>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default
            )]
            transparent_bridge: Vec<TransparentBridge>,

            #[serde(
                rename(serialize = "ipxact:bitsInLau", deserialize = "bitsInLau"),
                default
            )]
            bits_in_lau: Option<UnsignedPositiveLongintExpression>,

            #[serde(
                rename(serialize = "ipxact:endianness", deserialize = "endianness"),
                default
            )]
            endianness: Option<Endianness>,

            #[serde(
                rename(serialize = "ipxact:parameters", deserialize = "parameters"),
                default
            )]
            parameters: Option<Parameters>,

            #[serde(
                rename(
                    serialize = "ipxact:vendorExtensions",
                    deserialize = "vendorExtensions"
                ),
                default
            )]
            vendor_extensions: Option<VendorExtensions>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let target = match (helper.memory_map_ref, helper.transparent_bridge) {
            (Some(memory_map_ref), transparent_bridge) if transparent_bridge.is_empty() => {
                IndirectInterfaceTarget::MemoryMapRef(memory_map_ref)
            }
            (None, transparent_bridge) if !transparent_bridge.is_empty() => {
                IndirectInterfaceTarget::TransparentBridges(transparent_bridge)
            }
            _ => {
                return Err(D::Error::custom(
                    "indirectInterface target must contain exactly one schema choice",
                ));
            }
        };

        Ok(Self {
            id: helper.id,
            name: helper.name,
            display_name: helper.display_name,
            description: helper.description,
            indirect_address_ref: helper.indirect_address_ref,
            indirect_data_ref: helper.indirect_data_ref,
            target,
            bits_in_lau: helper.bits_in_lau,
            endianness: helper.endianness,
            parameters: helper.parameters,
            vendor_extensions: helper.vendor_extensions,
        })
    }
}

/// Access target of an indirect interface.
#[derive(Debug, Clone, PartialEq)]
pub enum IndirectInterfaceTarget {
    MemoryMapRef(String),
    TransparentBridges(Vec<TransparentBridge>),
}

impl IndirectInterfaceTarget {
    pub fn transparent_bridge(bridge: TransparentBridge) -> Self {
        Self::TransparentBridges(vec![bridge])
    }
}

impl From<&str> for IndirectInterfaceTarget {
    fn from(memory_map_ref: &str) -> Self {
        Self::MemoryMapRef(memory_map_ref.into())
    }
}

impl From<String> for IndirectInterfaceTarget {
    fn from(memory_map_ref: String) -> Self {
        Self::MemoryMapRef(memory_map_ref)
    }
}

impl Serialize for IndirectInterfaceTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                skip_serializing_if = "Option::is_none"
            )]
            memory_map_ref: Option<String>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default,
                skip_serializing_if = "Vec::is_empty"
            )]
            transparent_bridge: Vec<TransparentBridge>,
        }

        let helper = match self {
            Self::MemoryMapRef(memory_map_ref) => Helper {
                memory_map_ref: Some(memory_map_ref.clone()),
                transparent_bridge: Vec::new(),
            },
            Self::TransparentBridges(transparent_bridge) => Helper {
                memory_map_ref: None,
                transparent_bridge: transparent_bridge.clone(),
            },
        };

        helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IndirectInterfaceTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                default
            )]
            memory_map_ref: Option<String>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default
            )]
            transparent_bridge: Vec<TransparentBridge>,
        }

        let helper = Helper::deserialize(deserializer)?;
        match (helper.memory_map_ref, helper.transparent_bridge) {
            (Some(memory_map_ref), transparent_bridge) if transparent_bridge.is_empty() => {
                Ok(Self::MemoryMapRef(memory_map_ref))
            }
            (None, transparent_bridge) if !transparent_bridge.is_empty() => {
                Ok(Self::TransparentBridges(transparent_bridge))
            }
            _ => Err(D::Error::custom(
                "indirectInterface target must contain exactly one schema choice",
            )),
        }
    }
}

/// Bus interface exposed by a component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusInterface {
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

    #[serde(rename(serialize = "ipxact:busType", deserialize = "busType"))]
    pub bus_type: ConfigurableLibraryRef,

    #[serde(
        rename(
            serialize = "ipxact:abstractionTypes",
            deserialize = "abstractionTypes"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub abstraction_types: Option<AbstractionTypes>,

    #[serde(rename = "$value")]
    pub mode: BusInterfaceMode,

    #[serde(
        rename(
            serialize = "ipxact:connectionRequired",
            deserialize = "connectionRequired"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_required: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:bitsInLau", deserialize = "bitsInLau"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bits_in_lau: Option<UnsignedPositiveLongintExpression>,

    #[serde(
        rename(serialize = "ipxact:bitSteering", deserialize = "bitSteering"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bit_steering: Option<BitSteeringExpression>,

    #[serde(
        rename(serialize = "ipxact:endianness", deserialize = "endianness"),
        skip_serializing_if = "Option::is_none"
    )]
    pub endianness: Option<Endianness>,

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

    #[serde(flatten, skip_deserializing)]
    pub extension_attributes: ExtensionAttributes,
}

impl BusInterface {
    pub fn new(
        name: impl Into<String>,
        bus_type: ConfigurableLibraryRef,
        mode: BusInterfaceMode,
    ) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            bus_type,
            abstraction_types: None,
            mode,
            connection_required: None,
            bits_in_lau: None,
            bit_steering: None,
            endianness: None,
            parameters: None,
            vendor_extensions: None,
            extension_attributes: ExtensionAttributes::default(),
        }
    }
}

/// Expression controlling bit steering for a bus interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitSteeringExpression {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,
}

impl BitSteeringExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            extension_attributes: ExtensionAttributes::default(),
        }
    }
}

/// Byte ordering used by a bus interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Endianness {
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "little")]
    Little,
}

/// Reference to a configurable IP-XACT definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurableLibraryRef {
    #[serde(
        rename(
            serialize = "ipxact:configurableElementValues",
            deserialize = "configurableElementValues"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub configurable_element_values: Option<ConfigurableElementValues>,

    #[serde(rename = "@vendor")]
    pub vendor: String,

    #[serde(rename = "@library")]
    pub library: String,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@version")]
    pub version: String,
}

impl ConfigurableLibraryRef {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            configurable_element_values: None,
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
        }
    }
}

/// Values overriding configurable elements in a referenced definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurableElementValues {
    #[serde(
        rename(
            serialize = "ipxact:configurableElementValue",
            deserialize = "configurableElementValue"
        ),
        default
    )]
    pub configurable_element_value: Vec<ConfigurableElementValue>,
}

impl ConfigurableElementValues {
    pub fn add(&mut self, value: ConfigurableElementValue) {
        self.configurable_element_value.push(value);
    }
}

/// One value overriding a configurable element by its identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurableElementValue {
    #[serde(rename = "@referenceId")]
    pub reference_id: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ConfigurableElementValue {
    pub fn new(reference_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            reference_id: reference_id.into(),
            id: None,
            extension_attributes: ExtensionAttributes::default(),
            value: value.into(),
        }
    }
}

/// Container for abstraction types supported by a bus interface.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractionTypes {
    #[serde(
        rename(serialize = "ipxact:abstractionType", deserialize = "abstractionType"),
        default
    )]
    pub abstraction_type: Vec<AbstractionType>,
}

impl AbstractionTypes {
    pub fn add(&mut self, abstraction_type: AbstractionType) {
        self.abstraction_type.push(abstraction_type);
    }
}

/// Abstraction definition reference and its logical-to-physical port maps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractionType {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<AbstractionViewRef>,

    #[serde(rename(serialize = "ipxact:abstractionRef", deserialize = "abstractionRef"))]
    pub abstraction_ref: ConfigurableLibraryRef,

    #[serde(
        rename(serialize = "ipxact:portMaps", deserialize = "portMaps"),
        skip_serializing_if = "Option::is_none"
    )]
    pub port_maps: Option<PortMaps>,
}

impl AbstractionType {
    pub fn new(abstraction_ref: ConfigurableLibraryRef) -> Self {
        Self {
            id: None,
            view_ref: Vec::new(),
            abstraction_ref,
            port_maps: None,
        }
    }
}

/// Model-view reference used by an interface abstraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractionViewRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl AbstractionViewRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for logical-to-physical port maps.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortMaps {
    #[serde(rename(serialize = "ipxact:portMap", deserialize = "portMap"), default)]
    pub port_map: Vec<PortMap>,
}

impl PortMaps {
    pub fn add(&mut self, port_map: PortMap) {
        self.port_map.push(port_map);
    }
}

/// Mapping from an abstraction-definition port to a component port or tie-off.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortMap {
    #[serde(rename = "@invert", skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:logicalPort", deserialize = "logicalPort"))]
    pub logical_port: LogicalPort,

    #[serde(rename = "$value")]
    pub target: PortMapTarget,

    #[serde(
        rename(serialize = "ipxact:isInformative", deserialize = "isInformative"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_informative: Option<bool>,
}

impl PortMap {
    pub fn new(logical_port: LogicalPort, target: PortMapTarget) -> Self {
        Self {
            invert: None,
            id: None,
            is_present: None,
            logical_port,
            target,
            is_informative: None,
        }
    }
}

/// Target of a logical abstraction port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortMapTarget {
    #[serde(rename(serialize = "ipxact:physicalPort", deserialize = "physicalPort"))]
    PhysicalPort(PhysicalPort),

    #[serde(rename(serialize = "ipxact:logicalTieOff", deserialize = "logicalTieOff"))]
    LogicalTieOff(UnsignedPositiveIntExpression),
}

/// Logical port from an abstraction definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicalPort {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:range", deserialize = "range"),
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<PortRange>,
}

impl LogicalPort {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            range: None,
        }
    }
}

/// Physical port from this component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalPort {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,
}

impl PhysicalPort {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Selected range of a logical abstraction port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortRange {
    #[serde(rename(serialize = "ipxact:left", deserialize = "left"))]
    pub left: UnsignedIntExpression,

    #[serde(rename(serialize = "ipxact:right", deserialize = "right"))]
    pub right: UnsignedIntExpression,
}

impl PortRange {
    pub fn new(
        left: impl Into<UnsignedIntExpression>,
        right: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

/// Component bus-interface mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BusInterfaceMode {
    #[serde(rename(serialize = "ipxact:master", deserialize = "master"))]
    Master(Master),

    #[serde(rename(serialize = "ipxact:slave", deserialize = "slave"))]
    Slave(Slave),

    #[serde(rename(serialize = "ipxact:system", deserialize = "system"))]
    System(System),

    #[serde(rename(serialize = "ipxact:mirroredSlave", deserialize = "mirroredSlave"))]
    MirroredSlave(MirroredSlave),

    #[serde(rename(serialize = "ipxact:mirroredMaster", deserialize = "mirroredMaster"))]
    MirroredMaster(MirroredMaster),

    #[serde(rename(serialize = "ipxact:mirroredSystem", deserialize = "mirroredSystem"))]
    MirroredSystem(MirroredSystem),

    #[serde(rename(serialize = "ipxact:monitor", deserialize = "monitor"))]
    Monitor(Monitor),
}

/// Master bus-interface details.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Master {
    #[serde(
        rename(serialize = "ipxact:addressSpaceRef", deserialize = "addressSpaceRef"),
        skip_serializing_if = "Option::is_none"
    )]
    pub address_space_ref: Option<AddressSpaceRef>,
}

/// Reference to an address space in the same component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceRef {
    #[serde(rename = "@addressSpaceRef")]
    pub address_space_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"),
        skip_serializing_if = "Option::is_none"
    )]
    pub base_address: Option<SignedLongintExpression>,
}

impl AddressSpaceRef {
    pub fn new(address_space_ref: impl Into<String>) -> Self {
        Self {
            address_space_ref: address_space_ref.into(),
            id: None,
            base_address: None,
        }
    }
}

/// Slave bus-interface details.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Slave {
    pub target: Option<SlaveTarget>,
    pub file_set_ref_group: Vec<SlaveFileSetRefGroup>,
}

impl Slave {
    pub fn memory_map_ref(memory_map_ref: impl Into<String>) -> Self {
        Self {
            target: Some(SlaveTarget::MemoryMapRef(MemoryMapRef::new(memory_map_ref))),
            file_set_ref_group: Vec::new(),
        }
    }

    pub fn transparent_bridge(bridge: TransparentBridge) -> Self {
        Self {
            target: Some(SlaveTarget::transparent_bridge(bridge)),
            file_set_ref_group: Vec::new(),
        }
    }

    pub fn transparent_bridges(bridges: Vec<TransparentBridge>) -> Self {
        Self {
            target: Some(SlaveTarget::TransparentBridges(bridges)),
            file_set_ref_group: Vec::new(),
        }
    }

    pub fn add_file_set_ref_group(&mut self, group: SlaveFileSetRefGroup) {
        self.file_set_ref_group.push(group);
    }
}

/// Optional access target of a slave bus interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlaveTarget {
    MemoryMapRef(MemoryMapRef),
    TransparentBridges(Vec<TransparentBridge>),
}

impl SlaveTarget {
    pub fn transparent_bridge(bridge: TransparentBridge) -> Self {
        Self::TransparentBridges(vec![bridge])
    }
}

impl Serialize for Slave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                skip_serializing_if = "Option::is_none"
            )]
            memory_map_ref: Option<MemoryMapRef>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default,
                skip_serializing_if = "Vec::is_empty"
            )]
            transparent_bridge: Vec<TransparentBridge>,

            #[serde(
                rename(serialize = "ipxact:fileSetRefGroup", deserialize = "fileSetRefGroup"),
                default,
                skip_serializing_if = "Vec::is_empty"
            )]
            file_set_ref_group: Vec<SlaveFileSetRefGroup>,
        }

        let (memory_map_ref, transparent_bridge) = match &self.target {
            None => (None, Vec::new()),
            Some(SlaveTarget::MemoryMapRef(memory_map_ref)) => {
                (Some(memory_map_ref.clone()), Vec::new())
            }
            Some(SlaveTarget::TransparentBridges(transparent_bridge)) => {
                if transparent_bridge.is_empty() {
                    return Err(S::Error::custom(
                        "slave transparentBridge choice requires at least one bridge",
                    ));
                }
                (None, transparent_bridge.clone())
            }
        };

        Helper {
            memory_map_ref,
            transparent_bridge,
            file_set_ref_group: self.file_set_ref_group.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Slave {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:memoryMapRef", deserialize = "memoryMapRef"),
                default
            )]
            memory_map_ref: Option<MemoryMapRef>,

            #[serde(
                rename(
                    serialize = "ipxact:transparentBridge",
                    deserialize = "transparentBridge"
                ),
                default
            )]
            transparent_bridge: Vec<TransparentBridge>,

            #[serde(
                rename(serialize = "ipxact:fileSetRefGroup", deserialize = "fileSetRefGroup"),
                default
            )]
            file_set_ref_group: Vec<SlaveFileSetRefGroup>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let target = match (helper.memory_map_ref, helper.transparent_bridge) {
            (None, transparent_bridge) if transparent_bridge.is_empty() => None,
            (Some(memory_map_ref), transparent_bridge) if transparent_bridge.is_empty() => {
                Some(SlaveTarget::MemoryMapRef(memory_map_ref))
            }
            (None, transparent_bridge) if !transparent_bridge.is_empty() => {
                Some(SlaveTarget::TransparentBridges(transparent_bridge))
            }
            _ => {
                return Err(D::Error::custom(
                    "slave target must contain at most one schema choice",
                ));
            }
        };

        Ok(Self {
            target,
            file_set_ref_group: helper.file_set_ref_group,
        })
    }
}

/// File-set references associated with a slave bus interface.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlaveFileSetRefGroup {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:group", deserialize = "group"),
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<String>,

    #[serde(
        rename(serialize = "ipxact:fileSetRef", deserialize = "fileSetRef"),
        default
    )]
    pub file_set_ref: Vec<FileSetRef>,
}

impl SlaveFileSetRefGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_group(group: impl Into<String>) -> Self {
        Self {
            group: Some(group.into()),
            ..Self::default()
        }
    }

    pub fn add(&mut self, file_set_ref: FileSetRef) {
        self.file_set_ref.push(file_set_ref);
    }
}

/// System bus-interface details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct System {
    #[serde(rename(serialize = "ipxact:group", deserialize = "group"))]
    pub group: String,
}

impl System {
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
        }
    }
}

/// Mirrored-slave bus-interface details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MirroredSlave {
    #[serde(
        rename(serialize = "ipxact:baseAddresses", deserialize = "baseAddresses"),
        skip_serializing_if = "Option::is_none"
    )]
    pub base_addresses: Option<MirroredSlaveBaseAddresses>,
}

/// Address range and remapped bases exposed by a mirrored slave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirroredSlaveBaseAddresses {
    #[serde(
        rename(serialize = "ipxact:remapAddress", deserialize = "remapAddress"),
        default
    )]
    pub remap_address: Vec<RemapAddress>,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,
}

/// Base address selected by an optional component remap state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemapAddress {
    #[serde(rename = "@state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl RemapAddress {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            state: None,
            id: None,
            value: value.into(),
        }
    }
}

/// Mirrored-master bus-interface details.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirroredMaster {}

/// Mirrored-system bus-interface details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirroredSystem {
    #[serde(rename(serialize = "ipxact:group", deserialize = "group"))]
    pub group: String,
}

impl MirroredSystem {
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
        }
    }
}

/// Passive monitor bus-interface details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Monitor {
    #[serde(rename = "@interfaceMode")]
    pub interface_mode: MonitoredInterfaceMode,

    #[serde(
        rename(serialize = "ipxact:group", deserialize = "group"),
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<String>,
}

impl Monitor {
    pub fn new(interface_mode: MonitoredInterfaceMode) -> Self {
        Self {
            interface_mode,
            group: None,
        }
    }
}

/// Interface mode observed by a passive monitor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoredInterfaceMode {
    #[serde(rename = "master")]
    Master,

    #[serde(rename = "slave")]
    Slave,

    #[serde(rename = "system")]
    System,

    #[serde(rename = "mirroredMaster")]
    MirroredMaster,

    #[serde(rename = "mirroredSlave")]
    MirroredSlave,

    #[serde(rename = "mirroredSystem")]
    MirroredSystem,
}

/// Reference to a memory map in the same component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryMapRef {
    #[serde(rename = "@memoryMapRef")]
    pub memory_map_ref: String,
}

impl MemoryMapRef {
    pub fn new(memory_map_ref: impl Into<String>) -> Self {
        Self {
            memory_map_ref: memory_map_ref.into(),
        }
    }
}

/// Transparent bridge from a slave bus interface to a master bus interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransparentBridge {
    #[serde(rename = "@masterRef")]
    pub master_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Container for component remap states.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemapStates {
    #[serde(
        rename(serialize = "ipxact:remapState", deserialize = "remapState"),
        default
    )]
    pub remap_state: Vec<RemapState>,
}

impl RemapStates {
    pub fn add(&mut self, remap_state: RemapState) {
        self.remap_state.push(remap_state);
    }
}

/// Named state that selects memory remaps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemapState {
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
        rename(serialize = "ipxact:remapPorts", deserialize = "remapPorts"),
        skip_serializing_if = "Option::is_none"
    )]
    pub remap_ports: Option<RemapPorts>,
}

impl RemapState {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            remap_ports: None,
        }
    }
}

/// Port conditions that activate a remap state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RemapPorts {
    #[serde(
        rename(serialize = "ipxact:remapPort", deserialize = "remapPort"),
        default
    )]
    pub remap_port: Vec<RemapPort>,
}

impl RemapPorts {
    pub fn add(&mut self, remap_port: RemapPort) {
        self.remap_port.push(remap_port);
    }
}

/// Single port value required to activate a remap state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemapPort {
    #[serde(rename = "@portRef")]
    pub port_ref: String,

    #[serde(
        rename(serialize = "ipxact:portIndex", deserialize = "portIndex"),
        skip_serializing_if = "Option::is_none"
    )]
    pub port_index: Option<UnsignedIntExpression>,

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: UnsignedIntExpression,
}

impl RemapPort {
    pub fn new(port_ref: impl Into<String>, value: impl Into<UnsignedIntExpression>) -> Self {
        Self {
            port_ref: port_ref.into(),
            port_index: None,
            value: value.into(),
        }
    }
}

/// Component model information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[serde(
        rename(serialize = "ipxact:views", deserialize = "views"),
        skip_serializing_if = "Option::is_none"
    )]
    pub views: Option<Views>,

    #[serde(
        rename(serialize = "ipxact:instantiations", deserialize = "instantiations"),
        skip_serializing_if = "Option::is_none"
    )]
    pub instantiations: Option<Instantiations>,

    #[serde(
        rename(serialize = "ipxact:ports", deserialize = "ports"),
        skip_serializing_if = "Option::is_none"
    )]
    pub ports: Option<Ports>,
}

/// Container for component model views.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Views {
    #[serde(rename(serialize = "ipxact:view", deserialize = "view"), default)]
    pub view: Vec<View>,
}

impl Views {
    pub fn add(&mut self, view: View) {
        self.view.push(view);
    }
}

/// Single component model view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
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
        rename(serialize = "ipxact:envIdentifier", deserialize = "envIdentifier"),
        default
    )]
    pub env_identifier: Vec<EnvironmentIdentifier>,

    #[serde(
        rename(
            serialize = "ipxact:componentInstantiationRef",
            deserialize = "componentInstantiationRef"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub component_instantiation_ref: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:designInstantiationRef",
            deserialize = "designInstantiationRef"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub design_instantiation_ref: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:designConfigurationInstantiationRef",
            deserialize = "designConfigurationInstantiationRef"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub design_configuration_instantiation_ref: Option<String>,
}

impl View {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            env_identifier: Vec::new(),
            component_instantiation_ref: None,
            design_instantiation_ref: None,
            design_configuration_instantiation_ref: None,
        }
    }
}

/// Environment identifier attached to a model view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentIdentifier {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl EnvironmentIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for component model instantiations.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Instantiations {
    #[serde(rename = "$value", default)]
    pub instantiation: Vec<Instantiation>,
}

impl Instantiations {
    pub fn add(&mut self, instantiation: Instantiation) {
        self.instantiation.push(instantiation);
    }
}

/// Schema choice inside a model instantiations container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instantiation {
    #[serde(rename(
        serialize = "ipxact:componentInstantiation",
        deserialize = "componentInstantiation"
    ))]
    Component(ComponentInstantiation),

    #[serde(rename(
        serialize = "ipxact:designInstantiation",
        deserialize = "designInstantiation"
    ))]
    Design(DesignInstantiation),

    #[serde(rename(
        serialize = "ipxact:designConfigurationInstantiation",
        deserialize = "designConfigurationInstantiation"
    ))]
    DesignConfiguration(DesignConfigurationInstantiation),
}

/// Reference to an IP-XACT design used by a model view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignInstantiation {
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

    #[serde(rename(serialize = "ipxact:designRef", deserialize = "designRef"))]
    pub design_ref: ConfigurableLibraryRef,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl DesignInstantiation {
    pub fn new(name: impl Into<String>, design_ref: ConfigurableLibraryRef) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            design_ref,
            vendor_extensions: None,
        }
    }
}

/// Reference to an IP-XACT design configuration used by a model view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignConfigurationInstantiation {
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
        rename(serialize = "ipxact:language", deserialize = "language"),
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<Language>,

    #[serde(rename(
        serialize = "ipxact:designConfigurationRef",
        deserialize = "designConfigurationRef"
    ))]
    pub design_configuration_ref: ConfigurableLibraryRef,

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
}

impl DesignConfigurationInstantiation {
    pub fn new(name: impl Into<String>, design_configuration_ref: ConfigurableLibraryRef) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            language: None,
            design_configuration_ref,
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// HDL language selection with optional strict matching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "@strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl Language {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            strict: None,
            value: value.into(),
        }
    }
}

/// Container for physical component ports.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    #[serde(rename(serialize = "ipxact:port", deserialize = "port"), default)]
    pub port: Vec<Port>,
}

impl Ports {
    pub fn add(&mut self, port: Port) {
        self.port.push(port);
    }
}

/// Physical component port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
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

    #[serde(rename = "$value")]
    pub style: PortStyle,

    #[serde(
        rename(serialize = "ipxact:arrays", deserialize = "arrays"),
        skip_serializing_if = "Option::is_none"
    )]
    pub arrays: Option<ConfigurableArrays>,

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

impl Port {
    pub fn new(name: impl Into<String>, style: PortStyle) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            style,
            arrays: None,
            access: None,
            vendor_extensions: None,
        }
    }
}

/// Netlister access characteristics for a component port.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortAccess {
    #[serde(
        rename(serialize = "ipxact:portAccessType", deserialize = "portAccessType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub port_access_type: Option<SimplePortAccess>,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<AccessHandles>,
}

/// Reference or pointer access used by a netlister.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimplePortAccess {
    #[serde(rename = "ref")]
    Reference,
    #[serde(rename = "ptr")]
    Pointer,
}

/// Container for HDL access handles attached to a port.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessHandles {
    #[serde(
        rename(serialize = "ipxact:accessHandle", deserialize = "accessHandle"),
        default
    )]
    pub access_handle: Vec<LeafAccessHandle>,
}

/// HDL path used to access a port in one or more views.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeafAccessHandle {
    #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<AccessViewRef>,

    #[serde(
        rename(serialize = "ipxact:indices", deserialize = "indices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub indices: Option<Indices>,

    #[serde(rename(serialize = "ipxact:slices", deserialize = "slices"))]
    pub slices: Slices,
}

impl LeafAccessHandle {
    pub fn new(slices: Slices) -> Self {
        Self {
            force: None,
            id: None,
            view_ref: Vec::new(),
            indices: None,
            slices,
        }
    }
}

/// View for which an HDL access handle applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessViewRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl AccessViewRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for simple HDL access handles attached to banks.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SimpleAccessHandles {
    #[serde(
        rename(serialize = "ipxact:accessHandle", deserialize = "accessHandle"),
        default
    )]
    pub access_handle: Vec<SimpleAccessHandle>,
}

/// HDL path used to access a bank in one or more views.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleAccessHandle {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<AccessViewRef>,

    #[serde(rename(serialize = "ipxact:pathSegments", deserialize = "pathSegments"))]
    pub path_segments: PathSegments,
}

impl SimpleAccessHandle {
    pub fn new(path_segments: PathSegments) -> Self {
        Self {
            id: None,
            view_ref: Vec::new(),
            path_segments,
        }
    }
}

/// Container for register-array HDL access handles.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IndexedAccessHandles {
    #[serde(
        rename(serialize = "ipxact:accessHandle", deserialize = "accessHandle"),
        default
    )]
    pub access_handle: Vec<IndexedAccessHandle>,
}

/// HDL path used to access one indexed register or register file in one or more views.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedAccessHandle {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<AccessViewRef>,

    #[serde(
        rename(serialize = "ipxact:indices", deserialize = "indices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub indices: Option<Indices>,

    #[serde(rename(serialize = "ipxact:pathSegments", deserialize = "pathSegments"))]
    pub path_segments: PathSegments,
}

impl IndexedAccessHandle {
    pub fn new(path_segments: PathSegments) -> Self {
        Self {
            id: None,
            view_ref: Vec::new(),
            indices: None,
            path_segments,
        }
    }
}

/// Container for field and address-block HDL access handles.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NonIndexedAccessHandles {
    #[serde(
        rename(serialize = "ipxact:accessHandle", deserialize = "accessHandle"),
        default
    )]
    pub access_handle: Vec<NonIndexedLeafAccessHandle>,
}

/// HDL path used to access a non-indexed field or address block in one or more views.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonIndexedLeafAccessHandle {
    #[serde(rename = "@force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<AccessViewRef>,

    #[serde(rename(serialize = "ipxact:slices", deserialize = "slices"))]
    pub slices: Slices,
}

impl NonIndexedLeafAccessHandle {
    pub fn new(slices: Slices) -> Self {
        Self {
            force: None,
            id: None,
            view_ref: Vec::new(),
            slices,
        }
    }
}

/// Component port style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum PortStyle {
    #[serde(rename(serialize = "ipxact:wire", deserialize = "wire"))]
    Wire(WirePort),

    #[serde(rename(serialize = "ipxact:transactional", deserialize = "transactional"))]
    Transactional(TransactionalPort),
}

/// Direction of a wire-style component port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortDirection {
    #[serde(rename = "in")]
    In,

    #[serde(rename = "out")]
    Out,

    #[serde(rename = "inout")]
    Inout,

    #[serde(rename = "phantom")]
    Phantom,
}

/// Wire-style component port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirePort {
    #[serde(
        rename = "@allLogicalDirectionsAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_logical_directions_allowed: Option<bool>,

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

    #[serde(
        rename(serialize = "ipxact:constraintSets", deserialize = "constraintSets"),
        skip_serializing_if = "Option::is_none"
    )]
    pub constraint_sets: Option<ConstraintSets>,
}

impl WirePort {
    pub fn new(direction: PortDirection) -> Self {
        Self {
            all_logical_directions_allowed: None,
            direction,
            vectors: None,
            wire_type_defs: None,
            drivers: None,
            constraint_sets: None,
        }
    }
}

/// Container for language-specific wire type definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WireTypeDefs {
    #[serde(
        rename(serialize = "ipxact:wireTypeDef", deserialize = "wireTypeDef"),
        default
    )]
    pub wire_type_def: Vec<WireTypeDef>,
}

/// Wire type applied to zero or more component views.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WireTypeDef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:typeName", deserialize = "typeName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_name: Option<WireTypeName>,

    #[serde(
        rename(serialize = "ipxact:typeDefinition", deserialize = "typeDefinition"),
        default
    )]
    pub type_definition: Vec<WireTypeDefinition>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<WireTypeViewRef>,
}

impl WireTypeDef {
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: Some(WireTypeName::new(type_name)),
            ..Self::default()
        }
    }
}

/// Language-level name of a wire type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireTypeName {
    #[serde(rename = "@constrained", skip_serializing_if = "Option::is_none")]
    pub constrained: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl WireTypeName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            constrained: None,
            value: value.into(),
        }
    }
}

/// File, package, or library containing a wire type definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireTypeDefinition {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl WireTypeDefinition {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Component view for which a wire type definition applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireTypeViewRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl WireTypeViewRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for explicit wire-port drivers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Drivers {
    #[serde(rename(serialize = "ipxact:driver", deserialize = "driver"), default)]
    pub driver: Vec<Driver>,
}

/// One wire-port driver, optionally scoped to a vector range.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Driver {
    #[serde(
        rename(serialize = "ipxact:range", deserialize = "range"),
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<PortRange>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub kind: Option<DriverKind>,
}

impl Driver {
    pub fn default_value(value: impl Into<UnsignedBitVectorExpression>) -> Self {
        Self {
            range: None,
            kind: Some(DriverKind::DefaultValue(value.into())),
        }
    }

    pub fn clock(clock_driver: ClockDriver) -> Self {
        Self {
            range: None,
            kind: Some(DriverKind::Clock(clock_driver)),
        }
    }

    pub fn single_shot(single_shot_driver: SingleShotDriver) -> Self {
        Self {
            range: None,
            kind: Some(DriverKind::SingleShot(single_shot_driver)),
        }
    }
}

/// Concrete source used to drive a wire port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriverKind {
    #[serde(rename(serialize = "ipxact:defaultValue", deserialize = "defaultValue"))]
    DefaultValue(UnsignedBitVectorExpression),

    #[serde(rename(serialize = "ipxact:clockDriver", deserialize = "clockDriver"))]
    Clock(ClockDriver),

    #[serde(rename(
        serialize = "ipxact:singleShotDriver",
        deserialize = "singleShotDriver"
    ))]
    SingleShot(SingleShotDriver),
}

/// Repeating clock waveform used to drive a wire port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClockDriver {
    #[serde(rename = "@clockName", skip_serializing_if = "Option::is_none")]
    pub clock_name: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:clockPeriod", deserialize = "clockPeriod"))]
    pub clock_period: ClockTimeExpression,

    #[serde(rename(
        serialize = "ipxact:clockPulseOffset",
        deserialize = "clockPulseOffset"
    ))]
    pub clock_pulse_offset: ClockTimeExpression,

    #[serde(rename(serialize = "ipxact:clockPulseValue", deserialize = "clockPulseValue"))]
    pub clock_pulse_value: UnsignedBitVectorExpression,

    #[serde(rename(
        serialize = "ipxact:clockPulseDuration",
        deserialize = "clockPulseDuration"
    ))]
    pub clock_pulse_duration: ClockTimeExpression,
}

impl ClockDriver {
    pub fn new(
        clock_period: impl Into<String>,
        clock_pulse_offset: impl Into<String>,
        clock_pulse_value: impl Into<UnsignedBitVectorExpression>,
        clock_pulse_duration: impl Into<String>,
    ) -> Self {
        Self {
            clock_name: None,
            id: None,
            clock_period: ClockTimeExpression::new(clock_period),
            clock_pulse_offset: ClockTimeExpression::new(clock_pulse_offset),
            clock_pulse_value: clock_pulse_value.into(),
            clock_pulse_duration: ClockTimeExpression::new(clock_pulse_duration),
        }
    }
}

/// Container for clocks that are not directly associated with component ports.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OtherClockDrivers {
    #[serde(
        rename(
            serialize = "ipxact:otherClockDriver",
            deserialize = "otherClockDriver"
        ),
        default
    )]
    pub other_clock_driver: Vec<OtherClockDriver>,
}

impl OtherClockDrivers {
    pub fn add(&mut self, driver: OtherClockDriver) {
        self.other_clock_driver.push(driver);
    }
}

/// Clock waveform that is not directly associated with a component port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherClockDriver {
    #[serde(rename = "@clockName")]
    pub clock_name: String,

    #[serde(rename = "@clockSource", skip_serializing_if = "Option::is_none")]
    pub clock_source: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:clockPeriod", deserialize = "clockPeriod"))]
    pub clock_period: ClockTimeExpression,

    #[serde(rename(
        serialize = "ipxact:clockPulseOffset",
        deserialize = "clockPulseOffset"
    ))]
    pub clock_pulse_offset: ClockTimeExpression,

    #[serde(rename(serialize = "ipxact:clockPulseValue", deserialize = "clockPulseValue"))]
    pub clock_pulse_value: UnsignedBitVectorExpression,

    #[serde(rename(
        serialize = "ipxact:clockPulseDuration",
        deserialize = "clockPulseDuration"
    ))]
    pub clock_pulse_duration: ClockTimeExpression,
}

impl OtherClockDriver {
    pub fn new(
        clock_name: impl Into<String>,
        clock_period: impl Into<String>,
        clock_pulse_offset: impl Into<String>,
        clock_pulse_value: impl Into<UnsignedBitVectorExpression>,
        clock_pulse_duration: impl Into<String>,
    ) -> Self {
        Self {
            clock_name: clock_name.into(),
            clock_source: None,
            id: None,
            clock_period: ClockTimeExpression::new(clock_period),
            clock_pulse_offset: ClockTimeExpression::new(clock_pulse_offset),
            clock_pulse_value: clock_pulse_value.into(),
            clock_pulse_duration: ClockTimeExpression::new(clock_pulse_duration),
        }
    }
}

/// Clock duration with optional delay units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClockTimeExpression {
    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    #[serde(rename = "@units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DelayUnit>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ClockTimeExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            extension_attributes: ExtensionAttributes::default(),
            minimum: None,
            maximum: None,
            units: None,
            value: value.into(),
        }
    }
}

/// Unit used for clock waveform durations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayUnit {
    #[serde(rename = "ps")]
    Picoseconds,
    #[serde(rename = "ns")]
    Nanoseconds,
}

/// One-shot waveform used to drive a wire port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleShotDriver {
    #[serde(rename(
        serialize = "ipxact:singleShotOffset",
        deserialize = "singleShotOffset"
    ))]
    pub single_shot_offset: RealExpression,

    #[serde(rename(serialize = "ipxact:singleShotValue", deserialize = "singleShotValue"))]
    pub single_shot_value: UnsignedBitVectorExpression,

    #[serde(rename(
        serialize = "ipxact:singleShotDuration",
        deserialize = "singleShotDuration"
    ))]
    pub single_shot_duration: RealExpression,
}

impl SingleShotDriver {
    pub fn new(
        single_shot_offset: impl Into<String>,
        single_shot_value: impl Into<UnsignedBitVectorExpression>,
        single_shot_duration: impl Into<String>,
    ) -> Self {
        Self {
            single_shot_offset: RealExpression::new(single_shot_offset),
            single_shot_value: single_shot_value.into(),
            single_shot_duration: RealExpression::new(single_shot_duration),
        }
    }
}

/// Container for wire-port constraint sets.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConstraintSets {
    #[serde(
        rename(serialize = "ipxact:constraintSet", deserialize = "constraintSet"),
        default
    )]
    pub constraint_set: Vec<ConstraintSet>,
}

/// Named set of constraints attached to a wire port.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConstraintSet {
    #[serde(rename = "@constraintSetId", skip_serializing_if = "Option::is_none")]
    pub constraint_set_id: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:name", deserialize = "name"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

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
        rename(serialize = "ipxact:vector", deserialize = "vector"),
        skip_serializing_if = "Option::is_none"
    )]
    pub vector: Option<PortRange>,

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

    #[serde(
        rename(
            serialize = "ipxact:timingConstraint",
            deserialize = "timingConstraint"
        ),
        default
    )]
    pub timing_constraint: Vec<TimingConstraint>,
}

impl ConstraintSet {
    pub fn new(constraint_set_id: impl Into<String>) -> Self {
        Self {
            constraint_set_id: Some(constraint_set_id.into()),
            ..Self::default()
        }
    }
}

/// Library cell attached to a drive or load constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellSpecification {
    #[serde(rename = "@cellStrength", skip_serializing_if = "Option::is_none")]
    pub cell_strength: Option<CellStrength>,

    #[serde(rename = "$value")]
    pub kind: CellSpecificationKind,
}

impl CellSpecification {
    pub fn function(function: CellFunction) -> Self {
        Self {
            cell_strength: None,
            kind: CellSpecificationKind::Function(function),
        }
    }

    pub fn class(class: CellClass) -> Self {
        Self {
            cell_strength: None,
            kind: CellSpecificationKind::Class(class),
        }
    }
}

/// Technology-independent cell selection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellSpecificationKind {
    #[serde(rename(serialize = "ipxact:cellFunction", deserialize = "cellFunction"))]
    Function(CellFunction),

    #[serde(rename(serialize = "ipxact:cellClass", deserialize = "cellClass"))]
    Class(CellClass),
}

/// Function and optional custom name of a technology-library cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellFunction {
    #[serde(rename = "@other", skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,

    #[serde(rename = "$text")]
    pub value: CellFunctionValue,
}

impl CellFunction {
    pub fn new(value: CellFunctionValue) -> Self {
        Self { other: None, value }
    }
}

/// Standard technology-library cell function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellFunctionValue {
    #[serde(rename = "nand2")]
    Nand2,
    #[serde(rename = "buf")]
    Buffer,
    #[serde(rename = "inv")]
    Inverter,
    #[serde(rename = "mux21")]
    Mux21,
    #[serde(rename = "dff")]
    Dff,
    #[serde(rename = "latch")]
    Latch,
    #[serde(rename = "xor2")]
    Xor2,
    #[serde(rename = "other")]
    Other,
}

/// Standard technology-library cell class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellClass {
    #[serde(rename = "combinational")]
    Combinational,
    #[serde(rename = "sequential")]
    Sequential,
}

/// Desired strength of a technology-library cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellStrength {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "high")]
    High,
}

/// Input-driving cell attached to a constraint set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveConstraint {
    #[serde(rename(
        serialize = "ipxact:cellSpecification",
        deserialize = "cellSpecification"
    ))]
    pub cell_specification: CellSpecification,
}

impl DriveConstraint {
    pub fn new(cell_specification: CellSpecification) -> Self {
        Self { cell_specification }
    }
}

/// Output-loading cell attached to a constraint set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadConstraint {
    #[serde(rename(
        serialize = "ipxact:cellSpecification",
        deserialize = "cellSpecification"
    ))]
    pub cell_specification: CellSpecification,

    #[serde(
        rename(serialize = "ipxact:count", deserialize = "count"),
        skip_serializing_if = "Option::is_none"
    )]
    pub count: Option<UnsignedPositiveIntExpression>,
}

impl LoadConstraint {
    pub fn new(cell_specification: CellSpecification) -> Self {
        Self {
            cell_specification,
            count: None,
        }
    }

    pub fn with_count(mut self, count: impl Into<UnsignedPositiveIntExpression>) -> Self {
        self.count = Some(count.into());
        self
    }
}

/// Port timing percentage relative to a named clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingConstraint {
    #[serde(rename = "@clockEdge", skip_serializing_if = "Option::is_none")]
    pub clock_edge: Option<ClockEdge>,

    #[serde(rename = "@delayType", skip_serializing_if = "Option::is_none")]
    pub delay_type: Option<DelayType>,

    #[serde(rename = "@clockName")]
    pub clock_name: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: f32,
}

impl TimingConstraint {
    pub fn new(value: f32, clock_name: impl Into<String>) -> Self {
        Self {
            clock_edge: None,
            delay_type: None,
            clock_name: clock_name.into(),
            id: None,
            value,
        }
    }
}

/// Clock edge used by a timing constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClockEdge {
    #[serde(rename = "rise")]
    Rise,
    #[serde(rename = "fall")]
    Fall,
}

/// Minimum or maximum timing-delay interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayType {
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
}

/// Container for wire-port vector dimensions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortVectors {
    #[serde(rename(serialize = "ipxact:vector", deserialize = "vector"), default)]
    pub vector: Vec<PortVector>,
}

impl PortVectors {
    pub fn add(&mut self, vector: PortVector) {
        self.vector.push(vector);
    }
}

/// Single wire-port vector dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortVector {
    #[serde(rename(serialize = "ipxact:left", deserialize = "left"))]
    pub left: UnsignedIntExpression,

    #[serde(rename(serialize = "ipxact:right", deserialize = "right"))]
    pub right: UnsignedIntExpression,
}

impl PortVector {
    pub fn new(
        left: impl Into<UnsignedIntExpression>,
        right: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

/// Initiative of a transactional component port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortInitiative {
    #[serde(rename = "requires")]
    Requires,

    #[serde(rename = "provides")]
    Provides,

    #[serde(rename = "both")]
    Both,

    #[serde(rename = "phantom")]
    Phantom,
}

/// Transactional component port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionalPort {
    #[serde(
        rename = "@allLogicalInitiativesAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_logical_initiatives_allowed: Option<bool>,

    #[serde(rename(serialize = "ipxact:initiative", deserialize = "initiative"))]
    pub initiative: PortInitiative,

    #[serde(
        rename(serialize = "ipxact:kind", deserialize = "kind"),
        skip_serializing_if = "Option::is_none"
    )]
    pub kind: Option<PortKind>,

    #[serde(
        rename(serialize = "ipxact:busWidth", deserialize = "busWidth"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bus_width: Option<UnsignedIntExpression>,

    #[serde(
        rename(serialize = "ipxact:protocol", deserialize = "protocol"),
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<Protocol>,

    #[serde(
        rename(serialize = "ipxact:transTypeDefs", deserialize = "transTypeDefs"),
        skip_serializing_if = "Option::is_none"
    )]
    pub trans_type_defs: Option<Box<TransTypeDefs>>,

    #[serde(
        rename(serialize = "ipxact:connection", deserialize = "connection"),
        skip_serializing_if = "Option::is_none"
    )]
    pub connection: Option<PortConnection>,
}

impl TransactionalPort {
    pub fn new(initiative: PortInitiative) -> Self {
        Self {
            all_logical_initiatives_allowed: None,
            initiative,
            kind: None,
            bus_width: None,
            protocol: None,
            trans_type_defs: None,
            connection: None,
        }
    }
}

/// Transactional port kind, including schema-defined custom values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortKind {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

impl PortKind {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            custom: None,
        }
    }
}

/// Connection-count bounds for a transactional port.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortConnection {
    #[serde(
        rename(serialize = "ipxact:maxConnections", deserialize = "maxConnections"),
        skip_serializing_if = "Option::is_none"
    )]
    pub max_connections: Option<UnsignedIntExpression>,

    #[serde(
        rename(serialize = "ipxact:minConnections", deserialize = "minConnections"),
        skip_serializing_if = "Option::is_none"
    )]
    pub min_connections: Option<UnsignedIntExpression>,
}

/// Container for transactional component-port type definitions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransTypeDefs {
    #[serde(
        rename(serialize = "ipxact:transTypeDef", deserialize = "transTypeDef"),
        default
    )]
    pub trans_type_def: Vec<TransTypeDef>,
}

impl TransTypeDefs {
    pub fn add(&mut self, trans_type_def: TransTypeDef) {
        self.trans_type_def.push(trans_type_def);
    }
}

/// Single transactional type definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransTypeDef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:typeName", deserialize = "typeName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_name: Option<TransactionalTypeName>,

    #[serde(
        rename(serialize = "ipxact:typeDefinition", deserialize = "typeDefinition"),
        default
    )]
    pub type_definition: Vec<TypeDefinition>,

    #[serde(
        rename(serialize = "ipxact:typeParameters", deserialize = "typeParameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_parameters: Option<TypeParameters>,

    #[serde(rename(serialize = "ipxact:viewRef", deserialize = "viewRef"), default)]
    pub view_ref: Vec<TypeDefViewRef>,
}

impl TransTypeDef {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Transactional type name and its exact-match attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionalTypeName {
    #[serde(rename = "@exact", skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl TransactionalTypeName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            exact: None,
            value: value.into(),
        }
    }
}

/// Include or source location that defines a port type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDefinition {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl TypeDefinition {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Type parameters and nested service type definitions for a port type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TypeParameters {
    #[serde(
        rename(serialize = "ipxact:typeParameter", deserialize = "typeParameter"),
        default
    )]
    pub type_parameter: Vec<TypeParameter>,

    #[serde(
        rename(serialize = "ipxact:serviceTypeDef", deserialize = "serviceTypeDef"),
        default
    )]
    pub service_type_def: Vec<ServiceTypeDef>,
}

/// Service type definition nested below transactional type parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceTypeDef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:typeName", deserialize = "typeName"))]
    pub type_name: ServiceTypeName,

    #[serde(
        rename(serialize = "ipxact:typeDefinition", deserialize = "typeDefinition"),
        default
    )]
    pub type_definition: Vec<TypeDefinition>,

    #[serde(
        rename(serialize = "ipxact:typeParameters", deserialize = "typeParameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_parameters: Option<TypeParameters>,
}

impl ServiceTypeDef {
    pub fn new(type_name: ServiceTypeName) -> Self {
        Self {
            id: None,
            type_name,
            type_definition: Vec::new(),
            type_parameters: None,
        }
    }
}

/// Service type name and its implicit declaration attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceTypeName {
    #[serde(rename = "@implicit", skip_serializing_if = "Option::is_none")]
    pub implicit: Option<bool>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl ServiceTypeName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            implicit: None,
            value: value.into(),
        }
    }
}

/// View name reference for a transactional type definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDefViewRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl TypeDefViewRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for master address spaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AddressSpaces {
    #[serde(
        rename(serialize = "ipxact:addressSpace", deserialize = "addressSpace"),
        default
    )]
    pub address_space: Vec<AddressSpace>,
}

impl AddressSpaces {
    pub fn add(&mut self, address_space: AddressSpace) {
        self.address_space.push(address_space);
    }
}

/// Address space referenced by a master bus interface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressSpace {
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

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,

    #[serde(rename(serialize = "ipxact:width", deserialize = "width"))]
    pub width: UnsignedIntExpression,

    #[serde(
        rename(serialize = "ipxact:segments", deserialize = "segments"),
        skip_serializing_if = "Option::is_none"
    )]
    pub segments: Option<Segments>,

    #[serde(
        rename(serialize = "ipxact:addressUnitBits", deserialize = "addressUnitBits"),
        skip_serializing_if = "Option::is_none"
    )]
    pub address_unit_bits: Option<UnsignedPositiveLongintExpression>,

    #[serde(
        rename(serialize = "ipxact:executableImage", deserialize = "executableImage"),
        default
    )]
    pub executable_image: Vec<ExecutableImage>,

    #[serde(
        rename(serialize = "ipxact:localMemoryMap", deserialize = "localMemoryMap"),
        skip_serializing_if = "Option::is_none"
    )]
    pub local_memory_map: Option<LocalMemoryMap>,

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
}

impl AddressSpace {
    pub fn new(
        name: impl Into<String>,
        range: impl Into<UnsignedPositiveLongintExpression>,
        width: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            range: range.into(),
            width: width.into(),
            segments: None,
            address_unit_bits: None,
            executable_image: Vec::new(),
            local_memory_map: None,
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Software image loaded into an address space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutableImage {
    #[serde(rename = "@imageId")]
    pub image_id: String,

    #[serde(rename = "@imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,

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
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(
        rename(serialize = "ipxact:languageTools", deserialize = "languageTools"),
        skip_serializing_if = "Option::is_none"
    )]
    pub language_tools: Option<LanguageTools>,

    #[serde(
        rename(serialize = "ipxact:fileSetRefGroup", deserialize = "fileSetRefGroup"),
        skip_serializing_if = "Option::is_none"
    )]
    pub file_set_ref_group: Option<FileSetRefGroup>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ExecutableImage {
    pub fn new(image_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            image_id: image_id.into(),
            image_type: None,
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            parameters: None,
            language_tools: None,
            file_set_ref_group: None,
            vendor_extensions: None,
        }
    }
}

/// Commands used to build and link an executable image.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LanguageTools {
    pub file_builder: Vec<LanguageFileBuilder>,
    pub linker: Option<LanguageLinker>,
}

/// Schema choice for linker flags and command-file configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageLinker {
    Flags {
        linker: StringExpression,
        linker_flags: StringExpression,
        linker_command_file: Option<LinkerCommandFile>,
    },
    CommandFile {
        linker: StringExpression,
        linker_command_file: LinkerCommandFile,
    },
}

impl LanguageLinker {
    pub fn flags(
        linker: impl Into<StringExpression>,
        linker_flags: impl Into<StringExpression>,
    ) -> Self {
        Self::Flags {
            linker: linker.into(),
            linker_flags: linker_flags.into(),
            linker_command_file: None,
        }
    }

    pub fn flags_with_command_file(
        linker: impl Into<StringExpression>,
        linker_flags: impl Into<StringExpression>,
        linker_command_file: LinkerCommandFile,
    ) -> Self {
        Self::Flags {
            linker: linker.into(),
            linker_flags: linker_flags.into(),
            linker_command_file: Some(linker_command_file),
        }
    }

    pub fn command_file(
        linker: impl Into<StringExpression>,
        linker_command_file: LinkerCommandFile,
    ) -> Self {
        Self::CommandFile {
            linker: linker.into(),
            linker_command_file,
        }
    }
}

impl Serialize for LanguageTools {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:fileBuilder", deserialize = "fileBuilder"),
                default,
                skip_serializing_if = "Vec::is_empty"
            )]
            file_builder: Vec<LanguageFileBuilder>,

            #[serde(
                rename(serialize = "ipxact:linker", deserialize = "linker"),
                skip_serializing_if = "Option::is_none"
            )]
            linker: Option<StringExpression>,

            #[serde(
                rename(serialize = "ipxact:linkerFlags", deserialize = "linkerFlags"),
                skip_serializing_if = "Option::is_none"
            )]
            linker_flags: Option<StringExpression>,

            #[serde(
                rename(
                    serialize = "ipxact:linkerCommandFile",
                    deserialize = "linkerCommandFile"
                ),
                skip_serializing_if = "Option::is_none"
            )]
            linker_command_file: Option<LinkerCommandFile>,
        }

        let (linker, linker_flags, linker_command_file) = match &self.linker {
            None => (None, None, None),
            Some(LanguageLinker::Flags {
                linker,
                linker_flags,
                linker_command_file,
            }) => (
                Some(linker.clone()),
                Some(linker_flags.clone()),
                linker_command_file.clone(),
            ),
            Some(LanguageLinker::CommandFile {
                linker,
                linker_command_file,
            }) => (
                Some(linker.clone()),
                None,
                Some(linker_command_file.clone()),
            ),
        };

        Helper {
            file_builder: self.file_builder.clone(),
            linker,
            linker_flags,
            linker_command_file,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LanguageTools {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:fileBuilder", deserialize = "fileBuilder"),
                default
            )]
            file_builder: Vec<LanguageFileBuilder>,

            #[serde(rename(serialize = "ipxact:linker", deserialize = "linker"), default)]
            linker: Option<StringExpression>,

            #[serde(
                rename(serialize = "ipxact:linkerFlags", deserialize = "linkerFlags"),
                default
            )]
            linker_flags: Option<StringExpression>,

            #[serde(
                rename(
                    serialize = "ipxact:linkerCommandFile",
                    deserialize = "linkerCommandFile"
                ),
                default
            )]
            linker_command_file: Option<LinkerCommandFile>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let linker = match (
            helper.linker,
            helper.linker_flags,
            helper.linker_command_file,
        ) {
            (None, None, None) => None,
            (Some(linker), Some(linker_flags), linker_command_file) => {
                Some(LanguageLinker::Flags {
                    linker,
                    linker_flags,
                    linker_command_file,
                })
            }
            (Some(linker), None, Some(linker_command_file)) => Some(LanguageLinker::CommandFile {
                linker,
                linker_command_file,
            }),
            (None, Some(_), _) | (None, None, Some(_)) => {
                return Err(D::Error::custom(
                    "languageTools linkerFlags/linkerCommandFile require linker",
                ));
            }
            (Some(_), None, None) => {
                return Err(D::Error::custom(
                    "languageTools linker requires linkerFlags or linkerCommandFile",
                ));
            }
        };

        Ok(Self {
            file_builder: helper.file_builder,
            linker,
        })
    }
}

/// Builder for one file type in an executable image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageFileBuilder {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:fileType", deserialize = "fileType"))]
    pub file_type: FileType,

    #[serde(rename(serialize = "ipxact:command", deserialize = "command"))]
    pub command: StringExpression,

    #[serde(
        rename(serialize = "ipxact:flags", deserialize = "flags"),
        skip_serializing_if = "Option::is_none"
    )]
    pub flags: Option<StringExpression>,

    #[serde(
        rename(
            serialize = "ipxact:replaceDefaultFlags",
            deserialize = "replaceDefaultFlags"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_default_flags: Option<BitExpression>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl LanguageFileBuilder {
    pub fn new(file_type: FileType, command: impl Into<StringExpression>) -> Self {
        Self {
            id: None,
            file_type,
            command: command.into(),
            flags: None,
            replace_default_flags: None,
            vendor_extensions: None,
        }
    }
}

/// Linker command-file configuration for an executable image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkerCommandFile {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: StringURIExpression,

    #[serde(rename(
        serialize = "ipxact:commandLineSwitch",
        deserialize = "commandLineSwitch"
    ))]
    pub command_line_switch: StringExpression,

    #[serde(rename(serialize = "ipxact:enable", deserialize = "enable"))]
    pub enable: BitExpression,

    #[serde(
        rename(serialize = "ipxact:generatorRef", deserialize = "generatorRef"),
        default
    )]
    pub generator_ref: Vec<GeneratorRef>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl LinkerCommandFile {
    pub fn new(
        name: impl Into<String>,
        command_line_switch: impl Into<StringExpression>,
        enable: impl Into<BitExpression>,
    ) -> Self {
        Self {
            name: StringURIExpression::new(name),
            command_line_switch: command_line_switch.into(),
            enable: enable.into(),
            generator_ref: Vec::new(),
            vendor_extensions: None,
        }
    }
}

/// Reference to a component-local generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRef {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl GeneratorRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Container for component-local generators.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentGenerators {
    #[serde(
        rename(
            serialize = "ipxact:componentGenerator",
            deserialize = "componentGenerator"
        ),
        default
    )]
    pub component_generator: Vec<ComponentGenerator>,
}

impl ComponentGenerators {
    pub fn add(&mut self, component_generator: ComponentGenerator) {
        self.component_generator.push(component_generator);
    }
}

/// Generator invoked for a component entity or instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentGenerator {
    #[serde(rename = "@hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(rename = "@scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<GeneratorScope>,

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
        rename(serialize = "ipxact:phase", deserialize = "phase"),
        skip_serializing_if = "Option::is_none"
    )]
    pub phase: Option<RealExpression>,

    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(
        rename(serialize = "ipxact:apiType", deserialize = "apiType"),
        skip_serializing_if = "Option::is_none"
    )]
    pub api_type: Option<GeneratorApi>,

    #[serde(
        rename(
            serialize = "ipxact:transportMethods",
            deserialize = "transportMethods"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub transport_methods: Option<TransportMethods>,

    #[serde(rename(serialize = "ipxact:generatorExe", deserialize = "generatorExe"))]
    pub generator_exe: String,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,

    #[serde(rename(serialize = "ipxact:group", deserialize = "group"), default)]
    pub group: Vec<GeneratorGroup>,
}

impl ComponentGenerator {
    pub fn new(name: impl Into<String>, generator_exe: impl Into<String>) -> Self {
        Self {
            hidden: None,
            scope: None,
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            phase: None,
            parameters: None,
            api_type: None,
            transport_methods: None,
            generator_exe: generator_exe.into(),
            vendor_extensions: None,
            group: Vec::new(),
        }
    }
}

/// Instance scope of a component generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneratorScope {
    #[serde(rename = "instance")]
    Instance,
    #[serde(rename = "entity")]
    Entity,
}

/// Floating-point expression used to order generators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealExpression {
    #[serde(flatten)]
    pub extension_attributes: ExtensionAttributes,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl RealExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            extension_attributes: ExtensionAttributes::default(),
            minimum: None,
            maximum: None,
            value: value.into(),
        }
    }
}

/// Generator API selection with optional XML identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorApi {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: GeneratorApiType,
}

impl GeneratorApi {
    pub fn new(value: GeneratorApiType) -> Self {
        Self { id: None, value }
    }
}

/// API exposed to a component generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneratorApiType {
    #[serde(rename = "TGI_2009")]
    Tgi2009,
    #[serde(rename = "TGI_2014_BASE")]
    Tgi2014Base,
    #[serde(rename = "TGI_2014_EXTENDED")]
    Tgi2014Extended,
    #[serde(rename = "none")]
    None,
}

/// Transport method wrapper for a component generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportMethods {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:transportMethod", deserialize = "transportMethod"))]
    pub transport_method: TransportMethod,
}

impl TransportMethods {
    pub fn file() -> Self {
        Self {
            id: None,
            transport_method: TransportMethod {
                id: None,
                value: TransportMethodType::File,
            },
        }
    }
}

/// One generator transport method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportMethod {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: TransportMethodType,
}

/// Generator transport methods defined by IEEE 1685-2014.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMethodType {
    #[serde(rename = "file")]
    File,
}

/// Group label attached to a component generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorGroup {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl GeneratorGroup {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Group of file-set references used to build an executable image.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSetRefGroup {
    #[serde(
        rename(serialize = "ipxact:fileSetRef", deserialize = "fileSetRef"),
        default
    )]
    pub file_set_ref: Vec<FileSetRef>,
}

impl FileSetRefGroup {
    pub fn add(&mut self, file_set_ref: FileSetRef) {
        self.file_set_ref.push(file_set_ref);
    }
}

/// Container for address-space segments.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Segments {
    #[serde(rename(serialize = "ipxact:segment", deserialize = "segment"), default)]
    pub segment: Vec<Segment>,
}

impl Segments {
    pub fn add(&mut self, segment: Segment) {
        self.segment.push(segment);
    }
}

/// Named range inside an address space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
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

    #[serde(rename(serialize = "ipxact:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: UnsignedLongintExpression,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Segment {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<UnsignedLongintExpression>,
        range: impl Into<UnsignedPositiveLongintExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            address_offset: address_offset.into(),
            range: range.into(),
            vendor_extensions: None,
        }
    }
}

/// Memory map private to an address space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalMemoryMap {
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

    #[serde(rename = "$value", default)]
    pub entries: Vec<LocalMemoryMapEntry>,
}

impl LocalMemoryMap {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            entries: Vec::new(),
        }
    }

    pub fn add_address_block(&mut self, address_block: AddressBlock) {
        self.entries
            .push(LocalMemoryMapEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_bank(&mut self, bank: LocalBank) {
        self.entries.push(LocalMemoryMapEntry::Bank(Box::new(bank)));
    }
}

/// Schema choice inside a local memory map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalMemoryMapEntry {
    #[serde(rename(serialize = "ipxact:addressBlock", deserialize = "addressBlock"))]
    AddressBlock(Box<AddressBlock>),

    #[serde(rename(serialize = "ipxact:bank", deserialize = "bank"))]
    Bank(Box<LocalBank>),
}

/// Top-level bank private to an address space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalBank {
    #[serde(rename = "@bankAlignment")]
    pub bank_alignment: BankAlignment,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<SimpleAccessHandles>,

    #[serde(rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"))]
    pub base_address: UnsignedLongintExpression,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename = "$value", default)]
    pub entries: Vec<LocalBankEntry>,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

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
}

impl LocalBank {
    pub fn new(
        name: impl Into<String>,
        base_address: impl Into<UnsignedLongintExpression>,
        bank_alignment: impl Into<BankAlignment>,
    ) -> Self {
        Self {
            bank_alignment: bank_alignment.into(),
            id: None,
            name: name.into(),
            access_handles: None,
            base_address: base_address.into(),
            is_present: None,
            entries: Vec::new(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_address_block(&mut self, address_block: BankedAddressBlock) {
        self.entries
            .push(LocalBankEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_bank(&mut self, bank: LocalBankedBank) {
        self.entries.push(LocalBankEntry::Bank(Box::new(bank)));
    }
}

/// Schema choice inside a local bank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalBankEntry {
    #[serde(rename(serialize = "ipxact:addressBlock", deserialize = "addressBlock"))]
    AddressBlock(Box<BankedAddressBlock>),

    #[serde(rename(serialize = "ipxact:bank", deserialize = "bank"))]
    Bank(Box<LocalBankedBank>),
}

/// Bank nested inside a local bank. Its address is supplied by the parent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalBankedBank {
    #[serde(rename = "@bankAlignment")]
    pub bank_alignment: BankAlignment,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<SimpleAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename = "$value", default)]
    pub entries: Vec<LocalBankEntry>,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

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
}

impl LocalBankedBank {
    pub fn new(name: impl Into<String>, bank_alignment: impl Into<BankAlignment>) -> Self {
        Self {
            bank_alignment: bank_alignment.into(),
            id: None,
            name: name.into(),
            access_handles: None,
            is_present: None,
            entries: Vec::new(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_address_block(&mut self, address_block: BankedAddressBlock) {
        self.entries
            .push(LocalBankEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_bank(&mut self, bank: LocalBankedBank) {
        self.entries.push(LocalBankEntry::Bank(Box::new(bank)));
    }
}

/// Container for slave memory maps.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryMaps {
    #[serde(
        rename(serialize = "ipxact:memoryMap", deserialize = "memoryMap"),
        default
    )]
    pub memory_map: Vec<MemoryMap>,
}

impl MemoryMaps {
    pub fn add(&mut self, memory_map: MemoryMap) {
        self.memory_map.push(memory_map);
    }
}

/// Slave memory map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMap {
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

    #[serde(rename = "$value", default)]
    pub entries: Vec<MemoryMapEntry>,

    #[serde(
        rename(serialize = "ipxact:memoryRemap", deserialize = "memoryRemap"),
        default
    )]
    pub memory_remap: Vec<MemoryRemap>,

    #[serde(
        rename(serialize = "ipxact:addressUnitBits", deserialize = "addressUnitBits"),
        skip_serializing_if = "Option::is_none"
    )]
    pub address_unit_bits: Option<UnsignedPositiveLongintExpression>,

    #[serde(
        rename(serialize = "ipxact:shared", deserialize = "shared"),
        skip_serializing_if = "Option::is_none"
    )]
    pub shared: Option<Shared>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl MemoryMap {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            entries: Vec::new(),
            memory_remap: Vec::new(),
            address_unit_bits: None,
            shared: None,
            vendor_extensions: None,
        }
    }

    pub fn add_address_block(&mut self, address_block: AddressBlock) {
        self.entries
            .push(MemoryMapEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_subspace_map(&mut self, subspace_map: SubspaceMap) {
        self.entries
            .push(MemoryMapEntry::SubspaceMap(Box::new(subspace_map)));
    }

    pub fn add_bank(&mut self, bank: Bank) {
        self.entries.push(MemoryMapEntry::Bank(Box::new(bank)));
    }

    pub fn add_memory_remap(&mut self, memory_remap: MemoryRemap) {
        self.memory_remap.push(memory_remap);
    }
}

/// Sharing policy for a memory-map definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shared {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "undefined")]
    Undefined,
}

/// Alternate memory map selected by a component remap state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryRemap {
    #[serde(rename = "@state")]
    pub state: String,

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

    #[serde(rename = "$value", default)]
    pub entries: Vec<MemoryMapEntry>,
}

impl MemoryRemap {
    pub fn new(name: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            state: state.into(),
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_present: None,
            entries: Vec::new(),
        }
    }

    pub fn add_address_block(&mut self, address_block: AddressBlock) {
        self.entries
            .push(MemoryMapEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_subspace_map(&mut self, subspace_map: SubspaceMap) {
        self.entries
            .push(MemoryMapEntry::SubspaceMap(Box::new(subspace_map)));
    }

    pub fn add_bank(&mut self, bank: Bank) {
        self.entries.push(MemoryMapEntry::Bank(Box::new(bank)));
    }
}

/// A memory map contains a schema choice, not nested `memoryMap` elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryMapEntry {
    #[serde(rename(serialize = "ipxact:addressBlock", deserialize = "addressBlock"))]
    AddressBlock(Box<AddressBlock>),

    #[serde(rename(serialize = "ipxact:bank", deserialize = "bank"))]
    Bank(Box<Bank>),

    #[serde(rename(serialize = "ipxact:subspaceMap", deserialize = "subspaceMap"))]
    SubspaceMap(Box<SubspaceMap>),
}

/// Top-level address block in a memory map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBlock {
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
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<NonIndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"))]
    pub base_address: UnsignedLongintExpression,

    #[serde(
        rename(serialize = "ipxact:typeIdentifier", deserialize = "typeIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,

    #[serde(rename(serialize = "ipxact:width", deserialize = "width"))]
    pub width: UnsignedIntExpression,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(rename = "$value", default)]
    pub register_data: Vec<RegisterData>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl AddressBlock {
    pub fn new(
        name: impl Into<String>,
        base_address: impl Into<UnsignedLongintExpression>,
        range: impl Into<UnsignedPositiveLongintExpression>,
        width: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            access_handles: None,
            is_present: None,
            base_address: base_address.into(),
            type_identifier: None,
            range: range.into(),
            width: width.into(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            register_data: Vec::new(),
            vendor_extensions: None,
        }
    }

    pub fn add_register(&mut self, register: Register) {
        self.register_data.push(RegisterData::Register(register));
    }
}

/// Address subspace mapped through a master bus interface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubspaceMap {
    #[serde(rename = "@masterRef")]
    pub master_ref: String,

    #[serde(rename = "@segmentRef", skip_serializing_if = "Option::is_none")]
    pub segment_ref: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"))]
    pub base_address: UnsignedLongintExpression,

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
}

impl SubspaceMap {
    pub fn new(
        name: impl Into<String>,
        master_ref: impl Into<String>,
        base_address: impl Into<UnsignedLongintExpression>,
    ) -> Self {
        Self {
            master_ref: master_ref.into(),
            segment_ref: None,
            name: name.into(),
            is_present: None,
            base_address: base_address.into(),
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Top-level bank in a memory map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    #[serde(rename = "@bankAlignment")]
    pub bank_alignment: BankAlignment,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<SimpleAccessHandles>,

    #[serde(rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"))]
    pub base_address: UnsignedLongintExpression,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename = "$value", default)]
    pub entries: Vec<BankEntry>,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

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
}

impl Bank {
    pub fn new(
        name: impl Into<String>,
        base_address: impl Into<UnsignedLongintExpression>,
        bank_alignment: impl Into<BankAlignment>,
    ) -> Self {
        Self {
            bank_alignment: bank_alignment.into(),
            id: None,
            name: name.into(),
            access_handles: None,
            base_address: base_address.into(),
            is_present: None,
            entries: Vec::new(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_address_block(&mut self, address_block: BankedAddressBlock) {
        self.entries
            .push(BankEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_bank(&mut self, bank: BankedBank) {
        self.entries.push(BankEntry::Bank(Box::new(bank)));
    }

    pub fn add_subspace_map(&mut self, subspace_map: BankedSubspaceMap) {
        self.entries.push(BankEntry::SubspaceMap(subspace_map));
    }
}

/// Legal alignment modes for banked memory-map entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankAlignment {
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "parallel")]
    Parallel,
}

/// Legal read/write access modes for address blocks, registers, and fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Access {
    #[serde(rename = "read-only")]
    ReadOnly,
    #[serde(rename = "write-only")]
    WriteOnly,
    #[serde(rename = "read-write")]
    ReadWrite,
    #[serde(rename = "writeOnce")]
    WriteOnce,
    #[serde(rename = "read-writeOnce")]
    ReadWriteOnce,
}

/// Legal usage classes for address blocks and banks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryUsage {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "reserved")]
    Reserved,
}

/// Schema choice inside a bank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BankEntry {
    #[serde(rename(serialize = "ipxact:addressBlock", deserialize = "addressBlock"))]
    AddressBlock(Box<BankedAddressBlock>),

    #[serde(rename(serialize = "ipxact:bank", deserialize = "bank"))]
    Bank(Box<BankedBank>),

    #[serde(rename(serialize = "ipxact:subspaceMap", deserialize = "subspaceMap"))]
    SubspaceMap(BankedSubspaceMap),
}

/// Bank nested inside another bank. Its address is supplied by the parent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankedBank {
    #[serde(rename = "@bankAlignment")]
    pub bank_alignment: BankAlignment,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<SimpleAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename = "$value", default)]
    pub entries: Vec<BankEntry>,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

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
}

impl BankedBank {
    pub fn new(name: impl Into<String>, bank_alignment: impl Into<BankAlignment>) -> Self {
        Self {
            bank_alignment: bank_alignment.into(),
            id: None,
            name: name.into(),
            access_handles: None,
            is_present: None,
            entries: Vec::new(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_address_block(&mut self, address_block: BankedAddressBlock) {
        self.entries
            .push(BankEntry::AddressBlock(Box::new(address_block)));
    }

    pub fn add_bank(&mut self, bank: BankedBank) {
        self.entries.push(BankEntry::Bank(Box::new(bank)));
    }

    pub fn add_subspace_map(&mut self, subspace_map: BankedSubspaceMap) {
        self.entries.push(BankEntry::SubspaceMap(subspace_map));
    }
}

/// Address subspace mapped inside a bank. Its address is supplied by the bank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankedSubspaceMap {
    #[serde(rename = "@masterRef")]
    pub master_ref: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:name", deserialize = "name"),
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

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
}

impl BankedSubspaceMap {
    pub fn new(master_ref: impl Into<String>) -> Self {
        Self {
            master_ref: master_ref.into(),
            id: None,
            name: None,
            is_present: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Address block nested inside a bank. Its address is supplied by the bank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankedAddressBlock {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<NonIndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,

    #[serde(rename(serialize = "ipxact:width", deserialize = "width"))]
    pub width: UnsignedIntExpression,

    #[serde(
        rename(serialize = "ipxact:usage", deserialize = "usage"),
        skip_serializing_if = "Option::is_none"
    )]
    pub usage: Option<MemoryUsage>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

    #[serde(rename = "$value", default)]
    pub register_data: Vec<RegisterData>,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl BankedAddressBlock {
    pub fn new(
        name: impl Into<String>,
        range: impl Into<UnsignedPositiveLongintExpression>,
        width: impl Into<UnsignedIntExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            access_handles: None,
            is_present: None,
            range: range.into(),
            width: width.into(),
            usage: None,
            is_volatile: None,
            access: None,
            parameters: None,
            register_data: Vec::new(),
            vendor_extensions: None,
        }
    }

    pub fn add_register(&mut self, register: Register) {
        self.register_data.push(RegisterData::Register(register));
    }
}

/// Register-data schema choice inside an address block or register file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegisterData {
    #[serde(rename(serialize = "ipxact:register", deserialize = "register"))]
    Register(Register),

    #[serde(rename(serialize = "ipxact:registerFile", deserialize = "registerFile"))]
    RegisterFile(RegisterFile),
}

/// One dimension of a register or register-file array.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterDim {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl RegisterDim {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            minimum: None,
            maximum: None,
            value: value.into(),
        }
    }
}

impl From<&str> for RegisterDim {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for RegisterDim {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Nested register file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterFile {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<IndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:dim", deserialize = "dim"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dim: Vec<RegisterDim>,

    #[serde(rename(serialize = "ipxact:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: UnsignedLongintExpression,

    #[serde(
        rename(serialize = "ipxact:typeIdentifier", deserialize = "typeIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: UnsignedPositiveLongintExpression,

    #[serde(rename = "$value", default)]
    pub register_data: Vec<RegisterData>,

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
}

impl RegisterFile {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<UnsignedLongintExpression>,
        range: impl Into<UnsignedPositiveLongintExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            access_handles: None,
            is_present: None,
            dim: Vec::new(),
            address_offset: address_offset.into(),
            type_identifier: None,
            range: range.into(),
            register_data: Vec::new(),
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_register(&mut self, register: Register) {
        self.register_data.push(RegisterData::Register(register));
    }
}

/// Register instance and definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
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
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<IndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:dim", deserialize = "dim"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dim: Vec<RegisterDim>,

    #[serde(rename(serialize = "ipxact:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: UnsignedLongintExpression,

    #[serde(
        rename(serialize = "ipxact:typeIdentifier", deserialize = "typeIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,

    #[serde(rename(serialize = "ipxact:size", deserialize = "size"))]
    pub size: UnsignedPositiveIntExpression,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

    #[serde(rename(serialize = "ipxact:field", deserialize = "field"), default)]
    pub field: Vec<Field>,

    #[serde(
        rename(
            serialize = "ipxact:alternateRegisters",
            deserialize = "alternateRegisters"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub alternate_registers: Option<AlternateRegisters>,

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
}

impl Register {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<UnsignedLongintExpression>,
        size: impl Into<UnsignedPositiveIntExpression>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            access_handles: None,
            is_present: None,
            dim: Vec::new(),
            address_offset: address_offset.into(),
            type_identifier: None,
            size: size.into(),
            is_volatile: None,
            access: None,
            field: Vec::new(),
            alternate_registers: None,
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.field.push(field);
    }
}

/// Alternate definitions for a register.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AlternateRegisters {
    #[serde(
        rename(
            serialize = "ipxact:alternateRegister",
            deserialize = "alternateRegister"
        ),
        default
    )]
    pub alternate_register: Vec<AlternateRegister>,
}

impl AlternateRegisters {
    pub fn add(&mut self, alternate_register: AlternateRegister) {
        self.alternate_register.push(alternate_register);
    }
}

/// Register definition selected by one or more alternate groups.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternateRegister {
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
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<IndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:alternateGroups", deserialize = "alternateGroups"))]
    pub alternate_groups: AlternateGroups,

    #[serde(
        rename(serialize = "ipxact:typeIdentifier", deserialize = "typeIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

    #[serde(rename(serialize = "ipxact:field", deserialize = "field"), default)]
    pub field: Vec<Field>,

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
}

impl AlternateRegister {
    pub fn new(name: impl Into<String>, alternate_groups: AlternateGroups) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            access_handles: None,
            is_present: None,
            alternate_groups,
            type_identifier: None,
            is_volatile: None,
            access: None,
            field: Vec::new(),
            parameters: None,
            vendor_extensions: None,
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.field.push(field);
    }
}

/// Group selectors shared by alternate register definitions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlternateGroups {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:alternateGroup", deserialize = "alternateGroup"),
        default
    )]
    pub alternate_group: Vec<AlternateGroup>,
}

impl AlternateGroups {
    pub fn new(alternate_group: impl Into<String>) -> Self {
        Self {
            id: None,
            alternate_group: vec![AlternateGroup::new(alternate_group)],
        }
    }

    pub fn add(&mut self, alternate_group: impl Into<String>) {
        self.alternate_group
            .push(AlternateGroup::new(alternate_group));
    }
}

/// Single alternate-register group selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlternateGroup {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

impl AlternateGroup {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id: None,
            value: value.into(),
        }
    }
}

/// Field within a register.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "@fieldID", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,

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
        rename(serialize = "ipxact:accessHandles", deserialize = "accessHandles"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access_handles: Option<NonIndexedAccessHandles>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:bitOffset", deserialize = "bitOffset"))]
    pub bit_offset: UnsignedIntExpression,

    #[serde(
        rename(serialize = "ipxact:resets", deserialize = "resets"),
        skip_serializing_if = "Option::is_none"
    )]
    pub resets: Option<Resets>,

    #[serde(
        rename(serialize = "ipxact:typeIdentifier", deserialize = "typeIdentifier"),
        skip_serializing_if = "Option::is_none"
    )]
    pub type_identifier: Option<String>,

    #[serde(rename(serialize = "ipxact:bitWidth", deserialize = "bitWidth"))]
    pub bit_width: UnsignedPositiveIntExpression,

    #[serde(
        rename(serialize = "ipxact:volatile", deserialize = "volatile"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_volatile: Option<bool>,

    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<Access>,

    #[serde(
        rename(
            serialize = "ipxact:enumeratedValues",
            deserialize = "enumeratedValues"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub enumerated_values: Option<EnumeratedValues>,

    #[serde(
        rename(
            serialize = "ipxact:modifiedWriteValue",
            deserialize = "modifiedWriteValue"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_write_value: Option<ModifiedWriteValue>,

    #[serde(
        rename(
            serialize = "ipxact:writeValueConstraint",
            deserialize = "writeValueConstraint"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub write_value_constraint: Option<WriteValueConstraint>,

    #[serde(
        rename(serialize = "ipxact:readAction", deserialize = "readAction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub read_action: Option<ReadAction>,

    #[serde(
        rename(serialize = "ipxact:testable", deserialize = "testable"),
        skip_serializing_if = "Option::is_none"
    )]
    pub testable: Option<Testable>,

    #[serde(
        rename(serialize = "ipxact:reserved", deserialize = "reserved"),
        skip_serializing_if = "Option::is_none"
    )]
    pub reserved: Option<BitExpression>,

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
}

impl Field {
    pub fn new(
        name: impl Into<String>,
        bit_offset: impl Into<UnsignedIntExpression>,
        bit_width: impl Into<UnsignedPositiveIntExpression>,
    ) -> Self {
        Self {
            id: None,
            field_id: None,
            name: name.into(),
            display_name: None,
            description: None,
            access_handles: None,
            is_present: None,
            bit_offset: bit_offset.into(),
            resets: None,
            type_identifier: None,
            bit_width: bit_width.into(),
            is_volatile: None,
            access: None,
            enumerated_values: None,
            modified_write_value: None,
            write_value_constraint: None,
            read_action: None,
            testable: None,
            reserved: None,
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Enumerated values accepted or produced by a field.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValues {
    #[serde(
        rename(serialize = "ipxact:enumeratedValue", deserialize = "enumeratedValue"),
        default
    )]
    pub enumerated_value: Vec<EnumeratedValue>,
}

/// Single field enumeration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValue {
    #[serde(rename = "@usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<EnumeratedValueUsage>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: UnsignedBitVectorExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

/// Legal read/write applicability for one enumerated field value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnumeratedValueUsage {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read-write")]
    ReadWrite,
}

impl EnumeratedValue {
    pub fn new(name: impl Into<String>, value: impl Into<UnsignedBitVectorExpression>) -> Self {
        Self {
            usage: None,
            id: None,
            name: name.into(),
            value: value.into(),
            vendor_extensions: None,
        }
    }
}

/// Legal values that may be written to a field.
#[derive(Debug, Clone, PartialEq)]
pub struct WriteValueConstraint {
    pub choice: WriteValueConstraintChoice,
}

/// Schema choice inside `writeValueConstraint`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WriteValueConstraintChoice {
    WriteAsRead {
        #[serde(rename(serialize = "ipxact:writeAsRead", deserialize = "writeAsRead"))]
        value: bool,
    },
    UseEnumeratedValues {
        #[serde(rename(
            serialize = "ipxact:useEnumeratedValues",
            deserialize = "useEnumeratedValues"
        ))]
        value: bool,
    },
    Range(WriteValueRange),
}

/// Inclusive write-value range for a field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteValueRange {
    #[serde(rename(serialize = "ipxact:minimum", deserialize = "minimum"))]
    pub minimum: UnsignedBitVectorExpression,

    #[serde(rename(serialize = "ipxact:maximum", deserialize = "maximum"))]
    pub maximum: UnsignedBitVectorExpression,
}

impl WriteValueConstraint {
    pub fn write_as_read(value: bool) -> Self {
        Self {
            choice: WriteValueConstraintChoice::WriteAsRead { value },
        }
    }

    pub fn use_enumerated_values(value: bool) -> Self {
        Self {
            choice: WriteValueConstraintChoice::UseEnumeratedValues { value },
        }
    }

    pub fn range(
        minimum: impl Into<UnsignedBitVectorExpression>,
        maximum: impl Into<UnsignedBitVectorExpression>,
    ) -> Self {
        Self {
            choice: WriteValueConstraintChoice::Range(WriteValueRange {
                minimum: minimum.into(),
                maximum: maximum.into(),
            }),
        }
    }
}

impl Serialize for WriteValueConstraint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a> {
            #[serde(
                rename(serialize = "ipxact:writeAsRead", deserialize = "writeAsRead"),
                skip_serializing_if = "Option::is_none"
            )]
            write_as_read: Option<bool>,

            #[serde(
                rename(
                    serialize = "ipxact:useEnumeratedValues",
                    deserialize = "useEnumeratedValues"
                ),
                skip_serializing_if = "Option::is_none"
            )]
            use_enumerated_values: Option<bool>,

            #[serde(
                rename(serialize = "ipxact:minimum", deserialize = "minimum"),
                skip_serializing_if = "Option::is_none"
            )]
            minimum: Option<&'a UnsignedBitVectorExpression>,

            #[serde(
                rename(serialize = "ipxact:maximum", deserialize = "maximum"),
                skip_serializing_if = "Option::is_none"
            )]
            maximum: Option<&'a UnsignedBitVectorExpression>,
        }

        let helper = match &self.choice {
            WriteValueConstraintChoice::WriteAsRead { value } => Helper {
                write_as_read: Some(*value),
                use_enumerated_values: None,
                minimum: None,
                maximum: None,
            },
            WriteValueConstraintChoice::UseEnumeratedValues { value } => Helper {
                write_as_read: None,
                use_enumerated_values: Some(*value),
                minimum: None,
                maximum: None,
            },
            WriteValueConstraintChoice::Range(range) => Helper {
                write_as_read: None,
                use_enumerated_values: None,
                minimum: Some(&range.minimum),
                maximum: Some(&range.maximum),
            },
        };

        helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WriteValueConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(
                rename(serialize = "ipxact:writeAsRead", deserialize = "writeAsRead"),
                default
            )]
            write_as_read: Option<bool>,

            #[serde(
                rename(
                    serialize = "ipxact:useEnumeratedValues",
                    deserialize = "useEnumeratedValues"
                ),
                default
            )]
            use_enumerated_values: Option<bool>,

            #[serde(rename(serialize = "ipxact:minimum", deserialize = "minimum"), default)]
            minimum: Option<UnsignedBitVectorExpression>,

            #[serde(rename(serialize = "ipxact:maximum", deserialize = "maximum"), default)]
            maximum: Option<UnsignedBitVectorExpression>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let branches = usize::from(helper.write_as_read.is_some())
            + usize::from(helper.use_enumerated_values.is_some())
            + usize::from(helper.minimum.is_some() || helper.maximum.is_some());

        if branches != 1 {
            return Err(D::Error::custom(
                "writeValueConstraint must contain exactly one schema choice",
            ));
        }

        if let Some(value) = helper.write_as_read {
            return Ok(Self::write_as_read(value));
        }

        if let Some(value) = helper.use_enumerated_values {
            return Ok(Self::use_enumerated_values(value));
        }

        match (helper.minimum, helper.maximum) {
            (Some(minimum), Some(maximum)) => Ok(Self::range(minimum, maximum)),
            _ => Err(D::Error::custom(
                "writeValueConstraint range choice requires minimum and maximum",
            )),
        }
    }
}

/// Write side effect of a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifiedWriteValue {
    #[serde(rename = "$text")]
    pub value: ModifiedWriteValueKind,

    #[serde(rename = "@modify", skip_serializing_if = "Option::is_none")]
    pub modify: Option<String>,
}

impl ModifiedWriteValue {
    pub fn new(value: ModifiedWriteValueKind) -> Self {
        Self {
            value,
            modify: None,
        }
    }

    pub fn with_modify(mut self, modify: impl Into<String>) -> Self {
        self.modify = Some(modify.into());
        self
    }
}

/// Legal write-side effects for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifiedWriteValueKind {
    #[serde(rename = "oneToClear")]
    OneToClear,
    #[serde(rename = "oneToSet")]
    OneToSet,
    #[serde(rename = "oneToToggle")]
    OneToToggle,
    #[serde(rename = "zeroToClear")]
    ZeroToClear,
    #[serde(rename = "zeroToSet")]
    ZeroToSet,
    #[serde(rename = "zeroToToggle")]
    ZeroToToggle,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "modify")]
    Modify,
}

/// Read side effect of a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadAction {
    #[serde(rename = "$text")]
    pub value: ReadActionKind,

    #[serde(rename = "@modify", skip_serializing_if = "Option::is_none")]
    pub modify: Option<String>,
}

impl ReadAction {
    pub fn new(value: ReadActionKind) -> Self {
        Self {
            value,
            modify: None,
        }
    }

    pub fn with_modify(mut self, modify: impl Into<String>) -> Self {
        self.modify = Some(modify.into());
        self
    }
}

/// Legal read-side effects for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadActionKind {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "modify")]
    Modify,
}

/// Automated register-test policy for a field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Testable {
    #[serde(rename = "$text")]
    pub value: bool,

    #[serde(rename = "@testConstraint", skip_serializing_if = "Option::is_none")]
    pub test_constraint: Option<TestConstraint>,
}

impl Testable {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            test_constraint: None,
        }
    }

    pub fn with_constraint(mut self, test_constraint: TestConstraint) -> Self {
        self.test_constraint = Some(test_constraint);
        self
    }
}

/// Legal automated register-test constraints for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestConstraint {
    #[serde(rename = "unconstrained")]
    Unconstrained,
    #[serde(rename = "restore")]
    Restore,
    #[serde(rename = "writeAsRead")]
    WriteAsRead,
    #[serde(rename = "readOnly")]
    ReadOnly,
}

/// Field reset values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Resets {
    #[serde(rename(serialize = "ipxact:reset", deserialize = "reset"), default)]
    pub reset: Vec<Reset>,
}

/// Single field reset value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename = "@resetTypeRef", skip_serializing_if = "Option::is_none")]
    pub reset_type_ref: Option<String>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: UnsignedBitVectorExpression,

    #[serde(
        rename(serialize = "ipxact:mask", deserialize = "mask"),
        skip_serializing_if = "Option::is_none"
    )]
    pub mask: Option<UnsignedBitVectorExpression>,
}

impl Reset {
    pub fn new(value: impl Into<UnsignedBitVectorExpression>) -> Self {
        Self {
            reset_type_ref: None,
            id: None,
            value: value.into(),
            mask: None,
        }
    }
}

/// Container for component-specific reset policies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResetTypes {
    #[serde(
        rename(serialize = "ipxact:resetType", deserialize = "resetType"),
        default
    )]
    pub reset_type: Vec<ResetType>,
}

impl ResetTypes {
    pub fn add(&mut self, reset_type: ResetType) {
        self.reset_type.push(reset_type);
    }
}

/// User-defined reset policy referenced by field reset values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetType {
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
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ResetType {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            vendor_extensions: None,
        }
    }
}
