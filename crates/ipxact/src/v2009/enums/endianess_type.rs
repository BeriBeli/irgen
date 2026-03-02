use serde::{Deserialize, Serialize};

/// Endianness type for memory and bus interfaces.
///
/// Maps to XML schema `endianessType` simple type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndianessType {
    #[serde(rename = "little")]
    Little,
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "unknown")]
    Unknown,
}

impl EndianessType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EndianessType::Little => "little",
            EndianessType::Big => "big",
            EndianessType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for EndianessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for EndianessType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "little" => Ok(EndianessType::Little),
            "big" => Ok(EndianessType::Big),
            "unknown" => Ok(EndianessType::Unknown),
            _ => Err(crate::Error::InvalidValue {
                field: "endianessType".to_string(),
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
    fn test_endianess_type_from_str() {
        assert_eq!(EndianessType::from_str("little").unwrap(), EndianessType::Little);
        assert_eq!(EndianessType::from_str("big").unwrap(), EndianessType::Big);
        assert_eq!(EndianessType::from_str("unknown").unwrap(), EndianessType::Unknown);
    }

    #[test]
    fn test_endianess_type_from_str_invalid() {
        assert!(EndianessType::from_str("invalid").is_err());
    }
}
