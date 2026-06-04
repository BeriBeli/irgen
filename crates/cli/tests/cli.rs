use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use irgen_cli::{Command, IpxactVersion, OutputFormat, parse_args, run};

fn args(values: &[&str]) -> impl Iterator<Item = OsString> {
    values
        .iter()
        .map(|value| OsString::from(*value))
        .collect::<Vec<_>>()
        .into_iter()
}

fn assert_parse_error_contains(values: &[&str], needles: &[&str]) {
    let error = parse_args(args(values)).unwrap_err();
    for needle in needles {
        assert!(
            error.contains(needle),
            "expected parse error to contain {needle:?}, got {error:?}"
        );
    }
}

#[test]
fn rejects_missing_input() {
    assert_parse_error_contains(&[], &["required", "input.xlsx"]);
}

#[test]
fn rejects_unknown_options() {
    assert_parse_error_contains(&["--wat"], &["unexpected argument", "--wat"]);
}

#[test]
fn accepts_version_flag() {
    let Command::Help(output) = parse_args(args(&["--version"])).unwrap() else {
        panic!("expected version output");
    };

    assert_eq!(output, format!("irgen {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn accepts_short_version_flag() {
    let Command::Help(output) = parse_args(args(&["-V"])).unwrap() else {
        panic!("expected version output");
    };

    assert_eq!(output, format!("irgen {}\n", env!("CARGO_PKG_VERSION")));
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
    let Command::Convert(parsed) = parse_args(args(&["input.xlsx", "--format", "ipxact"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_version, IpxactVersion::V2014);
    assert_eq!(parsed.output, PathBuf::from("input.xml"));
}

#[test]
fn defaults_ipxact_version_to_2014() {
    let Command::Convert(parsed) = parse_args(args(&["input.xlsx"])).unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_version, IpxactVersion::V2014);
}

#[test]
fn accepts_explicit_ipxact_2014_version() {
    let Command::Convert(parsed) =
        parse_args(args(&["input.xlsx", "--ipxact-version", "2014"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_version, IpxactVersion::V2014);
    assert_eq!(parsed.output, PathBuf::from("input.xml"));
}

#[test]
fn accepts_explicit_ipxact_2009_version() {
    let Command::Convert(parsed) =
        parse_args(args(&["input.xlsx", "--ipxact-version", "2009"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_version, IpxactVersion::V2009);
    assert_eq!(parsed.output, PathBuf::from("input.xml"));
}

#[test]
fn accepts_explicit_ipxact_2022_version() {
    let Command::Convert(parsed) =
        parse_args(args(&["input.xlsx", "--ipxact-version", "2022"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_version, IpxactVersion::V2022);
    assert_eq!(parsed.output, PathBuf::from("input.xml"));
}

#[test]
fn accepts_explicit_ralf_format() {
    let Command::Convert(parsed) = parse_args(args(&["input.xlsx", "--format", "ralf"])).unwrap()
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
fn generates_and_validates_complex_ipxact_2009_output() {
    generate_and_validate_complex_ipxact(
        "2009",
        "crates/ipxact/tests/fixtures/schemas/1685-2009/index.xsd",
        "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009",
        "<spirit:register><spirit:name>reg1</spirit:name>",
    );
}

#[test]
fn generates_and_validates_complex_ipxact_2022_output() {
    generate_and_validate_complex_ipxact(
        "2022",
        "crates/ipxact/tests/fixtures/schemas/1685-2022/index.xsd",
        "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        "<ipxact:register><ipxact:name>reg1</ipxact:name>",
    );
}

fn generate_and_validate_complex_ipxact(
    version_arg: &str,
    schema_rel: &str,
    namespace: &str,
    register_needle: &str,
) {
    if ProcessCommand::new("xmllint")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipping CLI IP-XACT {version_arg} validation because xmllint is not installed");
        return;
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let input = root.join("example.xlsx");
    let spec = root.join("snapsheet.toml");
    let schema = root.join(schema_rel);
    let output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-complex-{version_arg}.xml",
        std::process::id()
    ));
    let _ = fs::remove_file(&output);

    let result = run([
        OsString::from(input),
        OsString::from("--snapsheet-spec"),
        OsString::from(spec),
        OsString::from("--ipxact-version"),
        OsString::from(version_arg),
        OsString::from("-o"),
        OsString::from(&output),
        OsString::from("--validate"),
        OsString::from(schema),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output.as_path()));
    let xml = fs::read_to_string(&output).unwrap();
    assert!(xml.contains(namespace));
    assert!(xml.contains(register_needle));
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
    assert_parse_error_contains(
        &["input.xlsx", "--snapsheet-spec"],
        &["a value is required", "--snapsheet-spec"],
    );
}

#[test]
fn rejects_duplicate_snapsheet_spec() {
    assert_parse_error_contains(
        &[
            "input.xlsx",
            "--snapsheet-spec",
            "first.toml",
            "--snapsheet-spec",
            "second.toml",
        ],
        &["cannot be used multiple times", "--snapsheet-spec"],
    );
}

#[test]
fn rejects_regvue_format() {
    assert_parse_error_contains(
        &["input.xlsx", "--format", "regvue"],
        &["invalid value", "regvue", "ipxact"],
    );
}

#[test]
fn rejects_missing_ipxact_version() {
    assert_parse_error_contains(
        &["input.xlsx", "--ipxact-version"],
        &["a value is required", "--ipxact-version"],
    );
}

#[test]
fn rejects_duplicate_ipxact_version() {
    assert_parse_error_contains(
        &[
            "input.xlsx",
            "--ipxact-version",
            "2014",
            "--ipxact-version",
            "2014",
        ],
        &["cannot be used multiple times", "--ipxact-version"],
    );
}

#[test]
fn rejects_unsupported_ipxact_version() {
    assert_parse_error_contains(
        &["input.xlsx", "--ipxact-version", "2020"],
        &["invalid value", "2020", "2009", "2014", "2022"],
    );
}

#[test]
fn rejects_ipxact_version_for_non_ipxact_format() {
    assert_eq!(
        parse_args(args(&[
            "input.xlsx",
            "--format",
            "systemrdl",
            "--ipxact-version",
            "2014",
        ]))
        .err()
        .as_deref(),
        Some("--ipxact-version can only be used with --format ipxact")
    );
}

#[test]
fn reports_failing_spreadsheet_conversion() {
    let error = run(args(&["this-file-does-not-exist.xlsx"])).unwrap_err();

    assert!(error.to_string().contains("Xlsx error"));
}
