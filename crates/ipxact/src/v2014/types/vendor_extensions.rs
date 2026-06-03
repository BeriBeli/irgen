//! Structured vendor-extension elements for IP-XACT 2014.

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write as _;

use quick_xml::escape::unescape;
use quick_xml::events::{BytesCData, BytesEnd, BytesRef, BytesStart, BytesText, Event};
use quick_xml::reader::{NsReader, Reader};
use quick_xml::writer::Writer;
use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

const PROTECTED_QNAME_PREFIX: &str = "_ipxact_protected_qname_";

/// Container for tool-specific extension elements.
///
/// IP-XACT permits arbitrary XML elements inside `vendorExtensions`. Keeping
/// the XML tree structured avoids injecting unescaped raw XML while preserving
/// vendor namespaces, attributes, text, and nested elements.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VendorExtensions {
    pub element: Vec<VendorExtension>,
}

impl VendorExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, element: VendorExtension) {
        self.element.push(element);
    }

    /// Parse an isolated `vendorExtensions` container while preserving qualified
    /// element and attribute names.
    ///
    /// `quick_xml`'s Serde deserializer intentionally exposes local names. Use
    /// this event-reader entry point when a vendor extension must retain its
    /// namespace prefixes across a read-modify-write cycle.
    pub fn from_xml_str(xml: &str) -> crate::Result<Self> {
        let mut reader = NsReader::from_str(xml);

        loop {
            match reader.read_event()? {
                Event::Start(start) if start.local_name().as_ref() == b"vendorExtensions" => {
                    return parse_container(&mut reader, start.name().as_ref().to_vec());
                }
                Event::Empty(start) if start.local_name().as_ref() == b"vendorExtensions" => {
                    return Err(parse_error(
                        "vendorExtensions must contain at least one extension element",
                    ));
                }
                Event::Decl(_) | Event::Comment(_) | Event::Text(_) => {}
                Event::Eof => return Err(parse_error("missing vendorExtensions container")),
                event => {
                    return Err(parse_error(format!(
                        "expected vendorExtensions container, found {event:?}"
                    )));
                }
            }
        }
    }
}

/// One arbitrary XML element inside a vendor-extension container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorExtension {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<VendorExtension>,
}

impl VendorExtension {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: BTreeMap::new(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn add_child(&mut self, child: VendorExtension) {
        self.children.push(child);
    }
}

/// Arbitrary schema extension attributes allowed by IP-XACT `any.att`.
///
/// Attribute names are stored without the leading Serde `@`. Namespace
/// declarations such as `xmlns:tool` may be included when a prefixed extension
/// attribute needs an in-scope namespace declaration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtensionAttributes {
    pub attributes: BTreeMap<String, String>,
}

impl ExtensionAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(name.into(), value.into());
    }
}

impl Serialize for ExtensionAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.attributes.len()))?;
        for (name, value) in &self.attributes {
            map.serialize_entry(&format!("@{name}"), value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for ExtensionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ExtensionAttributesVisitor)
    }
}

struct ExtensionAttributesVisitor;

impl<'de> Visitor<'de> for ExtensionAttributesVisitor {
    type Value = ExtensionAttributes;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a map of arbitrary extension attributes")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut attributes = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            let Some(name) = key.strip_prefix('@') else {
                return Err(M::Error::custom(format!(
                    "extension attribute key must be an XML attribute, found {key}"
                )));
            };
            attributes.insert(decode_protected_qname(name.into()), map.next_value()?);
        }
        Ok(ExtensionAttributes { attributes })
    }
}

impl Serialize for VendorExtensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.element.len()))?;
        for element in &self.element {
            map.serialize_entry(&element.name, &VendorExtensionContent(element))?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for VendorExtensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VendorExtensionsVisitor)
    }
}

struct VendorExtensionsVisitor;

impl<'de> Visitor<'de> for VendorExtensionsVisitor {
    type Value = VendorExtensions;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a vendorExtensions element containing arbitrary XML elements")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut elements = Vec::new();
        while let Some(name) = map.next_key::<String>()? {
            let content = map.next_value::<OwnedVendorExtensionContent>()?;
            elements.push(content.into_element(name));
        }
        Ok(VendorExtensions { element: elements })
    }
}

struct VendorExtensionContent<'a>(&'a VendorExtension);

impl Serialize for VendorExtensionContent<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let element = self.0;
        let mut map = serializer.serialize_map(Some(
            element.attributes.len() + usize::from(element.text.is_some()) + element.children.len(),
        ))?;

        for (name, value) in &element.attributes {
            map.serialize_entry(&format!("@{name}"), value)?;
        }
        if let Some(text) = &element.text {
            map.serialize_entry("$text", text)?;
        }
        for child in &element.children {
            map.serialize_entry(&child.name, &VendorExtensionContent(child))?;
        }

        map.end()
    }
}

#[derive(Debug, Default)]
struct OwnedVendorExtensionContent {
    attributes: BTreeMap<String, String>,
    text: Option<String>,
    children: Vec<VendorExtension>,
}

impl OwnedVendorExtensionContent {
    fn into_element(self, name: String) -> VendorExtension {
        VendorExtension {
            name: decode_protected_qname(name),
            attributes: self
                .attributes
                .into_iter()
                .map(|(name, value)| (decode_protected_qname(name), value))
                .collect(),
            text: self.text,
            children: self.children,
        }
    }
}

impl<'de> Deserialize<'de> for OwnedVendorExtensionContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(OwnedVendorExtensionContentVisitor)
    }
}

struct OwnedVendorExtensionContentVisitor;

impl<'de> Visitor<'de> for OwnedVendorExtensionContentVisitor {
    type Value = OwnedVendorExtensionContent;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an arbitrary XML extension element")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut content = OwnedVendorExtensionContent::default();
        while let Some(key) = map.next_key::<String>()? {
            if let Some(attribute) = key.strip_prefix('@') {
                content
                    .attributes
                    .insert(attribute.into(), map.next_value()?);
            } else if key == "$text" {
                content.text = Some(map.next_value()?);
            } else {
                let child = map.next_value::<OwnedVendorExtensionContent>()?;
                content.children.push(child.into_element(key));
            }
        }
        Ok(content)
    }
}

fn parse_container(
    reader: &mut NsReader<&[u8]>,
    end_name: Vec<u8>,
) -> crate::Result<VendorExtensions> {
    let mut elements = Vec::new();
    loop {
        match reader.read_event()? {
            Event::Start(start) => elements.push(parse_element(reader, start)?),
            Event::Empty(start) => elements.push(parse_empty_element(start)?),
            Event::End(end) if end.name().as_ref() == end_name => break,
            Event::Comment(_) | Event::Text(_) => {}
            Event::Eof => return Err(parse_error("vendorExtensions container is not closed")),
            event => {
                return Err(parse_error(format!(
                    "unsupported content inside vendorExtensions: {event:?}"
                )));
            }
        }
    }

    if elements.is_empty() {
        return Err(parse_error(
            "vendorExtensions must contain at least one extension element",
        ));
    }

    Ok(VendorExtensions { element: elements })
}

fn parse_element(
    reader: &mut NsReader<&[u8]>,
    start: BytesStart<'_>,
) -> crate::Result<VendorExtension> {
    let end_name = start.name().as_ref().to_vec();
    let mut element = extension_from_start(&start)?;

    loop {
        match reader.read_event()? {
            Event::Start(start) => element.children.push(parse_element(reader, start)?),
            Event::Empty(start) => element.children.push(parse_empty_element(start)?),
            Event::Text(text) => append_text(&mut element.text, decode_text(text)?),
            Event::CData(text) => append_text(&mut element.text, decode_cdata(text)?),
            Event::GeneralRef(reference) => {
                append_text(&mut element.text, decode_reference(reference)?)
            }
            Event::End(end) if end.name().as_ref() == end_name => return Ok(element),
            Event::Comment(_) => {}
            Event::Eof => {
                return Err(parse_error(format!(
                    "extension element {} is not closed",
                    element.name
                )));
            }
            event => {
                return Err(parse_error(format!(
                    "unsupported content inside extension element {}: {event:?}",
                    element.name
                )));
            }
        }
    }
}

fn parse_empty_element(start: BytesStart<'_>) -> crate::Result<VendorExtension> {
    extension_from_start(&start)
}

fn extension_from_start(start: &BytesStart<'_>) -> crate::Result<VendorExtension> {
    let decoder = start.decoder();
    let name = decoder
        .decode(start.name().as_ref())
        .map_err(|error| parse_error(error.to_string()))?
        .into_owned();
    let mut extension = VendorExtension::new(name);

    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| parse_error(error.to_string()))?;
        let name = decoder
            .decode(attribute.key.as_ref())
            .map_err(|error| parse_error(error.to_string()))?
            .into_owned();
        let value = attribute
            .decode_and_unescape_value(decoder)
            .map_err(|error| parse_error(error.to_string()))?
            .into_owned();
        extension.attributes.insert(name, value);
    }

    Ok(extension)
}

fn decode_text(text: BytesText<'_>) -> crate::Result<String> {
    let decoded = text
        .xml_content()
        .map_err(|error| parse_error(error.to_string()))?;
    Ok(unescape(&decoded)
        .map_err(|error| parse_error(error.to_string()))?
        .into_owned())
}

fn decode_cdata(text: BytesCData<'_>) -> crate::Result<String> {
    text.decode()
        .map(|text| text.into_owned())
        .map_err(|error| parse_error(error.to_string()))
}

fn decode_reference(reference: BytesRef<'_>) -> crate::Result<String> {
    if let Some(character) = reference
        .resolve_char_ref()
        .map_err(|error| parse_error(error.to_string()))?
    {
        return Ok(character.to_string());
    }

    match reference
        .decode()
        .map_err(|error| parse_error(error.to_string()))?
        .as_ref()
    {
        "amp" => Ok("&".into()),
        "apos" => Ok("'".into()),
        "gt" => Ok(">".into()),
        "lt" => Ok("<".into()),
        "quot" => Ok("\"".into()),
        name => Err(parse_error(format!(
            "unsupported entity reference &{name};"
        ))),
    }
}

fn append_text(text: &mut Option<String>, value: String) {
    text.get_or_insert_with(String::new).push_str(&value);
}

fn parse_error(message: impl Into<String>) -> Error {
    Error::Parse(message.into())
}

/// Protect qualified names inside all `vendorExtensions` containers before
/// using quick-xml's Serde deserializer, which intentionally exposes only local
/// names. `VendorExtensions` decodes the temporary names recursively.
pub(crate) fn protect_qnames(xml: &str) -> crate::Result<String> {
    let mut reader = Reader::from_str(xml);
    let mut writer = Writer::new(Vec::new());
    let mut extension_depth = 0usize;

    loop {
        match reader.read_event()? {
            Event::Start(start)
                if extension_depth == 0 && start.local_name().as_ref() == b"vendorExtensions" =>
            {
                writer.write_event(Event::Start(protect_extension_attributes(start)?))?;
                extension_depth = 1;
            }
            Event::Start(start) if extension_depth > 0 => {
                writer.write_event(Event::Start(protect_start(start)?))?;
                extension_depth += 1;
            }
            Event::Start(start) => {
                writer.write_event(Event::Start(protect_extension_attributes(start)?))?;
            }
            Event::End(end) if extension_depth > 1 => {
                let end_name = end.name();
                let name = reader
                    .decoder()
                    .decode(end_name.as_ref())
                    .map_err(|error| parse_error(error.to_string()))?
                    .into_owned();
                writer.write_event(Event::End(BytesEnd::new(protect_qname(&name))))?;
                extension_depth -= 1;
            }
            Event::End(end) if extension_depth == 1 => {
                writer.write_event(Event::End(end))?;
                extension_depth = 0;
            }
            Event::Empty(start) if extension_depth > 0 => {
                writer.write_event(Event::Empty(protect_start(start)?))?;
            }
            Event::Empty(start) => {
                writer.write_event(Event::Empty(protect_extension_attributes(start)?))?;
            }
            Event::Eof => break,
            event => writer.write_event(event)?,
        }
    }

    if extension_depth != 0 {
        return Err(parse_error("vendorExtensions container is not closed"));
    }

    String::from_utf8(writer.into_inner()).map_err(|error| parse_error(error.to_string()))
}

fn protect_start(start: BytesStart<'_>) -> crate::Result<BytesStart<'static>> {
    let decoder = start.decoder();
    let start_name = start.name();
    let name = decoder
        .decode(start_name.as_ref())
        .map_err(|error| parse_error(error.to_string()))?
        .into_owned();
    let mut protected = BytesStart::new(protect_qname(&name));

    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| parse_error(error.to_string()))?;
        let name = decoder
            .decode(attribute.key.as_ref())
            .map_err(|error| parse_error(error.to_string()))?;
        let value = attribute
            .decode_and_unescape_value(decoder)
            .map_err(|error| parse_error(error.to_string()))?;
        let name = if name == "xmlns" || name.starts_with("xmlns:") {
            name.into_owned()
        } else {
            protect_qname(&name)
        };
        protected.push_attribute((name.as_str(), value.as_ref()));
    }

    Ok(protected)
}

fn protect_extension_attributes(start: BytesStart<'_>) -> crate::Result<BytesStart<'static>> {
    let decoder = start.decoder();
    let start_name = start.name();
    let name = decoder
        .decode(start_name.as_ref())
        .map_err(|error| parse_error(error.to_string()))?
        .into_owned();
    let mut protected = BytesStart::new(name);

    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| parse_error(error.to_string()))?;
        let name = decoder
            .decode(attribute.key.as_ref())
            .map_err(|error| parse_error(error.to_string()))?;
        let value = attribute
            .decode_and_unescape_value(decoder)
            .map_err(|error| parse_error(error.to_string()))?;
        let name = if should_protect_extension_attribute(&name) {
            protect_qname(&name)
        } else {
            name.into_owned()
        };
        protected.push_attribute((name.as_str(), value.as_ref()));
    }

    Ok(protected)
}

fn should_protect_extension_attribute(name: &str) -> bool {
    name.contains(':')
        && !name.starts_with("xmlns:")
        && !name.starts_with("xml:")
        && !name.starts_with("xsi:")
}

fn protect_qname(name: &str) -> String {
    let mut protected = String::from(PROTECTED_QNAME_PREFIX);
    for byte in name.as_bytes() {
        write!(protected, "{byte:02x}").expect("writing to a String should not fail");
    }
    protected
}

fn decode_protected_qname(name: String) -> String {
    let Some(encoded) = name.strip_prefix(PROTECTED_QNAME_PREFIX) else {
        return name;
    };
    if encoded.is_empty() || encoded.len() % 2 != 0 {
        return name;
    }

    let decoded = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| {
            std::str::from_utf8(chunk)
                .ok()
                .and_then(|chunk| u8::from_str_radix(chunk, 16).ok())
        })
        .collect::<Option<Vec<_>>>();
    decoded
        .and_then(|decoded| String::from_utf8(decoded).ok())
        .unwrap_or(name)
}
