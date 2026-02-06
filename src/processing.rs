mod excel;
mod parser;
mod schema;

use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, Xlsx, open_workbook};
use polars::prelude::DataFrame;
use tera::{Context, Tera};

use crate::assets::TemplateAssets;
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

    for template_name in TemplateAssets::iter() {
        let template_name = template_name.as_ref();
        let template_file = TemplateAssets::get(template_name).ok_or_else(|| {
            Error::TemplateInitialization {
                message: format!("Embedded template not found: {template_name}"),
            }
        })?;
        let template_str = std::str::from_utf8(template_file.data.as_ref()).map_err(|err| {
            Error::TemplateInitialization {
                message: format!("Template is not valid UTF-8 ({template_name}): {err}"),
            }
        })?;

        tera.add_raw_template(template_name, template_str)?;

        if let Some(alias_name) = Path::new(template_name).file_name().and_then(|s| s.to_str())
            && alias_name != template_name
            && tera.get_template(alias_name).is_err()
        {
            tera.add_raw_template(alias_name, template_str)?;
        }
    }

    tera.autoescape_on(vec![]);
    Ok(tera)
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
