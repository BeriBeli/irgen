use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::Error;
use crate::number::parse_literal;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SnapsheetConfig {
    pub workbook: WorkbookConfig,
    pub columns: ColumnsConfig,
    pub register: RegisterConfig,
    pub validation: ValidationConfig,
    pub reserved: ReservedConfig,
}

impl SnapsheetConfig {
    pub fn from_toml_file(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(|source| Error::ReadConfig {
            path: path.into(),
            source,
        })?;
        let config = toml::from_str::<Self>(&content).map_err(|source| Error::ParseConfig {
            path: path.into(),
            source,
        })?;
        config.validate(path)?;
        Ok(config)
    }

    fn validate(&self, path: &Path) -> Result<(), Error> {
        if self.register.max_array_elements == 0 {
            return Err(Error::Config {
                path: path.into(),
                message: "register.max_array_elements must be greater than zero".into(),
            });
        }
        let default_array_step_bytes =
            self.register
                .parse_default_array_step_bytes()
                .map_err(|message| Error::Config {
                    path: path.into(),
                    message: format!("invalid register.default_array_step_bytes: {message}"),
                })?;
        if default_array_step_bytes == 0 {
            return Err(Error::Config {
                path: path.into(),
                message: "register.default_array_step_bytes must be greater than zero".into(),
            });
        }
        if self.workbook.sheets.register_sheet != "block_name" {
            return Err(Error::Config {
                path: path.into(),
                message: "workbook.sheets.register_sheet currently supports only `block_name`"
                    .into(),
            });
        }
        if self.register.register_size != "infer_from_fields" {
            return Err(Error::Config {
                path: path.into(),
                message: "register.register_size currently supports only `infer_from_fields`"
                    .into(),
            });
        }
        if !matches!(
            self.register.blank_field_name.as_str(),
            "register_name" | "required"
        ) {
            return Err(Error::Config {
                path: path.into(),
                message: "register.blank_field_name must be `register_name` or `required`".into(),
            });
        }
        self.register.array.validate(path)?;
        self.reserved.validate(path)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorkbookConfig {
    pub sheets: WorkbookSheets,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorkbookSheets {
    pub version: String,
    pub address_map: String,
    pub register_sheet: String,
}

impl Default for WorkbookSheets {
    fn default() -> Self {
        Self {
            version: "version".into(),
            address_map: "address_map".into(),
            register_sheet: "block_name".into(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ColumnsConfig {
    pub version: VersionColumns,
    pub address_block: AddressBlockColumns,
    pub register: RegisterColumns,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct VersionColumns {
    pub vendor: String,
    pub library: String,
    pub name: String,
    pub version: String,
}

impl VersionColumns {
    pub(crate) fn required(&self) -> [&str; 4] {
        [
            self.vendor.as_str(),
            self.library.as_str(),
            self.name.as_str(),
            self.version.as_str(),
        ]
    }
}

impl Default for VersionColumns {
    fn default() -> Self {
        Self {
            vendor: "VENDOR".into(),
            library: "LIBRARY".into(),
            name: "NAME".into(),
            version: "VERSION".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AddressBlockColumns {
    pub name: String,
    pub offset: String,
    pub range: String,
}

impl AddressBlockColumns {
    pub(crate) fn required(&self) -> [&str; 3] {
        [
            self.name.as_str(),
            self.offset.as_str(),
            self.range.as_str(),
        ]
    }
}

impl Default for AddressBlockColumns {
    fn default() -> Self {
        Self {
            name: "BLOCK".into(),
            offset: "OFFSET".into(),
            range: "RANGE".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RegisterColumns {
    pub address: String,
    pub register: String,
    pub field: String,
    pub bit: String,
    pub width: String,
    pub access: String,
    pub reset: String,
    pub description: String,
}

impl RegisterColumns {
    pub(crate) fn required(&self) -> [&str; 8] {
        [
            self.address.as_str(),
            self.register.as_str(),
            self.field.as_str(),
            self.bit.as_str(),
            self.width.as_str(),
            self.access.as_str(),
            self.reset.as_str(),
            self.description.as_str(),
        ]
    }
}

impl Default for RegisterColumns {
    fn default() -> Self {
        Self {
            address: "ADDR".into(),
            register: "REG".into(),
            field: "FIELD".into(),
            bit: "BIT".into(),
            width: "WIDTH".into(),
            access: "ATTRIBUTE".into(),
            reset: "DEFAULT".into(),
            description: "DESCRIPTION".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RegisterConfig {
    pub inherit_address: bool,
    pub inherit_register: bool,
    pub default_description: String,
    pub default_array_step_bytes: String,
    pub max_array_elements: usize,
    pub register_size: String,
    pub require_byte_aligned: bool,
    pub blank_field_name: String,
    pub array: ArrayConfig,
}

impl RegisterConfig {
    pub(crate) fn parse_default_array_step_bytes(&self) -> Result<u64, String> {
        parse_literal(&self.default_array_step_bytes)
    }

    pub(crate) fn blank_field_name_uses_register(&self) -> bool {
        self.blank_field_name == "register_name"
    }
}

impl Default for RegisterConfig {
    fn default() -> Self {
        Self {
            inherit_address: false,
            inherit_register: false,
            default_description: "No Description".into(),
            default_array_step_bytes: "0x4".into(),
            max_array_elements: 1_000_000,
            register_size: "infer_from_fields".into(),
            require_byte_aligned: true,
            blank_field_name: "required".into(),
            array: ArrayConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ArrayConfig {
    pub enabled: bool,
    pub syntax: String,
    pub pattern: String,
}

impl ArrayConfig {
    fn validate(&self, path: &Path) -> Result<(), Error> {
        if !self.enabled {
            return Ok(());
        }
        if self.syntax != "range" {
            return Err(Error::Config {
                path: path.into(),
                message: "register.array.syntax currently supports only `range`".into(),
            });
        }
        if self.pattern != "{name}{n}, n=range({start?}, {end}, {step?})" {
            return Err(Error::Config {
                path: path.into(),
                message:
                    "register.array.pattern currently supports only `{name}{n}, n=range({start?}, {end}, {step?})`"
                        .into(),
            });
        }
        Ok(())
    }
}

impl Default for ArrayConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            syntax: "range".into(),
            pattern: "{name}{n}, n=range({start?}, {end}, {step?})".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ValidationConfig {
    pub reject_duplicate_blocks: bool,
    pub reject_overlapping_blocks: bool,
    pub reject_duplicate_registers: bool,
    pub reject_overlapping_registers: bool,
    pub reject_duplicate_fields: bool,
    pub reject_overlapping_fields: bool,
    pub check_bit_range_matches_width: bool,
    pub check_reset_fits_width: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            reject_duplicate_blocks: true,
            reject_overlapping_blocks: true,
            reject_duplicate_registers: true,
            reject_overlapping_registers: true,
            reject_duplicate_fields: true,
            reject_overlapping_fields: true,
            check_bit_range_matches_width: true,
            check_reset_fits_width: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReservedConfig {
    pub enabled: bool,
    pub patterns: Vec<String>,
}

impl ReservedConfig {
    fn validate(&self, path: &Path) -> Result<(), Error> {
        let defaults = Self::default().patterns;
        if self.enabled && self.patterns != defaults {
            return Err(Error::Config {
                path: path.into(),
                message:
                    "reserved.patterns currently supports only `^reserved[0-9]+$` and `^rsvd[0-9]+$`"
                        .into(),
            });
        }
        Ok(())
    }

    pub(crate) fn validate_field_name(&self, name: &str) -> bool {
        if !self.enabled {
            return true;
        }
        let lower = name.to_ascii_lowercase();
        if let Some(suffix) = lower.strip_prefix("reserved") {
            return !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit());
        }
        if let Some(suffix) = lower.strip_prefix("rsvd") {
            return !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit());
        }
        true
    }
}

impl Default for ReservedConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            patterns: vec!["^reserved[0-9]+$".into(), "^rsvd[0-9]+$".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn loads_toml_config_file() {
        let path = std::env::temp_dir().join(format!(
            "irgen-snapsheet-config-{}.toml",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"
[workbook.sheets]
version = "meta"
address_map = "map"
register_sheet = "block_name"

[columns.register]
address = "Address"
register = "Register"
field = "Field"
bit = "Bits"
width = "BitWidth"
access = "Access"
reset = "Reset"
description = "Desc"

[register]
default_array_step_bytes = "0x8"
default_description = "N/A"
register_size = "infer_from_fields"
blank_field_name = "register_name"

[register.array]
enabled = true
syntax = "range"
pattern = "{name}{n}, n=range({start?}, {end}, {step?})"

[validation]
check_reset_fits_width = false

[reserved]
enabled = true
patterns = ["^reserved[0-9]+$", "^rsvd[0-9]+$"]
"#,
        )
        .unwrap();

        let config = SnapsheetConfig::from_toml_file(&path).unwrap();

        assert_eq!(config.workbook.sheets.version, "meta");
        assert_eq!(config.workbook.sheets.address_map, "map");
        assert_eq!(config.workbook.sheets.register_sheet, "block_name");
        assert_eq!(config.columns.register.address, "Address");
        assert_eq!(config.register.parse_default_array_step_bytes(), Ok(0x8));
        assert_eq!(config.register.default_description, "N/A");
        assert!(config.register.blank_field_name_uses_register());
        assert_eq!(
            config.register.array.pattern,
            "{name}{n}, n=range({start?}, {end}, {step?})"
        );
        assert!(config.register.array.enabled);
        assert!(!config.validation.check_reset_fits_width);
        assert!(config.reserved.validate_field_name("reserved0"));
        assert!(!config.reserved.validate_field_name("reserved"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_invalid_default_array_step() {
        let path = std::env::temp_dir().join(format!(
            "irgen-snapsheet-invalid-config-{}.toml",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"
[register]
default_array_step_bytes = "0"
"#,
        )
        .unwrap();

        let error = SnapsheetConfig::from_toml_file(&path).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("default_array_step_bytes must be greater than zero")
        );

        fs::remove_file(path).unwrap();
    }
}
