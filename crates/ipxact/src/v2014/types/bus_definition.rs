//! IEEE 1685-2014 bus-definition root document.

use serde::{Deserialize, Serialize};

use super::Parameters;
use super::assertions::Assertions;
use super::component::{NAMESPACE, SCHEMA_LOCATION, XSI_NAMESPACE};
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

/// Root element for an IEEE 1685-2014 bus-definition document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:busDefinition", deserialize = "busDefinition"))]
pub struct BusDefinition {
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

    #[serde(rename(
        serialize = "ipxact:directConnection",
        deserialize = "directConnection"
    ))]
    pub direct_connection: bool,

    #[serde(
        rename(serialize = "ipxact:broadcast", deserialize = "broadcast"),
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast: Option<bool>,

    #[serde(rename(serialize = "ipxact:isAddressable", deserialize = "isAddressable"))]
    pub is_addressable: bool,

    #[serde(
        rename(serialize = "ipxact:extends", deserialize = "extends"),
        skip_serializing_if = "Option::is_none"
    )]
    pub extends: Option<LibraryRefType>,

    #[serde(
        rename(serialize = "ipxact:maxMasters", deserialize = "maxMasters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub max_masters: Option<UnsignedIntExpression>,

    #[serde(
        rename(serialize = "ipxact:maxSlaves", deserialize = "maxSlaves"),
        skip_serializing_if = "Option::is_none"
    )]
    pub max_slaves: Option<UnsignedIntExpression>,

    #[serde(
        rename(
            serialize = "ipxact:systemGroupNames",
            deserialize = "systemGroupNames"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub system_group_names: Option<SystemGroupNames>,

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

impl BusDefinition {
    pub fn new(
        vendor: impl Into<String>,
        library: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        direct_connection: bool,
        is_addressable: bool,
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
            direct_connection,
            broadcast: None,
            is_addressable,
            extends: None,
            max_masters: None,
            max_slaves: None,
            system_group_names: None,
            description: None,
            parameters: None,
            assertions: None,
            vendor_extensions: None,
        }
    }

    /// Parse a bus definition while preserving qualified names inside vendor
    /// extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }
}

/// Unsigned integer expression with optional resolver bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsignedIntExpression {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "@maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
}

impl UnsignedIntExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            minimum: None,
            maximum: None,
        }
    }
}

impl From<&str> for UnsignedIntExpression {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for UnsignedIntExpression {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Container for valid system-interface group names.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemGroupNames {
    #[serde(
        rename(serialize = "ipxact:systemGroupName", deserialize = "systemGroupName"),
        default
    )]
    pub system_group_name: Vec<SystemGroupName>,
}

impl SystemGroupNames {
    pub fn add(&mut self, system_group_name: SystemGroupName) {
        self.system_group_name.push(system_group_name);
    }
}

/// One system-interface group name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemGroupName {
    #[serde(rename = "$text")]
    pub value: String,

    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SystemGroupName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            id: None,
        }
    }
}
