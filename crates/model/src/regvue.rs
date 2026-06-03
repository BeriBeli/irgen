use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Schema {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Root {
    desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    doc: Option<String>,
    children: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    expanded: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    data_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    default_reset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct EnumValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    doc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Reset {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Field {
    name: String,
    nbits: i32,
    lsb: i32,
    access: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    reset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    r#enum: Option<EnumValue>,
    // experimental
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    repr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    custom_decode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    custom_encode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Element {
    r#type: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    version: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    fields: Option<Vec<Field>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    data_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    default_reset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Document {
    schema: Schema,
    root: Root,
    elements: HashMap<String, Element>,
}
