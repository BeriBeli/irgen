use irgen_model::attr::{
    extract_access_value, extract_modified_write_value, extract_read_action_value,
};
use irgen_model::base::{Block, Component, Field, Register, RegisterFile};
use quick_xml::events::Event;
use quick_xml::{Reader, Writer};

use crate::{Error, Result};

const NS_1_4: &str = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.4";
const NS_1_5: &str = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.5";
const NS_2009: &str = "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009";
const NS_2014: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2014";
const NS_2022: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2022";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Version {
    V1_4,
    V1_5,
    V2009,
    V2014,
    V2022,
}

pub fn serialize_1_4(component: &Component) -> Result<String> {
    serialize_component(component, Version::V1_4)
}

pub fn serialize_1_5(component: &Component) -> Result<String> {
    serialize_component(component, Version::V1_5)
}

pub fn serialize_2009(component: &Component) -> Result<String> {
    serialize_component(component, Version::V2009)
}

pub fn serialize_2014(component: &Component) -> Result<String> {
    serialize_component(component, Version::V2014)
}

pub fn serialize_2022(component: &Component) -> Result<String> {
    serialize_component(component, Version::V2022)
}

fn serialize_component(component: &Component, version: Version) -> Result<String> {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(r#"<ipxact:component xmlns:ipxact=""#);
    xml.push_str(namespace(version));
    xml.push_str(r#"" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#);
    element(&mut xml, "vendor", component.vendor());
    element(&mut xml, "library", component.library());
    element(&mut xml, "name", component.name());
    element(&mut xml, "version", component.version());
    xml.push_str("<ipxact:memoryMaps><ipxact:memoryMap>");
    element(&mut xml, "name", component.name());
    for block in component.blks() {
        write_address_block(&mut xml, block, version)?;
    }
    xml.push_str("</ipxact:memoryMap></ipxact:memoryMaps></ipxact:component>");
    if version.uses_spirit_prefix() {
        xml = xml
            .replace("xmlns:ipxact=", "xmlns:spirit=")
            .replace("ipxact:", "spirit:");
    }
    format_xml(&xml)
}

fn write_address_block(xml: &mut String, block: &Block, version: Version) -> Result<()> {
    xml.push_str("<ipxact:addressBlock>");
    element(xml, "name", block.name());
    element(xml, "baseAddress", block.offset());
    element(xml, "range", block.range());
    element(xml, "width", block.size());
    for register in block.regs() {
        write_register(xml, register, version, Some(block.name()), None, None)?;
    }
    for register_file in block.register_files() {
        write_register_file(xml, register_file, version, block)?;
    }

    xml.push_str("</ipxact:addressBlock>");
    Ok(())
}

fn write_register_file(
    xml: &mut String,
    register_file: &RegisterFile,
    version: Version,
    block: &Block,
) -> Result<()> {
    if version == Version::V1_4 {
        for register in register_file.regs() {
            let name = format!("{}_{}", register_file.name(), register.name());
            let offset = add_address_offsets(register_file.offset(), register.offset());
            write_register(
                xml,
                register,
                version,
                Some(block.name()),
                Some(&name),
                Some((&offset, Some(register_file.dim()))),
            )?;
        }
        return Ok(());
    }

    xml.push_str("<ipxact:registerFile>");
    element(xml, "name", register_file.name());
    write_block_access_handles(xml, Some(block.name()), version);
    match version {
        Version::V2022 => {
            xml.push_str("<ipxact:array>");
            element(xml, "dim", register_file.dim());
            xml.push_str("</ipxact:array>");
        }
        Version::V1_4 | Version::V1_5 | Version::V2009 | Version::V2014 => {
            element(xml, "dim", register_file.dim())
        }
    }
    element(xml, "addressOffset", register_file.offset());
    element(xml, "range", register_file.range());
    for register in register_file.regs() {
        write_register(xml, register, version, Some(block.name()), None, None)?;
    }
    xml.push_str("</ipxact:registerFile>");
    Ok(())
}

fn write_register(
    xml: &mut String,
    register: &Register,
    version: Version,
    block_name: Option<&str>,
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
    write_block_access_handles(xml, block_name, version);
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
        write_field(xml, field, version)?;
    }
    xml.push_str("</ipxact:register>");
    Ok(())
}

fn write_field(xml: &mut String, field: &Field, version: Version) -> Result<()> {
    xml.push_str("<ipxact:field>");
    element(xml, "name", field.name());
    optional_element(xml, "description", field.desc());
    match version {
        Version::V1_4 | Version::V1_5 | Version::V2009 => {}
        Version::V2014 => {
            if should_emit_hdl_path(field)
                && let Some(path) = field.hdl_path()
            {
                write_sliced_access_handles_2014(xml, path);
            }
        }
        Version::V2022 => {
            if should_emit_hdl_path(field)
                && let Some(path) = field.hdl_path()
            {
                write_sliced_access_handles_2022(xml, path);
            }
        }
    }
    element(xml, "bitOffset", field.offset());
    match version {
        Version::V1_4 | Version::V1_5 | Version::V2009 => element(xml, "bitWidth", field.width()),
        Version::V2014 => {
            write_resets(xml, field.reset(), version);
            element(xml, "bitWidth", field.width());
        }
        Version::V2022 => {
            element(xml, "bitWidth", field.width());
            write_resets(xml, field.reset(), version);
        }
    }

    match version {
        Version::V1_4 => {
            element(xml, "access", &access_value(field)?);
        }
        Version::V1_5 | Version::V2009 | Version::V2014 => {
            element(xml, "access", &access_value(field)?);
            optional_owned_element(xml, "modifiedWriteValue", modified_write_value(field)?);
            optional_owned_element(xml, "readAction", read_action_value(field)?);
        }
        Version::V2022 => {
            xml.push_str("<ipxact:fieldAccessPolicies><ipxact:fieldAccessPolicy>");
            element(xml, "access", &access_value(field)?);
            optional_owned_element(xml, "modifiedWriteValue", modified_write_value(field)?);
            optional_owned_element(xml, "readAction", read_action_value(field)?);
            xml.push_str("</ipxact:fieldAccessPolicy></ipxact:fieldAccessPolicies>");
        }
    }

    xml.push_str("</ipxact:field>");
    Ok(())
}

fn write_resets(xml: &mut String, reset: &str, version: Version) {
    if reset.is_empty() && version == Version::V2022 {
        return;
    }
    xml.push_str("<ipxact:resets><ipxact:reset>");
    element(xml, "value", reset);
    xml.push_str("</ipxact:reset></ipxact:resets>");
}

fn write_block_access_handles(xml: &mut String, block_name: Option<&str>, version: Version) {
    let Some(block_name) = block_name else {
        return;
    };
    let path = block_hdl_path_macro(block_name);
    match version {
        Version::V1_4 | Version::V1_5 | Version::V2009 => {}
        Version::V2014 => write_simple_access_handles_2014(xml, &path),
        Version::V2022 => write_simple_access_handles_2022(xml, &path),
    }
}

fn write_sliced_access_handles_2014(xml: &mut String, path: &str) {
    xml.push_str("<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>");
    element(xml, "pathSegmentName", path);
    xml.push_str("</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>");
}

fn write_sliced_access_handles_2022(xml: &mut String, path: &str) {
    xml.push_str("<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments>");
    element(xml, "pathSegment", path);
    xml.push_str("</ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>");
}

fn write_simple_access_handles_2014(xml: &mut String, path: &str) {
    xml.push_str(
        "<ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>",
    );
    element(xml, "pathSegmentName", path);
    xml.push_str(
        "</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>",
    );
}

fn write_simple_access_handles_2022(xml: &mut String, path: &str) {
    xml.push_str("<ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments>");
    element(xml, "pathSegment", path);
    xml.push_str("</ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>");
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

fn namespace(version: Version) -> &'static str {
    match version {
        Version::V1_4 => NS_1_4,
        Version::V1_5 => NS_1_5,
        Version::V2009 => NS_2009,
        Version::V2014 => NS_2014,
        Version::V2022 => NS_2022,
    }
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

fn block_hdl_path_macro(block_name: &str) -> String {
    let mut macro_name = String::from("`");
    for ch in block_name.chars() {
        if ch.is_ascii_alphanumeric() {
            macro_name.push(ch.to_ascii_uppercase());
        } else {
            macro_name.push('_');
        }
    }
    macro_name.push_str("_HDL_PATH");
    macro_name
}

impl Version {
    fn uses_spirit_prefix(self) -> bool {
        matches!(self, Self::V1_4 | Self::V1_5 | Self::V2009)
    }
}

fn add_address_offsets(base: &str, offset: &str) -> String {
    let Some(base) = parse_integer(base) else {
        return offset.to_owned();
    };
    let Some(offset) = parse_integer(offset) else {
        return format!("{base}+{offset}");
    };
    format!("0x{:x}", base + offset)
}

fn parse_integer(value: &str) -> Option<u64> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        value.parse().ok()
    }
}
