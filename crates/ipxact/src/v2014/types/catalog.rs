//! IP-XACT Catalog type

use serde::{Deserialize, Serialize};

use super::component::{NAMESPACE, SCHEMA_LOCATION, XSI_NAMESPACE};
use super::ipxact_files::IpxactFiles;
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

/// Catalog - root element for IP-XACT catalog files
///
/// Maps to XML schema `catalogType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:catalog", deserialize = "catalog"))]
pub struct Catalog {
    #[serde(rename = "@xmlns:ipxact", default = "namespace")]
    pub xmlns_ipxact: String,

    #[serde(rename = "@xmlns:xsi", default = "xsi_namespace")]
    pub xmlns_xsi: String,

    #[serde(
        rename(serialize = "@xsi:schemaLocation", deserialize = "@schemaLocation"),
        default = "schema_location"
    )]
    pub schema_location: String,

    /// Vendor name (required)
    #[serde(rename(serialize = "ipxact:vendor", deserialize = "vendor"))]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename(serialize = "ipxact:library", deserialize = "library"))]
    pub library: String,

    /// Component name (required)
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    /// Version (required)
    #[serde(rename(serialize = "ipxact:version", deserialize = "version"))]
    pub version: String,

    /// Description
    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// Nested catalogs
    #[serde(
        rename(serialize = "ipxact:catalogs", deserialize = "catalogs"),
        skip_serializing_if = "Option::is_none"
    )]
    pub catalogs: Option<IpxactFiles>,

    /// Bus definitions
    #[serde(
        rename(serialize = "ipxact:busDefinitions", deserialize = "busDefinitions"),
        skip_serializing_if = "Option::is_none"
    )]
    pub bus_definitions: Option<IpxactFiles>,

    /// Abstraction definitions
    #[serde(
        rename(
            serialize = "ipxact:abstractionDefinitions",
            deserialize = "abstractionDefinitions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub abstraction_definitions: Option<IpxactFiles>,

    /// Components
    #[serde(
        rename(serialize = "ipxact:components", deserialize = "components"),
        skip_serializing_if = "Option::is_none"
    )]
    pub components: Option<IpxactFiles>,

    /// Abstractors
    #[serde(
        rename(serialize = "ipxact:abstractors", deserialize = "abstractors"),
        skip_serializing_if = "Option::is_none"
    )]
    pub abstractors: Option<IpxactFiles>,

    /// Designs
    #[serde(
        rename(serialize = "ipxact:designs", deserialize = "designs"),
        skip_serializing_if = "Option::is_none"
    )]
    pub designs: Option<IpxactFiles>,

    /// Design configurations
    #[serde(
        rename(
            serialize = "ipxact:designConfigurations",
            deserialize = "designConfigurations"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub design_configurations: Option<IpxactFiles>,

    /// Generator chains
    #[serde(
        rename(serialize = "ipxact:generatorChains", deserialize = "generatorChains"),
        skip_serializing_if = "Option::is_none"
    )]
    pub generator_chains: Option<IpxactFiles>,

    /// Vendor extensions
    #[serde(
        rename(
            serialize = "ipxact:vendorExtensions",
            deserialize = "vendorExtensions"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Catalog {
    /// Parse a catalog while preserving qualified names inside vendor
    /// extensions.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let xml = protect_qnames(xml)?;
        quick_xml::de::from_str(&xml).map_err(|error| crate::Error::Parse(error.to_string()))
    }

    pub fn new(vendor: String, library: String, name: String, version: String) -> Self {
        Self {
            xmlns_ipxact: namespace(),
            xmlns_xsi: xsi_namespace(),
            schema_location: schema_location(),
            vendor,
            library,
            name,
            version,
            description: None,
            catalogs: None,
            bus_definitions: None,
            abstraction_definitions: None,
            components: None,
            abstractors: None,
            designs: None,
            design_configurations: None,
            generator_chains: None,
            vendor_extensions: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_new() {
        let catalog = Catalog::new(
            "vendor".to_string(),
            "library".to_string(),
            "catalog_name".to_string(),
            "1.0".to_string(),
        );

        assert_eq!(catalog.vendor, "vendor");
        assert_eq!(catalog.library, "library");
        assert_eq!(catalog.name, "catalog_name");
        assert_eq!(catalog.version, "1.0");
    }
}
