use serde::{Deserialize, Serialize};

use crate::v2009::AccessType;

/// Register type - defines a register within an address block.
///
/// Maps to XML schema anonymous complex type within registerFile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Field elements within the register
    #[serde(default, rename = "field")]
    pub field: Vec<Field>,

    /// Type identifier
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    /// Volatile flag
    #[serde(rename = "volatile", skip_serializing_if = "Option::is_none")]
    pub is_volatile: Option<bool>,

    /// Access type
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessType>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Field type - defines a field within a register.
///
/// Maps to XML schema `fieldType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// Unique name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Bit offset from bit 0 of the register (required)
    #[serde(rename = "bitOffset")]
    pub bit_offset: u32,

    /// Type identifier
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    /// Bit width
    #[serde(rename = "bitWidth", skip_serializing_if = "Option::is_none")]
    pub bit_width: Option<BitWidth>,

    /// Volatile flag
    #[serde(rename = "volatile", skip_serializing_if = "Option::is_none")]
    pub is_volatile: Option<bool>,

    /// Access type
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessType>,

    /// Enumerated values
    #[serde(rename = "enumeratedValues", skip_serializing_if = "Option::is_none")]
    pub enumerated_values: Option<EnumeratedValues>,

    /// Modified write value
    #[serde(rename = "modifiedWriteValue", skip_serializing_if = "Option::is_none")]
    pub modified_write_value: Option<ModifiedWriteValue>,

    /// Write value constraint
    #[serde(rename = "writeValueConstraint", skip_serializing_if = "Option::is_none")]
    pub write_value_constraint: Option<WriteValueConstraint>,

    /// Read action
    #[serde(rename = "readAction", skip_serializing_if = "Option::is_none")]
    pub read_action: Option<ReadAction>,

    /// Testable flag
    #[serde(rename = "testable", skip_serializing_if = "Option::is_none")]
    pub testable: Option<bool>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Bit width value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitWidth {
    /// The width value
    #[serde(rename = "$value")]
    pub value: Option<String>,

    /// Resolve attribute
    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Enumerated values for a field
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValues {
    /// List of enumerated values
    #[serde(default, rename = "enumeratedValue")]
    pub enumerated_value: Vec<EnumeratedValue>,
}

/// Single enumerated value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValue {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Value (required)
    #[serde(rename = "value")]
    pub value: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Modified write value type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModifiedWriteValue {
    #[serde(rename = "oneToClear")]
    OneToClear,
    #[serde(rename = "oneToSet")]
    OneToSet,
    #[serde(rename = "oneToToggle")]
    OneToToggle,
    #[serde(rename = "zeroToClear")]
    ZeroToClear,
    #[serde(rename = "zeroToSet")]
    ZeroToSet,
    #[serde(rename = "zeroToToggle")]
    ZeroToToggle,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "modify")]
    Modify,
}

/// Write value constraint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteValueConstraint {
    /// Minimum value
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    /// Maximum value
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
}

/// Read action type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadAction {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "modifyExternal")]
    ModifyExternal,
}

impl Register {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            field: Vec::new(),
            type_identifier: None,
            is_volatile: None,
            access: None,
            parameters: None,
            vendor_extensions: None,
            id: None,
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.field.push(field);
    }
}

impl Field {
    pub fn new(name: String, bit_offset: u32) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            bit_offset,
            type_identifier: None,
            bit_width: None,
            is_volatile: None,
            access: None,
            enumerated_values: None,
            modified_write_value: None,
            write_value_constraint: None,
            read_action: None,
            testable: None,
            parameters: None,
            vendor_extensions: None,
            id: None,
        }
    }

    pub fn with_bit_width(mut self, width: u32) -> Self {
        self.bit_width = Some(BitWidth {
            value: Some(width.to_string()),
            resolve: None,
            id: None,
        });
        self
    }
}

impl BitWidth {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            resolve: None,
            id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_new() {
        let reg = Register::new("CTRL".to_string());
        assert_eq!(reg.name, "CTRL");
        assert!(reg.field.is_empty());
    }

    #[test]
    fn test_register_with_field() {
        let mut reg = Register::new("CTRL".to_string());
        let field = Field::new("EN".to_string(), 0).with_bit_width(1);
        reg.add_field(field);
        assert_eq!(reg.field.len(), 1);
    }

    #[test]
    fn test_field_new() {
        let field = Field::new("EN".to_string(), 0).with_bit_width(1);
        assert_eq!(field.name, "EN");
        assert_eq!(field.bit_offset, 0);
    }
}
