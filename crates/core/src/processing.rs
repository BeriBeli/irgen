mod excel;
mod parser;
mod schema;

use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, Xlsx, open_workbook};
use polars::prelude::DataFrame;
use tera::{Context, Tera};

use crate::assets::TemplateAssets;
use crate::config;
use crate::error::Error;
use std::collections::HashMap;

use excel::ToDataFrame as _;
use parser::parse_register;
use schema::base::{df_to_blks, df_to_compo, df_to_regs};
pub use schema::{base, ipxact, regvue};

// Initialize Tera templates
lazy_static::lazy_static! {
    static ref TERA: Result<Tera, Error> = build_tera();
}

fn build_tera() -> Result<Tera, Error> {
    let mut tera = Tera::default();
    load_user_templates(&mut tera)?;
    load_embedded_templates(&mut tera)?;
    tera.autoescape_on(vec![]);
    Ok(tera)
}

fn load_user_templates(tera: &mut Tera) -> Result<(), Error> {
    let templates_dir = config::templates_dir()?;
    if !templates_dir.exists() {
        return Ok(());
    }

    let mut template_files = Vec::new();
    collect_tera_files(&templates_dir, &mut template_files)?;
    template_files.sort();

    for template_file in template_files {
        let rel_path = template_file.strip_prefix(&templates_dir).map_err(|_| {
            Error::TemplateInitialization {
                message: format!(
                    "Failed to resolve template relative path: {}",
                    template_file.display()
                ),
            }
        })?;
        let template_name = rel_path.to_string_lossy().replace('\\', "/");
        let template_content = fs::read_to_string(&template_file)?;
        register_template(tera, &template_name, &template_content)?;
    }
    Ok(())
}

fn collect_tera_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_tera_files(&path, files)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("tera"))
        {
            files.push(path);
        }
    }
    Ok(())
}

fn load_embedded_templates(tera: &mut Tera) -> Result<(), Error> {
    for template_name in TemplateAssets::iter() {
        let template_name = template_name.as_ref();
        let template_file =
            TemplateAssets::get(template_name).ok_or_else(|| Error::TemplateInitialization {
                message: format!("Embedded template not found: {template_name}"),
            })?;
        let template_str = std::str::from_utf8(template_file.data.as_ref()).map_err(|err| {
            Error::TemplateInitialization {
                message: format!("Template is not valid UTF-8 ({template_name}): {err}"),
            }
        })?;
        register_template(tera, template_name, template_str)?;
    }
    Ok(())
}

fn register_template(
    tera: &mut Tera,
    template_name: &str,
    template_content: &str,
) -> Result<(), Error> {
    add_template_if_missing(tera, template_name, template_content)?;
    if let Some(alias_name) = Path::new(template_name)
        .file_name()
        .and_then(|s| s.to_str())
        && alias_name != template_name
    {
        add_template_if_missing(tera, alias_name, template_content)?;
    }
    Ok(())
}

fn add_template_if_missing(
    tera: &mut Tera,
    template_name: &str,
    template_content: &str,
) -> Result<(), Error> {
    if tera.get_template(template_name).is_ok() {
        return Ok(());
    }
    tera.add_raw_template(template_name, template_content)?;
    Ok(())
}

fn tera_engine() -> Result<&'static Tera, Error> {
    TERA.as_ref().map_err(|err| Error::TemplateInitialization {
        message: err.to_string(),
    })
}

fn build_template_context(compo: &base::Component) -> Result<Context, Error> {
    // Keep root fields for backward compatibility, and expose both aliases.
    let mut context = Context::from_serialize(compo)?;
    context.insert("compo", compo);
    context.insert("component", compo);
    Ok(context)
}

pub struct LoadResult {
    pub compo: base::Component,
    pub directory: PathBuf,
    pub file: PathBuf,
    pub file_size: Option<u64>,
    pub sheet_count: Option<usize>,
}

pub fn load_excel(input: &Path) -> Result<LoadResult, Error> {
    let directory = input
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let file = input.to_path_buf();
    let file_size = fs::metadata(input).map(|m| m.len()).ok();
    let mut wb: Xlsx<_> = open_workbook(input)?;

    let sheets = wb.worksheets();
    let sheet_count = Some(sheets.len());

    let mut df_map: HashMap<String, DataFrame> = sheets
        .iter()
        .map(|(sheet_name, range_data)| {
            range_data.to_data_frame().map(|df| (sheet_name.into(), df))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let compo = {
        let compo_df = df_map.remove("version").ok_or_else(|| Error::KeyNotFound {
            key: "version".into(),
        })?;

        df_to_compo(compo_df, || {
            let blks_df = df_map
                .remove("address_map")
                .ok_or_else(|| Error::KeyNotFound {
                    key: "address_map".into(),
                })?;

            df_to_blks(blks_df, |s| {
                let regs_df = df_map
                    .remove(s)
                    .ok_or_else(|| Error::KeyNotFound { key: s.into() })?;
                let parsered_df = parse_register(regs_df)?;

                df_to_regs(parsered_df)
            })
        })?
    };

    Ok(LoadResult {
        compo,
        directory,
        file,
        file_size,
        sheet_count,
    })
}

pub fn export_ipxact_xml(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let xml_str = {
        let ipxact_component = ipxact::Component::try_from(compo)?;
        quick_xml::se::to_string(&ipxact_component)?
    };

    let xml_file = output.with_extension("xml");

    fs::write(xml_file, xml_str)?;
    Ok(())
}

pub fn export_regvue_json(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let json_str = {
        let regvue_doc = regvue::Document::try_from(compo)?;
        serde_json::to_string_pretty(&regvue_doc)?
    };

    let json_file = output.with_extension("json");

    fs::write(json_file, json_str)?;
    Ok(())
}

pub fn export_c_header(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let context = build_template_context(compo)?;
    let content = tera_engine()?.render("c_header.tera", &context)?;
    let output_file = output.with_extension("h");
    fs::write(output_file, content)?;
    Ok(())
}

pub fn export_uvm_ral(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let context = build_template_context(compo)?;
    let content = tera_engine()?.render("uvm_ral.tera", &context)?;
    let output_file = output.with_extension("sv");
    fs::write(output_file, content)?;
    Ok(())
}

pub fn export_sv_rtl(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let context = build_template_context(compo)?;
    let content = tera_engine()?.render("sv_rtl.tera", &context)?;
    let output_file = output.with_extension("sv");
    fs::write(output_file, content)?;
    Ok(())
}

pub fn export_html(output: &Path, compo: &base::Component) -> Result<(), Error> {
    let context = build_template_context(compo)?;
    let content = tera_engine()?.render("html.tera", &context)?;
    let output_file = output.with_extension("html");
    fs::write(output_file, content)?;
    Ok(())
}
