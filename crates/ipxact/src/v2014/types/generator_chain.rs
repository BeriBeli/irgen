//! IEEE 1685-2014 generator-chain root document.

use serde::{Deserialize, Serialize};

use super::assertions::Assertions;
use super::component::{
    Choices, ConfigurableLibraryRef, GeneratorApi, GeneratorGroup, NAMESPACE, Parameters,
    RealExpression, SCHEMA_LOCATION, TransportMethods, XSI_NAMESPACE,
};
use super::string_expression::StringURIExpression;
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

/// Root element for an IEEE 1685-2014 generator-chain document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:generatorChain", deserialize = "generatorChain"))]
pub struct GeneratorChain {
    #[serde(rename = "@xmlns:ipxact", default = "namespace")]
    pub xmlns_ipxact: String,

    #[serde(rename = "@xmlns:xsi", default = "xsi_namespace")]
    pub xmlns_xsi: String,

    #[serde(
        rename(serialize = "@xsi:schemaLocation", deserialize = "@schemaLocation"),
        default = "schema_location"
    )]
    pub schema_location: String,

    #[serde(rename = "@hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

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

    #[serde(rename = "$value", default)]
    pub entry: Vec<GeneratorChainEntry>,

    #[serde(
        rename(serialize = "ipxact:chainGroup", deserialize = "chainGroup"),
        default
    )]
    pub chain_group: Vec<GeneratorGroup>,

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
        rename(serialize = "ipxact:choices", deserialize = "choices"),
        skip_serializing_if = "Option::is_none"
    )]
    pub choices: Option<Choices>,

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

impl GeneratorChain {
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
            hidden: None,
            id: None,
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
            entry: Vec::new(),
            chain_group: Vec::new(),
            display_name: None,
            description: None,
            choices: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    pub fn add(&mut self, entry: impl Into<GeneratorChainEntry>) {
        self.entry.push(entry.into());
    }

    /// Parse a generator chain while preserving qualified names inside vendor
    /// extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// One ordered selection or generator invocation in a chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeneratorChainEntry {
    #[serde(rename(
        serialize = "ipxact:generatorChainSelector",
        deserialize = "generatorChainSelector"
    ))]
    GeneratorChainSelector(GeneratorChainSelector),

    #[serde(rename(
        serialize = "ipxact:componentGeneratorSelector",
        deserialize = "componentGeneratorSelector"
    ))]
    ComponentGeneratorSelector(ComponentGeneratorSelector),

    #[serde(rename(serialize = "ipxact:generator", deserialize = "generator"))]
    Generator(Box<ChainGenerator>),
}

impl From<GeneratorChainSelector> for GeneratorChainEntry {
    fn from(value: GeneratorChainSelector) -> Self {
        Self::GeneratorChainSelector(value)
    }
}

impl From<ComponentGeneratorSelector> for GeneratorChainEntry {
    fn from(value: ComponentGeneratorSelector) -> Self {
        Self::ComponentGeneratorSelector(value)
    }
}

impl From<ChainGenerator> for GeneratorChainEntry {
    fn from(value: ChainGenerator) -> Self {
        Self::Generator(Box::new(value))
    }
}

/// Selector for another generator-chain document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorChainSelector {
    #[serde(rename = "@unique", skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "$value")]
    pub selection: GeneratorChainSelection,
}

impl GeneratorChainSelector {
    pub fn groups(group_selector: GroupSelector) -> Self {
        Self {
            unique: None,
            id: None,
            selection: GeneratorChainSelection::GroupSelector(group_selector),
        }
    }

    pub fn chain(generator_chain_ref: ConfigurableLibraryRef) -> Self {
        Self {
            unique: None,
            id: None,
            selection: GeneratorChainSelection::GeneratorChainRef(generator_chain_ref),
        }
    }
}

/// Selection criterion for another generator chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeneratorChainSelection {
    #[serde(rename(serialize = "ipxact:groupSelector", deserialize = "groupSelector"))]
    GroupSelector(GroupSelector),

    #[serde(rename(
        serialize = "ipxact:generatorChainRef",
        deserialize = "generatorChainRef"
    ))]
    GeneratorChainRef(ConfigurableLibraryRef),
}

/// Selector for component-local generators in the current design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentGeneratorSelector {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:groupSelector", deserialize = "groupSelector"))]
    pub group_selector: GroupSelector,
}

impl ComponentGeneratorSelector {
    pub fn new(group_selector: GroupSelector) -> Self {
        Self {
            id: None,
            group_selector,
        }
    }
}

/// Names used to select grouped chains or component-local generators.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupSelector {
    #[serde(
        rename(
            serialize = "@multipleGroupSelectionOperator",
            deserialize = "@multipleGroupSelectionOperator"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub multiple_group_selection_operator: Option<MultipleGroupSelectionOperator>,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"), default)]
    pub name: Vec<GeneratorGroup>,
}

impl GroupSelector {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            multiple_group_selection_operator: None,
            id: None,
            name: vec![GeneratorGroup::new(name)],
        }
    }

    pub fn add(&mut self, name: impl Into<String>) {
        self.name.push(GeneratorGroup::new(name));
    }
}

/// Group combination policy for generator selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultipleGroupSelectionOperator {
    #[serde(rename = "and")]
    And,

    #[serde(rename = "or")]
    Or,
}

/// Generator invocation embedded directly in a generator chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainGenerator {
    #[serde(rename = "@hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

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
    pub generator_exe: StringURIExpression,

    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ChainGenerator {
    pub fn new(name: impl Into<String>, generator_exe: impl Into<String>) -> Self {
        Self {
            hidden: None,
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            phase: None,
            parameters: None,
            api_type: None,
            transport_methods: None,
            generator_exe: StringURIExpression::new(generator_exe),
            vendor_extensions: None,
        }
    }
}
