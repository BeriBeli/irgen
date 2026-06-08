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
    #[error("duplicate IP-XACT memoryMap name `{name}`")]
    DuplicateMemoryMapName { name: String },
    #[error("template rendering error: {0}")]
    Template(#[from] askama::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
