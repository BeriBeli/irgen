use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Args, Parser, Subcommand, ValueEnum, builder::styling, error::ErrorKind};

const CLI_STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Cyan.on_default().bold())
    .usage(styling::AnsiColor::Cyan.on_default().bold())
    .literal(styling::AnsiColor::Green.on_default().bold())
    .placeholder(styling::AnsiColor::Yellow.on_default())
    .error(styling::AnsiColor::Red.on_default().bold())
    .valid(styling::AnsiColor::Green.on_default())
    .invalid(styling::AnsiColor::Yellow.on_default());

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    #[value(name = "all")]
    All,
    #[value(name = "ip-xact")]
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
            Self::Ipxact => Some("xml"),
            Self::Ralf => Some("ralf"),
            Self::SystemRdl => Some("rdl"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactStandard {
    #[value(name = "ieee-1685-2022")]
    V2022,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactFileLayout {
    #[value(name = "single")]
    Single,
    #[value(name = "blocks")]
    Blocks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactFileType {
    #[value(name = "package")]
    Package,
    #[value(name = "header")]
    Header,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IpxactOutputFormat {
    #[value(name = "uvm-reg")]
    UvmReg,
    #[value(name = "html")]
    Html,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConvertArgs {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub format: OutputFormat,
    pub ipxact_standard: IpxactStandard,
    pub snapsheet_spec: Option<PathBuf>,
    pub validate_xsd: Option<PathBuf>,
    pub bus_bytes: Option<String>,
    pub backdoor: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IpxactArgs {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub format: IpxactOutputFormat,
    pub file_layout: IpxactFileLayout,
    pub file_type: IpxactFileType,
    pub coverage: bool,
    pub view: Option<String>,
    pub mode: Option<String>,
    pub library_paths: Vec<PathBuf>,
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
            "--validate can only be used with --format ip-xact".into(),
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

    let mut config = if let Some(spec) = &args.snapsheet_spec {
        irgen_snapsheet::SnapsheetConfig::from_toml_file(spec)
    } else {
        Ok(irgen_snapsheet::SnapsheetConfig::default())
    }
    .map_err(|error| CliError::Runtime(error.to_string()))?;
    if let Some(bus_bytes) = &args.bus_bytes {
        config.register.bus_bytes = bus_bytes.clone();
        config
            .register
            .parse_bus_bytes()
            .map_err(|message| CliError::Usage(format!("invalid --bus-bytes: {message}")))?;
    }
    if args.backdoor {
        config.register.backdoor = true;
    }
    let loaded = irgen_snapsheet::load_excel_with_config(&args.input, &config)
        .map_err(|error| CliError::Runtime(error.to_string()))?;
    let output_path = resolved_output_path(&args, &loaded.compo);
    if args.format == OutputFormat::All {
        write_all_outputs(&loaded.compo, &output_path)?;
        return Ok(Some(output_path));
    }

    let content = match args.format {
        OutputFormat::All => unreachable!("ALL output is handled before string serialization"),
        OutputFormat::Ipxact => serialize_ipxact(&loaded.compo, args.ipxact_standard)?,
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
    let render_options = irgen_uvmreg::RenderOptions {
        coverage: args.coverage,
        file_type: match args.file_type {
            IpxactFileType::Package => irgen_uvmreg::FileType::Package,
            IpxactFileType::Header => irgen_uvmreg::FileType::Header,
        },
    };
    match args.format {
        IpxactOutputFormat::UvmReg => match args.file_layout {
            IpxactFileLayout::Single => {
                let component = parse_ipxact_with_directory_resolver(
                    &args.input,
                    &xml,
                    args.view.clone(),
                    args.mode.clone(),
                    &args.library_paths,
                )?;
                let output_path = args
                    .output
                    .unwrap_or_else(|| default_ipxact_output_path(&component.name, args.file_type));
                let content =
                    irgen_uvmreg::serialize_uvm_reg_with_options(&component, render_options)
                        .map_err(|error| CliError::Runtime(error.to_string()))?;
                write_text_output(&output_path, content)?;
                Ok(Some(output_path))
            }
            IpxactFileLayout::Blocks => {
                let component = parse_ipxact_with_directory_resolver(
                    &args.input,
                    &xml,
                    args.view.clone(),
                    args.mode.clone(),
                    &args.library_paths,
                )?;
                let output_path = args.output.unwrap_or_else(|| {
                    default_ipxact_blocks_output_path(&component.name, args.file_type)
                });
                let files = irgen_uvmreg::serialize_uvm_reg_by_block_with_options(
                    &component,
                    render_options,
                )
                .map_err(|error| CliError::Runtime(error.to_string()))?;
                write_rendered_files(&output_path, files)?;
                Ok(Some(output_path))
            }
        },
        IpxactOutputFormat::Html => {
            let component = parse_ipxact_with_directory_resolver(
                &args.input,
                &xml,
                args.view.clone(),
                args.mode.clone(),
                &args.library_paths,
            )?;
            let docs_component = irgen_docs::component_from_ipxact_model(&component);
            let output_path = args
                .output
                .unwrap_or_else(|| default_ipxact_html_output_path(docs_component.name()));
            write_html_output(&docs_component, &output_path)?;
            Ok(Some(output_path))
        }
    }
}

fn parse_ipxact_with_directory_resolver(
    input: &Path,
    xml: &str,
    preferred_view: Option<String>,
    preferred_mode: Option<String>,
    library_paths: &[PathBuf],
) -> Result<irgen_ipxact_model::Component, CliError> {
    let base_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let mut search_paths = vec![base_dir.to_path_buf()];
    search_paths.extend(library_paths.iter().cloned());
    let mut cache = Vec::<(irgen_ipxact_parser::LibraryRef, String)>::new();

    irgen_ipxact_parser::parse_ipxact_with_options_and_resolver(
        xml,
        irgen_ipxact_parser::ParseOptions {
            preferred_view,
            preferred_mode,
        },
        |reference| find_ipxact_by_vlnv(&search_paths, reference, &mut cache),
    )
    .map_err(|error| CliError::Runtime(error.to_string()))
}

fn find_ipxact_by_vlnv(
    search_paths: &[PathBuf],
    reference: &irgen_ipxact_parser::LibraryRef,
    cache: &mut Vec<(irgen_ipxact_parser::LibraryRef, String)>,
) -> irgen_ipxact_parser::Result<Option<String>> {
    if let Some((_, xml)) = cache
        .iter()
        .find(|(library_ref, _)| library_ref == reference)
    {
        return Ok(Some(xml.clone()));
    }

    let mut matches = Vec::new();
    for search_path in search_paths {
        collect_ipxact_matches_in_path(search_path, reference, &mut matches);
    }

    match matches.len() {
        0 => Err(
            irgen_ipxact_parser::Error::ExternalTypeDefinitionsNotFoundIn {
                reference: reference.key(),
                searched: search_paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect(),
            },
        ),
        1 => {
            let resolved = matches.remove(0);
            cache.push((reference.clone(), resolved.xml.clone()));
            Ok(Some(resolved.xml))
        }
        _ => Err(
            irgen_ipxact_parser::Error::ExternalTypeDefinitionsAmbiguous {
                reference: reference.key(),
                matches: matches
                    .into_iter()
                    .map(|resolved| resolved.path.display().to_string())
                    .collect(),
            },
        ),
    }
}

struct ResolvedIpxact {
    path: PathBuf,
    xml: String,
}

fn collect_ipxact_matches_in_path(
    base_dir: &Path,
    reference: &irgen_ipxact_parser::LibraryRef,
    matches: &mut Vec<ResolvedIpxact>,
) {
    let Ok(entries) = fs::read_dir(base_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("xml") {
            continue;
        }
        collect_ipxact_matches_in_file(base_dir, &path, reference, matches);
    }
}

fn collect_ipxact_matches_in_file(
    base_dir: &Path,
    path: &Path,
    reference: &irgen_ipxact_parser::LibraryRef,
    matches: &mut Vec<ResolvedIpxact>,
) {
    let Ok(xml) = fs::read_to_string(path) else {
        return;
    };
    if let Ok(catalog_files) = irgen_ipxact_parser::catalog_file_refs(&xml) {
        let catalog_dir = path.parent().unwrap_or(base_dir);
        for catalog_file in catalog_files {
            let referenced_path = catalog_dir.join(&catalog_file.name);
            if &catalog_file.library_ref == reference {
                let Ok(referenced_xml) = fs::read_to_string(&referenced_path) else {
                    continue;
                };
                matches.push(ResolvedIpxact {
                    path: referenced_path,
                    xml: referenced_xml,
                });
                continue;
            }
            collect_ipxact_matches_in_file(catalog_dir, &referenced_path, reference, matches);
        }
        return;
    }

    if let Ok(library_ref) = irgen_ipxact_parser::document_library_ref(&xml)
        && &library_ref == reference
    {
        matches.push(ResolvedIpxact {
            path: path.to_path_buf(),
            xml,
        });
    }
}

fn write_all_outputs(
    component: &irgen_snapsheet::model::Component,
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
        &output.join(format!("{stem}-ip-xact-ieee-1685-2022.xml")),
        serialize_ipxact(component, IpxactStandard::V2022)?,
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
    Ok(())
}

fn serialize_ipxact(
    component: &irgen_snapsheet::model::Component,
    standard: IpxactStandard,
) -> Result<String, CliError> {
    match standard {
        IpxactStandard::V2022 => {
            ip_xact::serialize_2022(component).map_err(|error| CliError::Runtime(error.to_string()))
        }
    }
}

fn write_html_output(
    component: &irgen_docs::model::Component,
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

fn resolved_output_path(
    args: &ConvertArgs,
    component: &irgen_snapsheet::model::Component,
) -> PathBuf {
    args.output.clone().unwrap_or_else(|| match args.format {
        OutputFormat::All | OutputFormat::Ipxact | OutputFormat::Ralf | OutputFormat::SystemRdl => {
            default_component_output_path(component, args.format)
        }
    })
}

fn default_component_output_path(
    component: &irgen_snapsheet::model::Component,
    format: OutputFormat,
) -> PathBuf {
    let stem = component_file_stem(component);
    match format.file_extension() {
        Some(extension) => PathBuf::from(stem).with_extension(extension),
        None => PathBuf::from(stem),
    }
}

fn component_file_stem(component: &irgen_snapsheet::model::Component) -> String {
    file_stem_from_name(component.name())
}

fn default_ipxact_output_path(component_name: &str, file_type: IpxactFileType) -> PathBuf {
    let stem = file_stem_from_name(component_name);
    match file_type {
        IpxactFileType::Package => PathBuf::from(format!("ral_{stem}_pkg.sv")),
        IpxactFileType::Header => PathBuf::from(format!("ral_{stem}.sv")),
    }
}

fn default_ipxact_blocks_output_path(component_name: &str, file_type: IpxactFileType) -> PathBuf {
    let stem = file_stem_from_name(component_name);
    match file_type {
        IpxactFileType::Package => PathBuf::from(format!("ral_{stem}_pkg")),
        IpxactFileType::Header => PathBuf::from(format!("ral_{stem}")),
    }
}

fn default_ipxact_html_output_path(component_name: &str) -> PathBuf {
    PathBuf::from(format!("{}-html", file_stem_from_name(component_name)))
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

fn write_rendered_files(
    output: &Path,
    files: Vec<irgen_uvmreg::RenderedFile>,
) -> Result<(), CliError> {
    fs::create_dir_all(output).map_err(|error| {
        CliError::Runtime(format!(
            "failed to create output directory {}: {error}",
            output.display()
        ))
    })?;
    for file in files {
        write_text_output(&output.join(file.path), file.content)?;
    }
    Ok(())
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
    propagate_version = true,
    subcommand_required = true,
    arg_required_else_help = false,
    disable_help_subcommand = true,
    about = "Convert register descriptions between supported file formats.",
    after_help = "Run `irgen snapsheet --help` or `irgen ip-xact --help` for command-specific options.",
    styles = CLI_STYLES
)]
struct RawCli {
    #[command(subcommand)]
    command: RawCommand,
}

#[derive(Debug, Subcommand)]
enum RawCommand {
    #[command(
        name = "snapsheet",
        about = "Convert a register spreadsheet into IP-XACT, RALF, SystemRDL, or all outputs."
    )]
    Snapsheet(RawSnapsheetArgs),
    #[command(
        name = "ip-xact",
        about = "Generate UVM register model SystemVerilog or HTML docs from an IP-XACT component XML file."
    )]
    Ipxact(RawIpxactArgs),
}

#[derive(Debug, Args)]
struct RawSnapsheetArgs {
    #[arg(value_name = "input.xlsx")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", value_name = "path")]
    output: Option<PathBuf>,

    #[arg(
        short = 'f',
        long = "format",
        value_enum,
        default_value = "ip-xact",
        value_name = "name"
    )]
    format: OutputFormat,

    #[arg(long = "standard", value_enum, value_name = "standard")]
    ipxact_standard: Option<IpxactStandard>,

    #[arg(long = "config", value_name = "snapsheet.toml")]
    snapsheet_spec: Option<PathBuf>,

    #[arg(long = "validate", value_name = "schema.xsd")]
    validate_xsd: Option<PathBuf>,

    #[arg(long = "bus-bytes", value_name = "bytes", default_value = "4")]
    bus_bytes: Option<String>,

    #[arg(long = "backdoor")]
    backdoor: bool,
}

#[derive(Debug, Args)]
struct RawIpxactArgs {
    #[arg(value_name = "input.xml")]
    input: PathBuf,

    #[arg(short = 'o', long = "output", value_name = "path")]
    output: Option<PathBuf>,

    #[arg(
        short = 'f',
        long = "format",
        value_enum,
        default_value = "uvm-reg",
        value_name = "name"
    )]
    format: IpxactOutputFormat,

    #[arg(
        long = "file-layout",
        alias = "output-layout",
        value_enum,
        default_value = "single",
        value_name = "layout"
    )]
    file_layout: IpxactFileLayout,

    #[arg(
        long = "file-type",
        value_enum,
        default_value = "package",
        value_name = "type"
    )]
    file_type: IpxactFileType,

    #[arg(long = "coverage")]
    coverage: bool,

    #[arg(long = "view", value_name = "viewRef")]
    view: Option<String>,

    #[arg(long = "mode", value_name = "modeRef")]
    mode: Option<String>,

    #[arg(long = "library-path", value_name = "path")]
    library_paths: Vec<PathBuf>,
}

pub fn parse_args(args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    match RawCli::try_parse_from(std::iter::once(OsString::from("irgen")).chain(args)) {
        Ok(raw) => match raw.command {
            RawCommand::Snapsheet(raw) => convert_raw_args(raw),
            RawCommand::Ipxact(raw) => Ok(Command::Ipxact(IpxactArgs {
                input: raw.input,
                output: raw.output,
                format: raw.format,
                file_layout: raw.file_layout,
                file_type: raw.file_type,
                coverage: raw.coverage,
                view: raw.view,
                mode: raw.mode,
                library_paths: raw.library_paths,
            })),
        },
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            Ok(Command::Help(format!("{}", error.render().ansi())))
        }
        Err(error) => Err(error.to_string()),
    }
}

fn convert_raw_args(raw: RawSnapsheetArgs) -> Result<Command, String> {
    if raw.format != OutputFormat::Ipxact && raw.ipxact_standard.is_some() {
        return Err("--standard can only be used with --format ip-xact".into());
    }
    let ipxact_standard = raw.ipxact_standard.unwrap_or(IpxactStandard::V2022);
    Ok(Command::Convert(ConvertArgs {
        input: raw.input,
        output: raw.output,
        format: raw.format,
        ipxact_standard,
        snapsheet_spec: raw.snapsheet_spec,
        validate_xsd: raw.validate_xsd,
        bus_bytes: raw.bus_bytes,
        backdoor: raw.backdoor,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(name: &str) -> irgen_snapsheet::model::Component {
        irgen_snapsheet::model::Component::new(
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
            ipxact_standard: IpxactStandard::V2022,
            snapsheet_spec: None,
            validate_xsd: None,
            bus_bytes: Some("4".into()),
            backdoor: false,
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
            default_ipxact_output_path("soc/regs", IpxactFileType::Package),
            PathBuf::from("ral_soc_regs_pkg.sv")
        );
        assert_eq!(
            default_ipxact_output_path("soc/regs", IpxactFileType::Header),
            PathBuf::from("ral_soc_regs.sv")
        );
        assert_eq!(
            default_ipxact_blocks_output_path("soc/regs", IpxactFileType::Package),
            PathBuf::from("ral_soc_regs_pkg")
        );
        assert_eq!(
            default_ipxact_blocks_output_path("soc/regs", IpxactFileType::Header),
            PathBuf::from("ral_soc_regs")
        );
    }

    #[test]
    fn ipxact_html_default_output_uses_component_name_without_extension() {
        assert_eq!(
            default_ipxact_html_output_path("soc/regs"),
            PathBuf::from("soc_regs-html")
        );
    }
}
