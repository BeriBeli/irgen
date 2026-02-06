mod excel;
mod parser;
mod schema;

use crate::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, Xlsx, open_workbook};
use polars::prelude::DataFrame;

use excel::ToDataFrame as _;
use parser::parse_register;
use schema::base::{df_to_blks, df_to_compo, df_to_regs};
pub use schema::{base, ipxact, regvue};

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

pub fn export_c_header(_output: &Path, _compo: &base::Component) -> Result<(), Error> {
    todo!()
}

pub fn export_uvm_ral(_output: &Path, _compo: &base::Component) -> Result<(), Error> {
    todo!()
}

pub fn export_sv_rtl(_output: &Path, _compo: &base::Component) -> Result<(), Error> {
    todo!()
}

pub fn export_html(_output: &Path, _compo: &base::Component) -> Result<(), Error> {
    todo!()
}
