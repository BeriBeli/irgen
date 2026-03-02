//! String expression types for IP-XACT 2014

use serde::{Deserialize, Serialize};

/// String expression - a string that can contain expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringExpression {
    /// The expression value
    #[serde(rename = "$value")]
    pub value: String,

    /// Whether value is an expression
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl StringExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            format: None,
        }
    }
}

/// String URI expression - a URI-valued string expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringURIExpression {
    /// The URI value
    #[serde(rename = "$value")]
    pub value: String,
}

impl StringURIExpression {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
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
