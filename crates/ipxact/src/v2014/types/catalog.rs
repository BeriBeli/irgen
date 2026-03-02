//! IP-XACT Catalog type

use serde::{Deserialize, Serialize};

use super::ipxact_files::IpxactFiles;
use super::vendor_extensions::VendorExtensions;

/// Catalog - root element for IP-XACT catalog files
///
/// Maps to XML schema `catalogType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "catalog")]
pub struct Catalog {
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

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Nested catalogs
    #[serde(rename = "catalogs", skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<IpxactFiles>,

    /// Bus definitions
    #[serde(rename = "busDefinitions", skip_serializing_if = "Option::is_none")]
    pub bus_definitions: Option<IpxactFiles>,

    /// Abstraction definitions
    #[serde(rename = "abstractionDefinitions", skip_serializing_if = "Option::is_none")]
    pub abstraction_definitions: Option<IpxactFiles>,

    /// Components
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<IpxactFiles>,

    /// Abstractors
    #[serde(rename = "abstractors", skip_serializing_if = "Option::is_none")]
    pub abstractors: Option<IpxactFiles>,

    /// Designs
    #[serde(rename = "designs", skip_serializing_if = "Option::is_none")]
    pub designs: Option<IpxactFiles>,

    /// Design configurations
    #[serde(rename = "designConfigurations", skip_serializing_if = "Option::is_none")]
    pub design_configurations: Option<IpxactFiles>,

    /// Generator chains
    #[serde(rename = "generatorChains", skip_serializing_if = "Option::is_none")]
    pub generator_chains: Option<IpxactFiles>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl Catalog {
    pub fn new(vendor: String, library: String, name: String, version: String) -> Self {
        Self {
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
