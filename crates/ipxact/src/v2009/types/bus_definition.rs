use serde::{Deserialize, Serialize};

/// BusDefinition type - defines a bus protocol.
///
/// Maps to XML schema `busDefinitionType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusDefinition {
    /// Vendor name (required)
    #[serde(rename = "vendor")]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename = "library")]
    pub library: String,

    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version (required)
    #[serde(rename = "version")]
    pub version: String,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

impl BusDefinition {
    pub fn new(vendor: String, library: String, name: String, version: String) -> Self {
        Self {
            vendor,
            library,
            name,
            version,
            description: None,
            vendor_extensions: None,
        }
    }
}

/// AbstractionDefinition type - defines the abstraction view of a bus.
///
/// Maps to XML schema anonymous complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractionDefinition {
    /// Vendor name (required)
    #[serde(rename = "vendor")]
    pub vendor: String,

    /// Library name (required)
    #[serde(rename = "library")]
    pub library: String,

    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version (required)
    #[serde(rename = "version")]
    pub version: String,

    /// Bus type reference
    #[serde(rename = "busType")]
    pub bus_type: super::bus_interface::LibraryRef,

    /// Extends
    #[serde(rename = "extends", skip_serializing_if = "Option::is_none")]
    pub extends: Option<super::bus_interface::LibraryRef>,

    /// Ports
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<AbstractionPorts>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

impl AbstractionDefinition {
    pub fn new(vendor: String, library: String, name: String, version: String, bus_type: super::bus_interface::LibraryRef) -> Self {
        Self {
            vendor,
            library,
            name,
            version,
            bus_type,
            extends: None,
            ports: None,
            description: None,
            vendor_extensions: None,
        }
    }
}

/// Container for abstraction ports
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AbstractionPorts {
    /// List of abstraction ports
    #[serde(default, rename = "port")]
    pub port: Vec<AbstractionPort>,
}

/// Single abstraction port definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractionPort {
    /// Logical name (required)
    #[serde(rename = "logicalName")]
    pub logical_name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Wire abstraction
    #[serde(rename = "wire", skip_serializing_if = "Option::is_none")]
    pub wire: Option<WireAbstraction>,

    /// Transactional abstraction
    #[serde(rename = "transactional", skip_serializing_if = "Option::is_none")]
    pub transactional: Option<TransactionalAbstraction>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Wire abstraction definition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WireAbstraction {
    /// Qualifier
    #[serde(rename = "qualifier", skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<Qualifier>,

    /// On system groups
    #[serde(rename = "onSystem", skip_serializing_if = "Option::is_none")]
    pub on_system: Option<Vec<OnSystem>>,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

/// Qualifier for wire signals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Qualifier {
    /// Is address
    #[serde(rename = "isAddress", skip_serializing_if = "Option::is_none")]
    pub is_address: Option<bool>,

    /// Is data
    #[serde(rename = "isData", skip_serializing_if = "Option::is_none")]
    pub is_data: Option<bool>,

    /// Is clock
    #[serde(rename = "isClock", skip_serializing_if = "Option::is_none")]
    pub is_clock: Option<bool>,

    /// Is reset
    #[serde(rename = "isReset", skip_serializing_if = "Option::is_none")]
    pub is_reset: Option<bool>,
}

/// On system group definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnSystem {
    /// Group name (required)
    #[serde(rename = "group")]
    pub group: String,

    /// Presence
    #[serde(rename = "presence", skip_serializing_if = "Option::is_none")]
    pub presence: Option<String>,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

/// Transactional abstraction definition
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionalAbstraction {
    /// Protocol
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TransactionalProtocol>,

    /// Presence
    #[serde(rename = "presence", skip_serializing_if = "Option::is_none")]
    pub presence: Option<String>,
}

/// Transactional protocol
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionalProtocol {
    /// Protocol type (required)
    #[serde(rename = "protocolType")]
    pub protocol_type: ProtocolTypeDef,

    /// Transaction type
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionTypeDef>,
}

/// Protocol type definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolTypeDef {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Transaction type definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionTypeDef {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2009::LibraryRef;

    #[test]
    fn test_bus_definition_new() {
        let bus_def = BusDefinition::new(
            "vendor".to_string(),
            "library".to_string(),
            "amba".to_string(),
            "1.0".to_string(),
        );
        assert_eq!(bus_def.name, "amba");
    }

    #[test]
    fn test_abstraction_definition_new() {
        let bus_type = LibraryRef::new("vendor".to_string(), "apb".to_string(), "1.0".to_string());
        let abs_def = AbstractionDefinition::new(
            "vendor".to_string(),
            "library".to_string(),
            "apb_abs".to_string(),
            "1.0".to_string(),
            bus_type,
        );
        assert_eq!(abs_def.name, "apb_abs");
    }
}
