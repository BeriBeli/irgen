use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// Simple IP-XACT 2014 implementation

const IEEE1685_2014_NS: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014";
const XSI_NS: &str = "http://www.w3.org/2001/XMLSchema-instance";
const SCHEMA_LOCATION: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014 http://www.accellera.org/XMLSchema/IPXACT/1685-2014/index.xsd";

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
#[serde(rename = "ipxact:component")]
pub struct Component {
    #[serde(rename = "@xmlns:ipxact")]
    #[builder(default=IEEE1685_2014_NS.into())]
    xmlns_ipxact: String,
    #[serde(rename = "@xmlns:xsi")]
    #[builder(default=XSI_NS.into())]
    xmlns_xsi: String,
    #[serde(rename = "@xsi:schemaLocation")]
    #[builder(default=SCHEMA_LOCATION.into())]
    schema_location: String,
    #[serde(rename = "ipxact:vendor")]
    vendor: String,
    #[serde(rename = "ipxact:library")]
    library: String,
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:version")]
    version: String,
    #[serde(rename = "ipxact:memoryMaps")]
    memory_maps: MemoryMaps,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct MemoryMaps {
    #[serde(rename = "ipxact:memoryMap")]
    memory_map: Vec<MemoryMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct MemoryMap {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:addressBlock")]
    address_block: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Block {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "ipxact:baseAddress")]
    base_address: String,
    #[serde(rename = "ipxact:range")]
    range: String,
    #[serde(rename = "ipxact:width")]
    width: String,
    #[serde(rename = "ipxact:register")]
    register: Vec<Register>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Register {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "ipxact:addressOffset")]
    address_offset: String,
    #[serde(rename = "ipxact:size")]
    size: String,
    #[serde(rename = "ipxact:field")]
    field: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Field {
    #[serde(rename = "ipxact:name")]
    name: String,
    #[serde(rename = "ipxact:description", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    description: Option<String>,
    #[serde(rename = "ipxact:bitOffset")]
    bit_offset: String,
    #[serde(rename = "ipxact:bitWidth")]
    bit_width: String,
    #[serde(rename = "ipxact:access")]
    access: String,
    #[serde(
        rename = "ipxact:modifiedWriteValue",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    modified_write_value: Option<String>,
    #[serde(rename = "ipxact:readAction", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    read_action: Option<String>,
    #[serde(rename = "ipxact:resets")]
    resets: Resets,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Resets {
    #[serde(rename = "ipxact:reset")]
    reset: Vec<Reset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct Reset {
    #[serde(rename = "ipxact:value")]
    value: String,
}
