use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

use crate::error::Error;
use crate::model::{Block, Component, Field, FieldOptions, Register, RegisterFile};

pub fn parse_ipxact(xml: &str) -> Result<Component, Error> {
    let root = parse_xml(xml)?;
    Ok(Component::new(
        root.child_text("vendor")?,
        root.child_text("library")?,
        root.child_text("name")?,
        root.child_text("version")?,
        parse_blocks(&root)?,
    ))
}

fn parse_blocks(root: &XmlNode) -> Result<Vec<Block>, Error> {
    let mut blocks = Vec::new();
    let memory_maps = root
        .child("memoryMaps")
        .ok_or(Error::MissingElement("memoryMaps"))?;
    for memory_map in memory_maps.children_named("memoryMap") {
        for block in memory_map.children_named("addressBlock") {
            blocks.push(parse_block(block)?);
        }
    }
    Ok(blocks)
}

fn parse_block(node: &XmlNode) -> Result<Block, Error> {
    let mut registers = Vec::new();
    let mut register_files = Vec::new();
    for child in &node.children {
        match child.name.as_str() {
            "register" => registers.push(parse_register(child)?),
            "registerFile" => register_files.push(parse_register_file(child)?),
            _ => {}
        }
    }

    Ok(Block::new_with_register_files(
        node.child_text("name")?,
        node.child_text("baseAddress")?,
        node.child_text("range")?,
        node.child_text("width")?,
        registers,
        register_files,
    ))
}

fn parse_register_file(node: &XmlNode) -> Result<RegisterFile, Error> {
    Ok(RegisterFile::new(
        node.child_text("name")?,
        node.child_text("addressOffset")?,
        register_file_stride(node)?,
        register_file_dim(node),
        node.children_named("register")
            .map(parse_register)
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn register_file_dim(node: &XmlNode) -> String {
    node.child("array")
        .and_then(|array| array.optional_child_text("dim"))
        .or_else(|| node.optional_child_text("dim"))
        .unwrap_or_else(|| "1".into())
}

fn register_file_stride(node: &XmlNode) -> Result<String, Error> {
    if let Some(stride) = node
        .child("array")
        .and_then(|array| array.optional_child_text("stride"))
    {
        return Ok(stride);
    }
    node.child_text("range")
}

fn parse_register(node: &XmlNode) -> Result<Register, Error> {
    Ok(Register::new_with_description(
        node.child_text("name")?,
        node.child_text("addressOffset")?,
        node.child_text("size")?,
        node.optional_child_text("description").unwrap_or_default(),
        node.children_named("field")
            .map(parse_field)
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn parse_field(node: &XmlNode) -> Result<Field, Error> {
    Ok(Field::new_with_options(FieldOptions {
        name: node.child_text("name")?,
        offset: node.child_text("bitOffset")?,
        width: node.child_text("bitWidth")?,
        attr: field_access(node).unwrap_or_default(),
        reset: field_reset(node).unwrap_or_default(),
        desc: node.optional_child_text("description").unwrap_or_default(),
        hdl_path: None,
        testable: field_policy_text(node, "testable").and_then(|value| parse_bool(&value)),
        reserved: field_policy_text(node, "reserved")
            .and_then(|value| parse_bool(&value))
            .unwrap_or(false),
    }))
}

fn field_access(node: &XmlNode) -> Option<String> {
    field_policy_text(node, "access").or_else(|| node.optional_child_text("access"))
}

fn field_policy_text(node: &XmlNode, name: &str) -> Option<String> {
    node.child("fieldAccessPolicies")
        .and_then(|policies| policies.child("fieldAccessPolicy"))
        .and_then(|policy| policy.optional_child_text(name))
}

fn field_reset(node: &XmlNode) -> Option<String> {
    node.child("resets")
        .and_then(|resets| resets.child("reset"))
        .and_then(|reset| reset.optional_child_text("value"))
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

#[derive(Debug)]
struct XmlNode {
    name: String,
    text: String,
    children: Vec<XmlNode>,
}

impl XmlNode {
    fn new(name: String) -> Self {
        Self {
            name,
            text: String::new(),
            children: Vec::new(),
        }
    }

    fn child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|child| child.name == name)
    }

    fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a XmlNode> {
        self.children.iter().filter(move |child| child.name == name)
    }

    fn child_text(&self, name: &'static str) -> Result<String, Error> {
        self.child(name)
            .map(|child| child.text.trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or(Error::MissingElement(name))
    }

    fn optional_child_text(&self, name: &str) -> Option<String> {
        self.child(name)
            .map(|child| child.text.trim().to_string())
            .filter(|value| !value.is_empty())
    }
}

fn parse_xml(xml: &str) -> Result<XmlNode, Error> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut stack = Vec::new();
    let mut root = None;

    loop {
        match reader.read_event()? {
            Event::Start(event) => stack.push(xml_node(&event)),
            Event::Empty(event) => {
                let node = xml_node(&event);
                push_node(&mut stack, &mut root, node);
            }
            Event::Text(event) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&event.decode()?);
                }
            }
            Event::CData(event) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&event.decode()?);
                }
            }
            Event::GeneralRef(event) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&xml_general_ref_text(&event.decode()?));
                }
            }
            Event::End(event) => {
                let node = stack.pop().ok_or_else(|| {
                    Error::UnexpectedEnd(local_name_from_bytes(event.name().as_ref()))
                })?;
                push_node(&mut stack, &mut root, node);
            }
            Event::Eof => break,
            Event::Decl(_) | Event::PI(_) | Event::DocType(_) | Event::Comment(_) => {}
        }
    }

    root.ok_or(Error::MissingElement("component"))
}

fn push_node(stack: &mut [XmlNode], root: &mut Option<XmlNode>, node: XmlNode) {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
    } else {
        *root = Some(node);
    }
}

fn xml_node(event: &BytesStart<'_>) -> XmlNode {
    XmlNode::new(local_name_from_bytes(event.name().as_ref()))
}

fn local_name_from_bytes(value: &[u8]) -> String {
    let local = value
        .iter()
        .rposition(|byte| *byte == b':')
        .map(|index| &value[index + 1..])
        .unwrap_or(value);
    String::from_utf8_lossy(local).into_owned()
}

fn xml_general_ref_text(reference: &str) -> String {
    match reference {
        "amp" => "&".into(),
        "lt" => "<".into(),
        "gt" => ">".into(),
        "quot" => "\"".into(),
        "apos" => "'".into(),
        reference
            if reference
                .strip_prefix("#x")
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .and_then(char::from_u32)
                .is_some() =>
        {
            reference
                .strip_prefix("#x")
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .and_then(char::from_u32)
                .unwrap()
                .to_string()
        }
        reference
            if reference
                .strip_prefix('#')
                .and_then(|decimal| decimal.parse::<u32>().ok())
                .and_then(char::from_u32)
                .is_some() =>
        {
            reference
                .strip_prefix('#')
                .and_then(|decimal| decimal.parse::<u32>().ok())
                .and_then(char::from_u32)
                .unwrap()
                .to_string()
        }
        _ => format!("&{reference};"),
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
}
