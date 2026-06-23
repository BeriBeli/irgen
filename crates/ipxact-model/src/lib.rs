mod expression;

pub use expression::{
    ExpressionError, ExpressionResult, parse_bool_expr_with_symbols, parse_u64_expr,
    parse_u64_expr_with_symbols,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub vendor: String,
    pub library: String,
    pub name: String,
    pub version: String,
    pub address_spaces: Vec<AddressSpace>,
    pub blocks: Vec<AddressBlock>,
    pub subspace_maps: Vec<SubspaceMap>,
    pub memory_remaps: Vec<MemoryRemap>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressSpace {
    pub name: String,
    pub address_unit_bits: String,
    pub segments: Vec<Segment>,
    pub blocks: Vec<AddressBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub name: String,
    pub address_offset: String,
    pub range: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRemap {
    pub name: String,
    pub map_name: String,
    pub blocks: Vec<AddressBlock>,
    pub subspace_maps: Vec<SubspaceMap>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubspaceMap {
    pub name: String,
    pub map_name: String,
    pub base_address: String,
    pub address_unit_bits: String,
    pub initiator_ref: String,
    pub address_space_ref: Option<String>,
    pub segment_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressBlock {
    pub name: String,
    pub map_name: String,
    pub base_address: String,
    pub range: String,
    pub width: String,
    pub description: String,
    pub address_unit_bits: String,
    pub usage: Option<String>,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub hdl_path: Option<String>,
    pub registers: Vec<Register>,
    pub register_files: Vec<RegisterFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterFile {
    pub name: String,
    pub address_offset: String,
    pub range: String,
    pub description: String,
    pub dim: String,
    pub dims: Vec<String>,
    pub stride: Option<String>,
    pub hdl_path: Option<String>,
    pub registers: Vec<Register>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub address_offset: String,
    pub size: String,
    pub description: String,
    pub dim: String,
    pub dims: Vec<String>,
    pub stride: Option<String>,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub hdl_path: Option<String>,
    pub indexed_hdl_paths: Vec<IndexedHdlPath>,
    pub fields: Vec<Field>,
    pub alternate_registers: Vec<AlternateRegister>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedHdlPath {
    pub indices: Vec<String>,
    pub path: String,
    pub slices: Vec<HdlPathSlice>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlPathSlice {
    pub path: String,
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternateRegister {
    pub name: String,
    pub description: String,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub hdl_path: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub bit_offset: String,
    pub bit_width: String,
    pub description: String,
    pub access: Option<String>,
    pub modified_write_value: Option<String>,
    pub read_action: Option<String>,
    pub volatile: Option<String>,
    pub testable: Option<String>,
    pub reserved: Option<String>,
    pub reset: Option<String>,
    pub resets: Vec<Reset>,
    pub hdl_path: Option<String>,
    pub hdl_path_slices: Vec<HdlPathSlice>,
    pub indexed_hdl_paths: Vec<IndexedHdlPath>,
    pub enumerated_values: Vec<EnumeratedValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reset {
    pub value: String,
    pub mask: Option<String>,
    pub reset_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedValue {
    pub name: String,
    pub value: String,
}
