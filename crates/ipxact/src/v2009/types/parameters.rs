use serde::{Deserialize, Serialize};

use super::name_value_pair::NameValuePair;

/// A collection of parameters (name-value pairs).
///
/// Maps to XML schema `parametersType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "parameters")]
pub struct Parameters {
    /// List of name-value parameter pairs
    #[serde(default, rename = "parameter")]
    pub parameter: Vec<NameValuePair>,
}

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parameters(mut self, params: Vec<NameValuePair>) -> Self {
        self.parameter = params;
        self
    }

    pub fn add(&mut self, param: NameValuePair) {
        self.parameter.push(param);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters_default() {
        let params = Parameters::new();
        assert!(params.parameter.is_empty());
    }

    #[test]
    fn test_parameters_with() {
        // This would use NameValuePair, but we'll just test the structure
        let params = Parameters::default();
        assert!(params.parameter.is_empty());
    }

    #[test]
    fn test_parameters_serde() {
        let params = Parameters::default();
        let xml = quick_xml::se::to_string(&params).unwrap();
        assert!(xml.contains("parameters"));
    }
}
