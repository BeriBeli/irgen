use super::processing::{ipxact, regvue};

/// Unified error type for irgen application
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    Calamine(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::prelude::PolarsError),

    #[error("XML Serialization error: {0}")]
    XmlSe(#[from] quick_xml::SeError),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("key not found: {key}")]
    KeyNotFound { key: String },

    #[error("empty data: {context}")]
    Empty { context: String },

    #[error("component not loaded: {context}")]
    NotLoaded { context: String },

    #[error("IP-XACT Component Error: {0}")]
    IpXactComponent(#[from] ipxact::ComponentBuilderError),

    #[error("IP-XACT MemoryMaps Error: {0}")]
    IpXactMemoryMaps(#[from] ipxact::MemoryMapsBuilderError),

    #[error("IP-XACT MemoryMap Error: {0}")]
    IpXactMemoryMap(#[from] ipxact::MemoryMapBuilderError),

    #[error("IP-XACT Block Error: {0}")]
    IpxactBlock(#[from] ipxact::BlockBuilderError),

    #[error("IP-XACT Register Error: {0}")]
    IpxactRegister(#[from] ipxact::RegisterBuilderError),

    #[error("IP-XACT Field Error: {0}")]
    IpxactField(#[from] ipxact::FieldBuilderError),

    #[error("IP-XACT Resets Error: {0}")]
    IpxactResets(#[from] ipxact::ResetsBuilderError),

    #[error("IP-XACT Reset Error: {0}")]
    IpxactReset(#[from] ipxact::ResetBuilderError),

    #[error("Regvue Schema error: {0}")]
    RegvueSchema(#[from] regvue::SchemaBuilderError),

    #[error("Regvue Link error: {0}")]
    RegvueLink(#[from] regvue::LinkBuilderError),

    #[error("Regvue Root error: {0}")]
    RegvueRoot(#[from] regvue::RootBuilderError),

    #[error("Regvue EnumValue error: {0}")]
    RegvueEnumValue(#[from] regvue::EnumValueBuilderError),

    #[error("Regvue Reset error: {0}")]
    RegvueReset(#[from] regvue::ResetBuilderError),

    #[error("Regvue Field error: {0}")]
    RegvueField(#[from] regvue::FieldBuilderError),

    #[error("Regvue Element error: {0}")]
    RegvueElement(#[from] regvue::ElementBuilderError),

    #[error("Regvue Document error: {0}")]
    RegvueDocument(#[from] regvue::DocumentBuilderError),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("invalid attribute: {attribute}")]
    InvalidAttribute { attribute: String },
}
