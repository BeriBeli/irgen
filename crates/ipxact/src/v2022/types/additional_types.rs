#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractionDefinition {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "busType", skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<Box<LibraryRefType>>,

    #[serde(rename = "_extends", skip_serializing_if = "Option::is_none")]
    pub _extends: Option<Box<LibraryRefType>>,

    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Box<super::Ports>>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractionDefPortConstraintsType {
    #[serde(rename = "content")]
    pub content: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractionTypes {
    #[serde(rename = "abstractionType")]
    pub abstraction_type: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorBusInterfaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "abstractionTypes", skip_serializing_if = "Option::is_none")]
    pub abstraction_types: Option<Box<AbstractionTypes>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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

    #[serde(rename = "instantiations", skip_serializing_if = "Option::is_none")]
    pub instantiations: Option<String>,

    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Box<super::Ports>>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbstractorModeType {
    #[serde(rename = "initiator")]
    Initiator,
    #[serde(rename = "target")]
    Target,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortStructuredType {
    #[serde(rename = "struct", skip_serializing_if = "Option::is_none")]
    pub r#struct: Option<String>,

    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<String>,

    #[serde(rename = "_interface", skip_serializing_if = "Option::is_none")]
    pub _interface: Option<Box<crate::v2009::types::Interface>>,

    #[serde(rename = "vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Box<ExtendedVectorsType>>,

    #[serde(rename = "subPorts", skip_serializing_if = "Option::is_none")]
    pub sub_ports: Option<String>,

    #[serde(rename = "structPortTypeDefs", skip_serializing_if = "Option::is_none")]
    pub struct_port_type_defs: Option<Box<StructPortTypeDefs>>,

    #[serde(rename = "packed", skip_serializing_if = "Option::is_none")]
    pub packed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortTransactionalType {
    #[serde(rename = "initiative", skip_serializing_if = "Option::is_none")]
    pub initiative: Option<Box<InitiativeType>>,

    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<Kind>>,

    #[serde(rename = "busWidth", skip_serializing_if = "Option::is_none")]
    pub bus_width: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Box<QualifierType>>,

    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Box<Protocol>>,

    #[serde(rename = "transTypeDefs", skip_serializing_if = "Option::is_none")]
    pub trans_type_defs: Option<Box<TransTypeDefs>>,

    #[serde(rename = "connection", skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,

    #[serde(rename = "allLogicalInitiativesAllowed", skip_serializing_if = "Option::is_none")]
    pub all_logical_initiatives_allowed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<Box<AbstractorPortWireType>>,

    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<Box<AbstractorPortTransactionalType>>,

    #[serde(rename = "structured", skip_serializing_if = "Option::is_none")]
    pub structured: Option<Box<AbstractorPortStructuredType>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<super::Arrays>>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<PortAccessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorPortWireType {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractorSubPortType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<Box<AbstractorPortWireType>>,

    #[serde(rename = "structured", skip_serializing_if = "Option::is_none")]
    pub structured: Option<Box<AbstractorPortStructuredType>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<super::Arrays>>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<PortAccessType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "isIO", skip_serializing_if = "Option::is_none")]
    pub is_i_o: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

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

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "abstractorMode", skip_serializing_if = "Option::is_none")]
    pub abstractor_mode: Option<String>,

    #[serde(rename = "busType", skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<Box<LibraryRefType>>,

    #[serde(rename = "abstractorInterfaces", skip_serializing_if = "Option::is_none")]
    pub abstractor_interfaces: Option<String>,

    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Box<AbstractorModelType>>,

    #[serde(rename = "abstractorGenerators", skip_serializing_if = "Option::is_none")]
    pub abstractor_generators: Option<Box<AbstractorGenerators>>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "fileSets", skip_serializing_if = "Option::is_none")]
    pub file_sets: Option<Box<FileSets>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
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
    #[serde(rename = "no-access")]
    NoAccess,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ActiveInterface {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "excludePorts", skip_serializing_if = "Option::is_none")]
    pub exclude_ports: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBankDefinitionType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<UnsignedLongintExpression>>,

    #[serde(rename = "bankDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub bank_definition_ref: Option<String>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<UnsignedLongintExpression>>,

    #[serde(rename = "bankDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub bank_definition_ref: Option<String>,

    #[serde(rename = "addressBlockOrBankOrSubspaceMap")]
    pub address_block_or_bank_or_subspace_map: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBlockDefinitions {
    #[serde(rename = "addressBlockDefinition")]
    pub address_block_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBlockRef {
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Box<IndicesType>>,

    #[serde(rename = "addressBlockRef", skip_serializing_if = "Option::is_none")]
    pub address_block_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBlockType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<Box<super::Array>>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<UnsignedLongintExpression>>,

    #[serde(rename = "addressBlockDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub address_block_definition_ref: Option<String>,

    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Box<UnsignedPositiveIntExpression>>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "registerData")]
    pub register_data: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "misalignmentAllowed", skip_serializing_if = "Option::is_none")]
    pub misalignment_allowed: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressSpaces {
    #[serde(rename = "addressSpace")]
    pub address_space: Vec<super::AddressSpace>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddrSpaceRefType {
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AdHocConnection {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "tiedValue", skip_serializing_if = "Option::is_none")]
    pub tied_value: Option<Box<ComplexTiedValueExpression>>,

    #[serde(rename = "portReferences", skip_serializing_if = "Option::is_none")]
    pub port_references: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AdHocConnections {
    #[serde(rename = "adHocConnection")]
    pub ad_hoc_connection: Vec<AdHocConnection>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AlternateRegisterRef {
    #[serde(rename = "alternateRegisterRef", skip_serializing_if = "Option::is_none")]
    pub alternate_register_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AlternateRegisters {
    #[serde(rename = "alternateRegister")]
    pub alternate_register: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiType {
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Assertion {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "_assert", skip_serializing_if = "Option::is_none")]
    pub _assert: Option<Box<UnsignedBitExpression>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Assertions {
    #[serde(rename = "assertion")]
    pub assertion: Vec<Assertion>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankAlignmentType {
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "parallel")]
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankDefinitions {
    #[serde(rename = "bankDefinition")]
    pub bank_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "bankDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub bank_definition_ref: Option<String>,

    #[serde(rename = "addressBlockOrBankOrSubspaceMap")]
    pub address_block_or_bank_or_subspace_map: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedBlockType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Box<UnsignedPositiveIntExpression>>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "registerData")]
    pub register_data: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedDefinitionBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "bankDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub bank_definition_ref: Option<String>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankedSubspaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "initiatorRef", skip_serializing_if = "Option::is_none")]
    pub initiator_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankRef {
    #[serde(rename = "bankRef", skip_serializing_if = "Option::is_none")]
    pub bank_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BusDefinition {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "directConnection", skip_serializing_if = "Option::is_none")]
    pub direct_connection: Option<bool>,

    #[serde(rename = "broadcast", skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<bool>,

    #[serde(rename = "isAddressable", skip_serializing_if = "Option::is_none")]
    pub is_addressable: Option<bool>,

    #[serde(rename = "_extends", skip_serializing_if = "Option::is_none")]
    pub _extends: Option<Box<LibraryRefType>>,

    #[serde(rename = "maxInitiators", skip_serializing_if = "Option::is_none")]
    pub max_initiators: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "maxTargets", skip_serializing_if = "Option::is_none")]
    pub max_targets: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "systemGroupNames", skip_serializing_if = "Option::is_none")]
    pub system_group_names: Option<String>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BusInterfaces {
    #[serde(rename = "busInterface")]
    pub bus_interface: Vec<BusInterfaceType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BusInterfaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "busType", skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<Box<ConfigurableLibraryRefType>>,

    #[serde(rename = "abstractionTypes", skip_serializing_if = "Option::is_none")]
    pub abstraction_types: Option<Box<AbstractionTypes>>,

    #[serde(rename = "initiator", skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,

    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(rename = "mirroredTarget", skip_serializing_if = "Option::is_none")]
    pub mirrored_target: Option<String>,

    #[serde(rename = "mirroredInitiator", skip_serializing_if = "Option::is_none")]
    pub mirrored_initiator: Option<String>,

    #[serde(rename = "mirroredSystem", skip_serializing_if = "Option::is_none")]
    pub mirrored_system: Option<String>,

    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<String>,

    #[serde(rename = "connectionRequired", skip_serializing_if = "Option::is_none")]
    pub connection_required: Option<bool>,

    #[serde(rename = "bitsInLau", skip_serializing_if = "Option::is_none")]
    pub bits_in_lau: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "bitSteering", skip_serializing_if = "Option::is_none")]
    pub bit_steering: Option<Box<UnsignedBitExpression>>,

    #[serde(rename = "endianness", skip_serializing_if = "Option::is_none")]
    pub endianness: Option<Box<EndianessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "catalogs", skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<Box<IpxactFilesType>>,

    #[serde(rename = "busDefinitions", skip_serializing_if = "Option::is_none")]
    pub bus_definitions: Option<Box<IpxactFilesType>>,

    #[serde(rename = "abstractionDefinitions", skip_serializing_if = "Option::is_none")]
    pub abstraction_definitions: Option<Box<IpxactFilesType>>,

    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Box<IpxactFilesType>>,

    #[serde(rename = "abstractors", skip_serializing_if = "Option::is_none")]
    pub abstractors: Option<Box<IpxactFilesType>>,

    #[serde(rename = "designs", skip_serializing_if = "Option::is_none")]
    pub designs: Option<Box<IpxactFilesType>>,

    #[serde(rename = "designConfigurations", skip_serializing_if = "Option::is_none")]
    pub design_configurations: Option<Box<IpxactFilesType>>,

    #[serde(rename = "generatorChains", skip_serializing_if = "Option::is_none")]
    pub generator_chains: Option<Box<IpxactFilesType>>,

    #[serde(rename = "typeDefinitions", skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<Box<IpxactFilesType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CellSpecification {
    #[serde(rename = "cellFunction", skip_serializing_if = "Option::is_none")]
    pub cell_function: Option<String>,

    #[serde(rename = "cellClass", skip_serializing_if = "Option::is_none")]
    pub cell_class: Option<Box<CellClassValueType>>,

    #[serde(rename = "cellStrength", skip_serializing_if = "Option::is_none")]
    pub cell_strength: Option<Box<CellStrengthValueType>>,

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
pub struct Channels {
    #[serde(rename = "channel")]
    pub channel: Vec<super::Channel>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Choices {
    #[serde(rename = "choice")]
    pub choice: Vec<super::Choice>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearboxElementRefType {
    #[serde(rename = "location")]
    pub location: Vec<SlicesType>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearboxElementType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "clearboxType", skip_serializing_if = "Option::is_none")]
    pub clearbox_type: Option<Box<SimpleClearboxType>>,

    #[serde(rename = "driveable", skip_serializing_if = "Option::is_none")]
    pub driveable: Option<bool>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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
    pub clock_pulse_value: Option<Box<UnsignedBitExpression>>,

    #[serde(rename = "clockPulseDuration", skip_serializing_if = "Option::is_none")]
    pub clock_pulse_duration: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComplexBaseExpression {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComplexTiedValueExpression {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentGenerators {
    #[serde(rename = "componentGenerator")]
    pub component_generator: Vec<InstanceGeneratorType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentInstance {
    #[serde(rename = "instanceName", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "componentRef", skip_serializing_if = "Option::is_none")]
    pub component_ref: Option<Box<ConfigurableLibraryRefType>>,

    #[serde(rename = "powerDomainLinks", skip_serializing_if = "Option::is_none")]
    pub power_domain_links: Option<Box<PowerDomainLinks>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentInstances {
    #[serde(rename = "componentInstance")]
    pub component_instance: Vec<ComponentInstance>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentInstantiationType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "isVirtual", skip_serializing_if = "Option::is_none")]
    pub is_virtual: Option<bool>,

    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<LanguageType>>,

    #[serde(rename = "libraryName", skip_serializing_if = "Option::is_none")]
    pub library_name: Option<String>,

    #[serde(rename = "packageName", skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,

    #[serde(rename = "moduleName", skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,

    #[serde(rename = "architectureName", skip_serializing_if = "Option::is_none")]
    pub architecture_name: Option<String>,

    #[serde(rename = "configurationName", skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,

    #[serde(rename = "moduleParameters", skip_serializing_if = "Option::is_none")]
    pub module_parameters: Option<String>,

    #[serde(rename = "defaultFileBuilder")]
    pub default_file_builder: Vec<FileBuilderType>,

    #[serde(rename = "fileSetRef")]
    pub file_set_ref: Vec<FileSetRef>,

    #[serde(rename = "constraintSetRef")]
    pub constraint_set_ref: Vec<ConstraintSetRef>,

    #[serde(rename = "clearboxElementRefs", skip_serializing_if = "Option::is_none")]
    pub clearbox_element_refs: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentPortDirectionType {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "inout")]
    Inout,
    #[serde(rename = "phantom")]
    Phantom,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ComponentType {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "typeDefinitions", skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<Box<TypeDefinitions>>,

    #[serde(rename = "powerDomains", skip_serializing_if = "Option::is_none")]
    pub power_domains: Option<String>,

    #[serde(rename = "busInterfaces", skip_serializing_if = "Option::is_none")]
    pub bus_interfaces: Option<Box<BusInterfaces>>,

    #[serde(rename = "indirectInterfaces", skip_serializing_if = "Option::is_none")]
    pub indirect_interfaces: Option<Box<IndirectInterfaces>>,

    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Box<Channels>>,

    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<String>,

    #[serde(rename = "addressSpaces", skip_serializing_if = "Option::is_none")]
    pub address_spaces: Option<Box<AddressSpaces>>,

    #[serde(rename = "memoryMaps", skip_serializing_if = "Option::is_none")]
    pub memory_maps: Option<Box<MemoryMaps>>,

    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<Box<ModelType>>,

    #[serde(rename = "componentGenerators", skip_serializing_if = "Option::is_none")]
    pub component_generators: Option<Box<ComponentGenerators>>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "fileSets", skip_serializing_if = "Option::is_none")]
    pub file_sets: Option<Box<FileSets>>,

    #[serde(rename = "clearboxElements", skip_serializing_if = "Option::is_none")]
    pub clearbox_elements: Option<String>,

    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Box<crate::v2009::types::Cpus>>,

    #[serde(rename = "otherClockDrivers", skip_serializing_if = "Option::is_none")]
    pub other_clock_drivers: Option<Box<OtherClocks>>,

    #[serde(rename = "resetTypes", skip_serializing_if = "Option::is_none")]
    pub reset_types: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigurableArrays {
    #[serde(rename = "array")]
    pub array: Vec<super::Array>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigurableElementValue {
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigurableElementValues {
    #[serde(rename = "configurableElementValue")]
    pub configurable_element_value: Vec<ConfigurableElementValue>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConfigurableLibraryRefType {
    #[serde(rename = "configurableElementValues", skip_serializing_if = "Option::is_none")]
    pub configurable_element_values: Option<Box<ConfigurableElementValues>>,

    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConstraintSet {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vector", skip_serializing_if = "Option::is_none")]
    pub vector: Option<Box<Vector>>,

    #[serde(rename = "driveConstraint", skip_serializing_if = "Option::is_none")]
    pub drive_constraint: Option<Box<DriveConstraint>>,

    #[serde(rename = "loadConstraint", skip_serializing_if = "Option::is_none")]
    pub load_constraint: Option<Box<LoadConstraint>>,

    #[serde(rename = "timingConstraint")]
    pub timing_constraint: Vec<TimingConstraint>,

    #[serde(rename = "constraintSetId", skip_serializing_if = "Option::is_none")]
    pub constraint_set_id: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConstraintSetRef {
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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
pub struct Dependency {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Design {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "componentInstances", skip_serializing_if = "Option::is_none")]
    pub component_instances: Option<Box<ComponentInstances>>,

    #[serde(rename = "interconnections", skip_serializing_if = "Option::is_none")]
    pub interconnections: Option<Box<Interconnections>>,

    #[serde(rename = "adHocConnections", skip_serializing_if = "Option::is_none")]
    pub ad_hoc_connections: Option<Box<AdHocConnections>>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "designRef", skip_serializing_if = "Option::is_none")]
    pub design_ref: Option<Box<LibraryRefType>>,

    #[serde(rename = "generatorChainConfiguration")]
    pub generator_chain_configuration: Vec<ConfigurableLibraryRefType>,

    #[serde(rename = "interconnectionConfiguration")]
    pub interconnection_configuration: Vec<String>,

    #[serde(rename = "viewConfiguration")]
    pub view_configuration: Vec<String>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DesignConfigurationInstantiationType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<LanguageType>>,

    #[serde(rename = "designConfigurationRef", skip_serializing_if = "Option::is_none")]
    pub design_configuration_ref: Option<Box<ConfigurableLibraryRefType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DesignInstantiationType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "designRef", skip_serializing_if = "Option::is_none")]
    pub design_ref: Option<Box<ConfigurableLibraryRefType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Dim {
    #[serde(rename = "indexVar", skip_serializing_if = "Option::is_none")]
    pub index_var: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "inout")]
    Inout,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainTypeDefs {
    #[serde(rename = "domainTypeDef")]
    pub domain_type_def: Vec<DomainTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DriveConstraint {
    #[serde(rename = "cellSpecification", skip_serializing_if = "Option::is_none")]
    pub cell_specification: Option<Box<CellSpecification>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Drivers {
    #[serde(rename = "driver")]
    pub driver: Vec<DriverType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DriverType {
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<Range>>,

    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Box<QualifiedExpression>>,

    #[serde(rename = "clockDriver", skip_serializing_if = "Option::is_none")]
    pub clock_driver: Option<Box<ClockDriver>>,

    #[serde(rename = "singleShotDriver", skip_serializing_if = "Option::is_none")]
    pub single_shot_driver: Option<Box<SingleShotDriver>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeValueType {
    #[serde(rename = "rise")]
    Rise,
    #[serde(rename = "fall")]
    Fall,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndianessType {
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "little")]
    Little,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnumeratedValues {
    #[serde(rename = "enumerationDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub enumeration_definition_ref: Option<String>,

    #[serde(rename = "enumeratedValue")]
    pub enumerated_value: Vec<EnumeratedValueType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnumeratedValueType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnumerationDefinitions {
    #[serde(rename = "enumerationDefinition")]
    pub enumeration_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecutableImage {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "languageTools", skip_serializing_if = "Option::is_none")]
    pub language_tools: Option<String>,

    #[serde(rename = "fileSetRefGroup", skip_serializing_if = "Option::is_none")]
    pub file_set_ref_group: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,

    #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtendedVectorsType {
    #[serde(rename = "vector")]
    pub vector: Vec<Vector>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalPortReference {
    #[serde(rename = "subPortReference")]
    pub sub_port_reference: Vec<SubPortReference>,

    #[serde(rename = "partSelect", skip_serializing_if = "Option::is_none")]
    pub part_select: Option<Box<PartSelect>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "portRef", skip_serializing_if = "Option::is_none")]
    pub port_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalTypeDefinitions {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "typeDefinitionsRef", skip_serializing_if = "Option::is_none")]
    pub type_definitions_ref: Option<Box<ConfigurableLibraryRefType>>,

    #[serde(rename = "viewLinks", skip_serializing_if = "Option::is_none")]
    pub view_links: Option<Box<ViewLinks>>,

    #[serde(rename = "modeLinks", skip_serializing_if = "Option::is_none")]
    pub mode_links: Option<Box<ModeLinks>>,

    #[serde(rename = "resetTypeLinks", skip_serializing_if = "Option::is_none")]
    pub reset_type_links: Option<Box<ResetTypeLinks>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldAccessPolicyDefinitionRef {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "typeDefinitions", skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldAccessPolicyDefinitions {
    #[serde(rename = "fieldAccessPolicyDefinition")]
    pub field_access_policy_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldAccessPropertiesType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldMap {
    #[serde(rename = "fieldSlice", skip_serializing_if = "Option::is_none")]
    pub field_slice: Option<String>,

    #[serde(rename = "subPortReference")]
    pub sub_port_reference: Vec<SubPortReference>,

    #[serde(rename = "partSelect", skip_serializing_if = "Option::is_none")]
    pub part_select: Option<Box<PartSelect>>,

    #[serde(rename = "modeRef")]
    pub mode_ref: Vec<ModeRef>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldMaps {
    #[serde(rename = "fieldMap")]
    pub field_map: Vec<FieldMap>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldRef {
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Box<IndicesType>>,

    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<Box<super::Array>>,

    #[serde(rename = "bitOffset", skip_serializing_if = "Option::is_none")]
    pub bit_offset: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "fieldDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub field_definition_ref: Option<String>,

    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    #[serde(rename = "bitWidth", skip_serializing_if = "Option::is_none")]
    pub bit_width: Option<Box<UnsignedPositiveIntExpression>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "resets", skip_serializing_if = "Option::is_none")]
    pub resets: Option<String>,

    #[serde(rename = "aliasOf", skip_serializing_if = "Option::is_none")]
    pub alias_of: Option<String>,

    #[serde(rename = "fieldAccessPolicies", skip_serializing_if = "Option::is_none")]
    pub field_access_policies: Option<Box<super::FieldAccessPolicies>>,

    #[serde(rename = "enumeratedValues", skip_serializing_if = "Option::is_none")]
    pub enumerated_values: Option<Box<EnumeratedValues>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<IpxactURI>>,

    #[serde(rename = "fileType")]
    pub file_type: Vec<FileType>,

    #[serde(rename = "isStructural", skip_serializing_if = "Option::is_none")]
    pub is_structural: Option<bool>,

    #[serde(rename = "isIncludeFile", skip_serializing_if = "Option::is_none")]
    pub is_include_file: Option<String>,

    #[serde(rename = "logicalName", skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,

    #[serde(rename = "exportedName")]
    pub exported_name: Vec<String>,

    #[serde(rename = "buildCommand", skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,

    #[serde(rename = "dependency")]
    pub dependency: Vec<Dependency>,

    #[serde(rename = "define")]
    pub define: Vec<NameValuePairType>,

    #[serde(rename = "imageType")]
    pub image_type: Vec<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "fileId", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileBuilderType {
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<Box<FileType>>,

    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Box<StringExpression>>,

    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Box<StringExpression>>,

    #[serde(rename = "replaceDefaultFlags", skip_serializing_if = "Option::is_none")]
    pub replace_default_flags: Option<Box<UnsignedBitExpression>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileSetRef {
    #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileSets {
    #[serde(rename = "fileSet")]
    pub file_set: Vec<FileSetType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileSetType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "group")]
    pub group: Vec<String>,

    #[serde(rename = "file")]
    pub file: Vec<File>,

    #[serde(rename = "defaultFileBuilder")]
    pub default_file_builder: Vec<FileBuilderType>,

    #[serde(rename = "dependency")]
    pub dependency: Vec<Dependency>,

    #[serde(rename = "function")]
    pub function: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileType {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<SimpleFileType>>,

    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "libext", skip_serializing_if = "Option::is_none")]
    pub libext: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormatType {
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Generator {}

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

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "generatorChainSelectorOrComponentGeneratorSelectorOrGenerator")]
    pub generator_chain_selector_or_component_generator_selector_or_generator: Vec<String>,

    #[serde(rename = "chainGroup")]
    pub chain_group: Vec<String>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneratorRef {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneratorSelectorType {
    #[serde(rename = "groupSelector", skip_serializing_if = "Option::is_none")]
    pub group_selector: Option<Box<GroupSelector>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneratorType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<Box<RealExpression>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "apiType", skip_serializing_if = "Option::is_none")]
    pub api_type: Option<Box<ApiType>>,

    #[serde(rename = "apiService", skip_serializing_if = "Option::is_none")]
    pub api_service: Option<String>,

    #[serde(rename = "transportMethods", skip_serializing_if = "Option::is_none")]
    pub transport_methods: Option<String>,

    #[serde(rename = "generatorExe", skip_serializing_if = "Option::is_none")]
    pub generator_exe: Option<Box<IpxactURI>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupSelector {
    #[serde(rename = "name")]
    pub name: Vec<String>,

    #[serde(rename = "multipleGroupSelectionOperator", skip_serializing_if = "Option::is_none")]
    pub multiple_group_selection_operator: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HierInterfaceType {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "busRef", skip_serializing_if = "Option::is_none")]
    pub bus_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndicesType {
    #[serde(rename = "index")]
    pub index: Vec<Index>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndirectAddressRef {
    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<Box<super::AddressSpaceRef>>,

    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

    #[serde(rename = "memoryRemapRef", skip_serializing_if = "Option::is_none")]
    pub memory_remap_ref: Option<Box<MemoryRemapRef>>,

    #[serde(rename = "bankRef")]
    pub bank_ref: Vec<BankRef>,

    #[serde(rename = "addressBlockRef", skip_serializing_if = "Option::is_none")]
    pub address_block_ref: Option<Box<AddressBlockRef>>,

    #[serde(rename = "registerFileRef")]
    pub register_file_ref: Vec<RegisterFileRef>,

    #[serde(rename = "registerRef", skip_serializing_if = "Option::is_none")]
    pub register_ref: Option<Box<RegisterRef>>,

    #[serde(rename = "alternateRegisterRef", skip_serializing_if = "Option::is_none")]
    pub alternate_register_ref: Option<Box<AlternateRegisterRef>>,

    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<Box<FieldRef>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndirectDataRef {
    #[serde(rename = "addressSpaceRef", skip_serializing_if = "Option::is_none")]
    pub address_space_ref: Option<Box<super::AddressSpaceRef>>,

    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

    #[serde(rename = "memoryRemapRef", skip_serializing_if = "Option::is_none")]
    pub memory_remap_ref: Option<Box<MemoryRemapRef>>,

    #[serde(rename = "bankRef")]
    pub bank_ref: Vec<BankRef>,

    #[serde(rename = "addressBlockRef", skip_serializing_if = "Option::is_none")]
    pub address_block_ref: Option<Box<AddressBlockRef>>,

    #[serde(rename = "registerFileRef")]
    pub register_file_ref: Vec<RegisterFileRef>,

    #[serde(rename = "registerRef", skip_serializing_if = "Option::is_none")]
    pub register_ref: Option<Box<RegisterRef>>,

    #[serde(rename = "alternateRegisterRef", skip_serializing_if = "Option::is_none")]
    pub alternate_register_ref: Option<Box<AlternateRegisterRef>>,

    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<Box<FieldRef>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndirectInterfaces {
    #[serde(rename = "indirectInterface")]
    pub indirect_interface: Vec<IndirectInterfaceType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndirectInterfaceType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "indirectAddressRef", skip_serializing_if = "Option::is_none")]
    pub indirect_address_ref: Option<Box<IndirectAddressRef>>,

    #[serde(rename = "indirectDataRef", skip_serializing_if = "Option::is_none")]
    pub indirect_data_ref: Option<Box<IndirectDataRef>>,

    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

    #[serde(rename = "transparentBridge")]
    pub transparent_bridge: Vec<TransparentBridge>,

    #[serde(rename = "bitsInLau", skip_serializing_if = "Option::is_none")]
    pub bits_in_lau: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "endianness", skip_serializing_if = "Option::is_none")]
    pub endianness: Option<Box<EndianessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InitiativeType {
    #[serde(rename = "requires")]
    Requires,
    #[serde(rename = "provides")]
    Provides,
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "phantom")]
    Phantom,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InstanceGeneratorType {
    #[serde(rename = "group")]
    pub group: Vec<String>,

    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Interconnection {
    #[serde(rename = "content")]
    pub content: Vec<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Interconnections {
    #[serde(rename = "interconnectionOrMonitorInterconnection")]
    pub interconnection_or_monitor_interconnection: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InterfaceType {
    #[serde(rename = "componentInstanceRef", skip_serializing_if = "Option::is_none")]
    pub component_instance_ref: Option<String>,

    #[serde(rename = "busRef", skip_serializing_if = "Option::is_none")]
    pub bus_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IpxactFilesType {
    #[serde(rename = "ipxactFile")]
    pub ipxact_file: Vec<IpxactFileType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IpxactFileType {
    #[serde(rename = "vlnv", skip_serializing_if = "Option::is_none")]
    pub vlnv: Option<Box<LibraryRefType>>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<IpxactURI>>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IpxactURI {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Kind {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<KindType>>,

    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KindType {
    #[serde(rename = "tlm_port")]
    TlmPort,
    #[serde(rename = "tlm_socket")]
    TlmSocket,
    #[serde(rename = "simple_socket")]
    SimpleSocket,
    #[serde(rename = "multi_socket")]
    MultiSocket,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LanguageType {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LibraryRefType {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkerCommandFile {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<StringExpression>>,

    #[serde(rename = "commandLineSwitch", skip_serializing_if = "Option::is_none")]
    pub command_line_switch: Option<Box<StringExpression>>,

    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<Box<UnsignedBitExpression>>,

    #[serde(rename = "generatorRef")]
    pub generator_ref: Vec<GeneratorRef>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadConstraint {
    #[serde(rename = "cellSpecification", skip_serializing_if = "Option::is_none")]
    pub cell_specification: Option<Box<CellSpecification>>,

    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<Box<UnsignedPositiveIntExpression>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocalAddressBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<UnsignedLongintExpression>>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocalBankedBankType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<UsageType>>,

    #[serde(rename = "_volatile", skip_serializing_if = "Option::is_none")]
    pub _volatile: Option<bool>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "bankAlignment", skip_serializing_if = "Option::is_none")]
    pub bank_alignment: Option<Box<BankAlignmentType>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LocalMemoryMapType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryMapRefType {
    #[serde(rename = "modeRef")]
    pub mode_ref: Vec<ModeRef>,

    #[serde(rename = "memoryMapRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryMaps {
    #[serde(rename = "memoryMap")]
    pub memory_map: Vec<MemoryMapType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryMapType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "memoryMapDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub memory_map_definition_ref: Option<String>,

    #[serde(rename = "memoryMap")]
    pub memory_map: Vec<String>,

    #[serde(rename = "memoryRemap")]
    pub memory_remap: Vec<MemoryRemapType>,

    #[serde(rename = "addressUnitBits", skip_serializing_if = "Option::is_none")]
    pub address_unit_bits: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<Box<SharedType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryRemapDefinitions {
    #[serde(rename = "memoryRemapDefinition")]
    pub memory_remap_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryRemapDefinitionType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "modeRef")]
    pub mode_ref: Vec<ModeRef>,

    #[serde(rename = "remapDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub remap_definition_ref: Option<String>,

    #[serde(rename = "addressBlockOrBank")]
    pub address_block_or_bank: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryRemapRef {
    #[serde(rename = "memoryRemapRef", skip_serializing_if = "Option::is_none")]
    pub memory_remap_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryRemapType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "modeRef")]
    pub mode_ref: Vec<ModeRef>,

    #[serde(rename = "remapDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub remap_definition_ref: Option<String>,

    #[serde(rename = "memoryMap")]
    pub memory_map: Vec<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModeLinks {
    #[serde(rename = "modeLink")]
    pub mode_link: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModelType {
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Box<super::Views>>,

    #[serde(rename = "instantiations", skip_serializing_if = "Option::is_none")]
    pub instantiations: Option<String>,

    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Box<super::Ports>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModeRef {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u64>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModifiedWriteValue {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<ModifiedWriteValueType>>,

    #[serde(rename = "modify", skip_serializing_if = "Option::is_none")]
    pub modify: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifiedWriteValueType {
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModuleParameterArrays {
    #[serde(rename = "array")]
    pub array: Vec<super::Array>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ModuleParameterType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Box<Vectors>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<ModuleParameterArrays>>,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<StringExpression>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,

    #[serde(rename = "usageType", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,

    #[serde(rename = "dataTypeDefinition", skip_serializing_if = "Option::is_none")]
    pub data_type_definition: Option<String>,

    #[serde(rename = "constrained")]
    pub constrained: Vec<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,

    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(rename = "choiceRef", skip_serializing_if = "Option::is_none")]
    pub choice_ref: Option<String>,

    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,

    #[serde(rename = "configGroups")]
    pub config_groups: Vec<String>,

    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<FormatType>>,

    #[serde(rename = "sign", skip_serializing_if = "Option::is_none")]
    pub sign: Option<Box<SignType>>,

    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorInterconnection {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "monitoredActiveInterface", skip_serializing_if = "Option::is_none")]
    pub monitored_active_interface: Option<Box<MonitorInterfaceType>>,

    #[serde(rename = "monitorInterface")]
    pub monitor_interface: Vec<MonitorInterfaceType>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorInterfaceType {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NameValuePairType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<StringExpression>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OtherClockDriver {
    #[serde(rename = "clockName", skip_serializing_if = "Option::is_none")]
    pub clock_name: Option<String>,

    #[serde(rename = "clockSource", skip_serializing_if = "Option::is_none")]
    pub clock_source: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OtherClocks {
    #[serde(rename = "otherClockDriver")]
    pub other_clock_driver: Vec<OtherClockDriver>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "parameter")]
    pub parameter: Vec<ParameterType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ParameterType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Box<Vectors>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<ConfigurableArrays>>,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<StringExpression>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,

    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(rename = "choiceRef", skip_serializing_if = "Option::is_none")]
    pub choice_ref: Option<String>,

    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,

    #[serde(rename = "configGroups")]
    pub config_groups: Vec<String>,

    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<FormatType>>,

    #[serde(rename = "sign", skip_serializing_if = "Option::is_none")]
    pub sign: Option<Box<SignType>>,

    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PartSelect {
    #[serde(rename = "content")]
    pub content: Vec<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PathSegmentType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Port {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortAccessHandle {
    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<String>,

    #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
    pub slices: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortAccessType {
    #[serde(rename = "portAccessType", skip_serializing_if = "Option::is_none")]
    pub port_access_type: Option<Box<SimplePortAccessType>>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortPacketFieldsType {
    #[serde(rename = "packetField")]
    pub packet_field: Vec<PortPacketFieldType>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortPacketFieldType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Box<UnresolvedUnsignedPositiveIntExpression>>,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "endianness", skip_serializing_if = "Option::is_none")]
    pub endianness: Option<Box<EndianessType>>,

    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Box<QualifierType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortPathSegmentType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortStructuredType {
    #[serde(rename = "struct", skip_serializing_if = "Option::is_none")]
    pub r#struct: Option<String>,

    #[serde(rename = "union", skip_serializing_if = "Option::is_none")]
    pub union: Option<String>,

    #[serde(rename = "_interface", skip_serializing_if = "Option::is_none")]
    pub _interface: Option<Box<crate::v2009::types::Interface>>,

    #[serde(rename = "vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Box<ExtendedVectorsType>>,

    #[serde(rename = "subPorts", skip_serializing_if = "Option::is_none")]
    pub sub_ports: Option<String>,

    #[serde(rename = "structPortTypeDefs", skip_serializing_if = "Option::is_none")]
    pub struct_port_type_defs: Option<Box<StructPortTypeDefs>>,

    #[serde(rename = "packed", skip_serializing_if = "Option::is_none")]
    pub packed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortTransactionalType {
    #[serde(rename = "initiative", skip_serializing_if = "Option::is_none")]
    pub initiative: Option<Box<InitiativeType>>,

    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<Kind>>,

    #[serde(rename = "busWidth", skip_serializing_if = "Option::is_none")]
    pub bus_width: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Box<QualifierType>>,

    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Box<Protocol>>,

    #[serde(rename = "transTypeDefs", skip_serializing_if = "Option::is_none")]
    pub trans_type_defs: Option<Box<TransTypeDefs>>,

    #[serde(rename = "connection", skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,

    #[serde(rename = "powerConstraints", skip_serializing_if = "Option::is_none")]
    pub power_constraints: Option<String>,

    #[serde(rename = "allLogicalInitiativesAllowed", skip_serializing_if = "Option::is_none")]
    pub all_logical_initiatives_allowed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<Box<PortWireType>>,

    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<Box<PortTransactionalType>>,

    #[serde(rename = "structured", skip_serializing_if = "Option::is_none")]
    pub structured: Option<Box<PortStructuredType>>,

    #[serde(rename = "fieldMaps", skip_serializing_if = "Option::is_none")]
    pub field_maps: Option<Box<FieldMaps>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<super::Arrays>>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<PortAccessType>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortWireType {
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Box<ComponentPortDirectionType>>,

    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Box<QualifierType>>,

    #[serde(rename = "vectors", skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Box<ExtendedVectorsType>>,

    #[serde(rename = "wireTypeDefs", skip_serializing_if = "Option::is_none")]
    pub wire_type_defs: Option<Box<WireTypeDefs>>,

    #[serde(rename = "domainTypeDefs", skip_serializing_if = "Option::is_none")]
    pub domain_type_defs: Option<Box<DomainTypeDefs>>,

    #[serde(rename = "signalTypeDefs", skip_serializing_if = "Option::is_none")]
    pub signal_type_defs: Option<Box<SignalTypeDefs>>,

    #[serde(rename = "drivers", skip_serializing_if = "Option::is_none")]
    pub drivers: Option<Box<Drivers>>,

    #[serde(rename = "constraintSets", skip_serializing_if = "Option::is_none")]
    pub constraint_sets: Option<Box<ConstraintSets>>,

    #[serde(rename = "powerConstraints", skip_serializing_if = "Option::is_none")]
    pub power_constraints: Option<String>,

    #[serde(rename = "allLogicalDirectionsAllowed", skip_serializing_if = "Option::is_none")]
    pub all_logical_directions_allowed: Option<bool>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PowerDomainLinks {
    #[serde(rename = "powerDomainLink")]
    pub power_domain_link: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceType {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "illegal")]
    Illegal,
    #[serde(rename = "optional")]
    Optional,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Protocol {
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<Box<super::ProtocolType>>,

    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<Payload>>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolTypeType {
    #[serde(rename = "tlm")]
    Tlm,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct QualifiedExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct QualifierType {
    #[serde(rename = "isAddress", skip_serializing_if = "Option::is_none")]
    pub is_address: Option<bool>,

    #[serde(rename = "isData", skip_serializing_if = "Option::is_none")]
    pub is_data: Option<bool>,

    #[serde(rename = "isClock", skip_serializing_if = "Option::is_none")]
    pub is_clock: Option<bool>,

    #[serde(rename = "isReset", skip_serializing_if = "Option::is_none")]
    pub is_reset: Option<String>,

    #[serde(rename = "isValid", skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,

    #[serde(rename = "isInterrupt", skip_serializing_if = "Option::is_none")]
    pub is_interrupt: Option<bool>,

    #[serde(rename = "isClockEn", skip_serializing_if = "Option::is_none")]
    pub is_clock_en: Option<String>,

    #[serde(rename = "isPowerEn", skip_serializing_if = "Option::is_none")]
    pub is_power_en: Option<String>,

    #[serde(rename = "isOpcode", skip_serializing_if = "Option::is_none")]
    pub is_opcode: Option<bool>,

    #[serde(rename = "isProtection", skip_serializing_if = "Option::is_none")]
    pub is_protection: Option<bool>,

    #[serde(rename = "isFlowControl", skip_serializing_if = "Option::is_none")]
    pub is_flow_control: Option<String>,

    #[serde(rename = "isUser", skip_serializing_if = "Option::is_none")]
    pub is_user: Option<String>,

    #[serde(rename = "isRequest", skip_serializing_if = "Option::is_none")]
    pub is_request: Option<bool>,

    #[serde(rename = "isResponse", skip_serializing_if = "Option::is_none")]
    pub is_response: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "left", skip_serializing_if = "Option::is_none")]
    pub left: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<Box<UnsignedIntExpression>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadAction {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<ReadActionType>>,

    #[serde(rename = "modify", skip_serializing_if = "Option::is_none")]
    pub modify: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadActionType {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "modify")]
    Modify,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RealExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterFile {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "accessHandles", skip_serializing_if = "Option::is_none")]
    pub access_handles: Option<String>,

    #[serde(rename = "array", skip_serializing_if = "Option::is_none")]
    pub array: Option<Box<super::Array>>,

    #[serde(rename = "addressOffset", skip_serializing_if = "Option::is_none")]
    pub address_offset: Option<Box<UnsignedLongintExpression>>,

    #[serde(rename = "registerFileDefinitionRef", skip_serializing_if = "Option::is_none")]
    pub register_file_definition_ref: Option<String>,

    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<UnsignedPositiveLongintExpression>>,

    #[serde(rename = "accessPolicies", skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Box<super::AccessPolicies>>,

    #[serde(rename = "registerData")]
    pub register_data: Vec<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterFileDefinitions {
    #[serde(rename = "registerFileDefinition")]
    pub register_file_definition: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterFileRef {
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Box<IndicesType>>,

    #[serde(rename = "registerFileRef", skip_serializing_if = "Option::is_none")]
    pub register_file_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RegisterRef {
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Box<IndicesType>>,

    #[serde(rename = "registerRef", skip_serializing_if = "Option::is_none")]
    pub register_ref: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequiresDriver {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,

    #[serde(rename = "driverType", skip_serializing_if = "Option::is_none")]
    pub driver_type: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "mask", skip_serializing_if = "Option::is_none")]
    pub mask: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "resetTypeRef", skip_serializing_if = "Option::is_none")]
    pub reset_type_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetTypeLinks {
    #[serde(rename = "resetTypeLink")]
    pub reset_type_link: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReturnTypeType {
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "int")]
    Int,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

    #[serde(rename = "typeParameters", skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<Box<TypeParameters>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedType {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "undefined")]
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalTypeDef {
    #[serde(rename = "signalType", skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<String>,

    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalTypeDefs {
    #[serde(rename = "signalTypeDef")]
    pub signal_type_def: Vec<SignalTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SignedLongintExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignType {
    #[serde(rename = "signed")]
    Signed,
    #[serde(rename = "unsigned")]
    Unsigned,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimpleAccessHandle {
    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "pathSegments", skip_serializing_if = "Option::is_none")]
    pub path_segments: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimpleClearboxType {
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "interface")]
    Interface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimpleFileType {
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
    #[serde(rename = "vhdlSource-2002")]
    VhdlSource2002,
    #[serde(rename = "vhdlSource-2008")]
    VhdlSource2008,
    #[serde(rename = "verilogSource")]
    VerilogSource,
    #[serde(rename = "verilogSource-95")]
    VerilogSource95,
    #[serde(rename = "verilogSource-2001")]
    VerilogSource2001,
    #[serde(rename = "verilogSource-2005")]
    VerilogSource2005,
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
    #[serde(rename = "systemVerilogSource-3.1a")]
    SystemVerilogSource31a,
    #[serde(rename = "systemVerilogSource-2009")]
    SystemVerilogSource2009,
    #[serde(rename = "systemVerilogSource-2012")]
    SystemVerilogSource2012,
    #[serde(rename = "systemVerilogSource-2017")]
    SystemVerilogSource2017,
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
    #[serde(rename = "systemCSource-2.3")]
    SystemCSource23,
    #[serde(rename = "systemCBinaryLibrary")]
    SystemCBinaryLibrary,
    #[serde(rename = "veraSource")]
    VeraSource,
    #[serde(rename = "eSource")]
    ESource,
    #[serde(rename = "perlSource")]
    PerlSource,
    #[serde(rename = "tclSource")]
    TclSource,
    #[serde(rename = "OVASource")]
    OVASource,
    #[serde(rename = "SVASource")]
    SVASource,
    #[serde(rename = "pslSource")]
    PslSource,
    #[serde(rename = "vhdlAmsSource")]
    VhdlAmsSource,
    #[serde(rename = "verilogAmsSource")]
    VerilogAmsSource,
    #[serde(rename = "systemCAmsSource")]
    SystemCAmsSource,
    #[serde(rename = "libertySource")]
    LibertySource,
    #[serde(rename = "spiceSource")]
    SpiceSource,
    #[serde(rename = "systemRDL")]
    SystemRDL,
    #[serde(rename = "systemRDL-1.0")]
    SystemRDL10,
    #[serde(rename = "systemRDL-2.0")]
    SystemRDL20,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimplePortAccessType {
    #[serde(rename = "ref")]
    Ref,
    #[serde(rename = "ptr")]
    Ptr,
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
pub struct SlicedAccessHandle {
    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "slices", skip_serializing_if = "Option::is_none")]
    pub slices: Option<Box<SlicesType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SlicesType {
    #[serde(rename = "slice")]
    pub slice: Vec<SliceType>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SliceType {
    #[serde(rename = "pathSegments", skip_serializing_if = "Option::is_none")]
    pub path_segments: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<Range>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StringExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StructPortTypeDefs {
    #[serde(rename = "structPortTypeDef")]
    pub struct_port_type_def: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubPortReference {
    #[serde(rename = "partSelect", skip_serializing_if = "Option::is_none")]
    pub part_select: Option<Box<PartSelect>>,

    #[serde(rename = "subPortRef", skip_serializing_if = "Option::is_none")]
    pub sub_port_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubPortType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<Box<PortWireType>>,

    #[serde(rename = "structured", skip_serializing_if = "Option::is_none")]
    pub structured: Option<Box<PortStructuredType>>,

    #[serde(rename = "arrays", skip_serializing_if = "Option::is_none")]
    pub arrays: Option<Box<super::Arrays>>,

    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Box<PortAccessType>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "isIO", skip_serializing_if = "Option::is_none")]
    pub is_i_o: Option<bool>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SubspaceRefType {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "baseAddress", skip_serializing_if = "Option::is_none")]
    pub base_address: Option<Box<SignedLongintExpression>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "initiatorRef", skip_serializing_if = "Option::is_none")]
    pub initiator_ref: Option<String>,

    #[serde(rename = "segmentRef", skip_serializing_if = "Option::is_none")]
    pub segment_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

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

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionalPowerConstraintType {
    #[serde(rename = "powerDomainRef", skip_serializing_if = "Option::is_none")]
    pub power_domain_ref: Option<String>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransparentBridge {
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "initiatorRef", skip_serializing_if = "Option::is_none")]
    pub initiator_ref: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMethodType {
    #[serde(rename = "file")]
    File,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

    #[serde(rename = "typeParameters", skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<Box<TypeParameters>>,

    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransTypeDefs {
    #[serde(rename = "transTypeDef")]
    pub trans_type_def: Vec<TransTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TypeDefinitions {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "shortDescription", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "externalTypeDefinitions")]
    pub external_type_definitions: Vec<ExternalTypeDefinitions>,

    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<String>,

    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Box<super::Views>>,

    #[serde(rename = "fieldAccessPolicyDefinitions", skip_serializing_if = "Option::is_none")]
    pub field_access_policy_definitions: Option<Box<FieldAccessPolicyDefinitions>>,

    #[serde(rename = "enumerationDefinitions", skip_serializing_if = "Option::is_none")]
    pub enumeration_definitions: Option<Box<EnumerationDefinitions>>,

    #[serde(rename = "fieldDefinitions", skip_serializing_if = "Option::is_none")]
    pub field_definitions: Option<Box<super::FieldDefinitions>>,

    #[serde(rename = "registerDefinitions", skip_serializing_if = "Option::is_none")]
    pub register_definitions: Option<Box<super::RegisterDefinitions>>,

    #[serde(rename = "registerFileDefinitions", skip_serializing_if = "Option::is_none")]
    pub register_file_definitions: Option<Box<RegisterFileDefinitions>>,

    #[serde(rename = "addressBlockDefinitions", skip_serializing_if = "Option::is_none")]
    pub address_block_definitions: Option<Box<AddressBlockDefinitions>>,

    #[serde(rename = "bankDefinitions", skip_serializing_if = "Option::is_none")]
    pub bank_definitions: Option<Box<BankDefinitions>>,

    #[serde(rename = "memoryMapDefinitions", skip_serializing_if = "Option::is_none")]
    pub memory_map_definitions: Option<Box<super::MemoryMapDefinitions>>,

    #[serde(rename = "memoryRemapDefinitions", skip_serializing_if = "Option::is_none")]
    pub memory_remap_definitions: Option<Box<MemoryRemapDefinitions>>,

    #[serde(rename = "resetTypes", skip_serializing_if = "Option::is_none")]
    pub reset_types: Option<String>,

    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Box<Choices>>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<Parameters>>,

    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Box<Assertions>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TypeParameters {
    #[serde(rename = "typeParameter")]
    pub type_parameter: Vec<ModuleParameterType>,

    #[serde(rename = "serviceTypeDef")]
    pub service_type_def: Vec<ServiceTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnresolvedStringExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnresolvedUnsignedBitExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnresolvedUnsignedPositiveIntExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedBitExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedBitVectorExpression {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedIntExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedLongintExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedPositiveIntExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UnsignedPositiveLongintExpression {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsageType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "reserved")]
    Reserved,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Vector {
    #[serde(rename = "left", skip_serializing_if = "Option::is_none")]
    pub left: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<Box<UnsignedIntExpression>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Vectors {
    #[serde(rename = "vector")]
    pub vector: Vec<Vector>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ViewLinks {
    #[serde(rename = "viewLink")]
    pub view_link: Vec<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ViewRef {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Wire {
    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Box<QualifierType>>,

    #[serde(rename = "onSystem")]
    pub on_system: Vec<super::OnSystem>,

    #[serde(rename = "onInitiator", skip_serializing_if = "Option::is_none")]
    pub on_initiator: Option<String>,

    #[serde(rename = "onTarget", skip_serializing_if = "Option::is_none")]
    pub on_target: Option<String>,

    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "requiresDriver", skip_serializing_if = "Option::is_none")]
    pub requires_driver: Option<Box<RequiresDriver>>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WirePowerConstraintType {
    #[serde(rename = "powerDomainRef", skip_serializing_if = "Option::is_none")]
    pub power_domain_ref: Option<String>,

    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Box<Range>>,

    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<Box<super::vendor_extensions::VendorExtensions>>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WireTypeDef {
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    #[serde(rename = "typeDefinition")]
    pub type_definition: Vec<String>,

    #[serde(rename = "viewRef")]
    pub view_ref: Vec<ViewRef>,

    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WireTypeDefs {
    #[serde(rename = "wireTypeDef")]
    pub wire_type_def: Vec<WireTypeDef>,

}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WriteValueConstraintType {
    #[serde(rename = "writeAsRead", skip_serializing_if = "Option::is_none")]
    pub write_as_read: Option<bool>,

    #[serde(rename = "useEnumeratedValues", skip_serializing_if = "Option::is_none")]
    pub use_enumerated_values: Option<bool>,

    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Box<UnsignedBitVectorExpression>>,

    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Box<UnsignedBitVectorExpression>>,

}

