//! Component instantiation type for IP-XACT 2014

use serde::{Deserialize, Serialize};

use super::component::{
    BitExpression, ConstraintSetRef, FileBuilder, FileSetRef, Language, ParameterExpression,
    ParameterFormat, ParameterPrefix, ParameterResolve, ParameterSign, ParameterUnit, Parameters,
    PortVectors, WhiteboxElementRefs,
};
use super::configurable_arrays::ConfigurableArrays;
use super::vendor_extensions::{ExtensionAttributes, VendorExtensions};

/// Component instantiation - defines how to instantiate a component
///
/// Maps to XML schema `componentInstantiationType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(
    serialize = "ipxact:componentInstantiation",
    deserialize = "componentInstantiation"
))]
pub struct ComponentInstantiation {
    /// ID attribute
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Unique name (required)
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    /// Display name
    #[serde(
        rename(serialize = "ipxact:displayName", deserialize = "displayName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    /// Description
    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// Whether this is a virtual component
    #[serde(
        rename(serialize = "ipxact:isVirtual", deserialize = "isVirtual"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_virtual: Option<bool>,

    /// Language for the instantiation
    #[serde(
        rename(serialize = "ipxact:language", deserialize = "language"),
        skip_serializing_if = "Option::is_none"
    )]
    pub language: Option<Language>,

    /// Library name
    #[serde(
        rename(serialize = "ipxact:libraryName", deserialize = "libraryName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub library_name: Option<String>,

    /// Package name
    #[serde(
        rename(serialize = "ipxact:packageName", deserialize = "packageName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,

    /// Module name
    #[serde(
        rename(serialize = "ipxact:moduleName", deserialize = "moduleName"),
        skip_serializing_if = "Option::is_none"
    )]
    pub module_name: Option<String>,

    /// Architecture name
    #[serde(
        rename(
            serialize = "ipxact:architectureName",
            deserialize = "architectureName"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture_name: Option<String>,

    /// Configuration name
    #[serde(
        rename(
            serialize = "ipxact:configurationName",
            deserialize = "configurationName"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_name: Option<String>,

    /// HDL model parameters.
    #[serde(
        rename(
            serialize = "ipxact:moduleParameters",
            deserialize = "moduleParameters"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub module_parameters: Option<ModuleParameters>,

    /// Default builders used for referenced file sets.
    #[serde(
        rename(
            serialize = "ipxact:defaultFileBuilder",
            deserialize = "defaultFileBuilder"
        ),
        default
    )]
    pub default_file_builder: Vec<FileBuilder>,

    /// References to component-local file sets.
    #[serde(
        rename(serialize = "ipxact:fileSetRef", deserialize = "fileSetRef"),
        default
    )]
    pub file_set_ref: Vec<FileSetRef>,

    /// References to wire-port constraint sets.
    #[serde(
        rename(
            serialize = "ipxact:constraintSetRef",
            deserialize = "constraintSetRef"
        ),
        default
    )]
    pub constraint_set_ref: Vec<ConstraintSetRef>,

    /// Visible component-local whitebox elements.
    #[serde(
        rename(
            serialize = "ipxact:whiteboxElementRefs",
            deserialize = "whiteboxElementRefs"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub whitebox_element_refs: Option<WhiteboxElementRefs>,

    /// Configurable properties attached to this instantiation.
    #[serde(
        rename(serialize = "ipxact:parameters", deserialize = "parameters"),
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters: Option<Parameters>,

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

impl ComponentInstantiation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: name.into(),
            display_name: None,
            description: None,
            is_virtual: None,
            language: None,
            library_name: None,
            package_name: None,
            module_name: None,
            architecture_name: None,
            configuration_name: None,
            module_parameters: None,
            default_file_builder: Vec::new(),
            file_set_ref: Vec::new(),
            constraint_set_ref: Vec::new(),
            whitebox_element_refs: None,
            parameters: None,
            vendor_extensions: None,
        }
    }
}

/// Container for HDL model parameters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ModuleParameters {
    #[serde(
        rename(serialize = "ipxact:moduleParameter", deserialize = "moduleParameter"),
        default
    )]
    pub module_parameter: Vec<ModuleParameter>,
}

impl ModuleParameters {
    pub fn add(&mut self, module_parameter: ModuleParameter) {
        self.module_parameter.push(module_parameter);
    }
}

/// HDL model parameter with optional language-specific typing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleParameter {
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

    #[serde(rename = "@dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,

    #[serde(rename = "@usageType", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<ModuleParameterUsage>,

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

    #[serde(
        rename(serialize = "ipxact:isPresent", deserialize = "isPresent"),
        skip_serializing_if = "Option::is_none"
    )]
    pub is_present: Option<BitExpression>,
}

impl ModuleParameter {
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
            data_type: None,
            usage_type: None,
            extension_attributes: ExtensionAttributes::default(),
            name: name.into(),
            display_name: None,
            description: None,
            vectors: None,
            arrays: None,
            value: value.into(),
            vendor_extensions: None,
            is_present: None,
        }
    }
}

/// Language-level usage of an HDL module parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleParameterUsage {
    #[serde(rename = "nontyped")]
    Nontyped,
    #[serde(rename = "typed")]
    Typed,
}
