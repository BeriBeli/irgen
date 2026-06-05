use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use calamine::{CellType, DataType, Range};

use crate::error::{Error, ValidationIssue};

#[derive(Debug, Clone)]
pub(crate) struct Row {
    number: usize,
    values: HashMap<String, String>,
}

impl Row {
    pub(crate) fn number(&self) -> usize {
        self.number
    }

    pub(crate) fn get(&self, column: &str) -> Option<&str> {
        self.values.get(column).map(String::as_str)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Table {
    sheet: String,
    headers: HashSet<String>,
    rows: Vec<Row>,
}

impl Table {
    pub(crate) fn from_range<T>(sheet: impl Into<String>, range: &Range<T>) -> Result<Self, Error>
    where
        T: DataType + CellType + Display,
    {
        let sheet = sheet.into();
        let all_rows = range.rows().collect::<Vec<_>>();
        let header_row = all_rows.first().ok_or_else(|| {
            Error::validation(&sheet, None, None, None, None, "worksheet is empty")
        })?;

        let mut headers = Vec::with_capacity(header_row.len());
        let mut seen = HashSet::new();
        let mut issues = Vec::new();
        for (index, cell) in header_row.iter().enumerate() {
            let header = cell.to_string().trim().to_owned();
            if header.is_empty() {
                issues.push(ValidationIssue::new(
                    &sheet,
                    Some(1),
                    Some(&format!("#{}", index + 1)),
                    None,
                    None,
                    "column header is empty",
                ));
                headers.push(header);
                continue;
            }
            if !seen.insert(header.clone()) {
                issues.push(ValidationIssue::new(
                    &sheet,
                    Some(1),
                    Some(&header),
                    None,
                    None,
                    "column header is duplicated",
                ));
            }
            headers.push(header);
        }
        if !issues.is_empty() {
            return Err(Error::validation_issues(issues));
        }

        let rows = all_rows
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(index, cells)| {
                let values = headers
                    .iter()
                    .zip(cells.iter())
                    .filter_map(|(header, cell)| {
                        if cell.is_empty() {
                            None
                        } else {
                            let value = cell.to_string().trim().to_owned();
                            (!value.is_empty()).then(|| (header.clone(), value))
                        }
                    })
                    .collect::<HashMap<_, _>>();
                (!values.is_empty()).then_some(Row {
                    number: index + 1,
                    values,
                })
            })
            .collect();

        Ok(Self {
            sheet,
            headers: seen,
            rows,
        })
    }

    pub(crate) fn sheet(&self) -> &str {
        &self.sheet
    }

    pub(crate) fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub(crate) fn require_columns(&self, columns: &[&str]) -> Result<(), Error> {
        let mut issues = Vec::new();
        for column in columns {
            if !self.headers.contains(*column) {
                issues.push(ValidationIssue::new(
                    &self.sheet,
                    Some(1),
                    Some(column),
                    None,
                    None,
                    "required column is missing",
                ));
            }
        }
        if !issues.is_empty() {
            return Err(Error::validation_issues(issues));
        }
        Ok(())
    }

    pub(crate) fn has_column(&self, column: &str) -> bool {
        self.headers.contains(column)
    }

    pub(crate) fn require<'a>(
        &self,
        row: &'a Row,
        column: &str,
        block: Option<&str>,
        register: Option<&str>,
    ) -> Result<&'a str, Error> {
        row.get(column).ok_or_else(|| {
            Error::validation(
                &self.sheet,
                Some(row.number()),
                Some(column),
                block,
                register,
                "required value is missing",
            )
        })
    }

    #[cfg(test)]
    pub(crate) fn for_test(sheet: &str, headers: &[&str], rows: &[&[&str]]) -> Self {
        Self {
            sheet: sheet.into(),
            headers: headers.iter().map(|header| (*header).into()).collect(),
            rows: rows
                .iter()
                .enumerate()
                .map(|(index, cells)| Row {
                    number: index + 2,
                    values: headers
                        .iter()
                        .zip(cells.iter())
                        .filter(|(_, cell)| !cell.is_empty())
                        .map(|(header, cell)| ((*header).into(), (*cell).into()))
                        .collect(),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use calamine::Data;

    use super::*;

    #[test]
    fn ignores_empty_rows_including_trailing_rows() {
        let mut range = Range::<Data>::new((0, 0), (3, 1));
        range.set_value((0, 0), Data::String("NAME".into()));
        range.set_value((0, 1), Data::String("VALUE".into()));
        range.set_value((1, 0), Data::String("entry".into()));
        range.set_value((1, 1), Data::Int(1));

        let table = Table::from_range("sheet", &range).unwrap();

        assert_eq!(table.rows().len(), 1);
        assert_eq!(table.rows()[0].number(), 2);
    }
}
