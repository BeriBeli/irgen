mod parse;

pub use parse::{
    CatalogFileRef, LibraryRef, ParseOptions, catalog_file_refs, document_library_ref,
    parse_ipxact, parse_ipxact_with_options, parse_ipxact_with_options_and_resolver,
    parse_ipxact_with_resolver,
};

pub fn parse_numeric_expr(field: &'static str, value: &str) -> Result<u64> {
    irgen_ipxact_model::parse_u64_expr(field, value).map_err(Error::from)
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
    #[error("unsupported IP-XACT feature `{feature}` on {kind} `{name}`")]
    UnsupportedElementFeature {
        kind: &'static str,
        name: String,
        feature: String,
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
    #[error("unsupported IP-XACT namespace `{0}` (only IEEE 1685-2022 is supported)")]
    UnsupportedNamespace(String),
    #[error("unsupported IP-XACT element `{element}` in IEEE 1685-2022 input")]
    UnsupportedElement { element: String },
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
    #[error("duplicate IP-XACT {kind} name `{name}` under {parent_kind} `{parent}`")]
    DuplicateIpXactName {
        kind: &'static str,
        parent_kind: &'static str,
        parent: String,
        name: String,
    },
    #[error("duplicate IP-XACT memoryMap name `{name}`")]
    DuplicateMemoryMapName { name: String },
}

impl From<irgen_ipxact_model::ExpressionError> for Error {
    fn from(error: irgen_ipxact_model::ExpressionError) -> Self {
        match error {
            irgen_ipxact_model::ExpressionError::InvalidNumber { field, value } => {
                Self::InvalidNumber { field, value }
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
