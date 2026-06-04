use askama::Template;
use irgen_model::base::Component;

use crate::error::Error;
use crate::view::{BlockView, DocumentView, FieldView, RegisterView};

const CSS: &str = include_str!("../assets/register_reference.css");
const JS: &str = include_str!("../assets/register_reference.js");

pub fn serialize_html(component: &Component) -> Result<String, Error> {
    let view = DocumentView::new(component)?;
    let document = HtmlDocument::from_view(&view);
    RegisterReferenceTemplate {
        document: &document,
        css: CSS,
        script: JS,
    }
    .render()
    .map_err(Error::from)
}

#[derive(Template)]
#[template(path = "register_reference.html")]
struct RegisterReferenceTemplate<'a> {
    document: &'a HtmlDocument,
    css: &'a str,
    script: &'a str,
}

#[derive(Debug)]
struct HtmlDocument {
    component_name: String,
    vendor: String,
    library: String,
    version: String,
    blocks: Vec<HtmlBlock>,
}

#[derive(Debug)]
struct HtmlBlock {
    name: String,
    anchor: String,
    offset: String,
    range: String,
    size: String,
    registers: Vec<HtmlRegister>,
}

#[derive(Debug)]
struct HtmlRegister {
    anchor: String,
    name: String,
    description: String,
    has_description: bool,
    offset: String,
    size: String,
    search_text: String,
    fields: Vec<HtmlField>,
    has_fields: bool,
}

#[derive(Debug)]
struct HtmlField {
    anchor: String,
    bits: String,
    bit_width: u64,
    name: String,
    attr: String,
    reset: String,
    description: String,
    has_description: bool,
    has_reset: bool,
}

impl HtmlDocument {
    fn from_view(view: &DocumentView<'_>) -> Self {
        Self {
            component_name: normalize_text(view.component.name()),
            vendor: normalize_text(view.component.vendor()),
            library: normalize_text(view.component.library()),
            version: normalize_text(view.component.version()),
            blocks: view.blocks.iter().map(HtmlBlock::from_view).collect(),
        }
    }
}

impl HtmlBlock {
    fn from_view(block: &BlockView<'_>) -> Self {
        Self {
            name: normalize_text(block.block.name()),
            anchor: block.anchor.clone(),
            offset: normalize_text(block.block.offset()),
            range: normalize_text(block.block.range()),
            size: format!("{} bits", block.block.size()),
            registers: block
                .registers
                .iter()
                .map(|register| HtmlRegister::from_view(block, register))
                .collect(),
        }
    }
}

impl HtmlRegister {
    fn from_view(block: &BlockView<'_>, register: &RegisterView<'_>) -> Self {
        let fields = register
            .fields
            .iter()
            .map(HtmlField::from_view)
            .collect::<Vec<_>>();
        let description = normalize_text(register.register.desc());
        Self {
            anchor: register.anchor.clone(),
            name: normalize_text(&register.display_name),
            has_description: !description.is_empty(),
            description,
            offset: normalize_text(&register.display_offset),
            size: format!("{} bits", register.register.size()),
            search_text: register_search_text(block, register),
            has_fields: !fields.is_empty(),
            fields,
        }
    }
}

impl HtmlField {
    fn from_view(field: &FieldView<'_>) -> Self {
        let description = normalize_text(field.field.desc());
        let reset = normalize_text(field.field.reset());
        Self {
            anchor: field.anchor.clone(),
            bits: field.bits(),
            bit_width: field.bit_width(),
            name: normalize_text(field.field.name()),
            attr: normalize_text(field.field.attr()),
            has_description: !description.is_empty(),
            has_reset: !reset.is_empty(),
            description,
            reset,
        }
    }
}

fn register_search_text(block: &BlockView<'_>, register: &RegisterView<'_>) -> String {
    let mut terms = vec![
        block.block.name().to_string(),
        register.display_name.clone(),
        register.display_offset.clone(),
        register.register.desc().to_string(),
    ];
    for field in &register.fields {
        terms.push(field.field.name().to_string());
        terms.push(field.field.desc().to_string());
    }
    normalize_text(&terms.join(" "))
}

fn normalize_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use irgen_model::base::{Block, Component, Field, Register, RegisterFile};

    use super::serialize_html;

    #[test]
    fn serializes_printable_register_html() {
        let component = Component::new(
            "demo".into(),
            "regs".into(),
            "example".into(),
            "1.0".into(),
            vec![Block::new(
                "csr".into(),
                "0x0".into(),
                "0x100".into(),
                "32".into(),
                vec![Register::new_with_description(
                    "STATUS_COMMAND".into(),
                    "0x4".into(),
                    "32".into(),
                    "Status and command register.".into(),
                    vec![
                        Field::new(
                            "STATUS".into(),
                            "16".into(),
                            "16".into(),
                            "RO".into(),
                            "0x0".into(),
                            "Status bits.".into(),
                        ),
                        Field::new(
                            "COMMAND".into(),
                            "0".into(),
                            "16".into(),
                            "RW".into(),
                            "0x1".into(),
                            "Command bits.".into(),
                        ),
                    ],
                )],
            )],
        );

        let html = serialize_html(&component).unwrap();
        assert!(html.contains("<!doctype html>"));
        assert!(!html.contains("Print or Save PDF"));
        assert!(html.contains("data-page=\"summary\""));
        assert!(html.contains("data-target=\"block-csr\""));
        assert!(html.contains("id=\"theme-toggle\""));
        assert!(html.contains("id=\"register-search\""));
        assert!(html.contains("id=\"register-csr-status-command\""));
        assert!(html.contains("<dd>STATUS_COMMAND</dd>"));
        assert!(html.contains("<dd>Status and command register.</dd>"));
        assert!(!html.contains("STATUS_COMMAND Register."));
        assert!(html.contains(
            "data-search=\"csr STATUS_COMMAND 0x4 Status and command register. STATUS Status bits. COMMAND Command bits.\""
        ));
        assert!(html.contains("return target;\n  }\n\n  function highlightRegister"));
        assert!(html.contains("role=\"columnheader\" style=\"--span:16\">31:16</div>"));
        assert!(html.contains("<span class=\"offset-value\">0x4</span>"));
        assert!(html.contains("<span class=\"offset-badge\">0x4</span>"));
        assert!(html.contains("<strong>Value After Reset:</strong> 0x1"));
        assert!(!html.contains("@media print"));
        assert!(!html.contains("@page"));
    }

    #[test]
    fn expands_register_files_into_register_entries() {
        let component = Component::new(
            "demo".into(),
            "regs".into(),
            "example".into(),
            "1.0".into(),
            vec![Block::new_with_register_files(
                "csr".into(),
                "0x0".into(),
                "0x100".into(),
                "32".into(),
                vec![],
                vec![RegisterFile::new(
                    "channel".into(),
                    "0x100".into(),
                    "0x10".into(),
                    "2".into(),
                    vec![Register::new(
                        "ctrl".into(),
                        "0x4".into(),
                        "32".into(),
                        vec![Field::new(
                            "ENABLE".into(),
                            "0".into(),
                            "1".into(),
                            "RW".into(),
                            "0".into(),
                            "Enable channel.".into(),
                        )],
                    )],
                )],
            )],
        );

        let html = serialize_html(&component).unwrap();
        assert!(html.contains("channel[0].ctrl"));
        assert!(html.contains("channel[1].ctrl"));
        assert!(!html.contains("channel[0].ctrl Register."));
        assert!(html.contains("0x104"));
        assert!(html.contains("<span class=\"offset-value\">0x104</span>"));
        assert!(html.contains("0x114"));
        assert!(!html.contains("<dt>Source</dt>"));
        assert!(!html.contains("channel[0] + 0x4"));
        assert!(!html.contains("Register File"));
        assert!(!html.contains("register-file-section"));
    }
}
