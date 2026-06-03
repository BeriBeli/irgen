mod excel;
mod number;
mod register;
mod transform;

pub mod error;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, Xlsx, open_workbook};
use irgen_model::base::Component;

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
    let directory = input
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let file = input.to_path_buf();
    let file_size = fs::metadata(input).map(|metadata| metadata.len()).ok();
    let mut workbook: Xlsx<_> = open_workbook(input)?;
    let sheets = workbook.worksheets();
    let sheet_count = Some(sheets.len());

    let mut tables = sheets
        .iter()
        .map(|(sheet, range)| Table::from_range(sheet, range).map(|table| (sheet.clone(), table)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    let version = tables
        .remove("version")
        .ok_or_else(|| Error::MissingSheet {
            sheet: "version".into(),
        })?;
    let address_map = tables
        .remove("address_map")
        .ok_or_else(|| Error::MissingSheet {
            sheet: "address_map".into(),
        })?;

    let blocks = parse_blocks(&address_map, |block, range| {
        let registers = tables.remove(block).ok_or_else(|| Error::MissingSheet {
            sheet: block.into(),
        })?;
        parse_registers(&registers, block, range)
    })?;
    let compo = parse_component(&version, blocks)?;

    Ok(LoadResult {
        compo,
        directory,
        file,
        file_size,
        sheet_count,
    })
}
