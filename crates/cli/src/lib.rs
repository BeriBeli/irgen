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
    #[value(name = "1.4")]
    V1_4,
    #[value(name = "1.5")]
    V1_5,
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
    pub output: Option<PathBuf>,
    pub format: OutputFormat,
    pub ipxact_version: IpxactVersion,
    pub snapsheet_spec: Option<PathBuf>,
    pub validate_xsd: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IpxactArgs {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub coverage: bool,
}

#[derive(Debug)]
pub enum Command {
    Convert(ConvertArgs),
    Ipxact(IpxactArgs),
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
    match parse_args(args).map_err(CliError::Usage)? {
        Command::Convert(args) => run_convert(args),
        Command::Ipxact(args) => run_ipxact(args),
        Command::Help(help) => {
            print!("{help}");
            Ok(None)
        }
    }
}

fn run_convert(args: ConvertArgs) -> Result<Option<PathBuf>, CliError> {
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
    let output_path = resolved_output_path(&args, &loaded.compo);
    if args.format == OutputFormat::Html {
        write_html_output(&loaded.compo, &output_path)?;
        return Ok(Some(output_path));
    }
    if args.format == OutputFormat::All {
        write_all_outputs(&loaded.compo, &output_path)?;
        return Ok(Some(output_path));
    }

    let content = match args.format {
        OutputFormat::All => unreachable!("ALL output is handled before string serialization"),
        OutputFormat::Html => unreachable!("HTML output is handled before string serialization"),
        OutputFormat::Ipxact => serialize_ipxact(&loaded.compo, args.ipxact_version)?,
        OutputFormat::Ralf => irgen_ralf::serialize_ralf(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
        OutputFormat::SystemRdl => irgen_systemrdl::serialize_systemrdl(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    };
    write_text_output(&output_path, content)?;
    if let Some(schema) = args.validate_xsd {
        validate_ipxact_xml(&schema, &output_path).map_err(CliError::Runtime)?;
    }
    Ok(Some(output_path))
}

fn run_ipxact(args: IpxactArgs) -> Result<Option<PathBuf>, CliError> {
    let xml = fs::read_to_string(&args.input).map_err(|error| {
        CliError::Runtime(format!("failed to read {}: {error}", args.input.display()))
    })?;
    let component = parse_ipxact_with_directory_resolver(&args.input, &xml)?;
    let output_path = args
        .output
        .unwrap_or_else(|| default_ipxact_output_path(&component.name));
    let content = irgen_uvmreg::serialize_uvm_reg_with_options(
        &component,
        irgen_uvmreg::RenderOptions {
            coverage: args.coverage,
        },
    )
    .map_err(|error| CliError::Runtime(error.to_string()))?;
    write_text_output(&output_path, content)?;
    Ok(Some(output_path))
}

fn parse_ipxact_with_directory_resolver(
    input: &Path,
    xml: &str,
) -> Result<irgen_uvmreg::Component, CliError> {
    let base_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let mut cache = Vec::<(irgen_uvmreg::LibraryRef, String)>::new();

    irgen_uvmreg::parse_ipxact_with_resolver(xml, |reference| {
        Ok(find_ipxact_by_vlnv(base_dir, reference, &mut cache))
    })
    .map_err(|error| CliError::Runtime(error.to_string()))
}

fn find_ipxact_by_vlnv(
    base_dir: &Path,
    reference: &irgen_uvmreg::LibraryRef,
    cache: &mut Vec<(irgen_uvmreg::LibraryRef, String)>,
) -> Option<String> {
    if let Some((_, xml)) = cache
        .iter()
        .find(|(library_ref, _)| library_ref == reference)
    {
        return Some(xml.clone());
    }

    let entries = fs::read_dir(base_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("xml") {
            continue;
        }
        let Ok(xml) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(library_ref) = irgen_uvmreg::document_library_ref(&xml) else {
            continue;
        };
        cache.push((library_ref.clone(), xml.clone()));
        if &library_ref == reference {
            return Some(xml);
        }
    }

    None
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

    let stem = component_file_stem(component);
    write_text_output(
        &output.join(format!("{stem}-ipxact-1.4.xml")),
        serialize_ipxact(component, IpxactVersion::V1_4)?,
    )?;
    write_text_output(
        &output.join(format!("{stem}-ipxact-1.5.xml")),
        serialize_ipxact(component, IpxactVersion::V1_5)?,
    )?;
    write_text_output(
        &output.join(format!("{stem}-ipxact-2009.xml")),
        serialize_ipxact(component, IpxactVersion::V2009)?,
    )?;
    write_text_output(
        &output.join(format!("{stem}-ipxact-2014.xml")),
        serialize_ipxact(component, IpxactVersion::V2014)?,
    )?;
    write_text_output(
        &output.join(format!("{stem}-ipxact-2022.xml")),
        serialize_ipxact(component, IpxactVersion::V2022)?,
    )?;
    write_text_output(
        &output.join(format!("{stem}.ralf")),
        irgen_ralf::serialize_ralf(component)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
    )?;
    write_text_output(
        &output.join(format!("{stem}.rdl")),
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
        IpxactVersion::V1_4 => {
            ip_xact::serialize_1_4(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
        IpxactVersion::V1_5 => {
            ip_xact::serialize_1_5(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
        IpxactVersion::V2009 => {
            ip_xact::serialize_2009(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
        IpxactVersion::V2014 => {
            ip_xact::serialize_2014(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
        IpxactVersion::V2022 => {
            ip_xact::serialize_2022(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
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

fn resolved_output_path(args: &ConvertArgs, component: &irgen_model::base::Component) -> PathBuf {
    args.output.clone().unwrap_or_else(|| match args.format {
        OutputFormat::Html => default_input_output_path(&args.input, args.format),
        OutputFormat::All | OutputFormat::Ipxact | OutputFormat::Ralf | OutputFormat::SystemRdl => {
            default_component_output_path(component, args.format)
        }
    })
}

fn default_input_output_path(input: &Path, format: OutputFormat) -> PathBuf {
    let stem = input.file_stem().unwrap_or(input.as_os_str());
    match format.file_extension() {
        Some(extension) => PathBuf::from(stem).with_extension(extension),
        None => PathBuf::from(stem),
    }
}

fn default_component_output_path(
    component: &irgen_model::base::Component,
    format: OutputFormat,
) -> PathBuf {
    let stem = component_file_stem(component);
    match format.file_extension() {
        Some(extension) => PathBuf::from(stem).with_extension(extension),
        None => PathBuf::from(stem),
    }
}

fn component_file_stem(component: &irgen_model::base::Component) -> String {
    file_stem_from_name(component.name())
}

fn default_ipxact_output_path(component_name: &str) -> PathBuf {
    PathBuf::from(format!("ral_{}.sv", file_stem_from_name(component_name)))
}

fn file_stem_from_name(name: &str) -> String {
    let stem = name
        .trim()
        .chars()
        .map(|ch| match ch {
            '/' | '\\' => '_',
            _ => ch,
        })
        .collect::<String>();
    if stem.is_empty() {
        "component".into()
    } else {
        stem
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
struct RawSnapsheetArgs {
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

#[derive(Debug, Parser)]
#[command(
    name = "irgen ip-xact",
    version,
    about = "Generate UVM register model SystemVerilog from an IP-XACT component XML file."
)]
struct RawIpxactArgs {
    #[arg(value_name = "input.xml")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", value_name = "path")]
    output: Option<PathBuf>,

    #[arg(long = "coverage")]
    coverage: bool,
}

pub fn parse_args(args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    let args = args.collect::<Vec<_>>();
    match args.first().and_then(|value| value.to_str()) {
        Some("ip-xact") => parse_ipxact_args(args.into_iter().skip(1)),
        Some("snapsheet") => parse_snapsheet_args("irgen snapsheet", args.into_iter().skip(1)),
        _ => parse_snapsheet_args("irgen", args.into_iter()),
    }
}

fn parse_snapsheet_args(
    command_name: &'static str,
    args: impl Iterator<Item = OsString>,
) -> Result<Command, String> {
    match RawSnapsheetArgs::try_parse_from(
        std::iter::once(OsString::from(command_name)).chain(args),
    ) {
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

fn parse_ipxact_args(args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    match RawIpxactArgs::try_parse_from(
        std::iter::once(OsString::from("irgen ip-xact")).chain(args),
    ) {
        Ok(raw) => Ok(Command::Ipxact(IpxactArgs {
            input: raw.input,
            output: raw.output,
            coverage: raw.coverage,
        })),
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

fn convert_raw_args(raw: RawSnapsheetArgs) -> Result<Command, String> {
    if raw.format != OutputFormat::Ipxact && raw.ipxact_version.is_some() {
        return Err("--ipxact-version can only be used with --format ipxact".into());
    }
    let ipxact_version = raw.ipxact_version.unwrap_or(IpxactVersion::V2014);
    Ok(Command::Convert(ConvertArgs {
        input: raw.input,
        output: raw.output,
        format: raw.format,
        ipxact_version,
        snapsheet_spec: raw.snapsheet_spec,
        validate_xsd: raw.validate_xsd,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(name: &str) -> irgen_model::base::Component {
        irgen_model::base::Component::new(
            "example.com".into(),
            "IP".into(),
            name.into(),
            "1.0".into(),
            Vec::new(),
        )
    }

    fn args(format: OutputFormat, output: Option<PathBuf>) -> ConvertArgs {
        ConvertArgs {
            input: PathBuf::from("input.xlsx"),
            output,
            format,
            ipxact_version: IpxactVersion::V2014,
            snapsheet_spec: None,
            validate_xsd: None,
        }
    }

    #[test]
    fn defaults_single_file_outputs_to_component_name() {
        let component = component("soc_regs");

        assert_eq!(
            resolved_output_path(&args(OutputFormat::Ipxact, None), &component),
            PathBuf::from("soc_regs.xml")
        );
        assert_eq!(
            resolved_output_path(&args(OutputFormat::Ralf, None), &component),
            PathBuf::from("soc_regs.ralf")
        );
        assert_eq!(
            resolved_output_path(&args(OutputFormat::SystemRdl, None), &component),
            PathBuf::from("soc_regs.rdl")
        );
    }

    #[test]
    fn defaults_all_output_directory_to_component_name() {
        assert_eq!(
            resolved_output_path(&args(OutputFormat::All, None), &component("soc_regs")),
            PathBuf::from("soc_regs")
        );
    }

    #[test]
    fn keeps_html_default_directory_based_on_input_name() {
        assert_eq!(
            resolved_output_path(&args(OutputFormat::Html, None), &component("soc_regs")),
            PathBuf::from("input")
        );
    }

    #[test]
    fn explicit_output_path_takes_precedence() {
        assert_eq!(
            resolved_output_path(
                &args(OutputFormat::Ralf, Some(PathBuf::from("custom/out.ralf"))),
                &component("soc_regs")
            ),
            PathBuf::from("custom/out.ralf")
        );
    }

    #[test]
    fn component_output_name_does_not_create_subdirectories() {
        assert_eq!(component_file_stem(&component("soc/regs")), "soc_regs");
    }

    #[test]
    fn ipxact_default_output_uses_ral_prefix() {
        assert_eq!(
            default_ipxact_output_path("soc/regs"),
            PathBuf::from("ral_soc_regs.sv")
        );
    }
}
