use serde::{Deserialize, Serialize};

/// IP-XACT version enum with namespace constants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpXactVersion {
    #[serde(rename = "IEEE_1685_2009")]
    Ieee1685_2009,
    #[serde(rename = "IEEE_1685_2014")]
    Ieee1685_2014,
    #[serde(rename = "IEEE_1685_2022")]
    Ieee1685_2022,
}

impl IpXactVersion {
    /// Returns the XML namespace for this version
    pub fn namespace(&self) -> &'static str {
        match self {
            IpXactVersion::Ieee1685_2009 => {
                "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009"
            }
            IpXactVersion::Ieee1685_2014 => "http://www.accellera.org/XMLSchema/IPXACT/1685-2014",
            IpXactVersion::Ieee1685_2022 => "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        }
    }

    /// Returns the schema location URL for this version
    pub fn schema_location(&self) -> String {
        format!("{} {}", self.namespace(), self.schema_url())
    }

    /// Returns the schema URL for this version
    pub fn schema_url(&self) -> &'static str {
        match self {
            IpXactVersion::Ieee1685_2009 => {
                "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009/index.xsd"
            }
            IpXactVersion::Ieee1685_2014 => {
                "http://www.accellera.org/XMLSchema/IPXACT/1685-2014/index.xsd"
            }
            IpXactVersion::Ieee1685_2022 => {
                "http://www.accellera.org/XMLSchema/IPXACT/1685-2022/index.xsd"
            }
        }
    }

    /// Returns the short version string (e.g., "1685-2009")
    pub fn version_string(&self) -> &'static str {
        match self {
            IpXactVersion::Ieee1685_2009 => "1685-2009",
            IpXactVersion::Ieee1685_2014 => "1685-2014",
            IpXactVersion::Ieee1685_2022 => "1685-2022",
        }
    }

    /// Parse version from namespace URL
    pub fn from_namespace(ns: &str) -> Option<Self> {
        match ns {
            ns if ns.contains("1685-2009") => Some(IpXactVersion::Ieee1685_2009),
            ns if ns.contains("1685-2014") => Some(IpXactVersion::Ieee1685_2014),
            ns if ns.contains("1685-2022") => Some(IpXactVersion::Ieee1685_2022),
            _ => None,
        }
    }

    /// Parse version from version string
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "1685-2009" | "IEEE_1685_2009" => Some(IpXactVersion::Ieee1685_2009),
            "1685-2014" | "IEEE_1685_2014" => Some(IpXactVersion::Ieee1685_2014),
            "1685-2022" | "IEEE_1685_2022" => Some(IpXactVersion::Ieee1685_2022),
            _ => None,
        }
    }
}

impl std::fmt::Display for IpXactVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.version_string())
    }
}

impl std::str::FromStr for IpXactVersion {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s).ok_or_else(|| crate::Error::UnknownVersion(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(
            IpXactVersion::Ieee1685_2009.namespace(),
            "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009"
        );
        assert_eq!(
            IpXactVersion::Ieee1685_2014.namespace(),
            "http://www.accellera.org/XMLSchema/IPXACT/1685-2014"
        );
        assert_eq!(
            IpXactVersion::Ieee1685_2022.namespace(),
            "http://www.accellera.org/XMLSchema/IPXACT/1685-2022"
        );
    }

    #[test]
    fn test_version_string() {
        assert_eq!(IpXactVersion::Ieee1685_2009.version_string(), "1685-2009");
        assert_eq!(IpXactVersion::Ieee1685_2014.version_string(), "1685-2014");
        assert_eq!(IpXactVersion::Ieee1685_2022.version_string(), "1685-2022");
    }

    #[test]
    fn test_schema_location() {
        let expected = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009 \
            http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009/index.xsd";
        assert_eq!(IpXactVersion::Ieee1685_2009.schema_location(), expected);
    }

    #[test]
    fn test_from_namespace() {
        assert_eq!(
            IpXactVersion::from_namespace("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009"),
            Some(IpXactVersion::Ieee1685_2009)
        );
        assert_eq!(
            IpXactVersion::from_namespace("http://www.accellera.org/XMLSchema/IPXACT/1685-2014"),
            Some(IpXactVersion::Ieee1685_2014)
        );
        assert_eq!(IpXactVersion::from_namespace("http://other.namespace"), None);
    }

    #[test]
    fn test_from_string() {
        assert_eq!(IpXactVersion::from_string("1685-2009"), Some(IpXactVersion::Ieee1685_2009));
        assert_eq!(IpXactVersion::from_string("IEEE_1685_2014"), Some(IpXactVersion::Ieee1685_2014));
        assert_eq!(IpXactVersion::from_string("invalid"), None);
    }
}
