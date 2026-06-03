//! IEEE IP-XACT 1685 Schema Implementation in Rust
//!
//! This crate provides Rust data structures for parsing, manipulating, and
//! serializing IP-XACT files (IEEE 1685 standard for XML metadata).
//!
//! # Supported Versions
//! - IEEE 1685-2009 (SPIRIT 1685-2009)
//! - IEEE 1685-2014 (IP-XACT 2014)
//! - IEEE 1685-2022 (IP-XACT 2022)
//!
//! # Example
//! ```rust,ignore
//! use ip_xact::v2009::Component;
//!
//! // Parse IP-XACT XML
//! let xml = std::fs::read_to_string("component.xml").unwrap();
//! let component: Component = serde_xml_rs::from_str(&xml).unwrap();
//! ```

pub mod error;

pub mod v2009;
pub mod v2014;
pub mod v2022;

pub use error::{Error, Result};
