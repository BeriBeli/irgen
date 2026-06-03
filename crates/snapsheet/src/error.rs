use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    location: String,
    message: String,
}

impl ValidationIssue {
    pub(crate) fn new(
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
        Self {
            location: parts.join(", "),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.location, self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssues {
    issues: Vec<ValidationIssue>,
}

impl ValidationIssues {
    pub(crate) fn new(issues: Vec<ValidationIssue>) -> Self {
        Self { issues }
    }

    pub(crate) fn into_vec(self) -> Vec<ValidationIssue> {
        self.issues
    }
}

impl std::fmt::Display for ValidationIssues {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.issues.len() == 1 {
            return write!(formatter, "{}", self.issues[0]);
        }

        writeln!(formatter, "{} validation errors:", self.issues.len())?;
        for issue in &self.issues {
            writeln!(formatter, "  - {issue}")?;
        }
        Ok(())
    }
}

/// Errors produced while reading a register spreadsheet.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Calamine error: {0}")]
    Calamine(#[from] calamine::Error),

    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),

    #[error("missing worksheet `{sheet}`")]
    MissingSheet { sheet: String },

    #[error("failed to read snapsheet config {}: {source}", path.display())]
    ReadConfig {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse snapsheet config {}: {source}", path.display())]
    ParseConfig {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("invalid snapsheet config {}: {message}", path.display())]
    Config { path: PathBuf, message: String },

    #[error("{0}")]
    Validation(ValidationIssue),

    #[error("{0}")]
    ValidationIssues(ValidationIssues),
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
        Self::Validation(ValidationIssue::new(
            sheet, row, column, block, register, message,
        ))
    }

    pub(crate) fn validation_issues(issues: Vec<ValidationIssue>) -> Self {
        Self::ValidationIssues(ValidationIssues::new(issues))
    }

    pub(crate) fn into_validation_issues(self) -> Result<Vec<ValidationIssue>, Self> {
        match self {
            Self::Validation(issue) => Ok(vec![issue]),
            Self::ValidationIssues(issues) => Ok(issues.into_vec()),
            error => Err(error),
        }
    }
}
