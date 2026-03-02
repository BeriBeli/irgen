use serde::{Deserialize, Serialize};

/// A name-value pair used for parameters and configurable element values.
///
/// Maps to XML schema `nameValuePairType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameValuePair {
    /// Unique name identifier (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Optional display name for UI purposes
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Optional description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The value of the parameter
    #[serde(rename = "value")]
    pub value: ParameterValue,
}

/// The value element with attributes for formatting and constraints.
///
/// Maps to XML schema anonymous complex type within `nameValuePairType`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    /// The actual value content
    #[serde(rename = "$value")]
    pub value: Option<String>,

    /// Format hint for the value (string, bitString, bool, long, float)
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Minimum value for numeric types
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,

    /// Maximum value for numeric types
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    /// Bit string length (required if format is bitString)
    #[serde(rename = "bitStringLength", skip_serializing_if = "Option::is_none")]
    pub bit_string_length: Option<u64>,

    /// Resolution type (immediate, user, dependent)
    #[serde(rename = "resolve", skip_serializing_if = "Option::is_none")]
    pub resolve: Option<String>,

    /// Prompt for user-resolved values
    #[serde(rename = "prompt", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Choice reference for enumerated values
    #[serde(rename = "choiceRef", skip_serializing_if = "Option::is_none")]
    pub choice_ref: Option<String>,

    /// Order for display
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,

    /// Configurable groups
    #[serde(rename = "configGroups", skip_serializing_if = "Option::is_none")]
    pub config_groups: Option<String>,

    /// ID attribute
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Dependency expression for dependent resolution
    #[serde(rename = "dependency", skip_serializing_if = "Option::is_none")]
    pub dependency: Option<String>,
}

impl NameValuePair {
    pub fn new(name: String, value: ParameterValue) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            value,
        }
    }

    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl ParameterValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            format: None,
            minimum: None,
            maximum: None,
            bit_string_length: None,
            resolve: None,
            prompt: None,
            choice_ref: None,
            order: None,
            config_groups: None,
            id: None,
            dependency: None,
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_resolve(mut self, resolve: impl Into<String>) -> Self {
        self.resolve = Some(resolve.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_value_pair_new() {
        let nvp = NameValuePair::new(
            "param1".to_string(),
            ParameterValue::new("value1"),
        );
        assert_eq!(nvp.name, "param1");
        assert_eq!(nvp.value.value, Some("value1".to_string()));
    }

    #[test]
    fn test_name_value_pair_with_options() {
        let nvp = NameValuePair::new(
            "param1".to_string(),
            ParameterValue::new("value1"),
        )
        .with_display_name("Parameter 1".to_string())
        .with_description("A test parameter".to_string());

        assert_eq!(nvp.display_name, Some("Parameter 1".to_string()));
        assert_eq!(nvp.description, Some("A test parameter".to_string()));
    }

    #[test]
    fn test_parameter_value_with_options() {
        let pv = ParameterValue::new("42")
            .with_format("long")
            .with_resolve("immediate");

        assert_eq!(pv.value, Some("42".to_string()));
        assert_eq!(pv.format, Some("long".to_string()));
        assert_eq!(pv.resolve, Some("immediate".to_string()));
    }
}
