#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractionDefPortConstraintsType {
    #[serde(rename = "content")]
    pub content: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorBusInterfaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "abstractionType", skip_serializing_if = "Option::is_none")]
    pub abstraction_type: Option<String>,

    #[serde(rename = "portMaps", skip_serializing_if = "Option::is_none")]
    pub port_maps: Option<Box<super::PortMaps>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorGenerators {
    #[serde(rename = "abstractorGenerator")]
    pub abstractor_generator: Vec<InstanceGeneratorType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorModelType {
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Box<super::Views>>,

    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Box<super::Ports>>,

    #[serde(rename = "modelParameters", skip_serializing_if = "Option::is_none")]
    pub model_parameters: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbstractorModeType {
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortType {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortWireType {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorType {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "abstractorMode", skip_serializing_if = "Option::is_none")]
    pub abstractor_mode: Option<String>,

    #[serde(rename = "busType", skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<String>,

    #[serde(rename = "abstractorInterfaces", skip_serializing_if = "Option::is_none")]
    pub abstractor_interfaces: Option<String>,

    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Box<AbstractorModelType>>,

    #[serde(rename = "abstractorGenerators", skip_serializing_if = "Option::is_none")]
    pub abstractor_generators: Option<Box<AbstractorGenerators>>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<super::Choices>>,

    #[serde(rename = "fileSets", skip_serializing_if = "Option::is_none")]
    pub file_sets: Option<Box<super::FileSets>>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorViewType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "envIdentifier")]
    pub env_identifier: Vec<String>,

    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "modelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,

    #[serde(rename = "defaultFileBuilder")]
    pub default_file_builder: Vec<FileBuilderType>,

    #[serde(rename = "fileSetRef")]
    pub file_set_ref: Vec<FileSetRef>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<super::BaseAddress>>,

    #[serde(rename = "addressBlockOrBankOrSubspaceMap")]
    pub address_block_or_bank_or_subspace_map: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<super::UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<crate::v2009::enums::access_type::AccessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddrSpaceRefType {
    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankAlignmentType {
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "parallel")]
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "addressBlockOrBankOrSubspaceMap")]
    pub address_block_or_bank_or_subspace_map: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<super::UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<crate::v2009::enums::access_type::AccessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedBlockType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<super::UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<crate::v2009::enums::access_type::AccessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "register")]
    pub register: Vec<super::Register>,

    #[serde(rename = "registerFile")]
    pub register_file: Vec<super::RegisterFile>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedSubspaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "masterRef", skip_serializing_if = "Option::is_none")]
    pub master_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellClassValueType {
    #[serde(rename = "combinational")]
    Combinational,
    #[serde(rename = "sequential")]
    Sequential,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellFunctionValueType {
    #[serde(rename = "nand2")]
    Nand2,
    #[serde(rename = "buf")]
    Buf,
    #[serde(rename = "inv")]
    Inv,
    #[serde(rename = "mux21")]
    Mux21,
    #[serde(rename = "dff")]
    Dff,
    #[serde(rename = "latch")]
    Latch,
    #[serde(rename = "xor2")]
    Xor2,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CellSpecification {
    #[serde(rename = "cellFunction", skip_serializing_if = "Option::is_none")]
    pub cell_function: Option<String>,

    #[serde(rename = "cellClass", skip_serializing_if = "Option::is_none")]
    pub cell_class: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellStrengthValueType {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClockDriver {
    #[serde(rename = "clockName", skip_serializing_if = "Option::is_none")]
    pub clock_name: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClockDriverType {
    #[serde(rename = "clockPeriod", skip_serializing_if = "Option::is_none")]
    pub clock_period: Option<String>,

    #[serde(rename = "clockPulseOffset", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_offset: Option<String>,

    #[serde(rename = "clockPulseValue", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_value: Option<String>,

    #[serde(rename = "clockPulseDuration", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_duration: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentGenerator {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConstraintSet {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Box<super::Vector>>,

    #[serde(rename = "driveConstraint", skip_serializing_if = "Option::is_none")]
    pub drive_constraint: Option<Box<DriveConstraint>>,

    #[serde(rename = "loadConstraint", skip_serializing_if = "Option::is_none")]
    pub load_constraint: Option<Box<LoadConstraint>>,

    #[serde(rename = "timingConstraint")]
    pub timing_constraint: Vec<TimingConstraint>,

    #[serde(rename = "constraintSetId", skip_serializing_if = "Option::is_none")]
    pub constraint_set_id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConstraintSets {
    #[serde(rename = "constraintSet")]
    pub constraint_set: Vec<ConstraintSet>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataTypeType {
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
    CharPtr,
    #[serde(rename = "void *")]
    VoidPtr,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultValue {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<FormatType>>,

    #[serde(rename = "rangeType", skip_serializing_if = "Option::is_none")]
    pub range_type: Option<Box<RangeTypeType>>,

    #[serde(rename = "choiceRef", skip_serializing_if = "Option::is_none")]
    pub choice_ref: Option<String>,

    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,

    #[serde(rename = "configGroups")]
    pub config_groups: Vec<String>,

    #[serde(rename = "bitStringLength", skip_serializing_if = "Option::is_none")]
    pub bit_string_length: Option<u64>,

    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<Box<ResolveType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "dependency", skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayValueType {
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DelayValueUnitType {
    #[serde(rename = "ps")]
    Ps,
    #[serde(rename = "ns")]
    Ns,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DesignConfiguration {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "designRef", skip_serializing_if = "Option::is_none")]
    pub design_ref: Option<String>,

    #[serde(rename = "generatorChainConfiguration")]
    pub generator_chain_configuration: Vec<String>,

    #[serde(rename = "interconnectionConfiguration")]
    pub interconnection_configuration: Vec<String>,

    #[serde(rename = "viewConfiguration")]
    pub view_configuration: Vec<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DriveConstraint {
    #[serde(rename = "cellSpecification", skip_serializing_if = "Option::is_none")]
    pub cell_specification: Option<Box<CellSpecification>>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeValueType {
    #[serde(rename = "rise")]
    Rise,
    #[serde(rename = "fall")]
    Fall,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecutableImage {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "languageTools", skip_serializing_if = "Option::is_none")]
    pub language_tools: Option<String>,

    #[serde(rename = "fileSetRefGroup", skip_serializing_if = "Option::is_none")]
    pub file_set_ref_group: Option<Box<super::bus_interface::FileSetRefGroup>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileBuilderType {
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    #[serde(rename = "userFileType", skip_serializing_if = "Option::is_none")]
    pub user_file_type: Option<String>,

    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,

    #[serde(rename = "replaceDefaultFlags", skip_serializing_if = "Option::is_none")]
    pub replace_default_flags: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileSetRef {
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatType {
    #[serde(rename = "bitString")]
    BitString,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneratorChain {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "generatorChainSelectorOrComponentGeneratorSelectorOrGenerator")]
    pub generator_chain_selector_or_component_generator_selector_or_generator: Vec<String>,

    #[serde(rename = "chainGroup")]
    pub chain_group: Vec<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<super::Choices>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneratorSelectorType {
    #[serde(rename = "groupSelector", skip_serializing_if = "Option::is_none")]
    pub group_selector: Option<Box<GroupSelector>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupSelector {
    #[serde(rename = "name")]
    pub name: Vec<String>,

    #[serde(rename = "multipleGroupSelectionOperator", skip_serializing_if = "Option::is_none")]
    pub multiple_group_selection_operator: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HierInterface {
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceGeneratorType {
    #[serde(rename = "group")]
    pub group: Vec<String>,

    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Interface {
    #[serde(rename = "componentRef", skip_serializing_if = "Option::is_none")]
    pub component_ref: Option<String>,

    #[serde(rename = "busRef", skip_serializing_if = "Option::is_none")]
    pub bus_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadConstraint {
    #[serde(rename = "cellSpecification", skip_serializing_if = "Option::is_none")]
    pub cell_specification: Option<Box<CellSpecification>>,

    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryMapRefType {
    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryRemapType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "memoryMap")]
    pub memory_map: Vec<String>,

    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorInterconnection {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "monitoredActiveInterface", skip_serializing_if = "Option::is_none")]
    pub monitored_active_interface: Option<Box<HierInterface>>,

    #[serde(rename = "monitorInterface")]
    pub monitor_interface: Vec<HierInterface>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NameValueTypeType {
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,

    #[serde(rename = "usageType", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Phase {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortAccessType {
    #[serde(rename = "portAccessType", skip_serializing_if = "Option::is_none")]
    pub port_access_type: Option<String>,

    #[serde(rename = "portAccessHandle", skip_serializing_if = "Option::is_none")]
    pub port_access_handle: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortDeclarationType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<Box<PortWireType>>,

    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<Box<PortTransactionalType>>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<PortAccessType>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortTransactionalType {
    #[serde(rename = "transTypeDef", skip_serializing_if = "Option::is_none")]
    pub trans_type_def: Option<Box<TransTypeDef>>,

    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    #[serde(rename = "connection", skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,

    #[serde(rename = "allLogicalInitiativesAllowed", skip_serializing_if = "Option::is_none")]
    pub all_logical_initiatives_allowed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortWireType {
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Box<crate::v2009::enums::component_port_direction::ComponentPortDirectionType>>,

    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Box<super::Vector>>,

    #[serde(rename = "wireTypeDefs", skip_serializing_if = "Option::is_none")]
    pub wire_type_defs: Option<Box<super::WireTypeDefs>>,

    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,

    #[serde(rename = "constraintSets", skip_serializing_if = "Option::is_none")]
    pub constraint_sets: Option<Box<ConstraintSets>>,

    #[serde(rename = "allLogicalDirectionsAllowed", skip_serializing_if = "Option::is_none")]
    pub all_logical_directions_allowed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeTypeType {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "unsigned int")]
    UnsignedInt,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "unsigned long")]
    UnsignedLong,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequiresDriver {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,

    #[serde(rename = "driverType", skip_serializing_if = "Option::is_none")]
    pub driver_type: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResolvedLibraryRefType {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolveType {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "dependent")]
    Dependent,
    #[serde(rename = "generated")]
    Generated,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(rename = "initiative", skip_serializing_if = "Option::is_none")]
    pub initiative: Option<String>,

    #[serde(rename = "typeName")]
    pub type_name: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceTypeDefs {
    #[serde(rename = "serviceTypeDef")]
    pub service_type_def: Vec<ServiceTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SingleShotDriver {
    #[serde(rename = "singleShotOffset", skip_serializing_if = "Option::is_none")]
    pub single_shot_offset: Option<String>,

    #[serde(rename = "singleShotValue", skip_serializing_if = "Option::is_none")]
    pub single_shot_value: Option<String>,

    #[serde(rename = "singleShotDuration", skip_serializing_if = "Option::is_none")]
    pub single_shot_duration: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubspaceRefType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<super::BaseAddress>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<super::Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::VendorExtensions>>,

    #[serde(rename = "masterRef", skip_serializing_if = "Option::is_none")]
    pub master_ref: Option<String>,

    #[serde(rename = "segmentRef", skip_serializing_if = "Option::is_none")]
    pub segment_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TimingConstraint {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,

    #[serde(rename = "clockEdge", skip_serializing_if = "Option::is_none")]
    pub clock_edge: Option<Box<EdgeValueType>>,

    #[serde(rename = "delayType", skip_serializing_if = "Option::is_none")]
    pub delay_type: Option<Box<DelayValueType>>,

    #[serde(rename = "clockName", skip_serializing_if = "Option::is_none")]
    pub clock_name: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ValueMaskConfigType {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WhiteboxElementRefType {
    #[serde(rename = "whiteboxPath")]
    pub whitebox_path: Vec<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

}

