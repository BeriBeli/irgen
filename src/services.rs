mod excel;
mod parser;
mod schema;

use crate::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use calamine::{Reader, Xlsx, open_workbook};
use polars::prelude::DataFrame;

use crate::state::AppState;
use excel::ToDataFrame as _;
use parser::parse_register;
use schema::base::{df_to_blks, df_to_compo, df_to_regs};
pub use schema::{base, ipxact, regvue};

pub fn load_excel(input: &Path, state: Arc<AppState>) -> Result<(), Error> {
    let directory = input.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
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
        let compo_df = df_map
            .remove("version")
            .ok_or_else(|| Error::KeyNotFound { key: "version".into() })?;

        df_to_compo(compo_df, || {
            let blks_df = df_map
                .remove("address_map")
                .ok_or_else(|| Error::KeyNotFound { key: "address_map".into() })?;

            df_to_blks(blks_df, |s| {
                let regs_df = df_map.remove(s).ok_or_else(|| Error::KeyNotFound { key: s.into() })?;
                let parsered_df = parse_register(regs_df)?;

                df_to_regs(parsered_df)
            })
        })?
    };

    state.set_file_metadata(file_size, sheet_count);
    state.load_component(compo, directory, file);
    Ok(())
}

pub fn export_ipxact_xml(output: &Path, state: Arc<AppState>) -> Result<(), Error> {
    let xml_str = {
        let guard = state.component_guard();
        let compo = guard
            .as_ref()
            .ok_or_else(|| Error::NotLoaded { context: "Component not loaded".into() })?;

        let ipxact_component = ipxact::Component::try_from(compo)?;
        quick_xml::se::to_string(&ipxact_component)?
    };

    let xml_file = output.with_extension("xml");

    fs::write(xml_file, xml_str)?;
    Ok(())
}

pub fn export_regvue_json(output: &Path, state: Arc<AppState>) -> Result<(), Error> {
    let json_str = {
        let guard = state.component_guard();
        let compo = guard
            .as_ref()
            .ok_or_else(|| Error::NotLoaded { context: "Component not loaded".into() })?;

        let regvue_doc = regvue::Document::try_from(compo)?;
        serde_json::to_string_pretty(&regvue_doc)?
    };

    let json_file = output.with_extension("json");

    fs::write(json_file, json_str)?;
    Ok(())
}
