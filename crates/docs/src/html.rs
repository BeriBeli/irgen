use askama::Template;
use irgen_model::base::Component;
use serde::Serialize;

use crate::error::Error;
use crate::view::{BlockView, DocumentView, FieldView, RegisterSource, RegisterView};

const CSS: &str = include_str!("../assets/register_reference.css");
const JS: &str = include_str!("../assets/register_reference.js");

#[derive(Debug)]
pub struct HtmlSite {
    pub index: String,
    pub pages: Vec<HtmlPage>,
}

#[derive(Debug)]
pub struct HtmlPage {
    pub filename: String,
    pub content: String,
}

pub fn serialize_html(component: &Component) -> Result<String, Error> {
    Ok(serialize_html_site(component, ".", "index.html")?.index)
}

pub fn serialize_html_site(
    component: &Component,
    asset_dir: &str,
    index_file: &str,
) -> Result<HtmlSite, Error> {
    let mut pages = Vec::new();
    let index = serialize_html_site_stream(component, asset_dir, index_file, |page| {
        pages.push(page);
        Ok(())
    })?;
    Ok(HtmlSite { index, pages })
}

pub fn serialize_html_site_stream<F>(
    component: &Component,
    asset_dir: &str,
    index_file: &str,
    mut write_page: F,
) -> Result<String, Error>
where
    F: FnMut(HtmlPage) -> Result<(), Error>,
{
    let view = DocumentView::new(component)?;
    let index_prefix = href_prefix(asset_dir);
    let index_css_href = format!("{index_prefix}assets/register_reference.css");
    let index_script_href = format!("{index_prefix}assets/register_reference.js");
    let index_document = HtmlDocument::from_view(&view, &index_prefix, false);
    let block_document = HtmlDocument::from_view(&view, "", false);
    let register_document = HtmlDocument::from_view(&view, "../", false);
    let (index_href, nested_index_href) = if index_prefix.is_empty() {
        (index_file.to_string(), format!("../{index_file}"))
    } else {
        (format!("../{index_file}"), format!("../../{index_file}"))
    };
    write_page(HtmlPage {
        filename: "assets/register_reference.css".into(),
        content: CSS.into(),
    })?;
    write_page(HtmlPage {
        filename: "assets/register_reference.js".into(),
        content: JS.into(),
    })?;
    let index = RegisterReferenceTemplate {
        document: &index_document,
        css_href: &index_css_href,
        script_href: &index_script_href,
    }
    .render()
    .map_err(Error::from)?;
    for block in &block_document.blocks {
        let content = BlockReferenceTemplate {
            document: &block_document,
            block,
            index_href: &index_href,
            css_href: "assets/register_reference.css",
            script_href: "assets/register_reference.js",
        }
        .render()
        .map_err(Error::from)?;
        write_page(HtmlPage {
            filename: block.filename.clone(),
            content,
        })?;
    }
    for (source_block, nav_block) in view.blocks.iter().zip(register_document.blocks.iter()) {
        for source_register in &source_block.registers {
            let register = HtmlRegister::from_view(
                &source_block.anchor,
                "../",
                source_block,
                source_register,
                true,
            );
            let content = RegisterDetailTemplate {
                document: &register_document,
                block: nav_block,
                register: &register,
                index_href: &nested_index_href,
                css_href: "../assets/register_reference.css",
                script_href: "../assets/register_reference.js",
            }
            .render()
            .map_err(Error::from)?;
            write_page(HtmlPage {
                filename: register.filename.clone(),
                content,
            })?;
        }
    }

    Ok(index)
}

fn href_prefix(asset_dir: &str) -> String {
    let trimmed = asset_dir.trim_matches('/');
    if trimmed.is_empty() || trimmed == "." {
        String::new()
    } else {
        format!("{trimmed}/")
    }
}

#[derive(Template)]
#[template(path = "register_reference.html")]
struct RegisterReferenceTemplate<'a> {
    document: &'a HtmlDocument,
    css_href: &'a str,
    script_href: &'a str,
}

#[derive(Template)]
#[template(path = "block_reference.html")]
struct BlockReferenceTemplate<'a> {
    document: &'a HtmlDocument,
    block: &'a HtmlBlock,
    index_href: &'a str,
    css_href: &'a str,
    script_href: &'a str,
}

#[derive(Template)]
#[template(path = "register_detail.html")]
struct RegisterDetailTemplate<'a> {
    document: &'a HtmlDocument,
    block: &'a HtmlBlock,
    register: &'a HtmlRegister,
    index_href: &'a str,
    css_href: &'a str,
    script_href: &'a str,
}

#[derive(Debug)]
struct HtmlDocument {
    component_name: String,
    vendor: String,
    library: String,
    version: String,
    blocks: Vec<HtmlBlock>,
    search_index_json: String,
}

#[derive(Debug)]
struct HtmlBlock {
    name: String,
    anchor: String,
    filename: String,
    href: String,
    offset: String,
    range: String,
    size: String,
    registers: Vec<HtmlRegister>,
}

#[derive(Debug)]
struct HtmlRegister {
    anchor: String,
    filename: String,
    href: String,
    display_name: String,
    description: String,
    has_description: bool,
    offset: String,
    size: String,
    source: HtmlRegisterSource,
    search_text: String,
    fields: Vec<HtmlField>,
    has_fields: bool,
}

#[derive(Debug)]
struct HtmlRegisterSource {
    is_register_file: bool,
    dim: String,
    stride: String,
    offset_formula: String,
    index_range: String,
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

#[derive(Debug, Serialize)]
struct SearchEntry<'a> {
    page: &'a str,
    anchor: &'a str,
    href: String,
    name: &'a str,
    offset: &'a str,
    search: &'a str,
}

impl HtmlDocument {
    fn from_view(view: &DocumentView<'_>, href_prefix: &str, include_fields: bool) -> Self {
        let blocks = view
            .blocks
            .iter()
            .map(|block| HtmlBlock::from_view(block, href_prefix, include_fields))
            .collect::<Vec<_>>();
        let search_index_json = json_for_script(&search_entries(&blocks));

        Self {
            component_name: normalize_text(view.component.name()),
            vendor: normalize_text(view.component.vendor()),
            library: normalize_text(view.component.library()),
            version: normalize_text(view.component.version()),
            blocks,
            search_index_json,
        }
    }
}

impl HtmlBlock {
    fn from_view(block: &BlockView<'_>, href_prefix: &str, include_fields: bool) -> Self {
        let anchor = block.anchor.clone();
        let filename = format!("{anchor}.html");
        let registers = block
            .registers
            .iter()
            .map(|register| {
                HtmlRegister::from_view(&anchor, href_prefix, block, register, include_fields)
            })
            .collect();
        Self {
            name: normalize_text(block.block.name()),
            anchor,
            href: format!("{href_prefix}{filename}"),
            filename,
            offset: normalize_text(block.block.offset()),
            range: normalize_text(block.block.range()),
            size: format!("{} bits", block.block.size()),
            registers,
        }
    }
}

impl HtmlRegister {
    fn from_view(
        block_anchor: &str,
        href_prefix: &str,
        block: &BlockView<'_>,
        register: &RegisterView<'_>,
        include_fields: bool,
    ) -> Self {
        let fields = if include_fields {
            register
                .fields
                .iter()
                .map(HtmlField::from_view)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let description = normalize_text(register.register.desc());
        let filename = format!("{block_anchor}/{}.html", register.anchor);
        let has_fields = !register.fields.is_empty();
        let source = HtmlRegisterSource::from_view(&register.source);
        let display_name = display_register_name(register);
        Self {
            anchor: register.anchor.clone(),
            href: format!("{href_prefix}{filename}"),
            filename,
            display_name,
            has_description: !description.is_empty(),
            description,
            offset: normalize_text(&register.display_offset),
            size: format!("{} bits", register.register.size()),
            source,
            search_text: register_search_text(block, register),
            has_fields,
            fields,
        }
    }
}

impl HtmlRegisterSource {
    fn from_view(source: &RegisterSource) -> Self {
        match source {
            RegisterSource::Direct => Self {
                is_register_file: false,
                dim: String::new(),
                stride: String::new(),
                offset_formula: String::new(),
                index_range: String::new(),
            },
            RegisterSource::RegisterFile {
                dim,
                stride,
                base_offset,
                child_offset,
                ..
            } => {
                let dim = normalize_text(dim);
                let stride = normalize_text(stride);
                let base_offset = normalize_text(base_offset);
                let child_offset = normalize_text(child_offset);
                Self {
                    is_register_file: true,
                    offset_formula: offset_formula(&base_offset, &stride, &child_offset),
                    index_range: index_range(&dim),
                    dim,
                    stride,
                }
            }
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
        display_register_name(register),
        register.display_offset.clone(),
        register.register.desc().to_string(),
    ];
    for field in &register.fields {
        terms.push(field.field.name().to_string());
    }
    normalize_text(&terms.join(" "))
}

fn search_entries(blocks: &[HtmlBlock]) -> Vec<SearchEntry<'_>> {
    blocks.iter().flat_map(search_entries_for_block).collect()
}

fn search_entries_for_block(block: &HtmlBlock) -> Vec<SearchEntry<'_>> {
    block
        .registers
        .iter()
        .map(|register| SearchEntry {
            page: &block.anchor,
            anchor: &register.anchor,
            href: register.href.clone(),
            name: &register.display_name,
            offset: &register.offset,
            search: &register.search_text,
        })
        .collect()
}

fn json_for_script<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("HTML support data should serialize")
        .replace("</", "<\\/")
}

fn normalize_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn display_register_name(register: &RegisterView<'_>) -> String {
    match &register.source {
        RegisterSource::Direct => normalize_text(&register.display_name),
        RegisterSource::RegisterFile { .. } => normalize_text(register.register.name()),
    }
}

fn offset_formula(base_offset: &str, stride: &str, child_offset: &str) -> String {
    if child_offset == "0" || child_offset == "0x0" {
        format!("{base_offset} + i * {stride}")
    } else {
        format!("{base_offset} + i * {stride} + {child_offset}")
    }
}

fn index_range(dim: &str) -> String {
    dim.parse::<u64>()
        .ok()
        .and_then(|dim| dim.checked_sub(1))
        .map(|last| format!("i = 0..{last}"))
        .unwrap_or_else(|| "i = 0..count-1".to_string())
}

#[cfg(test)]
mod tests {
    use irgen_model::base::{Block, Component, Field, Register, RegisterFile};

    use super::serialize_html_site;

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

        let site = serialize_html_site(&component, "example_files", "example.html").unwrap();
        let html = &site.index;
        let block_page = &site.pages[2].content;
        let register_page = &site.pages[3].content;
        assert!(html.contains("<!doctype html>"));
        assert!(html.contains("href=\"example_files/assets/register_reference.css\""));
        assert!(html.contains("src=\"example_files/assets/register_reference.js\""));
        assert!(!html.contains("Print or Save PDF"));
        assert!(html.contains("data-page=\"summary\""));
        assert!(html.contains("href=\"example_files/block-csr.html\""));
        assert!(html.contains("id=\"theme-toggle\""));
        assert!(html.contains("id=\"register-search\""));
        assert!(html.contains("id=\"register-search-index\""));
        assert!(
            html.contains("\"href\":\"example_files/block-csr/register-csr-status-command.html\"")
        );
        assert!(html.contains(
            "\"search\":\"csr STATUS_COMMAND 0x4 Status and command register. STATUS COMMAND\""
        ));
        assert!(!html.contains("data-search=\"csr STATUS_COMMAND"));
        assert!(!html.contains("Fields for Register: STATUS_COMMAND"));
        assert_eq!(site.pages[0].filename, "assets/register_reference.css");
        assert_eq!(site.pages[1].filename, "assets/register_reference.js");
        assert_eq!(site.pages[2].filename, "block-csr.html");
        assert_eq!(
            site.pages[3].filename,
            "block-csr/register-csr-status-command.html"
        );
        assert!(block_page.contains("href=\"assets/register_reference.css\""));
        assert!(block_page.contains("href=\"../example.html\""));
        assert!(block_page.contains("href=\"block-csr/register-csr-status-command.html\""));
        assert!(block_page.contains(
            "<td><a href=\"block-csr/register-csr-status-command.html\">STATUS_COMMAND</a></td>"
        ));
        assert!(!block_page.contains("Fields for Register: STATUS_COMMAND"));
        assert!(register_page.contains("href=\"../assets/register_reference.css\""));
        assert!(!register_page.contains("id=\"register-search\""));
        assert!(!register_page.contains("id=\"register-search-index\""));
        assert!(register_page.contains("href=\"../../example.html\""));
        assert!(register_page.contains("id=\"register-csr-status-command\""));
        assert!(register_page.contains("<dd>STATUS_COMMAND</dd>"));
        assert!(register_page.contains("<dd>Status and command register.</dd>"));
        assert!(!register_page.contains("STATUS_COMMAND Register."));
        assert!(register_page.contains("role=\"columnheader\" style=\"--span:16\">31:16</div>"));
        assert!(register_page.contains("<span class=\"offset-value\">0x4</span>"));
        assert!(register_page.contains("<span class=\"offset-badge\">0x4</span>"));
        assert!(register_page.contains("<strong>Value After Reset:</strong> 0x1"));
        assert!(!html.contains("@media print"));
        assert!(!html.contains("@page"));
    }

    #[test]
    fn block_pages_use_global_register_search_index() {
        let component = Component::new(
            "demo".into(),
            "regs".into(),
            "example".into(),
            "1.0".into(),
            vec![
                Block::new(
                    "csr".into(),
                    "0x0".into(),
                    "0x100".into(),
                    "32".into(),
                    vec![Register::new(
                        "CTRL".into(),
                        "0x0".into(),
                        "32".into(),
                        vec![],
                    )],
                ),
                Block::new(
                    "cfg".into(),
                    "0x100".into(),
                    "0x100".into(),
                    "32".into(),
                    vec![Register::new(
                        "MODE".into(),
                        "0x0".into(),
                        "32".into(),
                        vec![],
                    )],
                ),
            ],
        );

        let site = serialize_html_site(&component, "example_files", "example.html").unwrap();
        let csr_page = &site.pages[2].content;
        assert!(csr_page.contains("placeholder=\"Search registers\""));
        assert!(!csr_page.contains("Search this block"));
        assert!(csr_page.contains("\"name\":\"CTRL\""));
        assert!(csr_page.contains("\"name\":\"MODE\""));
        assert!(csr_page.contains("\"href\":\"block-cfg/register-cfg-mode.html\""));
    }

    #[test]
    fn keeps_register_files_unexpanded_in_register_docs() {
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

        let site = serialize_html_site(&component, "example_files", "example.html").unwrap();
        let html = &site.pages[2].content;
        let compact_html = compact_html(html);
        let register_page = &site.pages[3].content;
        assert!(!html.contains("channel[2].ctrl"));
        assert!(!html.contains("channel[0].ctrl"));
        assert!(!html.contains("channel[1].ctrl"));
        assert!(!html.contains("channel[].ctrl"));
        assert!(!html.contains("channel[2].ctrl Register."));
        assert!(html.contains("0x104"));
        assert!(html.contains("<span class=\"offset-value\">0x104</span>"));
        assert!(!html.contains("0x114"));
        assert!(html.contains("<th>Count</th>"));
        assert!(html.contains("<th>Stride</th>"));
        assert!(html.contains("\"name\":\"ctrl\""));
        assert!(html.contains("\"search\":\"csr ctrl 0x104 ENABLE\""));
        assert!(!html.contains("\"search\":\"csr channel"));
        assert!(
            compact_html.contains(
                "<td><ahref=\"block-csr/register-csr-channel-2-ctrl.html\">ctrl</a></td>"
            )
        );
        assert!(compact_html.contains("<td>2</td>"));
        assert!(compact_html.contains("<td>0x10</td>"));
        assert!(!html.contains("<dt>Source</dt>"));
        assert!(!html.contains("channel[2] + 0x4"));
        assert!(!html.contains("Register File"));
        assert!(!html.contains("register-file-section"));
        assert!(register_page.contains("<h3>ctrl</h3>"));
        assert!(register_page.contains("<dd>ctrl</dd>"));
        assert!(!register_page.contains("<h3>channel[2].ctrl</h3>"));
        assert!(register_page.contains("<dt>Instances</dt>"));
        assert!(register_page.contains("<dd>2</dd>"));
        assert!(!register_page.contains("<dt>Array</dt>"));
        assert!(!register_page.contains("<dd>channel[2]</dd>"));
        assert!(register_page.contains("<dt>Offset Formula</dt>"));
        assert!(register_page.contains("offset(i) = 0x100 + i * 0x10 + 0x4"));
        assert!(register_page.contains("i = 0..1"));
        assert!(!register_page.contains("<dt>Register File Offset</dt>"));
        assert!(!register_page.contains("<dt>Child Offset</dt>"));
        let register_pages = site
            .pages
            .iter()
            .filter(|page| page.filename.starts_with("block-csr/register-"))
            .count();
        assert_eq!(register_pages, 1);
    }

    fn compact_html(html: &str) -> String {
        html.split_whitespace().collect::<String>()
    }
}
