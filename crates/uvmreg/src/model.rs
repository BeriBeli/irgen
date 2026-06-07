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
    pub blocks: Vec<AddressBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRemap {
    pub name: String,
    pub map_name: String,
    pub mode_refs: Vec<ModeRef>,
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
    pub description: String,
    pub map_name: String,
    pub base_address: String,
    pub range: String,
    pub width: String,
    pub address_unit_bits: String,
    pub usage: Option<String>,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub access_policies: Vec<AccessPolicy>,
    pub hdl_path: Option<String>,
    pub registers: Vec<Register>,
    pub register_files: Vec<RegisterFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterFile {
    pub name: String,
    pub description: String,
    pub address_offset: String,
    pub range: String,
    pub dim: String,
    pub dims: Vec<String>,
    pub stride: Option<String>,
    pub hdl_path: Option<String>,
    pub registers: Vec<Register>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub address_offset: String,
    pub size: String,
    pub dim: String,
    pub dims: Vec<String>,
    pub stride: Option<String>,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub access_policies: Vec<AccessPolicy>,
    pub hdl_path: Option<String>,
    pub fields: Vec<Field>,
    pub alternate_registers: Vec<AlternateRegister>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternateRegister {
    pub name: String,
    pub description: String,
    pub volatile: Option<String>,
    pub access: Option<String>,
    pub access_policies: Vec<AccessPolicy>,
    pub hdl_path: Option<String>,
    pub groups_or_modes: Vec<ModeRef>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub bit_offset: String,
    pub bit_width: String,
    pub access: Option<String>,
    pub access_policies: Vec<AccessPolicy>,
    pub modified_write_value: Option<String>,
    pub read_action: Option<String>,
    pub volatile: Option<String>,
    pub reset: Option<String>,
    pub resets: Vec<Reset>,
    pub hdl_path: Option<String>,
    pub testable: Option<Testable>,
    pub reserved: Option<String>,
    pub write_value_constraint: Option<WriteValueConstraint>,
    pub access_restrictions: Vec<AccessRestriction>,
    pub broadcasts: Vec<Broadcast>,
    pub enumerated_values: Vec<EnumeratedValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reset {
    pub value: String,
    pub mask: Option<String>,
    pub reset_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Testable {
    pub value: String,
    pub test_constraint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteValueConstraint {
    pub write_as_read: Option<String>,
    pub use_enumerated_values: Option<String>,
    pub minimum: Option<String>,
    pub maximum: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessRestriction {
    pub mode_refs: Vec<ModeRef>,
    pub read_access_mask: Option<String>,
    pub write_access_mask: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPolicy {
    pub access: Option<String>,
    pub mode_refs: Vec<ModeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeRef {
    pub name: String,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Broadcast {
    pub target: Vec<FieldReferenceSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldReferenceSegment {
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratedValue {
    pub name: String,
    pub value: String,
    pub usage: Option<String>,
}
