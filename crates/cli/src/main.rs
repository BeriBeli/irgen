use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode};

const USAGE: &str = "Usage: irgen <input.xlsx> [-o <output>] [--format ipxact|ralf|systemrdl] [--snapsheet-spec <snapsheet.toml>] [--validate <schema.xsd>]\n\
                     \n\
                     Convert a register spreadsheet into an output file.\n\
                     \n\
                     Options:\n\
                       -o, --output <path>           Output path (default: input path with selected format extension)\n\
                       -f, --format <name>           Output format: ipxact (default), ralf, systemrdl\n\
                      --snapsheet-spec <snapsheet.toml>  TOML file describing snapsheet sheet names, columns, and parser rules\n\
                      --validate <path>             Validate IP-XACT XML with xmllint and the supplied XSD\n\
                       -h, --help                    Print help";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Ipxact,
    Ralf,
    SystemRdl,
}

impl OutputFormat {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "ipxact" => Ok(Self::Ipxact),
            "ralf" => Ok(Self::Ralf),
            "systemrdl" => Ok(Self::SystemRdl),
            _ => Err(format!(
                "unknown output format: {value}; expected ipxact, ralf, or systemrdl"
            )),
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Ipxact => "xml",
            Self::Ralf => "ralf",
            Self::SystemRdl => "rdl",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ConvertArgs {
    input: PathBuf,
    output: PathBuf,
    format: OutputFormat,
    snapsheet_spec: Option<PathBuf>,
    validate_xsd: Option<PathBuf>,
}

enum Command {
    Convert(ConvertArgs),
    Help,
}

#[derive(Debug)]
enum CliError {
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

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1)) {
        Ok(Some(output)) => {
            println!("Generated {}", output.display());
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(CliError::Usage(message)) => {
            eprintln!("error: {message}\n\n{USAGE}");
            ExitCode::FAILURE
        }
        Err(CliError::Runtime(message)) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl Iterator<Item = OsString>) -> Result<Option<PathBuf>, CliError> {
    let Command::Convert(args) = parse_args(args).map_err(CliError::Usage)? else {
        println!("{USAGE}");
        return Ok(None);
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
        OutputFormat::Ipxact => irgen_model::serialize_ipxact_xml(&loaded.compo)
            .map_err(|error| CliError::Runtime(error.to_string()))?,
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

fn parse_args(args: impl Iterator<Item = OsString>) -> Result<Command, String> {
    let mut args = args.peekable();
    let mut input = None;
    let mut output = None;
    let mut format = None;
    let mut snapsheet_spec = None;
    let mut validate_xsd = None;

    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("-h" | "--help") => return Ok(Command::Help),
            Some("-o" | "--output") => {
                let path = args
                    .next()
                    .ok_or_else(|| "missing path after --output".to_string())?;
                if output.replace(PathBuf::from(path)).is_some() {
                    return Err("--output may only be specified once".into());
                }
            }
            Some("-f" | "--format") => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing name after --format".to_string())?;
                let value = value
                    .to_str()
                    .ok_or_else(|| "output format must be valid UTF-8".to_string())?;
                if format.replace(OutputFormat::parse(value)?).is_some() {
                    return Err("--format may only be specified once".into());
                }
            }
            Some("--validate") => {
                let path = args
                    .next()
                    .ok_or_else(|| "missing path after --validate".to_string())?;
                if validate_xsd.replace(PathBuf::from(path)).is_some() {
                    return Err("--validate may only be specified once".into());
                }
            }
            Some("--snapsheet-spec") => {
                let path = args
                    .next()
                    .ok_or_else(|| "missing path after --snapsheet-spec".to_string())?;
                if snapsheet_spec.replace(PathBuf::from(path)).is_some() {
                    return Err("--snapsheet-spec may only be specified once".into());
                }
            }
            Some(flag) if flag.starts_with('-') => {
                return Err(format!("unknown option: {flag}"));
            }
            _ => {
                if input.replace(PathBuf::from(arg)).is_some() {
                    return Err("only one input spreadsheet may be specified".into());
                }
            }
        }
    }

    let input = input.ok_or_else(|| "missing input spreadsheet".to_string())?;
    let format = format.unwrap_or(OutputFormat::Ipxact);
    let output = output.unwrap_or_else(|| input.with_extension(format.extension()));
    Ok(Command::Convert(ConvertArgs {
        input,
        output,
        format,
        snapsheet_spec,
        validate_xsd,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> impl Iterator<Item = OsString> {
        values
            .iter()
            .map(|value| OsString::from(*value))
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn rejects_missing_input() {
        assert_eq!(
            parse_args(args(&[])).err().as_deref(),
            Some("missing input spreadsheet")
        );
    }

    #[test]
    fn rejects_unknown_options() {
        assert_eq!(
            parse_args(args(&["--wat"])).err().as_deref(),
            Some("unknown option: --wat")
        );
    }

    #[test]
    fn accepts_explicit_output_path() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "-o", "nested/output.xml"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.output, PathBuf::from("nested/output.xml"));
    }

    #[test]
    fn accepts_explicit_ipxact_format() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "--format", "ipxact"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.format, OutputFormat::Ipxact);
        assert_eq!(parsed.output, PathBuf::from("input.xml"));
    }

    #[test]
    fn accepts_explicit_ralf_format() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "--format", "ralf"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.format, OutputFormat::Ralf);
        assert_eq!(parsed.output, PathBuf::from("input.ralf"));
    }

    #[test]
    fn accepts_explicit_systemrdl_format() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "--format", "systemrdl"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.format, OutputFormat::SystemRdl);
        assert_eq!(parsed.output, PathBuf::from("input.rdl"));
    }

    #[test]
    fn generates_ralf_output() {
        let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example_simple.xlsx");
        let output = std::env::temp_dir().join(format!(
            "irgen-cli-test-{}-example.ralf",
            std::process::id()
        ));
        let _ = fs::remove_file(&output);

        let result = run([
            OsString::from(input),
            OsString::from("--format"),
            OsString::from("ralf"),
            OsString::from("-o"),
            OsString::from(&output),
        ]
        .into_iter())
        .unwrap();

        assert_eq!(result.as_deref(), Some(output.as_path()));
        let ralf = fs::read_to_string(&output).unwrap();
        assert!(ralf.contains("block regs {"));
        assert!(ralf.contains("register status @'h0 {"));
        assert!(ralf.contains("access ro;"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn generates_systemrdl_output() {
        let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example_simple.xlsx");
        let output =
            std::env::temp_dir().join(format!("irgen-cli-test-{}-example.rdl", std::process::id()));
        let _ = fs::remove_file(&output);

        let result = run([
            OsString::from(input),
            OsString::from("--format"),
            OsString::from("systemrdl"),
            OsString::from("-o"),
            OsString::from(&output),
        ]
        .into_iter())
        .unwrap();

        assert_eq!(result.as_deref(), Some(output.as_path()));
        let rdl = fs::read_to_string(&output).unwrap();
        assert!(rdl.contains("addrmap example_simple {"));
        assert!(rdl.contains("addrmap regs {"));
        assert!(rdl.contains("reg status {"));
        assert!(rdl.contains("sw = r;"));
        let _ = fs::remove_file(output);
    }

    #[test]
    fn generates_and_validates_ipxact_output() {
        if ProcessCommand::new("xmllint")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("skipping CLI IP-XACT validation because xmllint is not installed");
            return;
        }

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let input = root.join("example.xlsx");
        let spec = root.join("snapsheet.toml");
        let schema = root.join("crates/ipxact/tests/fixtures/schemas/1685-2014/index.xsd");
        let output =
            std::env::temp_dir().join(format!("irgen-cli-test-{}-example.xml", std::process::id()));
        let _ = fs::remove_file(&output);

        let result = run([
            OsString::from(input),
            OsString::from("--snapsheet-spec"),
            OsString::from(spec),
            OsString::from("-o"),
            OsString::from(&output),
            OsString::from("--validate"),
            OsString::from(schema),
        ]
        .into_iter())
        .unwrap();

        assert_eq!(result.as_deref(), Some(output.as_path()));
        let xml = fs::read_to_string(&output).unwrap();
        assert!(xml.contains("http://www.accellera.org/XMLSchema/IPXACT/1685-2014"));
        let reg1_start = xml
            .find("<ipxact:register><ipxact:name>reg1</ipxact:name>")
            .expect("generated IP-XACT should contain reg1");
        let reg2_offset = xml[reg1_start..]
            .find("<ipxact:register><ipxact:name>reg2</ipxact:name>")
            .expect("generated IP-XACT should contain reg2 after reg1");
        let reg1_xml = &xml[reg1_start..reg1_start + reg2_offset];
        assert_eq!(reg1_xml.matches("<ipxact:field>").count(), 4);
        let _ = fs::remove_file(output);
    }

    #[test]
    fn accepts_explicit_xsd_validation() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "--validate", "schema/index.xsd"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.validate_xsd, Some(PathBuf::from("schema/index.xsd")));
    }

    #[test]
    fn rejects_validation_for_non_ipxact_before_loading_workbook() {
        let error = run(args(&[
            "this-file-does-not-exist.xlsx",
            "--format",
            "ralf",
            "--validate",
            "schema/index.xsd",
        ]))
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "--validate can only be used with --format ipxact"
        );
    }

    #[test]
    fn rejects_missing_validation_schema_before_loading_workbook() {
        let missing_schema = PathBuf::from("schema/does-not-exist.xsd");
        let error = run([
            OsString::from("this-file-does-not-exist.xlsx"),
            OsString::from("--validate"),
            OsString::from(&missing_schema),
        ]
        .into_iter())
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            format!("validation schema not found: {}", missing_schema.display())
        );
    }

    #[test]
    fn rejects_missing_validation_schema_before_writing_output() {
        let input = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example_simple.xlsx");
        let output = std::env::temp_dir().join(format!(
            "irgen-cli-test-{}-missing-schema.xml",
            std::process::id()
        ));
        let missing_schema = PathBuf::from("schema/does-not-exist.xsd");
        let _ = fs::remove_file(&output);

        let error = run([
            OsString::from(input),
            OsString::from("-o"),
            OsString::from(&output),
            OsString::from("--validate"),
            OsString::from(&missing_schema),
        ]
        .into_iter())
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            format!("validation schema not found: {}", missing_schema.display())
        );
        assert!(!output.exists());
    }

    #[test]
    fn accepts_explicit_snapsheet_spec() {
        let Command::Convert(parsed) =
            parse_args(args(&["input.xlsx", "--snapsheet-spec", "snapsheet.toml"])).unwrap()
        else {
            panic!("expected conversion command");
        };

        assert_eq!(parsed.snapsheet_spec, Some(PathBuf::from("snapsheet.toml")));
    }

    #[test]
    fn rejects_missing_snapsheet_spec_path() {
        assert_eq!(
            parse_args(args(&["input.xlsx", "--snapsheet-spec"]))
                .err()
                .as_deref(),
            Some("missing path after --snapsheet-spec")
        );
    }

    #[test]
    fn rejects_duplicate_snapsheet_spec() {
        assert_eq!(
            parse_args(args(&[
                "input.xlsx",
                "--snapsheet-spec",
                "first.toml",
                "--snapsheet-spec",
                "second.toml",
            ]))
            .err()
            .as_deref(),
            Some("--snapsheet-spec may only be specified once")
        );
    }

    #[test]
    fn rejects_regvue_format() {
        assert_eq!(
            parse_args(args(&["input.xlsx", "--format", "regvue"]))
                .err()
                .as_deref(),
            Some("unknown output format: regvue; expected ipxact, ralf, or systemrdl")
        );
    }

    #[test]
    fn reports_failing_spreadsheet_conversion() {
        let error = run(args(&["this-file-does-not-exist.xlsx"])).unwrap_err();

        assert!(error.to_string().contains("Xlsx error"));
    }
}
