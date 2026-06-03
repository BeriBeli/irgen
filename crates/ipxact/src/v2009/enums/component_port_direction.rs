use serde::{Deserialize, Serialize};

/// Port direction type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentPortDirectionType {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "inout")]
    InOut,
}

impl std::fmt::Display for ComponentPortDirectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentPortDirectionType::In => write!(f, "in"),
            ComponentPortDirectionType::Out => write!(f, "out"),
            ComponentPortDirectionType::InOut => write!(f, "inout"),
        }
    }
}
