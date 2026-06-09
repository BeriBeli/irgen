mod attr;
pub mod config;
pub mod error;
pub mod model;

mod excel;
mod number;
mod register;
mod transform;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, open_workbook_auto};

pub use config::SnapsheetConfig;
use model::Component;

use error::Error;
use excel::Table;
use register::parse_registers;
use transform::{parse_blocks, parse_component};

pub struct LoadResult {
    pub compo: Component,
    pub directory: PathBuf,
    pub file: PathBuf,
    pub file_size: Option<u64>,
    pub sheet_count: Option<usize>,
}

pub fn load_excel(input: &Path) -> Result<LoadResult, Error> {
    load_excel_with_config(input, &SnapsheetConfig::default())
}

pub fn load_excel_with_config_file(input: &Path, config: &Path) -> Result<LoadResult, Error> {
    let config = SnapsheetConfig::from_toml_file(config)?;
    load_excel_with_config(input, &config)
}

pub fn load_excel_with_config(input: &Path, config: &SnapsheetConfig) -> Result<LoadResult, Error> {
    let directory = input
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let file = input.to_path_buf();
    let file_size = fs::metadata(input).map(|metadata| metadata.len()).ok();
    let mut workbook = open_workbook_auto(input)?;
    let sheets = workbook.worksheets();
    let sheet_count = Some(sheets.len());

    let mut tables = sheets
        .iter()
        .map(|(sheet, range)| Table::from_range(sheet, range).map(|table| (sheet.clone(), table)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    let version_sheet = &config.workbook.sheets.version;
    let version = tables
        .remove(version_sheet)
        .ok_or_else(|| Error::MissingSheet {
            sheet: version_sheet.into(),
        })?;
    let address_map_sheet = &config.workbook.sheets.address_map;
    let address_map = tables
        .remove(address_map_sheet)
        .ok_or_else(|| Error::MissingSheet {
            sheet: address_map_sheet.into(),
        })?;

    let blocks = parse_blocks(config, &address_map, |block, range| {
        let registers = tables.remove(block).ok_or_else(|| Error::MissingSheet {
            sheet: block.into(),
        })?;
        parse_registers(config, &registers, block, range)
    })?;
    let compo = parse_component(config, &version, blocks)?;

    Ok(LoadResult {
        compo,
        directory,
        file,
        file_size,
        sheet_count,
    })
}
