use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Parser, ValueEnum, error::ErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    #[value(name = "all")]
    All,
    #[value(name = "html")]
    Html,
    #[value(name = "ipxact")]
    Ipxact,
    #[value(name = "ralf")]
    Ralf,
    #[value(name = "systemrdl")]
    SystemRdl,
}

impl OutputFormat {
    fn file_extension(self) -> Option<&'static str> {
        match self {
            Self::All => None,
            Self::Html => None,
            Self::Ipxact => Some("xml"),
            Self::Ralf => Some("ralf"),
            Self::SystemRdl => Some("rdl"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactVersion {
    #[value(name = "2009")]
    V2009,
    #[value(name = "2014")]
    V2014,
    #[value(name = "2022")]
    V2022,
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
    if args.format == OutputFormat::Html {
        write_html_output(&loaded.compo, &args.output)?;
        return Ok(Some(args.output));
    }
    if args.format == OutputFormat::All {
        write_all_outputs(&loaded.compo, &args.output)?;
        return Ok(Some(args.output));
    }

    let output = match args.format {
        OutputFormat::All => unreachable!("ALL output is handled before string serialization"),
        OutputFormat::Html => unreachable!("HTML output is handled before string serialization"),
        OutputFormat::Ipxact => serialize_ipxact(&loaded.compo, args.ipxact_version)?,
        OutputFormat::Ralf => irgen_ralf::serialize_ralf(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
        OutputFormat::SystemRdl => irgen_systemrdl::serialize_systemrdl(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    };
    write_text_output(&args.output, output)?;
    if let Some(schema) = args.validate_xsd {
        validate_ipxact_xml(&schema, &args.output).map_err(CliError::Runtime)?;
    }
    Ok(Some(args.output))
}

fn write_all_outputs(
    component: &irgen_model::base::Component,
    output: &Path,
) -> Result<(), CliError> {
    fs::create_dir_all(output).map_err(|error| {
        CliError::Runtime(format!(
            "failed to create output directory {}: {error}",
            output.display()
        ))
    })?;

    write_text_output(
        &output.join("ipxact-2009.xml"),
        serialize_ipxact(component, IpxactVersion::V2009)?,
    )?;
    write_text_output(
        &output.join("ipxact-2014.xml"),
        serialize_ipxact(component, IpxactVersion::V2014)?,
    )?;
    write_text_output(
        &output.join("ipxact-2022.xml"),
        serialize_ipxact(component, IpxactVersion::V2022)?,
    )?;
    write_text_output(
        &output.join("ralf.ralf"),
        irgen_ralf::serialize_ralf(component)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    )?;
    write_text_output(
        &output.join("systemrdl.rdl"),
        irgen_systemrdl::serialize_systemrdl(component)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    )?;
    write_html_output(component, &output.join("html"))?;
    Ok(())
}

fn serialize_ipxact(
    component: &irgen_model::base::Component,
    version: IpxactVersion,
) -> Result<String, CliError> {
    match version {
        IpxactVersion::V2009 => irgen_model::serialize_ipxact_2009_xml(component)
            .map_err(|error| CliError::Runtime(error.to_string())),
        IpxactVersion::V2014 => irgen_model::serialize_ipxact_xml(component)
            .map_err(|error| CliError::Runtime(error.to_string())),
        IpxactVersion::V2022 => irgen_model::serialize_ipxact_2022_xml(component)
            .map_err(|error| CliError::Runtime(error.to_string())),
    }
}

fn write_html_output(
    component: &irgen_model::base::Component,
    output: &Path,
) -> Result<(), CliError> {
    fs::create_dir_all(output).map_err(|error| {
        CliError::Runtime(format!(
            "failed to create HTML output directory {}: {error}",
            output.display()
        ))
    })?;
    let index = irgen_docs::serialize_html_site_stream(component, ".", "index.html", |page| {
        let path = output.join(&page.filename);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                irgen_docs::Error::WritePage(format!(
                    "failed to create HTML output directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        fs::write(&path, page.content).map_err(|error| {
            irgen_docs::Error::WritePage(format!("failed to write {}: {error}", path.display()))
        })
    })
    .map_err(|error| CliError::Runtime(error.to_string()))?;
    let index_path = output.join("index.html");
    fs::write(&index_path, index).map_err(|error| {
        CliError::Runtime(format!("failed to write {}: {error}", index_path.display()))
    })?;
    Ok(())
}

fn default_output_path(input: &Path, format: OutputFormat) -> PathBuf {
    let stem = input.file_stem().unwrap_or(input.as_os_str());
    match format.file_extension() {
        Some(extension) => PathBuf::from(stem).with_extension(extension),
        None => PathBuf::from(stem),
    }
}

fn write_text_output(output: &Path, content: String) -> Result<(), CliError> {
    fs::write(output, content).map_err(|error| {
        CliError::Runtime(format!("failed to write {}: {error}", output.display()))
    })
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
    about = "Convert a register spreadsheet into an output path."
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
        .unwrap_or_else(|| default_output_path(&raw.input, raw.format));
    Ok(Command::Convert(ConvertArgs {
        input: raw.input,
        output,
        format: raw.format,
        ipxact_version,
        snapsheet_spec: raw.snapsheet_spec,
        validate_xsd: raw.validate_xsd,
    }))
}
