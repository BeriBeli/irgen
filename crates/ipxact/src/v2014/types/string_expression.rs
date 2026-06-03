//! String expression types for IP-XACT 2014

use serde::{Deserialize, Serialize};

/// String expression - a string that can contain expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringExpression {
    /// The expression value
    #[serde(rename = "$text")]
    pub value: String,
}

impl StringExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<String> for StringExpression {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for StringExpression {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// String URI expression - a URI-valued string expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringURIExpression {
    /// The URI value
    #[serde(rename = "$text")]
    pub value: String,
}

impl StringURIExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<String> for StringURIExpression {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for StringURIExpression {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
