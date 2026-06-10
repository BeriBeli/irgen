use irgen_snapsheet::model::{Block, Component, Field, Register, RegisterFile};
use quick_xml::events::Event;
use quick_xml::{Reader, Writer};

use crate::attr::{extract_access_value, extract_modified_write_value, extract_read_action_value};
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standard {
    Ieee1685_2022,
}

impl Standard {
    fn namespace(self) -> &'static str {
        match self {
            Self::Ieee1685_2022 => "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportOptions {
    pub standard: Standard,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            standard: Standard::Ieee1685_2022,
        }
    }
}

pub fn serialize(component: &Component) -> Result<String> {
    serialize_with_options(component, ExportOptions::default())
}

pub fn serialize_2022(component: &Component) -> Result<String> {
    serialize_with_options(
        component,
        ExportOptions {
            standard: Standard::Ieee1685_2022,
        },
    )
}

pub fn serialize_with_options(component: &Component, options: ExportOptions) -> Result<String> {
    serialize_component(component, options)
}

fn serialize_component(component: &Component, options: ExportOptions) -> Result<String> {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<ipxact:component xmlns:ipxact=""#);
    xml.push_str(options.standard.namespace());
    xml.push_str(r#"" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#);
    element(&mut xml, "vendor", component.vendor());
    element(&mut xml, "library", component.library());
    element(&mut xml, "name", component.name());
    element(&mut xml, "version", component.version());
    xml.push_str("<ipxact:memoryMaps><ipxact:memoryMap>");
    element(&mut xml, "name", component.name());
    for block in component.blks() {
        write_address_block(&mut xml, block, options)?;
    }
    xml.push_str("</ipxact:memoryMap></ipxact:memoryMaps></ipxact:component>");
    format_xml(&xml)
}

fn write_address_block(xml: &mut String, block: &Block, options: ExportOptions) -> Result<()> {
    xml.push_str("<ipxact:addressBlock>");
    element(xml, "name", block.name());
    element(xml, "baseAddress", block.offset());
    element(xml, "range", block.range());
    element(xml, "width", block.size());
    for register in block.regs() {
        write_register(xml, register, options, None, None)?;
    }
    for register_file in block.register_files() {
        write_register_file(xml, register_file, options)?;
    }

    xml.push_str("</ipxact:addressBlock>");
    Ok(())
}

fn write_register_file(
    xml: &mut String,
    register_file: &RegisterFile,
    options: ExportOptions,
) -> Result<()> {
    xml.push_str("<ipxact:registerFile>");
    element(xml, "name", register_file.name());
    xml.push_str("<ipxact:array>");
    element(xml, "dim", register_file.dim());
    xml.push_str("</ipxact:array>");
    element(xml, "addressOffset", register_file.offset());
    element(xml, "range", register_file.range());
    for register in register_file.regs() {
        write_register(xml, register, options, None, None)?;
    }
    xml.push_str("</ipxact:registerFile>");
    Ok(())
}

fn write_register(
    xml: &mut String,
    register: &Register,
    options: ExportOptions,
    name_override: Option<&str>,
    offset_and_dim_override: Option<(&str, Option<&str>)>,
) -> Result<()> {
    xml.push_str("<ipxact:register>");
    element(
        xml,
        "name",
        name_override.unwrap_or_else(|| register.name()),
    );
    optional_element(xml, "description", register.desc());
    if let Some(array) = register.array() {
        xml.push_str("<ipxact:array>");
        for dim in array.dims() {
            element(xml, "dim", dim);
        }
        if let Some(stride) = array.stride() {
            element(xml, "stride", stride);
        }
        xml.push_str("</ipxact:array>");
    }
    if let Some((_, Some(dim))) = offset_and_dim_override {
        element(xml, "dim", dim);
    }
    element(
        xml,
        "addressOffset",
        offset_and_dim_override
            .map(|(offset, _)| offset)
            .unwrap_or_else(|| register.offset()),
    );
    element(xml, "size", register.size());
    for field in register.fields() {
        write_field(xml, field, options)?;
    }
    xml.push_str("</ipxact:register>");
    Ok(())
}

fn write_field(xml: &mut String, field: &Field, options: ExportOptions) -> Result<()> {
    xml.push_str("<ipxact:field>");
    element(xml, "name", field.name());
    optional_element(xml, "description", field.desc());
    if should_emit_hdl_path(field)
        && let Some(path) = field.hdl_path()
    {
        write_sliced_access_handles(xml, path, options.standard);
    }
    element(xml, "bitOffset", field.offset());
    element(xml, "bitWidth", field.width());
    write_resets(xml, field.reset());
    xml.push_str("<ipxact:fieldAccessPolicies><ipxact:fieldAccessPolicy>");
    element(xml, "access", &access_value(field)?);
    optional_owned_element(xml, "modifiedWriteValue", modified_write_value(field)?);
    optional_owned_element(xml, "readAction", read_action_value(field)?);
    if let Some(testable) = field.testable() {
        element(xml, "testable", if testable { "true" } else { "false" });
    }
    if field.reserved() || is_reserved_field_name(field.name()) {
        element(xml, "reserved", "true");
    }
    xml.push_str("</ipxact:fieldAccessPolicy></ipxact:fieldAccessPolicies>");

    xml.push_str("</ipxact:field>");
    Ok(())
}

fn write_resets(xml: &mut String, reset: &str) {
    xml.push_str("<ipxact:resets><ipxact:reset>");
    if reset.trim().is_empty() || reset.trim() == "-" {
        element(xml, "value", "0");
        element(xml, "mask", "0");
    } else {
        element(xml, "value", reset);
    }
    xml.push_str("</ipxact:reset></ipxact:resets>");
}

fn write_sliced_access_handles(xml: &mut String, path: &str, standard: Standard) {
    match standard {
        Standard::Ieee1685_2022 => {
            xml.push_str("<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments>");
            element(xml, "pathSegment", path);
            xml.push_str("</ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>");
        }
    }
}

fn element(xml: &mut String, name: &str, value: &str) {
    xml.push_str("<ipxact:");
    xml.push_str(name);
    xml.push('>');
    escape_text(xml, value);
    xml.push_str("</ipxact:");
    xml.push_str(name);
    xml.push('>');
}

fn optional_element(xml: &mut String, name: &str, value: &str) {
    if !value.is_empty() {
        element(xml, name, value);
    }
}

fn optional_owned_element(xml: &mut String, name: &str, value: Option<String>) {
    if let Some(value) = value {
        element(xml, name, &value);
    }
}

fn escape_text(xml: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => xml.push_str("&amp;"),
            '<' => xml.push_str("&lt;"),
            '>' => xml.push_str("&gt;"),
            '"' => xml.push_str("&quot;"),
            '\'' => xml.push_str("&apos;"),
            _ => xml.push(ch),
        }
    }
}

fn format_xml(xml: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    loop {
        match reader.read_event()? {
            Event::Eof => break,
            event => writer.write_event(event)?,
        }
    }

    let mut formatted = String::from_utf8(writer.into_inner())
        .map_err(|error| Error::Serialize(error.to_string()))?;
    formatted.push('\n');
    Ok(formatted)
}

fn access_value(field: &Field) -> Result<String> {
    let value = extract_access_value(field.attr())?;
    validate_access(&value)?;
    Ok(value)
}

fn modified_write_value(field: &Field) -> Result<Option<String>> {
    extract_modified_write_value(field.attr())?
        .map(|value| validate_modified_write_value(&value).map(|_| value))
        .transpose()
}

fn read_action_value(field: &Field) -> Result<Option<String>> {
    extract_read_action_value(field.attr())?
        .map(|value| validate_read_action(&value).map(|_| value))
        .transpose()
}

fn validate_access(value: &str) -> Result<()> {
    match value {
        "read-only" | "write-only" | "read-write" | "writeOnce" | "read-writeOnce" => Ok(()),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn validate_modified_write_value(value: &str) -> Result<()> {
    match value {
        "oneToClear" | "oneToSet" | "oneToToggle" | "zeroToClear" | "zeroToSet"
        | "zeroToToggle" | "clear" | "set" | "modify" => Ok(()),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn validate_read_action(value: &str) -> Result<()> {
    match value {
        "clear" | "set" | "modify" => Ok(()),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn should_emit_hdl_path(field: &Field) -> bool {
    !is_reserved_field_name(field.name()) && field.hdl_path().is_some()
}

fn is_reserved_field_name(field_name: &str) -> bool {
    let lower = field_name.to_ascii_lowercase();
    let suffix = lower
        .strip_prefix("reserved")
        .or_else(|| lower.strip_prefix("rsvd"));

    suffix.is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}
