mod model;
mod parse;
mod render;

pub use model::*;
pub use parse::{LibraryRef, document_library_ref, parse_ipxact, parse_ipxact_with_resolver};
pub use render::{RenderOptions, serialize_uvm_reg, serialize_uvm_reg_with_options};

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
    #[error("template rendering error: {0}")]
    Template(#[from] askama::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
