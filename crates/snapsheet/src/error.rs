/// Errors produced while reading a register spreadsheet.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    Calamine(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),

    #[error("missing worksheet `{sheet}`")]
    MissingSheet { sheet: String },

    #[error("{location}: {message}")]
    Validation { location: String, message: String },
}

impl Error {
    pub(crate) fn validation(
        sheet: &str,
        row: Option<usize>,
        column: Option<&str>,
        block: Option<&str>,
        register: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        let mut parts = vec![format!("sheet `{sheet}`")];
        if let Some(row) = row {
            parts.push(format!("row {row}"));
        }
        if let Some(column) = column {
            parts.push(format!("column `{column}`"));
        }
        if let Some(block) = block {
            parts.push(format!("block `{block}`"));
        }
        if let Some(register) = register {
            parts.push(format!("register `{register}`"));
        }
        Self::Validation {
            location: parts.join(", "),
            message: message.into(),
        }
    }
}
