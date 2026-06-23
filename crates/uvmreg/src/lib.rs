mod numeric;
mod render;

pub use render::{
    FileType, RenderOptions, RenderedFile, serialize_uvm_reg, serialize_uvm_reg_by_block,
    serialize_uvm_reg_by_block_with_options, serialize_uvm_reg_with_options,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing required IP-XACT element `{0}`")]
    MissingElement(&'static str),
    #[error("invalid IP-XACT number for {field}: `{value}`")]
    InvalidNumber { field: &'static str, value: String },
    #[error(
        "unsupported IP-XACT access policy for field `{field}`: access=`{access}`, modifiedWriteValue={}, readAction={}",
        modified_write_value.as_deref().unwrap_or("<none>"),
        read_action.as_deref().unwrap_or("<none>")
    )]
    UnsupportedAccessPolicy {
        field: String,
        access: String,
        modified_write_value: Option<String>,
        read_action: Option<String>,
    },
    #[error(
        "unsupported IP-XACT memory access for memory block `{block}`: `{access}` (uvm_mem supports `read-write`/`RW` and `read-only`/`RO`)"
    )]
    UnsupportedMemoryAccess { block: String, access: String },
    #[error(
        "IP-XACT field `{field}` in register `{register}` spans bits {lsb}..{msb}, beyond register size {size}"
    )]
    FieldRangeExceedsRegisterSize {
        register: String,
        field: String,
        lsb: u64,
        msb: u64,
        size: u64,
    },
    #[error(
        "IP-XACT fields `{field}` and `{other}` overlap in register `{register}` at bits {lsb}..{msb}"
    )]
    FieldRangeOverlap {
        register: String,
        field: String,
        other: String,
        lsb: u64,
        msb: u64,
    },
    #[error(
        "IP-XACT address ranges `{name}` and `{other}` overlap in addressBlock `{block}` at offsets {start}..{end}"
    )]
    AddressRangeOverlap {
        block: String,
        name: String,
        other: String,
        start: u64,
        end: u64,
    },
    #[error(
        "IP-XACT address ranges `{name}` and `{other}` overlap in memoryMap `{map}` at offsets {start}..{end}"
    )]
    MapAddressRangeOverlap {
        map: String,
        name: String,
        other: String,
        start: u64,
        end: u64,
    },
    #[error(
        "IP-XACT address range `{name}` in registerFile `{register_file}` ends at offset {end}, beyond registerFile range {range}"
    )]
    RegisterFileRangeExceeded {
        register_file: String,
        name: String,
        end: u64,
        range: u64,
    },
    #[error(
        "IP-XACT address ranges `{name}` and `{other}` overlap in registerFile `{register_file}` at offsets {start}..{end}"
    )]
    RegisterFileAddressRangeOverlap {
        register_file: String,
        name: String,
        other: String,
        start: u64,
        end: u64,
    },
    #[error(
        "IP-XACT accessHandle indices for register `{register}` have {actual} dimensions, expected {expected}"
    )]
    AccessHandleIndexDimensionMismatch {
        register: String,
        expected: usize,
        actual: usize,
    },
    #[error(
        "IP-XACT accessHandle index {index} for `{owner}` dimension {dimension} is outside register array dimension size {size}"
    )]
    AccessHandleIndexOutOfRange {
        owner: String,
        dimension: usize,
        index: u64,
        size: u64,
    },
    #[error("duplicate IP-XACT accessHandle indices for `{owner}`: `{indices}`")]
    DuplicateAccessHandleIndices { owner: String, indices: String },
    #[error(
        "IP-XACT accessHandle slices for field `{field}` require ranges when more than one slice is present"
    )]
    AccessHandleSliceRangeMissing { field: String },
    #[error(
        "IP-XACT accessHandle slices for field `{field}` have total width {actual}, expected {expected}"
    )]
    AccessHandleSliceWidthMismatch {
        field: String,
        expected: u64,
        actual: u64,
    },
    #[error("duplicate generated SystemVerilog class name `{name}`")]
    DuplicateGeneratedClassName { name: String },
    #[error(
        "subspaceMap `{subspace}` segmentRef `{segment}` does not cover addressBlock `{block}`"
    )]
    SegmentRefRangeViolation {
        subspace: String,
        segment: String,
        block: String,
    },
    #[error(
        "subspaceMap `{subspace}` initiatorRef `{initiator}` does not resolve to a local addressSpace"
    )]
    SubspaceMapAddressSpaceNotFound { subspace: String, initiator: String },
    #[error(
        "subspaceMap `{subspace}` segmentRef `{segment}` was not found in addressSpace `{address_space}`"
    )]
    SubspaceMapSegmentNotFound {
        subspace: String,
        segment: String,
        address_space: String,
    },
    #[error("template rendering error: {0}")]
    Template(#[from] askama::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
