use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Parser, ValueEnum, error::ErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    #[value(name = "ipxact")]
    Ipxact,
    #[value(name = "ralf")]
    Ralf,
    #[value(name = "systemrdl")]
    SystemRdl,
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Ipxact => "xml",
            Self::Ralf => "ralf",
            Self::SystemRdl => "rdl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactVersion {
    #[value(name = "2014")]
    V2014,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConvertArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub format: OutputFormat,
    pub ipxact_version: IpxactVersion,
    pub snapsheet_spec: Option<PathBuf>,
    pub validate_xsd: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Command {
    Convert(ConvertArgs),
    Help(String),
}

#[derive(Debug)]
pub enum CliError {
    Usage(String),
    Runtime(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for CliError {}

pub fn run(args: impl Iterator<Item = OsString>) -> Result<Option<PathBuf>, CliError> {
    let args = match parse_args(args).map_err(CliError::Usage)? {
        Command::Convert(args) => args,
        Command::Help(help) => {
            print!("{help}");
            return Ok(None);
        }
    };

    if args.format != OutputFormat::Ipxact && args.validate_xsd.is_some() {
        return Err(CliError::Usage(
            "--validate can only be used with --format ipxact".into(),
        ));
    }

    if let Some(schema) = &args.validate_xsd
        && !schema.is_file()
    {
        return Err(CliError::Runtime(format!(
            "validation schema not found: {}",
            schema.display()
        )));
    }

    let loaded = if let Some(spec) = &args.snapsheet_spec {
        irgen_snapsheet::load_excel_with_config_file(&args.input, spec)
    } else {
        irgen_snapsheet::load_excel(&args.input)
    }
    .map_err(|error| CliError::Runtime(error.to_string()))?;
    let output = match args.format {
        OutputFormat::Ipxact => match args.ipxact_version {
            IpxactVersion::V2014 => irgen_model::serialize_ipxact_xml(&loaded.compo)
                .map_err(|error| CliError::Runtime(error.to_string()))?,
        },
        OutputFormat::Ralf => irgen_ralf::serialize_ralf(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
        OutputFormat::SystemRdl => irgen_systemrdl::serialize_systemrdl(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    };
    fs::write(&args.output, output).map_err(|error| {
        CliError::Runtime(format!(
            "failed to write {}: {error}",
            args.output.display()
        ))
    })?;
    if let Some(schema) = args.validate_xsd {
        validate_ipxact_xml(&schema, &args.output).map_err(CliError::Runtime)?;
    }
    Ok(Some(args.output))
}

fn validate_ipxact_xml(schema: &Path, output: &Path) -> Result<(), String> {
    let result = ProcessCommand::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema)
        .arg(output)
        .output()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                "failed to run xmllint: install xmllint or omit --validate".to_string()
            } else {
                format!("failed to run xmllint: {error}")
            }
        })?;

    if result.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&result.stderr);
    Err(format!(
        "IP-XACT validation failed for {} using {}: {}",
        output.display(),
        schema.display(),
        stderr.trim()
    ))
}

#[derive(Debug, Parser)]
#[command(
    name = "irgen",
    version,
    about = "Convert a register spreadsheet into an output file."
)]
struct RawArgs {
    #[arg(value_name = "input.xlsx")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", value_name = "path")]
    output: Option<PathBuf>,

    #[arg(
        short = 'f',
        long = "format",
        value_enum,
        default_value = "ipxact",
        value_name = "name"
    )]
    format: OutputFormat,

    #[arg(long = "ipxact-version", value_enum, value_name = "version")]
    ipxact_version: Option<IpxactVersion>,

    #[arg(long = "snapsheet-spec", value_name = "snapsheet.toml")]
    snapsheet_spec: Option<PathBuf>,

    #[arg(long = "validate", value_name = "schema.xsd")]
    validate_xsd: Option<PathBuf>,
}

pub fn parse_args(args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    match RawArgs::try_parse_from(std::iter::once(OsString::from("irgen")).chain(args)) {
        Ok(raw) => convert_raw_args(raw),
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            Ok(Command::Help(error.to_string()))
        }
        Err(error) => Err(error.to_string()),
    }
}

fn convert_raw_args(raw: RawArgs) -> Result<Command, String> {
    if raw.format != OutputFormat::Ipxact && raw.ipxact_version.is_some() {
        return Err("--ipxact-version can only be used with --format ipxact".into());
    }
    let ipxact_version = raw.ipxact_version.unwrap_or(IpxactVersion::V2014);
    let output = raw
        .output
        .unwrap_or_else(|| raw.input.with_extension(raw.format.extension()));
    Ok(Command::Convert(ConvertArgs {
        input: raw.input,
        output,
        format: raw.format,
        ipxact_version,
        snapsheet_spec: raw.snapsheet_spec,
        validate_xsd: raw.validate_xsd,
    }))
}
