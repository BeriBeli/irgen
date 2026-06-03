//! IEEE 1685-2014 design-configuration root document.

use serde::{Deserialize, Serialize};

use super::assertions::Assertions;
use super::component::{
    BitExpression, ConfigurableElementValues, ConfigurableLibraryRef, NAMESPACE, Parameters,
    SCHEMA_LOCATION, XSI_NAMESPACE,
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

/// Root element for an IEEE 1685-2014 design-configuration document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(
    serialize = "ipxact:designConfiguration",
    deserialize = "designConfiguration"
))]
pub struct DesignConfiguration {
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
        rename(serialize = "ipxact:designRef", deserialize = "designRef"),
        skip_serializing_if = "Option::is_none"
    )]
    pub design_ref: Option<LibraryRefType>,

    #[serde(
        rename(
            serialize = "ipxact:generatorChainConfiguration",
            deserialize = "generatorChainConfiguration"
        ),
        default
    )]
    pub generator_chain_configuration: Vec<ConfigurableLibraryRef>,

    #[serde(
        rename(
            serialize = "ipxact:interconnectionConfiguration",
            deserialize = "interconnectionConfiguration"
        ),
        default
    )]
    pub interconnection_configuration: Vec<InterconnectionConfiguration>,

    #[serde(
        rename(
            serialize = "ipxact:viewConfiguration",
            deserialize = "viewConfiguration"
        ),
        default
    )]
    pub view_configuration: Vec<ViewConfiguration>,

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

impl DesignConfiguration {
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
            design_ref: None,
            generator_chain_configuration: Vec::new(),
            interconnection_configuration: Vec::new(),
            view_configuration: Vec::new(),
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    /// Parse a design configuration while preserving qualified names inside
    /// vendor extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// Abstractor chains selected for one design interconnection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterconnectionConfiguration {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(
        serialize = "ipxact:interconnectionRef",
        deserialize = "interconnectionRef"
    ))]
    pub interconnection_ref: String,

    #[serde(
        rename(
            serialize = "ipxact:abstractorInstances",
            deserialize = "abstractorInstances"
        ),
        default
    )]
    pub abstractor_instances: Vec<AbstractorInstances>,
}

impl InterconnectionConfiguration {
    pub fn new(interconnection_ref: impl Into<String>) -> Self {
        Self {
            id: None,
            is_present: None,
            interconnection_ref: interconnection_ref.into(),
            abstractor_instances: Vec::new(),
        }
    }
}

/// One ordered abstractor chain, optionally scoped to broadcast interfaces.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractorInstances {
    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(
        rename(serialize = "ipxact:interfaceRef", deserialize = "interfaceRef"),
        default
    )]
    pub interface_ref: Vec<InterfaceRef>,

    #[serde(
        rename(
            serialize = "ipxact:abstractorInstance",
            deserialize = "abstractorInstance"
        ),
        default
    )]
    pub abstractor_instance: Vec<AbstractorInstance>,
}

impl AbstractorInstances {
    pub fn add(&mut self, instance: AbstractorInstance) {
        self.abstractor_instance.push(instance);
    }
}

/// Broadcast endpoint to which an abstractor chain applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceRef {
    #[serde(rename = "@componentRef")]
    pub component_ref: String,

    #[serde(rename = "@busRef")]
    pub bus_ref: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl InterfaceRef {
    pub fn new(component_ref: impl Into<String>, bus_ref: impl Into<String>) -> Self {
        Self {
            component_ref: component_ref.into(),
            bus_ref: bus_ref.into(),
            is_present: None,
        }
    }
}

/// One configured abstractor instance in an ordered chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractorInstance {
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

    #[serde(rename(serialize = "ipxact:abstractorRef", deserialize = "abstractorRef"))]
    pub abstractor_ref: ConfigurableLibraryRef,

    #[serde(rename(serialize = "ipxact:viewName", deserialize = "viewName"))]
    pub view_name: String,
}

impl AbstractorInstance {
    pub fn new(
        instance_name: impl Into<String>,
        abstractor_ref: ConfigurableLibraryRef,
        view_name: impl Into<String>,
    ) -> Self {
        Self {
            id: None,
            instance_name: instance_name.into(),
            display_name: None,
            description: None,
            abstractor_ref,
            view_name: view_name.into(),
        }
    }
}

/// Active component view selected for one design instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewConfiguration {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename(serialize = "ipxact:instanceName", deserialize = "instanceName"))]
    pub instance_name: String,

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,

    #[serde(rename(serialize = "ipxact:view", deserialize = "view"))]
    pub view: ViewSelection,
}

impl ViewConfiguration {
    pub fn new(instance_name: impl Into<String>, view_ref: impl Into<String>) -> Self {
        Self {
            id: None,
            instance_name: instance_name.into(),
            is_present: None,
            view: ViewSelection::new(view_ref),
        }
    }
}

/// Selected configured view and optional parameter overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewSelection {
    #[serde(rename = "@viewRef")]
    pub view_ref: String,

    #[serde(
        rename(
            serialize = "ipxact:configurableElementValues",
            deserialize = "configurableElementValues"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub configurable_element_values: Option<ConfigurableElementValues>,
}

impl ViewSelection {
    pub fn new(view_ref: impl Into<String>) -> Self {
        Self {
            view_ref: view_ref.into(),
            configurable_element_values: None,
        }
    }
}
