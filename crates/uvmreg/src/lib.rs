mod model;
mod numeric;
mod parse;
mod render;

pub use model::*;
pub use parse::{
    CatalogFileRef, LibraryRef, ParseOptions, catalog_file_refs, document_library_ref,
    parse_ipxact, parse_ipxact_with_options, parse_ipxact_with_options_and_resolver,
    parse_ipxact_with_resolver,
};
pub use render::{
    RenderOptions, RenderedFile, serialize_uvm_reg, serialize_uvm_reg_by_block,
    serialize_uvm_reg_by_block_with_options, serialize_uvm_reg_with_options,
};

pub fn ipxact_to_uvm_reg(xml: &str) -> Result<String> {
    serialize_uvm_reg(&parse_ipxact(xml)?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("XML text decoding error: {0}")]
    Text(#[from] quick_xml::encoding::EncodingError),
    #[error("XML attribute error: {0}")]
    Attribute(#[from] quick_xml::events::attributes::AttrError),
    #[error("unexpected XML end event `{0}`")]
    UnexpectedEnd(String),
    #[error("missing required IP-XACT element `{0}`")]
    MissingElement(&'static str),
    #[error("invalid IP-XACT number for {field}: `{value}`")]
    InvalidNumber { field: &'static str, value: String },
    #[error("invalid IP-XACT boolean for {field}: `{value}`")]
    InvalidBoolean { field: &'static str, value: String },
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
    #[error("unsupported IP-XACT feature `{feature}` on {kind} `{name}`")]
    UnsupportedElementFeature {
        kind: &'static str,
        name: String,
        feature: String,
    },
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
        "unsupported IP-XACT parameter expression `{parameter}` used in {field}: `{expression}`"
    )]
    UnsupportedParameterExpression {
        parameter: String,
        field: &'static str,
        expression: String,
    },
    #[error("IP-XACT root element must be component, found `{0}`")]
    UnsupportedRoot(String),
    #[error("external IP-XACT typeDefinitionsRef not found: `{0}`")]
    ExternalTypeDefinitionsNotFound(String),
    #[error(
        "external IP-XACT typeDefinitionsRef not found: `{reference}` (searched: {})",
        searched.join(", ")
    )]
    ExternalTypeDefinitionsNotFoundIn {
        reference: String,
        searched: Vec<String>,
    },
    #[error(
        "external IP-XACT typeDefinitionsRef is ambiguous: `{reference}` (matches: {})",
        matches.join(", ")
    )]
    ExternalTypeDefinitionsAmbiguous {
        reference: String,
        matches: Vec<String>,
    },
    #[error("IP-XACT {kind} not found: `{reference}`")]
    TypeDefinitionNotFound {
        kind: &'static str,
        reference: String,
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
    #[error("IP-XACT indexed accessHandle for `{owner}` is missing indices")]
    AccessHandleIndicesMissing { owner: String },
    #[error("IP-XACT indexed accessHandle for `{owner}` is missing a path")]
    IndexedAccessHandlePathMissing { owner: String },
    #[error("IP-XACT accessHandle for `{owner}` is missing a path")]
    AccessHandlePathMissing { owner: String },
    #[error(
        "IP-XACT accessHandle pathSegment for `{owner}` must not include SystemVerilog string quotes: `{segment}`"
    )]
    AccessHandlePathSegmentStringLiteral { owner: String, segment: String },
    #[error(
        "IP-XACT accessHandle for `{owner}` does not define requested view `{view}` and has no generic fallback"
    )]
    AccessHandleViewNotFound { owner: String, view: String },
    #[error(
        "IP-XACT {kind} `{name}` access policy does not define requested mode `{mode}` and has no generic fallback"
    )]
    AccessPolicyModeNotFound {
        kind: &'static str,
        name: String,
        mode: String,
    },
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
    #[error("duplicate IP-XACT {kind} name `{name}` under {parent_kind} `{parent}`")]
    DuplicateIpXactName {
        kind: &'static str,
        parent_kind: &'static str,
        parent: String,
        name: String,
    },
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
    #[error("duplicate IP-XACT memoryMap name `{name}`")]
    DuplicateMemoryMapName { name: String },
    #[error("template rendering error: {0}")]
    Template(#[from] askama::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
