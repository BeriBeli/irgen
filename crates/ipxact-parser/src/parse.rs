use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

use crate::{Error, Result};
use irgen_ipxact_model::{
    AddressBlock, AddressSpace, AlternateRegister, Component, EnumeratedValue, Field, HdlPathSlice,
    IndexedHdlPath, MemoryRemap, Register, RegisterFile, Reset, Segment, SubspaceMap,
    parse_bool_expr_with_symbols, parse_u64_expr, parse_u64_expr_with_symbols,
};

const IPXACT_2022_NAMESPACE: &str = "http://www.accellera.org/XMLSchema/IPXACT/1685-2022";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryRef {
    pub vendor: String,
    pub library: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogFileRef {
    pub library_ref: LibraryRef,
    pub name: String,
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

    fn from_vlnv_node(node: &XmlNode) -> Result<Self> {
        Self::from_node(node)
    }

    pub fn key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.vendor, self.library, self.name, self.version
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParseOptions {
    pub preferred_view: Option<String>,
    pub preferred_mode: Option<String>,
}

#[derive(Debug, Clone)]
struct XmlNode {
    name: String,
    text: String,
    attributes: Vec<(String, String)>,
    children: Vec<XmlNode>,
}

#[derive(Debug, Default, Clone)]
struct Definitions {
    options: ParseOptions,
    parameters: HashMap<String, u64>,
    unsupported_parameters: HashMap<String, String>,
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
    fn from_root_and_external(
        root: &XmlNode,
        external_roots: &[(String, XmlNode)],
        options: ParseOptions,
    ) -> Self {
        let mut definitions = Definitions {
            options,
            ..Definitions::default()
        };
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
        self.collect_parameter_values(node);
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

    fn collect_parameter_values(&mut self, node: &XmlNode) {
        if let Some(parameters) = node.child("parameters") {
            for parameter in parameters.children_named("parameter") {
                let Some(value) = parameter.optional_child_text("value") else {
                    continue;
                };
                if let Some(name) = parameter.optional_child_text("name") {
                    self.collect_numeric_parameter(name, &value);
                }
                if let Some(parameter_id) = parameter.attribute_text("parameterId") {
                    self.collect_numeric_parameter(parameter_id, &value);
                }
            }
        }

        if let Some(values) = node.child("configurableElementValues") {
            for value in values.children_named("configurableElementValue") {
                let Some(reference_id) = value.attribute_text("referenceId") else {
                    continue;
                };
                let value_text = value.text.trim();
                if value_text.is_empty() {
                    continue;
                }
                self.collect_numeric_parameter(reference_id, value_text);
            }
        }
    }

    fn collect_numeric_parameter(&mut self, key: String, value: &str) {
        match parse_u64_expr_with_symbols("parameter value", value, &self.parameters) {
            Ok(parsed) => {
                self.parameters.insert(key.clone(), parsed);
                self.unsupported_parameters.remove(&key);
            }
            Err(_) => {
                self.parameters.remove(&key);
                self.unsupported_parameters.insert(key, value.into());
            }
        }
    }

    fn with_node_parameter_values(&self, source: &XmlNode, instance: &XmlNode) -> Self {
        let mut definitions = self.clone();
        definitions.collect_parameter_values(source);
        if !std::ptr::eq(source, instance) {
            definitions.collect_parameter_values(instance);
        }
        definitions
    }

    fn address_block_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("addressBlockDefinition", &self.address_blocks, node, name)
    }

    fn memory_map_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("memoryMapDefinition", &self.memory_maps, node, name)
    }

    fn memory_remap_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("memoryRemapDefinition", &self.memory_remaps, node, name)
    }

    fn bank_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("bankDefinition", &self.banks, node, name)
    }

    fn register_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("registerDefinition", &self.registers, node, name)
    }

    fn register_file_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("registerFileDefinition", &self.register_files, node, name)
    }

    fn field_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("fieldDefinition", &self.fields, node, name)
    }

    fn enumeration_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref("enumerationDefinition", &self.enumerations, node, name)
    }

    fn field_access_policy_ref(&self, node: &XmlNode, name: &str) -> Result<Option<&XmlNode>> {
        self.definition_ref(
            "fieldAccessPolicyDefinition",
            &self.field_access_policies,
            node,
            name,
        )
    }

    fn definition_ref<'a>(
        &'a self,
        kind: &'static str,
        map: &'a HashMap<String, XmlNode>,
        node: &XmlNode,
        name: &str,
    ) -> Result<Option<&'a XmlNode>> {
        let Some(reference) = definition_ref(node, name) else {
            return Ok(None);
        };
        lookup_definition(map, &reference)
            .map(Some)
            .ok_or_else(|| Error::TypeDefinitionNotFound {
                kind,
                reference: reference.key(),
            })
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

impl DefinitionReference {
    fn key(&self) -> String {
        self.type_definitions
            .as_deref()
            .map(|scope| definition_key(scope, &self.name))
            .unwrap_or_else(|| self.name.clone())
    }
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

pub fn parse_ipxact_with_options(xml: &str, options: ParseOptions) -> Result<Component> {
    parse_ipxact_with_options_and_resolver(xml, options, |_| Ok(None))
}

pub fn parse_ipxact_with_resolver(
    xml: &str,
    mut resolver: impl FnMut(&LibraryRef) -> Result<Option<String>>,
) -> Result<Component> {
    parse_ipxact_with_options_and_resolver(xml, ParseOptions::default(), &mut resolver)
}

pub fn parse_ipxact_with_options_and_resolver(
    xml: &str,
    options: ParseOptions,
    mut resolver: impl FnMut(&LibraryRef) -> Result<Option<String>>,
) -> Result<Component> {
    let root = parse_xml(xml)?;
    if root.name != "component" {
        return Err(Error::UnsupportedRoot(root.name));
    }
    let mut external_roots = Vec::new();
    let mut resolved = HashMap::new();
    resolve_external_type_definitions(&root, &mut resolver, &mut resolved, &mut external_roots)?;
    let definitions = Definitions::from_root_and_external(&root, &external_roots, options);
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

pub fn catalog_file_refs(xml: &str) -> Result<Vec<CatalogFileRef>> {
    let root = parse_xml(xml)?;
    if root.name != "catalog" {
        return Err(Error::UnsupportedRoot(root.name));
    }

    root.children
        .iter()
        .flat_map(|group| group.children_named("ipxactFile"))
        .map(|file| {
            Ok(CatalogFileRef {
                library_ref: LibraryRef::from_vlnv_node(
                    file.child("vlnv").ok_or(Error::MissingElement("vlnv"))?,
                )?,
                name: file.child_text("name")?,
            })
        })
        .collect()
}

fn parse_component(root: &XmlNode, definitions: &Definitions) -> Result<Component> {
    let initiator_address_spaces = initiator_address_spaces(root);
    let address_spaces = parse_address_spaces(root, definitions)?;
    let component_name = root.child_text("name")?;
    let memory_maps = root.child("memoryMaps");
    let mut blocks = Vec::new();
    let mut subspace_maps = Vec::new();
    let mut memory_remaps = Vec::new();
    let mut memory_map_names = Vec::new();
    if let Some(memory_maps) = memory_maps {
        for memory_map in memory_maps.children_named("memoryMap") {
            let map_name = memory_map.child_text("name")?;
            if memory_map_names.contains(&map_name) {
                return Err(Error::DuplicateMemoryMapName { name: map_name });
            }
            memory_map_names.push(map_name.clone());
            let source = definitions
                .memory_map_ref(memory_map, "memoryMapDefinitionRef")?
                .unwrap_or(memory_map);
            let scoped_definitions = definitions.with_node_parameter_values(source, memory_map);
            let definitions = &scoped_definitions;
            let mut block_names = Vec::new();
            let mut subspace_map_names = Vec::new();
            let mut memory_remap_names = Vec::new();
            let address_unit_bits = normalize_numeric_text(
                definitions,
                "memoryMap addressUnitBits",
                memory_map
                    .optional_child_text("addressUnitBits")
                    .or_else(|| source.optional_child_text("addressUnitBits"))
                    .unwrap_or_else(|| "8".into()),
            )?;
            for block in source.children_named("addressBlock") {
                reject_unsupported_is_present(block)?;
                ensure_unique_ipxact_name(
                    &mut block_names,
                    "addressBlock",
                    "memoryMap",
                    &map_name,
                    &block.child_text("name")?,
                )?;
                blocks.push(parse_address_block(
                    block,
                    &map_name,
                    &address_unit_bits,
                    definitions,
                )?);
            }
            for bank in source.children_named("bank") {
                reject_unsupported_is_present(bank)?;
                blocks.extend(parse_bank(
                    bank,
                    0,
                    Vec::new(),
                    None,
                    &map_name,
                    &address_unit_bits,
                    definitions,
                )?);
            }
            for subspace_map in source.children_named("subspaceMap") {
                reject_unsupported_is_present(subspace_map)?;
                ensure_unique_ipxact_name(
                    &mut subspace_map_names,
                    "subspaceMap",
                    "memoryMap",
                    &map_name,
                    &subspace_map.child_text("name")?,
                )?;
                subspace_maps.push(parse_subspace_map(
                    subspace_map,
                    &map_name,
                    &address_unit_bits,
                    definitions,
                    &initiator_address_spaces,
                )?);
            }
            for memory_remap in source.children_named("memoryRemap") {
                reject_unsupported_is_present(memory_remap)?;
                if !memory_remap_matches_preferred_mode(memory_remap, definitions)? {
                    continue;
                }
                ensure_unique_ipxact_name(
                    &mut memory_remap_names,
                    "memoryRemap",
                    "memoryMap",
                    &map_name,
                    &memory_remap.child_text("name")?,
                )?;
                memory_remaps.push(parse_memory_remap(
                    memory_remap,
                    &map_name,
                    &address_unit_bits,
                    definitions,
                    &initiator_address_spaces,
                )?);
            }
        }
    }

    Ok(Component {
        vendor: root.child_text("vendor")?,
        library: root.child_text("library")?,
        name: component_name,
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
    let mut address_space_names = Vec::new();
    let component_name = root.child_text("name")?;

    for address_space in address_spaces_node.children_named("addressSpace") {
        reject_unsupported_is_present(address_space)?;
        let scoped_definitions =
            definitions.with_node_parameter_values(address_space, address_space);
        let definitions = &scoped_definitions;
        let name = address_space.child_text("name")?;
        ensure_unique_ipxact_name(
            &mut address_space_names,
            "addressSpace",
            "component",
            &component_name,
            &name,
        )?;
        let address_unit_bits = normalize_numeric_text(
            definitions,
            "addressSpace addressUnitBits",
            address_space
                .optional_child_text("addressUnitBits")
                .unwrap_or_else(|| "8".into()),
        )?;
        let segments = parse_segments(address_space, definitions)?;
        let mut blocks = Vec::new();

        if let Some(local_memory_map) = address_space.child("localMemoryMap") {
            let mut block_names = Vec::new();
            for block in local_memory_map.children_named("addressBlock") {
                reject_unsupported_is_present(block)?;
                ensure_unique_ipxact_name(
                    &mut block_names,
                    "addressBlock",
                    "localMemoryMap",
                    &name,
                    &block.child_text("name")?,
                )?;
                blocks.push(parse_address_block(
                    block,
                    &name,
                    &address_unit_bits,
                    definitions,
                )?);
            }
            for bank in local_memory_map.children_named("bank") {
                reject_unsupported_is_present(bank)?;
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
            segments,
            blocks,
        });
    }

    Ok(address_spaces)
}

fn parse_segments(address_space: &XmlNode, definitions: &Definitions) -> Result<Vec<Segment>> {
    let Some(segments) = address_space.child("segments") else {
        return Ok(Vec::new());
    };

    let mut parsed = Vec::new();
    let mut segment_names = Vec::new();
    let address_space_name = address_space.child_text("name")?;
    for segment in segments.children_named("segment") {
        reject_unsupported_is_present(segment)?;
        let name = segment.child_text("name")?;
        ensure_unique_ipxact_name(
            &mut segment_names,
            "segment",
            "addressSpace",
            &address_space_name,
            &name,
        )?;
        parsed.push(Segment {
            name,
            address_offset: required_numeric_text(
                definitions,
                segment,
                "addressOffset",
                "addressSpace segment addressOffset",
            )?,
            range: required_numeric_text(
                definitions,
                segment,
                "range",
                "addressSpace segment range",
            )?,
        });
    }
    Ok(parsed)
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
            Event::Start(event) => {
                if stack.is_empty() && root.is_none() {
                    validate_ipxact_namespace(&event, &reader)?;
                }
                stack.push(xml_node(&event, &reader)?);
            }
            Event::Empty(event) => {
                if stack.is_empty() && root.is_none() {
                    validate_ipxact_namespace(&event, &reader)?;
                }
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

fn validate_ipxact_namespace(event: &BytesStart<'_>, reader: &Reader<&[u8]>) -> Result<()> {
    let namespace = root_namespace(event, reader)?;
    if namespace.as_deref() == Some(IPXACT_2022_NAMESPACE) {
        return Ok(());
    }

    Err(Error::UnsupportedNamespace(
        namespace.unwrap_or_else(|| "<none>".into()),
    ))
}

fn root_namespace(event: &BytesStart<'_>, reader: &Reader<&[u8]>) -> Result<Option<String>> {
    let element_name = event.name();
    let element_name = element_name.as_ref();
    let prefix = element_name
        .iter()
        .position(|byte| *byte == b':')
        .map(|index| &element_name[..index]);

    for attribute in event.attributes() {
        let attribute = attribute?;
        let key = attribute.key.as_ref();
        let matches_namespace = match prefix {
            Some(prefix) => key
                .strip_prefix(b"xmlns:")
                .is_some_and(|attribute_prefix| attribute_prefix == prefix),
            None => key == b"xmlns",
        };
        if matches_namespace {
            return Ok(Some(
                attribute
                    .decode_and_unescape_value(reader.decoder())?
                    .into_owned(),
            ));
        }
    }

    Ok(None)
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
    let source = definitions
        .address_block_ref(node, "addressBlockDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let base_address = required_numeric_text(
        &scoped_definitions,
        node,
        "baseAddress",
        "addressBlock baseAddress",
    )?;
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
    definitions: &Definitions,
    initiator_address_spaces: &HashMap<String, String>,
) -> Result<SubspaceMap> {
    let initiator_ref = node
        .attribute_text("initiatorRef")
        .ok_or(Error::MissingElement("initiatorRef"))?;
    let address_space_ref = initiator_address_spaces.get(&initiator_ref).cloned();

    Ok(SubspaceMap {
        name: node.child_text("name")?,
        map_name: map_name.into(),
        base_address: required_numeric_text(
            definitions,
            node,
            "baseAddress",
            "subspaceMap baseAddress",
        )?,
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
        .memory_remap_ref(node, "remapDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;
    let remap_address_unit_bits = normalize_numeric_text(
        definitions,
        "memoryRemap addressUnitBits",
        node.optional_child_text("addressUnitBits")
            .or_else(|| source.optional_child_text("addressUnitBits"))
            .unwrap_or_else(|| address_unit_bits.into()),
    )?;
    let mut blocks = Vec::new();
    let mut subspace_maps = Vec::new();
    let mut block_names = Vec::new();
    let mut subspace_map_names = Vec::new();

    for block in source.children_named("addressBlock") {
        reject_unsupported_is_present(block)?;
        ensure_unique_ipxact_name(
            &mut block_names,
            "addressBlock",
            "memoryRemap",
            &name,
            &block.child_text("name")?,
        )?;
        blocks.push(parse_address_block_with_prefix(
            block,
            &name,
            map_name,
            &remap_address_unit_bits,
            definitions,
        )?);
    }
    for bank in source.children_named("bank") {
        reject_unsupported_is_present(bank)?;
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
        reject_unsupported_is_present(subspace_map)?;
        ensure_unique_ipxact_name(
            &mut subspace_map_names,
            "subspaceMap",
            "memoryRemap",
            &name,
            &subspace_map.child_text("name")?,
        )?;
        subspace_maps.push(parse_subspace_map_with_prefix(
            subspace_map,
            &name,
            map_name,
            &remap_address_unit_bits,
            definitions,
            initiator_address_spaces,
        )?);
    }

    Ok(MemoryRemap {
        name,
        map_name: map_name.into(),
        blocks,
        subspace_maps,
    })
}

fn memory_remap_matches_preferred_mode(node: &XmlNode, definitions: &Definitions) -> Result<bool> {
    let Some(preferred_mode) = definitions.options.preferred_mode.as_deref() else {
        return Ok(true);
    };

    let source = definitions
        .memory_remap_ref(node, "remapDefinitionRef")?
        .unwrap_or(node);
    let mode_source = if has_mode_ref(node) { node } else { source };

    Ok(!has_mode_ref(mode_source) || node_has_mode(mode_source, preferred_mode))
}

fn parse_address_block_with_prefix(
    node: &XmlNode,
    prefix: &str,
    map_name: &str,
    address_unit_bits: &str,
    definitions: &Definitions,
) -> Result<AddressBlock> {
    let name = format!("{}_{}", prefix, node.child_text("name")?);
    let source = definitions
        .address_block_ref(node, "addressBlockDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let base_address = required_numeric_text(
        &scoped_definitions,
        node,
        "baseAddress",
        "addressBlock baseAddress",
    )?;
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
    definitions: &Definitions,
    initiator_address_spaces: &HashMap<String, String>,
) -> Result<SubspaceMap> {
    let mut subspace_map = parse_subspace_map(
        node,
        map_name,
        address_unit_bits,
        definitions,
        initiator_address_spaces,
    )?;
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
        .address_block_ref(node, "addressBlockDefinitionRef")?
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

#[allow(clippy::too_many_arguments)]
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
    let mut child_names = Vec::new();
    let scoped_definitions = definitions.with_node_parameter_values(source, instance);
    let definitions = &scoped_definitions;
    validate_unsupported_access_policy_features(
        "addressBlock",
        name,
        instance,
        source,
        definitions,
    )?;

    for child in &source.children {
        match child.name.as_str() {
            "register" => {
                reject_unsupported_is_present(child)?;
                ensure_unique_ipxact_name(
                    &mut child_names,
                    "register",
                    "addressBlock",
                    name,
                    &child.child_text("name")?,
                )?;
                registers.push(parse_register(child, definitions)?)
            }
            "registerFile" => {
                reject_unsupported_is_present(child)?;
                ensure_unique_ipxact_name(
                    &mut child_names,
                    "registerFile",
                    "addressBlock",
                    name,
                    &child.child_text("name")?,
                )?;
                register_files.push(parse_register_file(child, definitions)?)
            }
            _ => {}
        }
    }

    Ok(AddressBlock {
        name: name.into(),
        map_name: map_name.into(),
        base_address,
        range: normalize_numeric_text(
            definitions,
            "addressBlock range",
            instance
                .optional_child_text("range")
                .unwrap_or(source.child_text("range")?),
        )?,
        width: normalize_numeric_text(
            definitions,
            "addressBlock width",
            instance
                .optional_child_text("width")
                .unwrap_or(source.child_text("width")?),
        )?,
        description: instance
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        address_unit_bits: address_unit_bits.into(),
        usage: instance
            .optional_child_text("usage")
            .or_else(|| source.optional_child_text("usage")),
        volatile: normalize_optional_bool_text(
            definitions,
            "addressBlock volatile",
            instance
                .optional_child_text("volatile")
                .or_else(|| source.optional_child_text("volatile")),
        )?,
        access: inherited_access_policy_access(
            "addressBlock",
            name,
            instance,
            source,
            definitions,
        )?
        .or_else(|| {
            instance
                .optional_child_text("access")
                .or_else(|| source.optional_child_text("access"))
        }),
        hdl_path: inherited_access_handle_path(
            parent_hdl_path,
            access_handle_path_for_node(instance, definitions, name)?,
        ),
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
        .bank_ref(node, "bankDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;
    let bank_address_unit_bits = normalize_numeric_text(
        definitions,
        "bank addressUnitBits",
        node.optional_child_text("addressUnitBits")
            .or_else(|| source.optional_child_text("addressUnitBits"))
            .unwrap_or_else(|| address_unit_bits.into()),
    )?;
    let mut path = parent_path;
    path.push(name);
    let base = node
        .optional_child_text("baseAddress")
        .map(|base| parse_u64_text(definitions, "bank baseAddress", &base))
        .transpose()?
        .map_or(inherited_base, |base| inherited_base + base);
    let bank_hdl_path = inherited_access_handle_path(
        parent_hdl_path.as_ref(),
        access_handle_path_for_node(node, definitions, &path.join("_"))?,
    );
    let alignment = node
        .attribute_text("bankAlignment")
        .or_else(|| source.attribute_text("bankAlignment"))
        .unwrap_or_else(|| "serial".into());
    let mut cursor = base;
    let mut blocks = Vec::new();

    for child in &source.children {
        match child.name.as_str() {
            "addressBlock" => {
                reject_unsupported_is_present(child)?;
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
                    cursor += parse_u64_text(definitions, "addressBlock range", &block.range)?;
                }
                blocks.push(block);
            }
            "bank" => {
                reject_unsupported_is_present(child)?;
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
                    cursor += bank_span(definitions, &child_blocks);
                }
                blocks.extend(child_blocks);
            }
            _ => {}
        }
    }

    Ok(blocks)
}

fn bank_span(definitions: &Definitions, blocks: &[AddressBlock]) -> u64 {
    let Some(min_base) = blocks
        .iter()
        .filter_map(|block| {
            parse_u64_text(definitions, "addressBlock baseAddress", &block.base_address).ok()
        })
        .min()
    else {
        return 0;
    };
    blocks
        .iter()
        .filter_map(|block| {
            let base = parse_u64_text(definitions, "addressBlock baseAddress", &block.base_address)
                .ok()?;
            let range = parse_u64_text(definitions, "addressBlock range", &block.range).ok()?;
            Some(base + range - min_base)
        })
        .max()
        .unwrap_or(0)
}

fn parse_register_file(node: &XmlNode, definitions: &Definitions) -> Result<RegisterFile> {
    let source = definitions
        .register_file_ref(node, "registerFileDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;
    let mut registers = Vec::new();
    let mut register_names = Vec::new();
    let register_file_name = node.child_text("name")?;
    let dims = element_dims("registerFile", &register_file_name, node, definitions)?;
    for register in source.children_named("register") {
        reject_unsupported_is_present(register)?;
        ensure_unique_ipxact_name(
            &mut register_names,
            "register",
            "registerFile",
            &register_file_name,
            &register.child_text("name")?,
        )?;
        registers.push(parse_register(register, definitions)?);
    }
    let hdl_path = access_handle_path_for_node(node, definitions, &register_file_name)?;

    Ok(RegisterFile {
        name: register_file_name,
        address_offset: required_numeric_text(
            definitions,
            node,
            "addressOffset",
            "registerFile addressOffset",
        )?,
        range: normalize_numeric_text(
            definitions,
            "registerFile range",
            node.optional_child_text("range")
                .unwrap_or(source.child_text("range")?),
        )?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        dim: total_dim_text(&dims),
        dims,
        stride: array_stride(node, definitions)?,
        hdl_path,
        registers,
    })
}

fn parse_register(node: &XmlNode, definitions: &Definitions) -> Result<Register> {
    let source = definitions
        .register_ref(node, "registerDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;
    let mut fields = Vec::new();
    let mut field_names = Vec::new();
    let register_name = node.child_text("name")?;
    let dims = element_dims("register", &register_name, node, definitions)?;
    validate_unsupported_access_policy_features(
        "register",
        &register_name,
        node,
        source,
        definitions,
    )?;
    for field in source.children_named("field") {
        reject_unsupported_is_present(field)?;
        ensure_unique_ipxact_name(
            &mut field_names,
            "field",
            "register",
            &register_name,
            &field.child_text("name")?,
        )?;
        fields.push(parse_field(field, definitions, Some(&register_name))?);
    }
    let alternate_registers = node
        .child("alternateRegisters")
        .map(|alternates| {
            let mut alternate_registers = Vec::new();
            let mut alternate_register_names = Vec::new();
            for alternate in alternates.children_named("alternateRegister") {
                reject_unsupported_is_present(alternate)?;
                ensure_unique_ipxact_name(
                    &mut alternate_register_names,
                    "alternateRegister",
                    "register",
                    &register_name,
                    &alternate.child_text("name")?,
                )?;
                alternate_registers.push(parse_alternate_register(alternate, definitions)?);
            }
            Ok::<_, Error>(alternate_registers)
        })
        .transpose()?
        .unwrap_or_default();
    let indexed_hdl_paths =
        indexed_access_handle_paths_for_node(node, definitions, &register_name)?;
    let hdl_path = access_handle_path_for_node(node, definitions, &register_name)?;
    let access =
        inherited_access_policy_access("register", &register_name, node, source, definitions)?
            .or_else(|| {
                node.optional_child_text("access")
                    .or_else(|| source.optional_child_text("access"))
            });

    Ok(Register {
        name: register_name,
        address_offset: required_numeric_text(
            definitions,
            node,
            "addressOffset",
            "register addressOffset",
        )?,
        size: normalize_numeric_text(
            definitions,
            "register size",
            node.optional_child_text("size")
                .unwrap_or(source.child_text("size")?),
        )?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        dim: total_dim_text(&dims),
        dims,
        stride: array_stride(node, definitions)?,
        volatile: normalize_optional_bool_text(
            definitions,
            "register volatile",
            node.optional_child_text("volatile")
                .or_else(|| source.optional_child_text("volatile")),
        )?,
        access,
        hdl_path,
        indexed_hdl_paths,
        fields,
        alternate_registers,
    })
}

fn parse_alternate_register(
    node: &XmlNode,
    definitions: &Definitions,
) -> Result<AlternateRegister> {
    let mut fields = Vec::new();
    let mut field_names = Vec::new();
    let alternate_register_name = node.child_text("name")?;
    validate_unsupported_access_policy_features(
        "alternateRegister",
        &alternate_register_name,
        node,
        node,
        definitions,
    )?;
    for field in node.children_named("field") {
        reject_unsupported_is_present(field)?;
        ensure_unique_ipxact_name(
            &mut field_names,
            "field",
            "alternateRegister",
            &alternate_register_name,
            &field.child_text("name")?,
        )?;
        fields.push(parse_field(
            field,
            definitions,
            Some(&alternate_register_name),
        )?);
    }
    let hdl_path = access_handle_path_for_node(node, definitions, &alternate_register_name)?;
    let access = access_policy_access(
        "alternateRegister",
        &alternate_register_name,
        node,
        definitions,
    )?
    .or_else(|| node.optional_child_text("access"));

    Ok(AlternateRegister {
        name: alternate_register_name,
        description: node.optional_child_text("description").unwrap_or_default(),
        volatile: normalize_optional_bool_text(
            definitions,
            "alternateRegister volatile",
            node.optional_child_text("volatile"),
        )?,
        access,
        hdl_path,
        fields,
    })
}

fn parse_field(
    node: &XmlNode,
    definitions: &Definitions,
    parent_name: Option<&str>,
) -> Result<Field> {
    let source = definitions
        .field_ref(node, "fieldDefinitionRef")?
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;
    let field_name = node.child_text("name")?;
    let access_handle_owner = parent_name
        .map(|parent_name| format!("{parent_name}.{field_name}"))
        .unwrap_or_else(|| field_name.clone());
    let inline_policies = effective_field_policies(node, definitions)?;
    let source_policies = effective_field_policies(source, definitions)?;
    let policy_nodes = if inline_policies.is_empty() {
        source_policies
    } else {
        inline_policies
    };
    let policy = selected_policy_node(
        &policy_nodes,
        definitions.options.preferred_mode.as_deref(),
        Some(("field", &field_name)),
    )?;
    validate_unsupported_features(
        "field",
        &field_name,
        &[Some(source), Some(node)],
        &[
            "access",
            "modifiedWriteValue",
            "readAction",
            "testable",
            "reserved",
        ],
    )?;
    validate_unsupported_features(
        "field",
        &field_name,
        &[policy, Some(source), Some(node)],
        &["writeValueConstraint", "broadcasts", "accessRestrictions"],
    )?;
    let access = policy.and_then(|policy| policy.optional_child_text("access"));
    let modified_write_value =
        policy.and_then(|policy| policy.optional_child_text("modifiedWriteValue"));
    let read_action = policy.and_then(|policy| policy.optional_child_text("readAction"));
    let testable = policy.and_then(|policy| policy.optional_child_text("testable"));
    let reserved = policy.and_then(|policy| policy.optional_child_text("reserved"));
    let resets = node
        .child("resets")
        .or_else(|| source.child("resets"))
        .map(|resets| parse_resets(resets, definitions))
        .transpose()?
        .unwrap_or_default();
    let reset = resets.first().map(|reset| reset.value.clone());
    let enumerated_values = node
        .child("enumeratedValues")
        .map(|enumerated_values| {
            parse_enumerated_values(enumerated_values, definitions, &field_name)
        })
        .or_else(|| {
            source.child("enumeratedValues").map(|enumerated_values| {
                parse_enumerated_values(enumerated_values, definitions, &field_name)
            })
        })
        .transpose()?
        .unwrap_or_default();

    Ok(Field {
        name: field_name,
        bit_offset: required_numeric_text(definitions, node, "bitOffset", "field bitOffset")?,
        bit_width: normalize_numeric_text(
            definitions,
            "field bitWidth",
            node.optional_child_text("bitWidth")
                .unwrap_or(source.child_text("bitWidth")?),
        )?,
        description: node
            .optional_child_text("description")
            .or_else(|| source.optional_child_text("description"))
            .unwrap_or_default(),
        access,
        modified_write_value,
        read_action,
        volatile: normalize_optional_bool_text(
            definitions,
            "field volatile",
            node.optional_child_text("volatile")
                .or_else(|| source.optional_child_text("volatile")),
        )?,
        testable: normalize_optional_bool_text(definitions, "field testable", testable)?,
        reserved: normalize_optional_bool_text(definitions, "field reserved", reserved)?,
        reset,
        resets,
        hdl_path: access_handle_path_for_node(node, definitions, &access_handle_owner)?,
        hdl_path_slices: access_handle_slices_for_node(node, definitions, &access_handle_owner)?,
        indexed_hdl_paths: indexed_access_handle_paths_for_node(
            node,
            definitions,
            &access_handle_owner,
        )?,
        enumerated_values,
    })
}

fn validate_unsupported_access_policy_features(
    kind: &'static str,
    name: &str,
    instance: &XmlNode,
    source: &XmlNode,
    definitions: &Definitions,
) -> Result<()> {
    let instance_policy_nodes = access_policy_nodes(instance);
    let instance_policy = selected_policy_node(
        &instance_policy_nodes,
        definitions.options.preferred_mode.as_deref(),
        Some((kind, name)),
    )?;
    validate_unsupported_features(
        kind,
        name,
        &[instance_policy, Some(instance)],
        &["accessRestrictions"],
    )?;

    if !std::ptr::eq(instance, source)
        && instance_policy
            .and_then(|policy| policy.optional_child_text("access"))
            .is_none()
    {
        let source_policy_nodes = access_policy_nodes(source);
        let source_policy = selected_policy_node(
            &source_policy_nodes,
            definitions.options.preferred_mode.as_deref(),
            Some((kind, name)),
        )?;
        validate_unsupported_features(
            kind,
            name,
            &[source_policy, Some(source)],
            &["accessRestrictions"],
        )?;
    }
    Ok(())
}

fn validate_unsupported_features(
    kind: &'static str,
    name: &str,
    nodes: &[Option<&XmlNode>],
    features: &[&str],
) -> Result<()> {
    for node in nodes.iter().flatten() {
        for feature in features {
            if node.child(feature).is_some() {
                return Err(Error::UnsupportedElementFeature {
                    kind,
                    name: name.into(),
                    feature: (*feature).into(),
                });
            }
        }
    }
    Ok(())
}

fn parse_resets(node: &XmlNode, definitions: &Definitions) -> Result<Vec<Reset>> {
    node.children_named("reset")
        .map(|reset| {
            if reset.child("resetTypeRef").is_some() {
                return Err(Error::UnsupportedElement {
                    element: "resetTypeRef".into(),
                });
            }
            Ok(Reset {
                value: required_numeric_text(definitions, reset, "value", "field reset")?,
                mask: reset
                    .optional_child_text("mask")
                    .map(|mask| normalize_numeric_text(definitions, "field reset mask", mask))
                    .transpose()?,
                reset_type: reset.attribute_text("resetTypeRef"),
            })
        })
        .collect()
}

fn effective_field_policies<'a>(
    node: &'a XmlNode,
    definitions: &'a Definitions,
) -> Result<Vec<&'a XmlNode>> {
    let Some(policies) = node.child("fieldAccessPolicies") else {
        return Ok(Vec::new());
    };

    policies
        .children_named("fieldAccessPolicy")
        .map(|policy| {
            Ok(policy
                .child("fieldAccessPolicyDefinitionRef")
                .map(|_| {
                    definitions.field_access_policy_ref(policy, "fieldAccessPolicyDefinitionRef")
                })
                .transpose()?
                .flatten()
                .unwrap_or(policy))
        })
        .collect()
}

fn selected_policy_node<'a>(
    policies: &'a [&'a XmlNode],
    preferred_mode: Option<&str>,
    owner: Option<(&'static str, &str)>,
) -> Result<Option<&'a XmlNode>> {
    if let Some(preferred_mode) = preferred_mode {
        let selected = policies
            .iter()
            .copied()
            .filter_map(|policy| {
                matching_mode_priority(policy, preferred_mode).map(|priority| (priority, policy))
            })
            .min_by_key(|(priority, _)| *priority)
            .map(|(_, policy)| policy);
        if selected.is_some() {
            return Ok(selected);
        }
        let generic = policies
            .iter()
            .copied()
            .find(|policy| !has_mode_ref(policy));
        if generic.is_some() {
            return Ok(generic);
        }
        if let Some((kind, name)) = owner.filter(|_| !policies.is_empty()) {
            return Err(Error::AccessPolicyModeNotFound {
                kind,
                name: name.into(),
                mode: preferred_mode.into(),
            });
        }
        return Ok(None);
    }

    Ok(policies
        .iter()
        .copied()
        .find(|policy| !has_mode_ref(policy))
        .or_else(|| policies.first().copied()))
}

fn parse_enumerated_values(
    node: &XmlNode,
    definitions: &Definitions,
    parent_field_name: &str,
) -> Result<Vec<EnumeratedValue>> {
    let source = node
        .child("enumerationDefinitionRef")
        .map(|_| definitions.enumeration_ref(node, "enumerationDefinitionRef"))
        .transpose()?
        .flatten()
        .unwrap_or(node);
    let scoped_definitions = definitions.with_node_parameter_values(source, node);
    let definitions = &scoped_definitions;

    let mut values = Vec::new();
    let mut value_names = Vec::new();
    for enumerated_value in source.children_named("enumeratedValue") {
        reject_unsupported_is_present(enumerated_value)?;
        let name = enumerated_value.child_text("name")?;
        ensure_unique_ipxact_name(
            &mut value_names,
            "enumeratedValue",
            "field",
            parent_field_name,
            &name,
        )?;
        values.push(EnumeratedValue {
            name,
            value: required_numeric_text(
                definitions,
                enumerated_value,
                "value",
                "enumeratedValue value",
            )?,
        });
    }
    Ok(values)
}

fn ensure_unique_ipxact_name(
    seen: &mut Vec<String>,
    kind: &'static str,
    parent_kind: &'static str,
    parent: &str,
    name: &str,
) -> Result<()> {
    if seen.iter().any(|seen_name| seen_name == name) {
        return Err(Error::DuplicateIpXactName {
            kind,
            parent_kind,
            parent: parent.into(),
            name: name.into(),
        });
    }
    seen.push(name.into());
    Ok(())
}

fn access_handle_path_with_preferred_view<'a>(
    access_handles: &'a XmlNode,
    preferred_view: Option<&str>,
    owner: &str,
) -> Result<Option<&'a XmlNode>> {
    let selected = selected_access_handles(access_handles, preferred_view, owner)?;
    for generic in selected
        .iter()
        .copied()
        .filter(|access_handle| access_handle.child("indices").is_none())
        .filter(|access_handle| access_handle.child("viewRef").is_none())
    {
        if access_handle_path_segments(generic, owner)?.is_some() {
            return Ok(Some(generic));
        }
    }

    for path_bearing in selected
        .iter()
        .copied()
        .filter(|access_handle| access_handle.child("indices").is_none())
    {
        if access_handle_path_segments(path_bearing, owner)?.is_some() {
            return Ok(Some(path_bearing));
        }
    }

    Ok(selected
        .into_iter()
        .find(|access_handle| access_handle.child("indices").is_none()))
}

fn selected_access_handles<'a>(
    access_handles: &'a XmlNode,
    preferred_view: Option<&str>,
    owner: &str,
) -> Result<Vec<&'a XmlNode>> {
    if let Some(preferred_view) = preferred_view {
        let selected = access_handles
            .children_named("accessHandle")
            .filter(|access_handle| access_handle_has_view(access_handle, preferred_view))
            .collect::<Vec<_>>();
        if !selected.is_empty() {
            return Ok(selected);
        }

        let generic = access_handles
            .children_named("accessHandle")
            .filter(|access_handle| access_handle.child("viewRef").is_none())
            .collect::<Vec<_>>();
        if !generic.is_empty() {
            return Ok(generic);
        }

        if access_handles
            .children_named("accessHandle")
            .next()
            .is_some()
        {
            return Err(Error::AccessHandleViewNotFound {
                owner: owner.into(),
                view: preferred_view.into(),
            });
        }

        return Ok(Vec::new());
    }

    let generic = access_handles
        .children_named("accessHandle")
        .filter(|access_handle| access_handle.child("viewRef").is_none())
        .collect::<Vec<_>>();
    if !generic.is_empty() {
        return Ok(generic);
    }

    Ok(access_handles.children_named("accessHandle").collect())
}

fn access_handle_path_for_node(
    node: &XmlNode,
    definitions: &Definitions,
    owner: &str,
) -> Result<Option<String>> {
    let Some(access_handles) = node.child("accessHandles") else {
        return Ok(None);
    };
    let Some(access_handle) = access_handle_path_with_preferred_view(
        access_handles,
        definitions.options.preferred_view.as_deref(),
        owner,
    )?
    else {
        return Ok(None);
    };
    access_handle_path_segments(access_handle, owner)?
        .ok_or_else(|| Error::AccessHandlePathMissing {
            owner: owner.into(),
        })
        .map(Some)
}

fn indexed_access_handle_paths_for_node(
    node: &XmlNode,
    definitions: &Definitions,
    owner: &str,
) -> Result<Vec<IndexedHdlPath>> {
    let Some(access_handles) = node.child("accessHandles") else {
        return Ok(Vec::new());
    };

    selected_access_handles(
        access_handles,
        definitions.options.preferred_view.as_deref(),
        owner,
    )?
    .into_iter()
    .filter(|access_handle| access_handle.child("indices").is_some())
    .map(|access_handle| {
        let indices = access_handle_indices(access_handle, definitions)?.ok_or_else(|| {
            Error::AccessHandleIndicesMissing {
                owner: owner.into(),
            }
        })?;
        let path = access_handle_path_segments(access_handle, owner)?.ok_or_else(|| {
            Error::IndexedAccessHandlePathMissing {
                owner: owner.into(),
            }
        })?;
        Ok(IndexedHdlPath {
            indices,
            path,
            slices: access_handle_slices(access_handle, definitions, owner)?,
        })
    })
    .collect()
}

fn access_handle_indices(
    access_handle: &XmlNode,
    definitions: &Definitions,
) -> Result<Option<Vec<String>>> {
    let Some(indices) = access_handle.child("indices") else {
        return Ok(None);
    };
    let parsed = indices
        .children_named("index")
        .map(|index| normalize_numeric_text(definitions, "accessHandle index", index.text.clone()))
        .collect::<Result<Vec<_>>>()?;
    Ok((!parsed.is_empty()).then_some(parsed))
}

fn access_handle_path_segments(access_handle: &XmlNode, owner: &str) -> Result<Option<String>> {
    let Some(path_segments) = first_descendant(access_handle, "pathSegments") else {
        return Ok(None);
    };
    let segments = path_segments
        .children_named("pathSegment")
        .map(|segment| path_segment_text(segment, owner))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok((!segments.is_empty()).then(|| segments.join(".")))
}

fn access_handle_slices_for_node(
    node: &XmlNode,
    definitions: &Definitions,
    owner: &str,
) -> Result<Vec<HdlPathSlice>> {
    let Some(access_handles) = node.child("accessHandles") else {
        return Ok(Vec::new());
    };

    let Some(access_handle) = access_handle_path_with_preferred_view(
        access_handles,
        definitions.options.preferred_view.as_deref(),
        owner,
    )?
    else {
        return Ok(Vec::new());
    };

    access_handle_slices(access_handle, definitions, owner)
}

fn access_handle_slices(
    access_handle: &XmlNode,
    definitions: &Definitions,
    owner: &str,
) -> Result<Vec<HdlPathSlice>> {
    let Some(slices) = access_handle.child("slices") else {
        return access_handle_path_segments(access_handle, owner)?
            .map(|path| {
                Ok(vec![HdlPathSlice {
                    path,
                    left: None,
                    right: None,
                }])
            })
            .unwrap_or_else(|| {
                Err(Error::AccessHandlePathMissing {
                    owner: owner.into(),
                })
            });
    };

    slices
        .children_named("slice")
        .map(|slice| {
            let path = access_handle_path_segments(slice, owner)?.ok_or_else(|| {
                Error::AccessHandlePathMissing {
                    owner: owner.into(),
                }
            })?;
            let range = slice.child("range");
            Ok(HdlPathSlice {
                path,
                left: range
                    .and_then(|range| range.optional_child_text("left"))
                    .map(|value| {
                        normalize_numeric_text(definitions, "accessHandle slice left", value)
                    })
                    .transpose()?,
                right: range
                    .and_then(|range| range.optional_child_text("right"))
                    .map(|value| {
                        normalize_numeric_text(definitions, "accessHandle slice right", value)
                    })
                    .transpose()?,
            })
        })
        .collect()
}

fn access_handle_has_view(access_handle: &XmlNode, preferred_view: &str) -> bool {
    access_handle
        .children_named("viewRef")
        .any(|view_ref| view_ref.text.trim() == preferred_view)
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

fn required_numeric_text(
    definitions: &Definitions,
    node: &XmlNode,
    name: &'static str,
    field: &'static str,
) -> Result<String> {
    normalize_numeric_text(definitions, field, node.child_text(name)?)
}

fn normalize_numeric_text(
    definitions: &Definitions,
    field: &'static str,
    value: String,
) -> Result<String> {
    if parse_u64_expr(field, &value).is_ok() {
        return Ok(value);
    }
    if let Ok(parsed) = parse_u64_expr_with_symbols(field, &value, &definitions.parameters) {
        return Ok(parsed.to_string());
    }
    if let Some((parameter, expression)) = unsupported_parameter_reference(definitions, &value) {
        return Err(Error::UnsupportedParameterExpression {
            parameter: parameter.into(),
            field,
            expression: expression.into(),
        });
    }
    Ok(value)
}

fn normalize_optional_bool_text(
    definitions: &Definitions,
    field: &'static str,
    value: Option<String>,
) -> Result<Option<String>> {
    value
        .map(|value| normalize_bool_text(definitions, field, value))
        .transpose()
}

fn normalize_bool_text(
    definitions: &Definitions,
    field: &'static str,
    value: String,
) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.eq_ignore_ascii_case("true") {
        return Ok("true".into());
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return Ok("false".into());
    }
    if let Ok(parsed) = parse_bool_expr_with_symbols(field, trimmed, &definitions.parameters) {
        return Ok(if parsed { "true" } else { "false" }.into());
    }
    if let Some((parameter, expression)) = unsupported_parameter_reference(definitions, trimmed) {
        return Err(Error::UnsupportedParameterExpression {
            parameter: parameter.into(),
            field,
            expression: expression.into(),
        });
    }
    Err(Error::InvalidBoolean { field, value })
}

fn parse_u64_text(definitions: &Definitions, field: &'static str, value: &str) -> Result<u64> {
    parse_u64_expr(field, value)
        .or_else(|_| parse_u64_expr_with_symbols(field, value, &definitions.parameters))
        .map_err(|error| {
            unsupported_parameter_reference(definitions, value)
                .map(
                    |(parameter, expression)| Error::UnsupportedParameterExpression {
                        parameter: parameter.into(),
                        field,
                        expression: expression.into(),
                    },
                )
                .unwrap_or_else(|| Error::from(error))
        })
}

fn unsupported_parameter_reference<'a>(
    definitions: &'a Definitions,
    value: &str,
) -> Option<(&'a str, &'a str)> {
    definitions
        .unsupported_parameters
        .iter()
        .find(|(name, _)| expression_references_identifier(value, name))
        .map(|(name, expression)| (name.as_str(), expression.as_str()))
}

fn expression_references_identifier(value: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let mut start = None;
    for (index, ch) in value.char_indices() {
        if let Some(token_start) = start {
            if is_identifier_continue(ch) {
                continue;
            }
            if &value[token_start..index] == name {
                return true;
            }
            start = None;
        }
        if start.is_none() && is_identifier_start(ch) {
            start = Some(index);
        }
    }
    start
        .map(|token_start| &value[token_start..] == name)
        .unwrap_or(false)
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch == '.' || ch.is_ascii_alphanumeric()
}

fn reject_unsupported_is_present(node: &XmlNode) -> Result<()> {
    if node.child("isPresent").is_some() {
        return Err(Error::UnsupportedElement {
            element: "isPresent".into(),
        });
    }
    Ok(())
}

fn addr_text(value: u64) -> String {
    format!("0x{value:x}")
}

fn inherited_access_policy_access(
    kind: &'static str,
    name: &str,
    instance: &XmlNode,
    source: &XmlNode,
    definitions: &Definitions,
) -> Result<Option<String>> {
    if let Some(access) = access_policy_access(kind, name, instance, definitions)? {
        return Ok(Some(access));
    }
    access_policy_access(kind, name, source, definitions)
}

fn access_policy_nodes(node: &XmlNode) -> Vec<&XmlNode> {
    node.child("accessPolicies")
        .map(|policies| policies.children_named("accessPolicy").collect())
        .unwrap_or_default()
}

fn access_policy_access(
    kind: &'static str,
    name: &str,
    node: &XmlNode,
    definitions: &Definitions,
) -> Result<Option<String>> {
    let Some(policies) = node.child("accessPolicies") else {
        return Ok(None);
    };
    let policy_nodes = policies
        .children_named("accessPolicy")
        .filter(|policy| policy.child("access").is_some())
        .collect::<Vec<_>>();
    Ok(selected_policy_node(
        &policy_nodes,
        definitions.options.preferred_mode.as_deref(),
        Some((kind, name)),
    )?
    .and_then(|policy| policy.optional_child_text("access")))
}

fn has_mode_ref(node: &XmlNode) -> bool {
    node.child("modeRef").is_some()
}

fn node_has_mode(node: &XmlNode, preferred_mode: &str) -> bool {
    node.children_named("modeRef")
        .any(|mode_ref| mode_ref.text.trim() == preferred_mode)
}

fn matching_mode_priority(node: &XmlNode, preferred_mode: &str) -> Option<u64> {
    node.children_named("modeRef")
        .filter(|mode_ref| mode_ref.text.trim() == preferred_mode)
        .map(|mode_ref| {
            mode_ref
                .attribute_text("priority")
                .and_then(|priority| priority.trim().parse::<u64>().ok())
                .unwrap_or(u64::MAX)
        })
        .min()
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

fn element_dims(
    kind: &'static str,
    name: &str,
    node: &XmlNode,
    definitions: &Definitions,
) -> Result<Vec<String>> {
    if node.child("dim").is_some() {
        return Err(Error::UnsupportedElementFeature {
            kind,
            name: name.into(),
            feature: "direct dim".into(),
        });
    }

    let dims = node.child("array").map_or_else(
        || Ok(Vec::new()),
        |array| {
            array
                .children_named("dim")
                .filter(|dim| !dim.text.trim().is_empty())
                .map(|dim| {
                    let text = dim.text.trim();
                    normalize_numeric_text(definitions, "array dim", text.to_string())
                })
                .collect::<Result<Vec<_>>>()
        },
    )?;

    if dims.is_empty() {
        Ok(vec!["1".into()])
    } else {
        Ok(dims)
    }
}

fn total_dim_text(dims: &[String]) -> String {
    let mut total = 1u64;
    for dim in dims {
        let Ok(value) = parse_u64_expr("array dim", dim) else {
            return dims.join("*");
        };
        let Some(next) = total.checked_mul(value) else {
            return dims.join("*");
        };
        total = next;
    }
    total.to_string()
}

fn array_stride(node: &XmlNode, definitions: &Definitions) -> Result<Option<String>> {
    node.child("array")
        .and_then(|array| array.optional_child_text("stride"))
        .map(|stride| normalize_numeric_text(definitions, "array stride", stride))
        .transpose()
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

fn path_segment_text(node: &XmlNode, owner: &str) -> Result<Option<String>> {
    if node.child("pathSegmentName").is_some() {
        return Err(Error::UnsupportedElement {
            element: "pathSegmentName".into(),
        });
    }
    let Some(segment) = (!node.text.trim().is_empty()).then(|| node.text.trim().to_string()) else {
        return Ok(None);
    };
    let trimmed = segment.trim();
    if trimmed.starts_with('"') || trimmed.ends_with('"') {
        return Err(Error::AccessHandlePathSegmentStringLiteral {
            owner: owner.into(),
            segment,
        });
    }
    Ok(Some(trimmed.into()))
}
