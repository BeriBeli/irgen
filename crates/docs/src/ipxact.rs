use crate::error::Error;
use crate::model::{Block, Component, Field, FieldOptions, Register, RegisterFile};
use irgen_ipxact_model as ipxact_model;

pub fn parse_ipxact(xml: &str) -> Result<Component, Error> {
    Ok(component_from_ipxact_model(
        &irgen_ipxact_parser::parse_ipxact(xml)?,
    ))
}

pub fn component_from_ipxact_model(component: &ipxact_model::Component) -> Component {
    Component::new(
        component.vendor.clone(),
        component.library.clone(),
        component.name.clone(),
        component.version.clone(),
        component
            .blocks
            .iter()
            .map(block_from_ipxact_model)
            .collect(),
    )
}

fn block_from_ipxact_model(block: &ipxact_model::AddressBlock) -> Block {
    Block::new_with_register_files(
        block.name.clone(),
        block.base_address.clone(),
        block.range.clone(),
        block.width.clone(),
        block
            .registers
            .iter()
            .map(register_from_ipxact_model)
            .collect(),
        block
            .register_files
            .iter()
            .map(register_file_from_ipxact_model)
            .collect(),
    )
}

fn register_file_from_ipxact_model(register_file: &ipxact_model::RegisterFile) -> RegisterFile {
    RegisterFile::new(
        register_file.name.clone(),
        register_file.address_offset.clone(),
        register_file.range.clone(),
        register_file.dim.clone(),
        register_file
            .registers
            .iter()
            .map(register_from_ipxact_model)
            .collect(),
    )
}

fn register_from_ipxact_model(register: &ipxact_model::Register) -> Register {
    Register::new_with_description(
        register.name.clone(),
        register.address_offset.clone(),
        register.size.clone(),
        register.description.clone(),
        register
            .fields
            .iter()
            .map(field_from_ipxact_model)
            .collect(),
    )
}

fn field_from_ipxact_model(field: &ipxact_model::Field) -> Field {
    Field::new_with_options(FieldOptions {
        name: field.name.clone(),
        offset: field.bit_offset.clone(),
        width: field.bit_width.clone(),
        attr: field.access.clone().unwrap_or_default(),
        reset: field.reset.clone().unwrap_or_default(),
        desc: field.description.clone(),
        hdl_path: field.hdl_path.clone(),
        testable: field.testable.as_deref().and_then(parse_bool_text),
        reserved: field
            .reserved
            .as_deref()
            .and_then(parse_bool_text)
            .unwrap_or(false),
    })
}

fn parse_bool_text(value: &str) -> Option<bool> {
    match value.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_ipxact;

    #[test]
    fn parses_register_and_field_descriptions_into_docs_model() {
        let component = parse_ipxact(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>example.com</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>demo</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>demo</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:description>Status register description</ipxact:description>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:description>Ready field description</ipxact:description>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:resets><ipxact:reset><ipxact:value>0</ipxact:value></ipxact:reset></ipxact:resets>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy><ipxact:access>read-only</ipxact:access></ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#,
        )
        .unwrap();

        let register = &component.blks()[0].regs()[0];
        let field = &register.fields()[0];
        assert_eq!(register.desc(), "Status register description");
        assert_eq!(field.desc(), "Ready field description");
        assert_eq!(field.attr(), "read-only");
        assert_eq!(field.reset(), "0");
    }

    #[test]
    fn rejects_non_2022_namespace_in_docs_ipxact_input() {
        let error = parse_ipxact(
            r#"<ipxact:component xmlns:ipxact="urn:unsupported-ipxact-namespace">
  <ipxact:vendor>example.com</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>demo</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps/>
</ipxact:component>"#,
        )
        .unwrap_err()
        .to_string();

        assert_eq!(
            error,
            "unsupported IP-XACT namespace `urn:unsupported-ipxact-namespace` (only IEEE 1685-2022 is supported)"
        );
    }
}
