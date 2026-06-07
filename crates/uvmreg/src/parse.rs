use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

use crate::model::{
    AccessPolicy, AccessRestriction, AddressBlock, AddressSpace, AlternateRegister, Broadcast,
    Component, EnumeratedValue, Field, FieldReferenceSegment, MemoryRemap, ModeRef, Register,
    RegisterFile, Reset, SubspaceMap, Testable, WriteValueConstraint,
};
use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryRef {
    pub vendor: String,
    pub library: String,
    pub name: String,
    pub version: String,
}

impl LibraryRef {
    fn from_node(node: &XmlNode) -> Result<Self> {
        Ok(Self {
            vendor: node
                .attribute_text("vendor")
                .ok_or(Error::MissingElement("vendor"))?,
            library: node
                .attribute_text("library")
                .ok_or(Error::MissingElement("library"))?,
            name: node
                .attribute_text("name")
                .ok_or(Error::MissingElement("name"))?,
            version: node
                .attribute_text("version")
                .ok_or(Error::MissingElement("version"))?,
        })
    }

    pub fn key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.vendor, self.library, self.name, self.version
        )
    }
}

#[derive(Debug, Clone)]
struct XmlNode {
    name: String,
    text: String,
    attributes: Vec<(String, String)>,
    children: Vec<XmlNode>,
}

#[derive(Debug, Default)]
struct Definitions {
    memory_maps: HashMap<String, XmlNode>,
    memory_remaps: HashMap<String, XmlNode>,
    banks: HashMap<String, XmlNode>,
    address_blocks: HashMap<String, XmlNode>,
    registers: HashMap<String, XmlNode>,
    register_files: HashMap<String, XmlNode>,
    fields: HashMap<String, XmlNode>,
    enumerations: HashMap<String, XmlNode>,
    field_access_policies: HashMap<String, XmlNode>,
}

impl Definitions {
    fn from_root_and_external(root: &XmlNode, external_roots: &[(String, XmlNode)]) -> Self {
        let mut definitions = Definitions::default();
        for type_definitions in root.children_named("typeDefinitions") {
            definitions.collect_scoped(
                type_definitions,
                type_definitions.optional_child_text("name").as_deref(),
            );
        }
        if root.name == "typeDefinitions" {
            definitions.collect_scoped(root, root.optional_child_text("name").as_deref());
        }
        for (alias, external_root) in external_roots {
            definitions.collect_external(alias, external_root);
        }
        definitions.collect_scoped(root, None);
        definitions
    }

    fn collect_external(&mut self, alias: &str, root: &XmlNode) {
        if root.name == "typeDefinitions" {
            self.collect_scoped(root, Some(alias));
            self.collect_scoped(root, root.optional_child_text("name").as_deref());
            return;
        }

        for type_definitions in root.children_named("typeDefinitions") {
            self.collect_scoped(type_definitions, Some(alias));
            self.collect_scoped(
                type_definitions,
                type_definitions.optional_child_text("name").as_deref(),
            );
        }
    }

    fn collect_scoped(&mut self, node: &XmlNode, scope: Option<&str>) {
        if let Some(memory_map_definitions) = node.child("memoryMapDefinitions") {
            for definition in memory_map_definitions.children_named("memoryMapDefinition") {
                insert_definition(&mut self.memory_maps, scope, definition);
            }
        }
        if let Some(memory_remap_definitions) = node.child("memoryRemapDefinitions") {
            for definition in memory_remap_definitions.children_named("memoryRemapDefinition") {
                insert_definition(&mut self.memory_remaps, scope, definition);
            }
        }
        if let Some(bank_definitions) = node.child("bankDefinitions") {
            for definition in bank_definitions.children_named("bankDefinition") {
                insert_definition(&mut self.banks, scope, definition);
            }
        }
        if let Some(address_block_definitions) = node.child("addressBlockDefinitions") {
            for definition in address_block_definitions.children_named("addressBlockDefinition") {
                insert_definition(&mut self.address_blocks, scope, definition);
            }
        }
        if let Some(register_definitions) = node.child("registerDefinitions") {
            for definition in register_definitions.children_named("registerDefinition") {
                insert_definition(&mut self.registers, scope, definition);
            }
        }
        if let Some(register_file_definitions) = node.child("registerFileDefinitions") {
            for definition in register_file_definitions.children_named("registerFileDefinition") {
                insert_definition(&mut self.register_files, scope, definition);
            }
        }
        if let Some(field_definitions) = node.child("fieldDefinitions") {
            for definition in field_definitions.children_named("fieldDefinition") {
                insert_definition(&mut self.fields, scope, definition);
            }
        }
        if let Some(enumeration_definitions) = node.child("enumerationDefinitions") {
            for definition in enumeration_definitions.children_named("enumerationDefinition") {
                insert_definition(&mut self.enumerations, scope, definition);
            }
        }
        if let Some(field_access_policy_definitions) = node.child("fieldAccessPolicyDefinitions") {
            for definition in
                field_access_policy_definitions.children_named("fieldAccessPolicyDefinition")
            {
                insert_definition(&mut self.field_access_policies, scope, definition);
            }
        }
    }

    fn address_block_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.address_blocks, &reference))
    }

    fn memory_map_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.memory_maps, &reference))
    }

    fn memory_remap_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.memory_remaps, &reference))
    }

    fn bank_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name).and_then(|reference| lookup_definition(&self.banks, &reference))
    }

    fn register_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.registers, &reference))
    }

    fn register_file_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.register_files, &reference))
    }

    fn field_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name).and_then(|reference| lookup_definition(&self.fields, &reference))
    }

    fn enumeration_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.enumerations, &reference))
    }

    fn field_access_policy_ref(&self, node: &XmlNode, name: &str) -> Option<&XmlNode> {
        definition_ref(node, name)
            .and_then(|reference| lookup_definition(&self.field_access_policies, &reference))
    }
}

fn insert_definition(
    map: &mut HashMap<String, XmlNode>,
    scope: Option<&str>,
    definition: &XmlNode,
) {
    let Some(name) = definition.optional_child_text("name") else {
        return;
    };

    map.insert(name.clone(), definition.clone());
    if let Some(scope) = scope.filter(|scope| !scope.trim().is_empty()) {
        map.insert(definition_key(scope, &name), definition.clone());
    }
}

fn lookup_definition<'a>(
    map: &'a HashMap<String, XmlNode>,
    reference: &DefinitionReference,
) -> Option<&'a XmlNode> {
    reference
        .type_definitions
        .as_deref()
        .and_then(|scope| map.get(&definition_key(scope, &reference.name)))
        .or_else(|| map.get(&reference.name))
}

fn definition_key(scope: &str, name: &str) -> String {
    format!("{}::{}", scope.trim(), name.trim())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DefinitionReference {
    name: String,
    type_definitions: Option<String>,
}

impl XmlNode {
    fn new(name: String) -> Self {
        Self {
            name,
            text: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    fn child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|child| child.name == name)
    }

    fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a XmlNode> {
        self.children.iter().filter(move |child| child.name == name)
    }

    fn child_text(&self, name: &'static str) -> Result<String> {
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

    fn attribute_text(&self, name: &str) -> Option<String> {
        self.attributes
            .iter()
            .find_map(|(attr_name, value)| (attr_name == name).then(|| value.trim().to_string()))
            .filter(|value| !value.is_empty())
    }
}

pub fn parse_ipxact(xml: &str) -> Result<Component> {
    parse_ipxact_with_resolver(xml, |_| Ok(None))
}

pub fn parse_ipxact_with_resolver(
    xml: &str,
    mut resolver: impl FnMut(&LibraryRef) -> Result<Option<String>>,
) -> Result<Component> {
    let root = parse_xml(xml)?;
    if root.name != "component" {
        return Err(Error::UnsupportedRoot(root.name));
    }
    let mut external_roots = Vec::new();
    let mut resolved = HashMap::new();
    resolve_external_type_definitions(&root, &mut resolver, &mut resolved, &mut external_roots)?;
    let definitions = Definitions::from_root_and_external(&root, &external_roots);
    parse_component(&root, &definitions)
}

pub fn document_library_ref(xml: &str) -> Result<LibraryRef> {
    let root = parse_xml(xml)?;
    Ok(LibraryRef {
        vendor: root.child_text("vendor")?,
        library: root.child_text("library")?,
        name: root.child_text("name")?,
        version: root.child_text("version")?,
    })
}

fn parse_component(root: &XmlNode, definitions: &Definitions) -> Result<Component> {
    let initiator_address_spaces = initiator_address_spaces(root);
    let address_spaces = parse_address_spaces(root, definitions)?;
    let memory_maps = root.child("memoryMaps");
    let mut blocks = Vec::new();
    let mut subspace_maps = Vec::new();
    let mut memory_remaps = Vec::new();
    if let Some(memory_maps) = memory_maps {
        for memory_map in memory_maps.children_named("memoryMap") {
            let map_name = memory_map.child_text("name")?;
            let source = definitions
                .memory_map_ref(memory_map, "memoryMapDefinitionRef")
                .unwrap_or(memory_map);
            let address_unit_bits = memory_map
                .optional_child_text("addressUnitBits")
                .or_else(|| source.optional_child_text("addressUnitBits"))
                .unwrap_or_else(|| "8".into());
            for block in source.children_named("addressBlock") {
                blocks.push(parse_address_block(
                    block,
                    &map_name,
                    &address_unit_bits,
                    &definitions,
                )?);
            }
            for bank in source.children_named("bank") {
                blocks.extend(parse_bank(
                    bank,
                    0,
                    Vec::new(),
                    None,
                    &map_name,
                    &address_unit_bits,
                    &definitions,
                )?);
            }
            for subspace_map in source.children_named("subspaceMap") {
                subspace_maps.push(parse_subspace_map(
                    subspace_map,
                    &map_name,
                    &address_unit_bits,
                    &initiator_address_spaces,
                )?);
            }
            for memory_remap in source.children_named("memoryRemap") {
                memory_remaps.push(parse_memory_remap(
                    memory_remap,
                    &map_name,
                    &address_unit_bits,
                    &definitions,
                    &initiator_address_spaces,
                )?);
            }
        }
    }

    Ok(Component {
        vendor: root.child_text("vendor")?,
        library: root.child_text("library")?,
        name: root.child_text("name")?,
        version: root.child_text("version")?,
        address_spaces,
        blocks,
        subspace_maps,
        memory_remaps,
    })
}

fn parse_address_spaces(root: &XmlNode, definitions: &Definitions) -> Result<Vec<AddressSpace>> {
    let mut address_spaces = Vec::new();
    let Some(address_spaces_node) = root.child("addressSpaces") else {
        return Ok(address_spaces);
    };

    for address_space in address_spaces_node.children_named("addressSpace") {
        let name = address_space.child_text("name")?;
        let address_unit_bits = address_space
            .optional_child_text("addressUnitBits")
            .unwrap_or_else(|| "8".into());
        let mut blocks = Vec::new();

        if let Some(local_memory_map) = address_space.child("localMemoryMap") {
            for block in local_memory_map.children_named("addressBlock") {
                blocks.push(parse_address_block(
                    block,
                    &name,
                    &address_unit_bits,
                    definitions,
                )?);
            }
            for bank in local_memory_map.children_named("bank") {
                blocks.extend(parse_bank(
                    bank,
                    0,
                    Vec::new(),
                    None,
                    &name,
                    &address_unit_bits,
                    definitions,
                )?);
            }
        }

        address_spaces.push(AddressSpace {
            name,
            address_unit_bits,
            blocks,
        });
    }

    Ok(address_spaces)
}

fn initiator_address_spaces(root: &XmlNode) -> HashMap<String, String> {
    let mut refs = HashMap::new();
    let Some(bus_interfaces) = root.child("busInterfaces") else {
        return refs;
    };

    for bus_interface in bus_interfaces.children_named("busInterface") {
        let Some(name) = bus_interface.optional_child_text("name") else {
            continue;
        };
        let Some(address_space_ref) = bus_interface
            .child("initiator")
            .and_then(|initiator| initiator.child("addressSpaceRef"))
            .and_then(|address_space_ref| address_space_ref.attribute_text("addressSpaceRef"))
        else {
            continue;
        };
        refs.insert(name, address_space_ref);
    }

    refs
}

fn resolve_external_type_definitions(
    root: &XmlNode,
    resolver: &mut impl FnMut(&LibraryRef) -> Result<Option<String>>,
    resolved: &mut HashMap<String, XmlNode>,
    external_roots: &mut Vec<(String, XmlNode)>,
) -> Result<()> {
    for external in external_type_definitions(root) {
        let alias = external.child_text("name")?;
        let reference = LibraryRef::from_node(
            external
                .child("typeDefinitionsRef")
                .ok_or(Error::MissingElement("typeDefinitionsRef"))?,
        )?;
        let key = reference.key();
        let external_root = if let Some(root) = resolved.get(&key) {
            root.clone()
        } else {
            let Some(xml) = resolver(&reference)? else {
                return Err(Error::ExternalTypeDefinitionsNotFound(key));
            };
            let external_root = parse_xml(&xml)?;
            resolve_external_type_definitions(&external_root, resolver, resolved, external_roots)?;
            resolved.insert(key, external_root.clone());
            external_root
        };
        external_roots.push((alias, external_root));
    }
    Ok(())
}

fn external_type_definitions(root: &XmlNode) -> Vec<&XmlNode> {
    if root.name == "typeDefinitions" {
        return root.children_named("externalTypeDefinitions").collect();
    }

    root.children_named("typeDefinitions")
        .flat_map(|type_definitions| type_definitions.children_named("externalTypeDefinitions"))
        .collect()
}

fn parse_xml(xml: &str) -> Result<XmlNode> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut stack = Vec::new();
    let mut root = None;

    loop {
        match reader.read_event()? {
            Event::Start(event) => stack.push(xml_node(&event, &reader)?),
            Event::Empty(event) => {
                let node = xml_node(&event, &reader)?;
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
            Event::End(event) => {
                let node = stack.pop().ok_or_else(|| {
                    Error::UnexpectedEnd(local_name_from_bytes(event.name().as_ref()))
                })?;
                push_node(&mut stack, &mut root, node);
            }
            Event::Eof => break,
            Event::Decl(_)
            | Event::PI(_)
            | Event::DocType(_)
            | Event::Comment(_)
            | Event::GeneralRef(_) => {}
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

fn local_name(event: &BytesStart<'_>) -> String {
    local_name_from_bytes(event.name().as_ref())
}

fn xml_node(event: &BytesStart<'_>, reader: &Reader<&[u8]>) -> Result<XmlNode> {
    let mut node = XmlNode::new(local_name(event));
    for attribute in event.attributes() {
        let attribute = attribute?;
        let name = local_name_from_bytes(attribute.key.as_ref());
        let value = attribute.decode_and_unescape_value(reader.decoder())?;
        node.attributes.push((name, value.into_owned()));
    }
    Ok(node)
}

fn local_name_from_bytes(value: &[u8]) -> String {
    let local = value
        .iter()
        .rposition(|byte| *byte == b':')
        .map_or(value, |index| &value[index + 1..]);
    String::from_utf8_lossy(local).into_owned()
}

fn parse_address_block(
    node: &XmlNode,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<AddressBlock> {
    let base_address = node.child_text("baseAddress")?;
    let source = definitions
        .address_block_ref(node, "addressBlockDefinitionRef")
        .unwrap_or(node);
    parse_address_block_from(
        source,
        node,
        &node.child_text("name")?,
        base_address,
        None,
        map_name,
        address_unit_bits,
        definitions,
    )
}

fn parse_subspace_map(
    node: &XmlNode,
    map_name: &str,
    address_unit_bits: &str,
    initiator_address_spaces: &HashMap<String, String>,
) -> Result<SubspaceMap> {
    let initiator_ref = node
        .attribute_text("initiatorRef")
        .or_else(|| node.attribute_text("masterRef"))
        .ok_or(Error::MissingElement("initiatorRef"))?;
    let address_space_ref = initiator_address_spaces.get(&initiator_ref).cloned();

    Ok(SubspaceMap {
        name: node.child_text("name")?,
        map_name: map_name.into(),
        base_address: node.child_text("baseAddress")?,
        address_unit_bits: address_unit_bits.into(),
        initiator_ref,
        address_space_ref,
        segment_ref: node.attribute_text("segmentRef"),
    })
}

fn parse_memory_remap(
    node: &XmlNode,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
    initiator_address_spaces: &HashMap<String, String>,
) -> Result<MemoryRemap> {
    let name = node.child_text("name")?;
    let source = definitions
        .memory_remap_ref(node, "remapDefinitionRef")
        .unwrap_or(node);
    let remap_address_unit_bits = node
        .optional_child_text("addressUnitBits")
        .or_else(|| source.optional_child_text("addressUnitBits"))
        .unwrap_or_else(|| address_unit_bits.into());
    let mut blocks = Vec::new();
    let mut subspace_maps = Vec::new();

    for block in source.children_named("addressBlock") {
        blocks.push(parse_address_block_with_prefix(
            block,
            &name,
            map_name,
            &remap_address_unit_bits,
            definitions,
        )?);
    }
    for bank in source.children_named("bank") {
        blocks.extend(parse_bank(
            bank,
            0,
            vec![name.clone()],
            None,
            map_name,
            &remap_address_unit_bits,
            definitions,
        )?);
    }
    for subspace_map in source.children_named("subspaceMap") {
        subspace_maps.push(parse_subspace_map_with_prefix(
            subspace_map,
            &name,
            map_name,
            &remap_address_unit_bits,
            initiator_address_spaces,
        )?);
    }

    let mode_refs = node
        .children_named("modeRef")
        .map(parse_mode_ref)
        .collect::<Vec<_>>();
    let mode_refs = if mode_refs.is_empty() {
        source
            .children_named("modeRef")
            .map(parse_mode_ref)
            .collect()
    } else {
        mode_refs
    };

    Ok(MemoryRemap {
        name,
        map_name: map_name.into(),
        mode_refs,
        blocks,
        subspace_maps,
    })
}

fn parse_address_block_with_prefix(
    node: &XmlNode,
    prefix: &str,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<AddressBlock> {
    let name = format!("{}_{}", prefix, node.child_text("name")?);
    let base_address = node.child_text("baseAddress")?;
    let source = definitions
        .address_block_ref(node, "addressBlockDefinitionRef")
        .unwrap_or(node);
    parse_address_block_from(
        source,
        node,
        &name,
        base_address,
        None,
        map_name,
        address_unit_bits,
        definitions,
    )
}

fn parse_subspace_map_with_prefix(
    node: &XmlNode,
    prefix: &str,
    map_name: &str,
    address_unit_bits: &str,
    initiator_address_spaces: &HashMap<String, String>,
) -> Result<SubspaceMap> {
    let mut subspace_map =
        parse_subspace_map(node, map_name, address_unit_bits, initiator_address_spaces)?;
    subspace_map.name = format!("{}_{}", prefix, subspace_map.name);
    Ok(subspace_map)
}

fn parse_address_block_at(
    node: &XmlNode,
    name: &str,
    base_address: String,
    parent_hdl_path: Option<&String>,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<AddressBlock> {
    let source = definitions
        .address_block_ref(node, "addressBlockDefinitionRef")
        .unwrap_or(node);
    parse_address_block_from(
        source,
        node,
        name,
        base_address,
        parent_hdl_path,
        map_name,
        address_unit_bits,
        definitions,
    )
}

fn parse_address_block_from(
    source: &XmlNode,
    instance: &XmlNode,
    name: &str,
    base_address: String,
    parent_hdl_path: Option<&String>,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<AddressBlock> {
    let mut registers = Vec::new();
    let mut register_files = Vec::new();

    for child in &source.children {
        match child.name.as_str() {
            "register" => registers.push(parse_register(child, definitions)?),
            "registerFile" => register_files.push(parse_register_file(child, definitions)?),
            _ => {}
        }
    }

    let access_policies = inherited_access_policies(instance, source);
    Ok(AddressBlock {
        name: name.into(),
        description: instance
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        map_name: map_name.into(),
        base_address,
        range: instance
            .optional_child_text("range")
            .unwrap_or(source.child_text("range")?),
        width: instance
            .optional_child_text("width")
            .unwrap_or(source.child_text("width")?),
        address_unit_bits: address_unit_bits.into(),
        usage: instance
            .optional_child_text("usage")
            .or_else(|| source.optional_child_text("usage")),
        volatile: instance
            .optional_child_text("volatile")
            .or_else(|| source.optional_child_text("volatile")),
        access: selected_access(&access_policies).or_else(|| {
            instance
                .optional_child_text("access")
                .or_else(|| source.optional_child_text("access"))
        }),
        access_policies,
        hdl_path: inherited_access_handle_path(parent_hdl_path, access_handle_path(instance)),
        registers,
        register_files,
    })
}

fn parse_bank(
    node: &XmlNode,
    inherited_base: u64,
    parent_path: Vec<String>,
    parent_hdl_path: Option<String>,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<Vec<AddressBlock>> {
    let name = node.child_text("name")?;
    let source = definitions
        .bank_ref(node, "bankDefinitionRef")
        .unwrap_or(node);
    let bank_address_unit_bits = node
        .optional_child_text("addressUnitBits")
        .or_else(|| source.optional_child_text("addressUnitBits"))
        .unwrap_or_else(|| address_unit_bits.into());
    let mut path = parent_path;
    path.push(name);
    let base = node
        .optional_child_text("baseAddress")
        .map(|base| parse_u64_text("bank baseAddress", &base))
        .transpose()?
        .map_or(inherited_base, |base| inherited_base + base);
    let bank_hdl_path =
        inherited_access_handle_path(parent_hdl_path.as_ref(), access_handle_path(node));
    let alignment = node
        .attribute_text("bankAlignment")
        .or_else(|| source.attribute_text("bankAlignment"))
        .unwrap_or_else(|| "serial".into());
    let mut cursor = base;
    let mut blocks = Vec::new();

    for child in &source.children {
        match child.name.as_str() {
            "addressBlock" => {
                let child_name = format!("{}_{}", path.join("_"), child.child_text("name")?);
                let child_base = if alignment == "serial" { cursor } else { base };
                let block = parse_address_block_at(
                    child,
                    &child_name,
                    addr_text(child_base),
                    bank_hdl_path.as_ref(),
                    map_name,
                    &bank_address_unit_bits,
                    definitions,
                )?;
                if alignment == "serial" {
                    cursor += parse_u64_text("addressBlock range", &block.range)?;
                }
                blocks.push(block);
            }
            "bank" => {
                let child_base = if alignment == "serial" { cursor } else { base };
                let child_blocks = parse_bank(
                    child,
                    child_base,
                    path.clone(),
                    bank_hdl_path.clone(),
                    map_name,
                    &bank_address_unit_bits,
                    definitions,
                )?;
                if alignment == "serial" {
                    cursor += bank_span(&child_blocks);
                }
                blocks.extend(child_blocks);
            }
            _ => {}
        }
    }

    Ok(blocks)
}

fn bank_span(blocks: &[AddressBlock]) -> u64 {
    let Some(min_base) = blocks
        .iter()
        .filter_map(|block| parse_u64_text("addressBlock baseAddress", &block.base_address).ok())
        .min()
    else {
        return 0;
    };
    blocks
        .iter()
        .filter_map(|block| {
            let base = parse_u64_text("addressBlock baseAddress", &block.base_address).ok()?;
            let range = parse_u64_text("addressBlock range", &block.range).ok()?;
            Some(base + range - min_base)
        })
        .max()
        .unwrap_or(0)
}

fn parse_register_file(node: &XmlNode, definitions: &Definitions) -> Result<RegisterFile> {
    let source = definitions
        .register_file_ref(node, "registerFileDefinitionRef")
        .unwrap_or(node);
    let dims = element_dims(node);
    let registers = source
        .children_named("register")
        .map(|register| parse_register(register, definitions))
        .collect::<Result<Vec<_>>>()?;

    Ok(RegisterFile {
        name: node.child_text("name")?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        address_offset: node.child_text("addressOffset")?,
        range: node
            .optional_child_text("range")
            .unwrap_or(source.child_text("range")?),
        dim: total_dim_text(&dims),
        dims,
        stride: array_stride(node),
        hdl_path: access_handle_path(node),
        registers,
    })
}

fn parse_register(node: &XmlNode, definitions: &Definitions) -> Result<Register> {
    let source = definitions
        .register_ref(node, "registerDefinitionRef")
        .unwrap_or(node);
    let dims = element_dims(node);
    let fields = source
        .children_named("field")
        .map(|field| parse_field(field, definitions))
        .collect::<Result<Vec<_>>>()?;
    let alternate_registers = node
        .child("alternateRegisters")
        .map(|alternates| {
            alternates
                .children_named("alternateRegister")
                .map(|alternate| parse_alternate_register(alternate, definitions))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_default();

    let access_policies = inherited_access_policies(node, source);
    Ok(Register {
        name: node.child_text("name")?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        address_offset: node.child_text("addressOffset")?,
        size: node
            .optional_child_text("size")
            .unwrap_or(source.child_text("size")?),
        dim: total_dim_text(&dims),
        dims,
        stride: array_stride(node),
        volatile: node
            .optional_child_text("volatile")
            .or_else(|| source.optional_child_text("volatile")),
        access: selected_access(&access_policies).or_else(|| {
            node.optional_child_text("access")
                .or_else(|| source.optional_child_text("access"))
        }),
        access_policies,
        hdl_path: access_handle_path(node),
        fields,
        alternate_registers,
    })
}

fn parse_alternate_register(
    node: &XmlNode,
    definitions: &Definitions,
) -> Result<AlternateRegister> {
    let fields = node
        .children_named("field")
        .map(|field| parse_field(field, definitions))
        .collect::<Result<Vec<_>>>()?;

    let access_policies = parse_access_policies(node);
    Ok(AlternateRegister {
        name: node.child_text("name")?,
        description: node.optional_child_text("description").unwrap_or_default(),
        volatile: node.optional_child_text("volatile"),
        access: selected_access(&access_policies).or_else(|| node.optional_child_text("access")),
        access_policies,
        hdl_path: access_handle_path(node),
        groups_or_modes: alternate_groups_or_modes(node),
        fields,
    })
}

fn parse_field(node: &XmlNode, definitions: &Definitions) -> Result<Field> {
    let source = definitions
        .field_ref(node, "fieldDefinitionRef")
        .unwrap_or(node);
    let inline_policies = effective_field_policies(node, definitions);
    let source_policies = effective_field_policies(source, definitions);
    let policy_nodes = if inline_policies.is_empty() {
        source_policies
    } else {
        inline_policies
    };
    let access_policies = policy_nodes
        .iter()
        .map(|policy| access_policy_from_node(policy))
        .collect::<Vec<_>>();
    let policy = default_policy_node(&policy_nodes);
    let access = policy
        .and_then(|policy| policy.optional_child_text("access"))
        .or_else(|| source.optional_child_text("access"))
        .or_else(|| node.optional_child_text("access"));
    let modified_write_value = policy
        .and_then(|policy| policy.optional_child_text("modifiedWriteValue"))
        .or_else(|| source.optional_child_text("modifiedWriteValue"))
        .or_else(|| node.optional_child_text("modifiedWriteValue"));
    let read_action = policy
        .and_then(|policy| policy.optional_child_text("readAction"))
        .or_else(|| source.optional_child_text("readAction"))
        .or_else(|| node.optional_child_text("readAction"));
    let write_value_constraint = policy
        .and_then(|policy| policy.child("writeValueConstraint"))
        .or_else(|| source.child("writeValueConstraint"))
        .or_else(|| node.child("writeValueConstraint"))
        .map(parse_write_value_constraint);
    let testable = policy
        .and_then(|policy| policy.child("testable"))
        .or_else(|| source.child("testable"))
        .or_else(|| node.child("testable"))
        .map(parse_testable);
    let reserved = policy
        .and_then(|policy| policy.optional_child_text("reserved"))
        .or_else(|| source.optional_child_text("reserved"))
        .or_else(|| node.optional_child_text("reserved"));
    let access_restrictions = policy
        .and_then(|policy| policy.child("accessRestrictions"))
        .or_else(|| source.child("accessRestrictions"))
        .map(parse_access_restrictions)
        .unwrap_or_default();
    let broadcasts = policy
        .and_then(|policy| policy.child("broadcasts"))
        .or_else(|| source.child("broadcasts"))
        .map(parse_broadcasts)
        .unwrap_or_default();
    let resets = node
        .child("resets")
        .or_else(|| source.child("resets"))
        .map(parse_resets)
        .transpose()?
        .unwrap_or_default();
    let reset = resets.first().map(|reset| reset.value.clone());
    let enumerated_values = node
        .child("enumeratedValues")
        .map(|enumerated_values| parse_enumerated_values(enumerated_values, definitions))
        .or_else(|| {
            source
                .child("enumeratedValues")
                .map(|enumerated_values| parse_enumerated_values(enumerated_values, definitions))
        })
        .transpose()?
        .unwrap_or_default();

    Ok(Field {
        name: node.child_text("name")?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        bit_offset: node.child_text("bitOffset")?,
        bit_width: node
            .optional_child_text("bitWidth")
            .unwrap_or(source.child_text("bitWidth")?),
        access,
        access_policies,
        modified_write_value,
        read_action,
        volatile: node
            .optional_child_text("volatile")
            .or_else(|| source.optional_child_text("volatile")),
        reset,
        resets,
        hdl_path: access_handle_path(node),
        testable,
        reserved,
        write_value_constraint,
        access_restrictions,
        broadcasts,
        enumerated_values,
    })
}

fn parse_resets(node: &XmlNode) -> Result<Vec<Reset>> {
    node.children_named("reset")
        .map(|reset| {
            Ok(Reset {
                value: reset.child_text("value")?,
                mask: reset.optional_child_text("mask"),
                reset_type: reset
                    .optional_child_text("resetTypeRef")
                    .or_else(|| reset.attribute_text("resetTypeRef")),
            })
        })
        .collect()
}

fn parse_testable(node: &XmlNode) -> Testable {
    Testable {
        value: node.text.trim().to_string(),
        test_constraint: node.attribute_text("testConstraint"),
    }
}

fn parse_write_value_constraint(node: &XmlNode) -> WriteValueConstraint {
    WriteValueConstraint {
        write_as_read: node.optional_child_text("writeAsRead"),
        use_enumerated_values: node.optional_child_text("useEnumeratedValues"),
        minimum: node.optional_child_text("minimum"),
        maximum: node.optional_child_text("maximum"),
    }
}

fn effective_field_policies<'a>(
    node: &'a XmlNode,
    definitions: &'a Definitions,
) -> Vec<&'a XmlNode> {
    node.child("fieldAccessPolicies")
        .map(|policies| {
            policies
                .children_named("fieldAccessPolicy")
                .map(|policy| {
                    policy
                        .child("fieldAccessPolicyDefinitionRef")
                        .and_then(|_| {
                            definitions
                                .field_access_policy_ref(policy, "fieldAccessPolicyDefinitionRef")
                        })
                        .unwrap_or(policy)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn default_policy_node<'a>(policies: &'a [&'a XmlNode]) -> Option<&'a XmlNode> {
    policies
        .iter()
        .copied()
        .find(|policy| parse_mode_refs(policy).is_empty())
        .or_else(|| policies.first().copied())
}

fn parse_enumerated_values(
    node: &XmlNode,
    definitions: &Definitions,
) -> Result<Vec<EnumeratedValue>> {
    let source = node
        .child("enumerationDefinitionRef")
        .and_then(|_| definitions.enumeration_ref(node, "enumerationDefinitionRef"))
        .unwrap_or(node);

    source
        .children_named("enumeratedValue")
        .map(|enumerated_value| {
            Ok(EnumeratedValue {
                name: enumerated_value.child_text("name")?,
                value: enumerated_value.child_text("value")?,
                usage: enumerated_value.attribute_text("usage"),
            })
        })
        .collect()
}

fn parse_access_restrictions(node: &XmlNode) -> Vec<AccessRestriction> {
    node.children_named("accessRestriction")
        .map(|restriction| AccessRestriction {
            mode_refs: restriction
                .children_named("modeRef")
                .map(|mode_ref| ModeRef {
                    name: mode_ref.text.trim().to_string(),
                    priority: mode_ref.attribute_text("priority"),
                })
                .collect(),
            read_access_mask: restriction.optional_child_text("readAccessMask"),
            write_access_mask: restriction.optional_child_text("writeAccessMask"),
        })
        .collect()
}

fn parse_broadcasts(node: &XmlNode) -> Vec<Broadcast> {
    node.children_named("broadcastTo")
        .map(|broadcast_to| Broadcast {
            target: field_reference_segments(broadcast_to),
        })
        .collect()
}

fn alternate_groups_or_modes(node: &XmlNode) -> Vec<ModeRef> {
    let mut refs = node
        .children_named("modeRef")
        .map(parse_mode_ref)
        .collect::<Vec<_>>();

    if let Some(alternate_groups) = node.child("alternateGroups") {
        refs.extend(
            alternate_groups
                .children_named("alternateGroup")
                .map(parse_mode_ref),
        );
    }

    refs
}

fn parse_mode_ref(node: &XmlNode) -> ModeRef {
    ModeRef {
        name: node.text.trim().to_string(),
        priority: node.attribute_text("priority"),
    }
}

fn field_reference_segments(node: &XmlNode) -> Vec<FieldReferenceSegment> {
    let ref_names = [
        "addressSpaceRef",
        "memoryMapRef",
        "memoryRemapRef",
        "bankRef",
        "addressBlockRef",
        "registerFileRef",
        "registerRef",
        "alternateRegisterRef",
        "fieldRef",
    ];
    node.children
        .iter()
        .filter(|child| ref_names.contains(&child.name.as_str()))
        .filter_map(|child| {
            reference_name(child).map(|name| FieldReferenceSegment {
                kind: child.name.clone(),
                name,
            })
        })
        .collect()
}

fn reference_name(node: &XmlNode) -> Option<String> {
    node.attribute_text(&node.name)
        .or_else(|| (!node.text.trim().is_empty()).then(|| node.text.trim().to_string()))
}

fn access_handle_path(node: &XmlNode) -> Option<String> {
    let access_handles = node.child("accessHandles")?;
    let access_handle = access_handles.child("accessHandle")?;
    let path_segments = first_descendant(access_handle, "pathSegments")?;
    let segments = path_segments
        .children_named("pathSegment")
        .filter_map(path_segment_text)
        .collect::<Vec<_>>();
    (!segments.is_empty()).then(|| segments.join("."))
}

fn inherited_access_handle_path(parent: Option<&String>, child: Option<String>) -> Option<String> {
    match (parent, child) {
        (Some(parent), Some(child))
            if !parent.trim().starts_with('`') && !child.trim().starts_with('`') =>
        {
            Some(format!("{}.{}", parent.trim(), child.trim()))
        }
        (_, Some(child)) => Some(child),
        (Some(parent), None) => Some(parent.clone()),
        (None, None) => None,
    }
}

fn parse_u64_text(field: &'static str, value: &str) -> Result<u64> {
    let trimmed = value.trim();
    let parsed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .map(|hex| u64::from_str_radix(hex, 16))
        .or_else(|| {
            trimmed
                .strip_prefix("0b")
                .or_else(|| trimmed.strip_prefix("0B"))
                .map(|binary| u64::from_str_radix(binary, 2))
        })
        .unwrap_or_else(|| trimmed.parse::<u64>());
    parsed.map_err(|_| crate::Error::InvalidNumber {
        field,
        value: value.into(),
    })
}

fn addr_text(value: u64) -> String {
    format!("0x{value:x}")
}

fn inherited_access_policies(instance: &XmlNode, source: &XmlNode) -> Vec<AccessPolicy> {
    let policies = parse_access_policies(instance);
    if policies.is_empty() {
        parse_access_policies(source)
    } else {
        policies
    }
}

fn parse_access_policies(node: &XmlNode) -> Vec<AccessPolicy> {
    node.child("accessPolicies")
        .map(|policies| {
            policies
                .children_named("accessPolicy")
                .map(access_policy_from_node)
                .collect()
        })
        .unwrap_or_default()
}

fn access_policy_from_node(node: &XmlNode) -> AccessPolicy {
    AccessPolicy {
        access: node.optional_child_text("access"),
        mode_refs: parse_mode_refs(node),
    }
}

fn selected_access(policies: &[AccessPolicy]) -> Option<String> {
    policies
        .iter()
        .find(|policy| policy.mode_refs.is_empty() && policy.access.is_some())
        .or_else(|| policies.iter().find(|policy| policy.access.is_some()))
        .and_then(|policy| policy.access.clone())
}

fn parse_mode_refs(node: &XmlNode) -> Vec<ModeRef> {
    node.children_named("modeRef").map(parse_mode_ref).collect()
}

fn definition_ref(node: &XmlNode, name: &str) -> Option<DefinitionReference> {
    node.child(name).and_then(|definition_ref| {
        let name = definition_ref.text.trim().to_string();
        (!name.is_empty()).then(|| DefinitionReference {
            name,
            type_definitions: definition_ref.attribute_text("typeDefinitions"),
        })
    })
}

fn element_dims(node: &XmlNode) -> Vec<String> {
    if let Some(dim) = node.optional_child_text("dim") {
        return vec![dim];
    }

    let dims = node
        .child("array")
        .map(|array| {
            array
                .children_named("dim")
                .filter_map(|dim| {
                    let text = dim.text.trim();
                    (!text.is_empty()).then(|| text.to_string())
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if dims.is_empty() {
        vec!["1".into()]
    } else {
        dims
    }
}

fn total_dim_text(dims: &[String]) -> String {
    let mut total = 1u64;
    for dim in dims {
        let Ok(value) = parse_u64_text("array dim", dim) else {
            return dims.join("*");
        };
        let Some(next) = total.checked_mul(value) else {
            return dims.join("*");
        };
        total = next;
    }
    total.to_string()
}

fn array_stride(node: &XmlNode) -> Option<String> {
    node.child("array")
        .and_then(|array| array.optional_child_text("stride"))
}

fn first_descendant<'a>(node: &'a XmlNode, name: &str) -> Option<&'a XmlNode> {
    for child in &node.children {
        if child.name == name {
            return Some(child);
        }
        if let Some(descendant) = first_descendant(child, name) {
            return Some(descendant);
        }
    }
    None
}

fn path_segment_text(node: &XmlNode) -> Option<String> {
    node.optional_child_text("pathSegmentName")
        .or_else(|| (!node.text.trim().is_empty()).then(|| node.text.trim().to_string()))
}
