use serde::{Deserialize, Serialize};

/// File sets container.
///
/// Maps to XML schema anonymous `fileSets` element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSets {
    /// List of file sets.
    #[serde(default, rename = "fileSet")]
    pub file_set: Vec<FileSet>,
}

/// FileSet type - groups files together.
///
/// Maps to XML schema `fileSetType` complex type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSet {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Group identifier
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Files
    #[serde(default, rename = "file")]
    pub file: Vec<File>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl FileSet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            group: None,
            file: Vec::new(),
            vendor_extensions: None,
            id: None,
        }
    }
}

impl FileSets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, file_set: FileSet) {
        self.file_set.push(file_set);
    }
}

/// File type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// File type
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<Vec<String>>,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Choices type - defines enumerated choices for configurable elements.
///
/// Maps to XML schema `choicesType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Choices {
    /// List of choices
    #[serde(default, rename = "choice")]
    pub choice: Vec<Choice>,
}

/// Single choice definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Enumerated values
    #[serde(rename = "enumeratedValues", skip_serializing_if = "Option::is_none")]
    pub enumerated_values: Option<super::register::EnumeratedValues>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,

    /// ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Generator type - defines a generator configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Generator {
    /// Name (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Display name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Generator type
    #[serde(rename = "generatorType", skip_serializing_if = "Option::is_none")]
    pub generator_type: Option<GeneratorTypeRef>,

    /// Parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<super::Parameters>,

    /// Vendor extensions
    #[serde(rename = "vendorExtensions", skip_serializing_if = "Option::is_none")]
    pub vendor_extensions: Option<super::VendorExtensions>,
}

/// Generator type reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorTypeRef {
    /// Vendor
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    /// Library
    #[serde(rename = "library")]
    pub library: String,

    /// Name
    #[serde(rename = "name")]
    pub name: String,

    /// Version
    #[serde(rename = "version")]
    pub version: String,
}

/// Component generators container
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentGenerators {
    /// List of generators
    #[serde(default, rename = "generator")]
    pub generator: Vec<Generator>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_set_new() {
        let fs = FileSet::new("src_files".to_string());
        assert_eq!(fs.name, "src_files");
    }

    #[test]
    fn test_file() {
        let file = File {
            name: "main.rs".to_string(),
            file_type: Some(vec!["RustSource".to_string()]),
            display_name: None,
            description: None,
            vendor_extensions: None,
            id: None,
        };
        assert_eq!(file.name, "main.rs");
    }

    #[test]
    fn test_choices() {
        let mut choices = Choices::default();
        let choice = Choice {
            name: "mode".to_string(),
            display_name: Some("Mode".to_string()),
            description: None,
            enumerated_values: None,
            vendor_extensions: None,
            id: None,
        };
        choices.choice.push(choice);
        assert_eq!(choices.choice.len(), 1);
    }
}
