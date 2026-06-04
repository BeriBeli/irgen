//! IEEE 1685-2022 component model for register-oriented memory maps.

use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2022";
pub const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";
pub const SCHEMA_LOCATION: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2022 http://www.accellera.org/XMLSchema/IPXACT/1685-2022/index.xsd";

fn namespace() -> String {
    NAMESPACE.into()
}

fn xsi_namespace() -> String {
    XSI_NAMESPACE.into()
}

fn schema_location() -> String {
    SCHEMA_LOCATION.into()
}

/// Root component used for register-oriented IEEE 1685-2022 output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "ipxact:component", deserialize = "component"))]
pub struct Component {
    #[serde(rename = "@xmlns:ipxact", default = "namespace")]
    pub xmlns_ipxact: String,

    #[serde(rename = "@xmlns:xsi", default = "xsi_namespace")]
    pub xmlns_xsi: String,

    #[serde(
        rename(serialize = "@xsi:schemaLocation", deserialize = "@schemaLocation"),
        default = "schema_location"
    )]
    pub schema_location: String,

    #[serde(rename(serialize = "ipxact:vendor", deserialize = "vendor"))]
    pub vendor: String,

    #[serde(rename(serialize = "ipxact:library", deserialize = "library"))]
    pub library: String,

    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "ipxact:version", deserialize = "version"))]
    pub version: String,

    #[serde(
        rename(serialize = "ipxact:memoryMaps", deserialize = "memoryMaps"),
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maps: Option<MemoryMaps>,
}

impl Component {
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
            vendor: vendor.into(),
            library: library.into(),
            name: name.into(),
            version: version.into(),
            memory_maps: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoryMaps {
    #[serde(
        rename(serialize = "ipxact:memoryMap", deserialize = "memoryMap"),
        default
    )]
    pub memory_map: Vec<MemoryMap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMap {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:addressBlock", deserialize = "addressBlock"),
        default
    )]
    pub address_block: Vec<AddressBlock>,
}

impl MemoryMap {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            address_block: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBlock {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "ipxact:baseAddress", deserialize = "baseAddress"))]
    pub base_address: String,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: String,

    #[serde(rename(serialize = "ipxact:width", deserialize = "width"))]
    pub width: String,

    #[serde(
        rename(serialize = "ipxact:register", deserialize = "register"),
        default
    )]
    pub register: Vec<Register>,

    #[serde(
        rename(serialize = "ipxact:registerFile", deserialize = "registerFile"),
        default
    )]
    pub register_file: Vec<RegisterFile>,
}

impl AddressBlock {
    pub fn new(
        name: impl Into<String>,
        base_address: impl Into<String>,
        range: impl Into<String>,
        width: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            base_address: base_address.into(),
            range: range.into(),
            width: width.into(),
            register: Vec::new(),
            register_file: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterFile {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:array", deserialize = "array"),
        skip_serializing_if = "Option::is_none"
    )]
    pub array: Option<RegisterFileArray>,

    #[serde(rename(serialize = "ipxact:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: String,

    #[serde(rename(serialize = "ipxact:range", deserialize = "range"))]
    pub range: String,

    #[serde(
        rename(serialize = "ipxact:register", deserialize = "register"),
        default
    )]
    pub register: Vec<Register>,
}

impl RegisterFile {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<String>,
        range: impl Into<String>,
        dim: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            array: Some(RegisterFileArray {
                dim: vec![dim.into()],
            }),
            address_offset: address_offset.into(),
            range: range.into(),
            register: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RegisterFileArray {
    #[serde(rename(serialize = "ipxact:dim", deserialize = "dim"), default)]
    pub dim: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(rename(serialize = "ipxact:addressOffset", deserialize = "addressOffset"))]
    pub address_offset: String,

    #[serde(rename(serialize = "ipxact:size", deserialize = "size"))]
    pub size: String,

    #[serde(rename(serialize = "ipxact:field", deserialize = "field"), default)]
    pub field: Vec<Field>,
}

impl Register {
    pub fn new(
        name: impl Into<String>,
        address_offset: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            address_offset: address_offset.into(),
            size: size.into(),
            field: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename(serialize = "ipxact:name", deserialize = "name"))]
    pub name: String,

    #[serde(
        rename(serialize = "ipxact:description", deserialize = "description"),
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename(serialize = "ipxact:bitOffset", deserialize = "bitOffset"))]
    pub bit_offset: String,

    #[serde(rename(serialize = "ipxact:bitWidth", deserialize = "bitWidth"))]
    pub bit_width: String,

    #[serde(
        rename(serialize = "ipxact:resets", deserialize = "resets"),
        skip_serializing_if = "Option::is_none"
    )]
    pub resets: Option<Resets>,

    #[serde(
        rename(
            serialize = "ipxact:fieldAccessPolicies",
            deserialize = "fieldAccessPolicies"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub field_access_policies: Option<FieldAccessPolicies>,
}

impl Field {
    pub fn new(
        name: impl Into<String>,
        bit_offset: impl Into<String>,
        bit_width: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: None,
            bit_offset: bit_offset.into(),
            bit_width: bit_width.into(),
            resets: None,
            field_access_policies: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Resets {
    #[serde(rename(serialize = "ipxact:reset", deserialize = "reset"), default)]
    pub reset: Vec<Reset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reset {
    #[serde(rename(serialize = "ipxact:value", deserialize = "value"))]
    pub value: String,
}

impl Reset {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessPolicies {
    #[serde(
        rename(
            serialize = "ipxact:fieldAccessPolicy",
            deserialize = "fieldAccessPolicy"
        ),
        default
    )]
    pub field_access_policy: Vec<FieldAccessPolicy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessPolicy {
    #[serde(
        rename(serialize = "ipxact:access", deserialize = "access"),
        skip_serializing_if = "Option::is_none"
    )]
    pub access: Option<String>,

    #[serde(
        rename(
            serialize = "ipxact:modifiedWriteValue",
            deserialize = "modifiedWriteValue"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_write_value: Option<String>,

    #[serde(
        rename(serialize = "ipxact:readAction", deserialize = "readAction"),
        skip_serializing_if = "Option::is_none"
    )]
    pub read_action: Option<String>,
}

impl FieldAccessPolicy {
    pub fn new() -> Self {
        Self {
            access: None,
            modified_write_value: None,
            read_action: None,
        }
    }
}
