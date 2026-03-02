//! Component instantiation type for IP-XACT 2014

use serde::{Deserialize, Serialize};

use super::vendor_extensions::VendorExtensions;

/// Component instantiation - defines how to instantiate a component
///
/// Maps to XML schema `componentInstantiationType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "componentInstantiation")]
pub struct ComponentInstantiation {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this is a virtual component
    #[serde(rename = "isVirtual", skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,

    /// Language for the instantiation
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Library name
    #[serde(rename = "libraryName", skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,

    /// Package name
    #[serde(rename = "packageName", skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,

    /// Module name
    #[serde(rename = "moduleName", skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,

    /// Architecture name
    #[serde(rename = "architectureName", skip_serializing_if = "Option::is_none")]
    pub architecture_name: Option<String>,

    /// Configuration name
    #[serde(rename = "configurationName", skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<VendorExtensions>,
}

impl ComponentInstantiation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            is_virtual: None,
            language: None,
            library_name: None,
            package_name: None,
            module_name: None,
            architecture_name: None,
            configuration_name: None,
            id: None,
            vendor_extensions: None,
        }
    }
}
