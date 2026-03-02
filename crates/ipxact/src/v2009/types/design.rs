use serde::{Deserialize, Serialize};

/// Design type - the top-level structure for a system design.
///
/// Maps to XML schema `designType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Design {
    /// Vendor name (required)
    #[serde(rename = "vendor")]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename = "library")]
    pub library: String,

    /// Design name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version (required)
    #[serde(rename = "version")]
    pub version: String,

    /// Component instances
    #[serde(rename = "componentInstances", skip_serializing_if = "Option::is_none")]
    pub component_instances: Option<ComponentInstances>,

    /// Interconnections
    #[serde(rename = "interconnections", skip_serializing_if = "Option::is_none")]
    pub interconnections: Option<Interconnections>,

    /// Ad-hoc connections
    #[serde(rename = "adHocConnections", skip_serializing_if = "Option::is_none")]
    pub ad_hoc_connections: Option<AdHocConnections>,

    /// Hierarchical connections
    #[serde(rename = "hierConnections", skip_serializing_if = "Option::is_none")]
    pub hier_connections: Option<HierConnections>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

impl Design {
    pub fn new(vendor: String, library: String, name: String, version: String) -> Self {
        Self {
            vendor,
            library,
            name,
            version,
            component_instances: None,
            interconnections: None,
            ad_hoc_connections: None,
            hier_connections: None,
            description: None,
            vendor_extensions: None,
        }
    }
}

/// Container for component instances
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstances {
    /// List of component instances
    #[serde(default, rename = "componentInstance")]
    pub component_instance: Vec<ComponentInstance>,
}

/// Single component instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    /// Instance name (required)
    #[serde(rename = "instanceName")]
    pub instance_name: String,

    /// Component reference
    #[serde(rename = "componentRef")]
    pub component_ref: ComponentRef,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Configuration parameters
    #[serde(rename = "configurableElementValues", skip_serializing_if = "Option::is_none")]
    pub configurable_element_values: Option<ConfigurableElementValues>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// Instance ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Component reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentRef {
    /// Vendor
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Library
    #[serde(rename = "library")]
    pub library: String,

    /// Name
    #[serde(rename = "name")]
    pub name: String,

    /// Version
    #[serde(rename = "version")]
    pub version: String,
}

/// Configurable element values
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableElementValues {
    /// List of configurable element values
    #[serde(default, rename = "configurableElementValue")]
    pub configurable_element_value: Vec<ConfigurableElementValue>,
}

/// Single configurable element value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurableElementValue {
    /// Reference ID
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,

    /// Value (required)
    #[serde(rename = "value")]
    pub value: String,

    /// Resolve attribute
    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Container for interconnections
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Interconnections {
    /// List of interconnections
    #[serde(default, rename = "interconnection")]
    pub interconnection: Vec<Interconnection>,
}

/// Single interconnection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interconnection {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Active interface connections
    #[serde(default, rename = "activeInterfaceConnection")]
    pub active_interface_connection: Vec<InterfaceConnection>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Interface connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceConnection {
    /// Interface ref (required)
    #[serde(rename = "interfaceRef")]
    pub interface_ref: String,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Container for ad-hoc connections
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AdHocConnections {
    /// List of ad-hoc connections
    #[serde(default, rename = "adHocConnection")]
    pub ad_hoc_connection: Vec<AdHocConnection>,
}

/// Single ad-hoc connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdHocConnection {
    /// Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Port references
    #[serde(default, rename = "portReference")]
    pub port_reference: Vec<PortReference>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Port reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortReference {
    /// Port name (required)
    #[serde(rename = "portRef")]
    pub port_ref: String,

    /// Instance reference
    #[serde(rename = "instanceRef", skip_serializing_if = "Option::is_none")]
    pub instance_ref: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Hierarchical connections
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HierConnections {
    /// List of hierarchical connections
    #[serde(default, rename = "hierConnection")]
    pub hier_connection: Vec<HierConnection>,
}

/// Single hierarchical connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HierConnection {
    /// Interface reference
    #[serde(rename = "interfaceRef")]
    pub interface_ref: String,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_new() {
        let design = Design::new(
            "vendor".to_string(),
            "library".to_string(),
            "top".to_string(),
            "1.0".to_string(),
        );
        assert_eq!(design.name, "top");
        assert_eq!(design.version, "1.0");
    }

    #[test]
    fn test_component_ref() {
        let ref_ = ComponentRef {
            vendor: Some("vendor".to_string()),
            library: "lib".to_string(),
            name: "comp".to_string(),
            version: "1.0".to_string(),
        };
        assert_eq!(ref_.name, "comp");
    }
}
