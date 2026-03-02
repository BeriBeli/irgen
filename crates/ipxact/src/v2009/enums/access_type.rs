use serde::{Deserialize, Serialize};

/// The read/write accessibility of an address block.
///
/// Maps to XML schema `accessType` simple type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    #[serde(rename = "read-only")]
    ReadOnly,
    #[serde(rename = "write-only")]
    WriteOnly,
    #[serde(rename = "read-write")]
    ReadWrite,
    #[serde(rename = "writeOnce")]
    WriteOnce,
    #[serde(rename = "read-writeOnce")]
    ReadWriteOnce,
}

impl AccessType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessType::ReadOnly => "read-only",
            AccessType::WriteOnly => "write-only",
            AccessType::ReadWrite => "read-write",
            AccessType::WriteOnce => "writeOnce",
            AccessType::ReadWriteOnce => "read-writeOnce",
        }
    }
}

impl std::fmt::Display for AccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for AccessType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read-only" => Ok(AccessType::ReadOnly),
            "write-only" => Ok(AccessType::WriteOnly),
            "read-write" => Ok(AccessType::ReadWrite),
            "writeOnce" => Ok(AccessType::WriteOnce),
            "read-writeOnce" => Ok(AccessType::ReadWriteOnce),
            _ => Err(crate::Error::InvalidValue {
                field: "accessType".to_string(),
                value: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_access_type_from_str() {
        assert_eq!(AccessType::from_str("read-only").unwrap(), AccessType::ReadOnly);
        assert_eq!(AccessType::from_str("write-only").unwrap(), AccessType::WriteOnly);
        assert_eq!(AccessType::from_str("read-write").unwrap(), AccessType::ReadWrite);
        assert_eq!(AccessType::from_str("writeOnce").unwrap(), AccessType::WriteOnce);
        assert_eq!(AccessType::from_str("read-writeOnce").unwrap(), AccessType::ReadWriteOnce);
    }

    #[test]
    fn test_access_type_from_str_invalid() {
        assert!(AccessType::from_str("invalid").is_err());
    }

    #[test]
    fn test_access_type_display() {
        assert_eq!(AccessType::ReadOnly.to_string(), "read-only");
        assert_eq!(AccessType::WriteOnly.to_string(), "write-only");
        assert_eq!(AccessType::ReadWrite.to_string(), "read-write");
        assert_eq!(AccessType::WriteOnce.to_string(), "writeOnce");
        assert_eq!(AccessType::ReadWriteOnce.to_string(), "read-writeOnce");
    }

    #[test]
    fn test_access_type_serde_roundtrip() {
        let original = AccessType::ReadWrite;
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AccessType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
