use std::collections::HashMap;
use std::fmt::Display;

use calamine::{CellType, DataType, Range};
use polars::prelude::*;

use crate::error::Error;

pub trait ToDataFrame {
    fn to_data_frame(&self) -> Result<DataFrame, Error>;
}

impl<T> ToDataFrame for Range<T>
where
    T: DataType + CellType + Display,
{
    fn to_data_frame(&self) -> Result<DataFrame, Error> {
        let all_rows = self.rows().collect::<Vec<_>>();

        let mut header_counts = HashMap::<String, usize>::new();
        let headers: Vec<String> = all_rows
            .first()
            .ok_or_else(|| Error::Empty {
                context: "No data".into(),
            })?
            .iter()
            .map(|cell| {
                let count = header_counts.entry(cell.to_string()).or_insert(0);
                let name = if *count > 0 {
                    format!("{}_duplicated_{}", cell, count)
                } else {
                    cell.to_string()
                };
                *count += 1;
                name
            })
            .collect();

        let mut columns: Vec<Vec<Option<String>>> = vec![vec![]; headers.len()];
        columns.iter_mut().for_each(|v| v.reserve(all_rows.len()));

        for row in &all_rows[1..] {
            row.iter().enumerate().for_each(|(col_idx, cell)| {
                let cell_str = match cell {
                    c if c.is_datetime() => c.as_datetime().map(|dt| dt.to_string()),
                    c if c.is_empty() => None,
                    _ => Some(cell.to_string()),
                };
                columns[col_idx].push(cell_str);
            });
        }

        let columns: Vec<Column> = columns
            .into_iter()
            .zip(headers)
            .map(|(col, name)| Column::new((&name).into(), col))
            .collect();

        let df = DataFrame::new(columns)?;

        Ok(df)
    }
}
