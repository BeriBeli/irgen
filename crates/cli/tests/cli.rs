use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use irgen_cli::{Command, IpxactArgs, IpxactStandard, OutputFormat, parse_args, run};

fn args(values: &[&str]) -> impl Iterator<Item = OsString> {
    values
        .iter()
        .map(|value| OsString::from(*value))
        .collect::<Vec<_>>()
        .into_iter()
}

fn snapsheet_args(values: &[&str]) -> impl Iterator<Item = OsString> {
    std::iter::once(OsString::from("snapsheet"))
        .chain(args(values))
        .collect::<Vec<_>>()
        .into_iter()
}

fn compact_xml(xml: &str) -> String {
    xml.split_whitespace().collect()
}

fn normalize_newlines(text: String) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
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

fn assert_snapsheet_parse_error_contains(values: &[&str], needles: &[&str]) {
    let error = parse_args(snapsheet_args(values)).unwrap_err();
    for needle in needles {
        assert!(
            error.contains(needle),
            "expected parse error to contain {needle:?}, got {error:?}"
        );
    }
}

#[test]
fn rejects_missing_subcommand() {
    assert_parse_error_contains(&[], &["subcommand is required", "snapsheet", "ip-xact"]);
}

#[test]
fn rejects_bare_input_without_subcommand() {
    assert_parse_error_contains(
        &["input.xlsx"],
        &["unknown command", "input.xlsx", "snapsheet"],
    );
}

#[test]
fn rejects_missing_snapsheet_input() {
    assert_snapsheet_parse_error_contains(&[], &["required", "input.xlsx"]);
}

#[test]
fn rejects_unknown_options() {
    assert_parse_error_contains(&["--wat"], &["unexpected argument", "--wat"]);
}

#[test]
fn root_help_lists_subcommands() {
    let Command::Help(output) = parse_args(args(&["--help"])).unwrap() else {
        panic!("expected help output");
    };

    assert!(output.contains("Commands:"));
    assert!(output.contains("snapsheet"));
    assert!(output.contains("ip-xact"));
    assert!(output.contains("irgen snapsheet --help"));
    assert!(output.contains("irgen ip-xact --help"));
}

#[test]
fn short_root_help_lists_subcommands() {
    let Command::Help(output) = parse_args(args(&["-h"])).unwrap() else {
        panic!("expected help output");
    };

    assert!(output.contains("Commands:"));
    assert!(output.contains("snapsheet"));
    assert!(output.contains("ip-xact"));
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
        parse_args(snapsheet_args(&["input.xlsx", "-o", "nested/output.xml"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.output, Some(PathBuf::from("nested/output.xml")));
}

#[test]
fn accepts_explicit_ip_xact_format() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "nested/input.xlsx",
        "--format",
        "ip-xact",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V2014);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_snapsheet_subcommand_for_existing_flow() {
    let Command::Convert(parsed) = parse_args(args(&[
        "snapsheet",
        "nested/input.xlsx",
        "--format",
        "ralf",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.input, PathBuf::from("nested/input.xlsx"));
    assert_eq!(parsed.format, OutputFormat::Ralf);
}

#[test]
fn accepts_ipxact_subcommand() {
    let Command::Ipxact(parsed) = parse_args(args(&[
        "ip-xact",
        "nested/input.xml",
        "-o",
        "nested/uvmreg_demo.sv",
    ]))
    .unwrap() else {
        panic!("expected ip-xact command");
    };

    assert_eq!(
        parsed,
        IpxactArgs {
            input: PathBuf::from("nested/input.xml"),
            output: Some(PathBuf::from("nested/uvmreg_demo.sv")),
            file_layout: irgen_cli::IpxactFileLayout::Single,
            coverage: false,
            view: None,
            mode: None,
            library_paths: Vec::new(),
        }
    );
}

#[test]
fn accepts_ipxact_coverage_option() {
    let Command::Ipxact(parsed) =
        parse_args(args(&["ip-xact", "nested/input.xml", "--coverage"])).unwrap()
    else {
        panic!("expected ip-xact command");
    };

    assert_eq!(
        parsed,
        IpxactArgs {
            input: PathBuf::from("nested/input.xml"),
            output: None,
            file_layout: irgen_cli::IpxactFileLayout::Single,
            coverage: true,
            view: None,
            mode: None,
            library_paths: Vec::new(),
        }
    );
}

#[test]
fn accepts_ipxact_view_option() {
    let Command::Ipxact(parsed) =
        parse_args(args(&["ip-xact", "nested/input.xml", "--view", "gate"])).unwrap()
    else {
        panic!("expected ip-xact command");
    };

    assert_eq!(
        parsed,
        IpxactArgs {
            input: PathBuf::from("nested/input.xml"),
            output: None,
            file_layout: irgen_cli::IpxactFileLayout::Single,
            coverage: false,
            view: Some("gate".into()),
            mode: None,
            library_paths: Vec::new(),
        }
    );
}

#[test]
fn accepts_ipxact_mode_option() {
    let Command::Ipxact(parsed) = parse_args(args(&[
        "ip-xact",
        "nested/input.xml",
        "--mode",
        "diagnostic",
    ]))
    .unwrap() else {
        panic!("expected ip-xact command");
    };

    assert_eq!(
        parsed,
        IpxactArgs {
            input: PathBuf::from("nested/input.xml"),
            output: None,
            file_layout: irgen_cli::IpxactFileLayout::Single,
            coverage: false,
            view: None,
            mode: Some("diagnostic".into()),
            library_paths: Vec::new(),
        }
    );
}

#[test]
fn accepts_ipxact_library_path_option() {
    let Command::Ipxact(parsed) = parse_args(args(&[
        "ip-xact",
        "nested/input.xml",
        "--library-path",
        "ipxact_lib",
        "--library-path",
        "more_ipxact",
    ]))
    .unwrap() else {
        panic!("expected ip-xact command");
    };

    assert_eq!(
        parsed,
        IpxactArgs {
            input: PathBuf::from("nested/input.xml"),
            output: None,
            file_layout: irgen_cli::IpxactFileLayout::Single,
            coverage: false,
            view: None,
            mode: None,
            library_paths: vec![PathBuf::from("ipxact_lib"), PathBuf::from("more_ipxact")],
        }
    );
}

#[test]
fn accepts_ipxact_file_layout_option() {
    let Command::Ipxact(parsed) = parse_args(args(&[
        "ip-xact",
        "nested/input.xml",
        "--file-layout",
        "blocks",
    ]))
    .unwrap() else {
        panic!("expected ip-xact command");
    };

    assert_eq!(parsed.file_layout, irgen_cli::IpxactFileLayout::Blocks);
}

#[test]
fn accepts_explicit_html_format() {
    let Command::Convert(parsed) =
        parse_args(snapsheet_args(&["nested/input.xlsx", "--format", "html"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Html);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_all_format() {
    let Command::Convert(parsed) =
        parse_args(snapsheet_args(&["nested/input.xlsx", "--format", "all"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::All);
    assert_eq!(parsed.output, None);
}

#[test]
fn defaults_ipxact_standard_to_2014() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&["input.xlsx"])).unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V2014);
}

#[test]
fn accepts_explicit_ipxact_2014_standard() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "input.xlsx",
        "--standard",
        "ieee-1685-2014",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V2014);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_ipxact_1_4_standard() {
    let Command::Convert(parsed) =
        parse_args(snapsheet_args(&["input.xlsx", "--standard", "spirit-1.4"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V1_4);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_ipxact_1_5_standard() {
    let Command::Convert(parsed) =
        parse_args(snapsheet_args(&["input.xlsx", "--standard", "spirit-1.5"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V1_5);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_ipxact_2009_standard() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "input.xlsx",
        "--standard",
        "ieee-1685-2009",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V2009);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_ipxact_2022_standard() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "input.xlsx",
        "--standard",
        "ieee-1685-2022",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ipxact);
    assert_eq!(parsed.ipxact_standard, IpxactStandard::V2022);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_ralf_format() {
    let Command::Convert(parsed) =
        parse_args(snapsheet_args(&["nested/input.xlsx", "--format", "ralf"])).unwrap()
    else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::Ralf);
    assert_eq!(parsed.output, None);
}

#[test]
fn accepts_explicit_systemrdl_format() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "nested/input.xlsx",
        "--format",
        "systemrdl",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.format, OutputFormat::SystemRdl);
    assert_eq!(parsed.output, None);
}

#[test]
fn generates_ralf_output() {
    let input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-example.ralf",
        std::process::id()
    ));
    let _ = fs::remove_file(&output);

    let result = run([
        OsString::from("snapsheet"),
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
    let input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let output =
        std::env::temp_dir().join(format!("irgen-cli-test-{}-example.rdl", std::process::id()));
    let _ = fs::remove_file(&output);

    let result = run([
        OsString::from("snapsheet"),
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
fn generates_uvm_reg_from_ipxact_subcommand() {
    let input =
        std::env::temp_dir().join(format!("irgen-cli-test-{}-uvmreg.xml", std::process::id()));
    let output =
        std::env::temp_dir().join(format!("irgen-cli-test-{}-uvmreg.sv", std::process::id()));
    let _ = fs::remove_file(&input);
    let _ = fs::remove_file(&output);
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>example.com</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>demo</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps><ipxact:memoryMap><ipxact:name>demo</ipxact:name>
    <ipxact:addressBlock>
      <ipxact:name>regs</ipxact:name>
      <ipxact:baseAddress>0</ipxact:baseAddress>
      <ipxact:range>4</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:register>
        <ipxact:name>status</ipxact:name>
        <ipxact:accessHandles>
          <ipxact:accessHandle>
            <ipxact:viewRef>gate</ipxact:viewRef>
            <ipxact:pathSegments><ipxact:pathSegment>gate.status</ipxact:pathSegment></ipxact:pathSegments>
          </ipxact:accessHandle>
          <ipxact:accessHandle>
            <ipxact:pathSegments><ipxact:pathSegment>rtl.status</ipxact:pathSegment></ipxact:pathSegments>
          </ipxact:accessHandle>
        </ipxact:accessHandles>
        <ipxact:addressOffset>0</ipxact:addressOffset>
        <ipxact:size>32</ipxact:size>
        <ipxact:field>
          <ipxact:name>done</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:viewRef>gate</ipxact:viewRef>
              <ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>gate_done</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices>
            </ipxact:accessHandle>
            <ipxact:accessHandle>
              <ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>done_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:bitWidth>1</ipxact:bitWidth>
          <ipxact:resets><ipxact:reset><ipxact:value>0</ipxact:value></ipxact:reset></ipxact:resets>
          <ipxact:fieldAccessPolicies>
            <ipxact:fieldAccessPolicy>
              <ipxact:modeRef>diagnostic</ipxact:modeRef>
              <ipxact:access>write-only</ipxact:access>
            </ipxact:fieldAccessPolicy>
            <ipxact:fieldAccessPolicy><ipxact:access>read-only</ipxact:access></ipxact:fieldAccessPolicy>
          </ipxact:fieldAccessPolicies>
        </ipxact:field>
      </ipxact:register>
    </ipxact:addressBlock>
  </ipxact:memoryMap></ipxact:memoryMaps>
</ipxact:component>"#,
    )
    .unwrap();

    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output.as_path()));
    let sv = fs::read_to_string(&output).unwrap();
    assert!(sv.contains("`ifndef RAL_DEMO_SV"));
    assert!(sv.contains("class ral_sys_demo extends uvm_reg_block;"));
    assert!(sv.contains("class ral_block_regs extends uvm_reg_block;"));
    assert!(sv.contains("class ral_reg_regs_status extends uvm_reg;"));
    assert!(!sv.contains("build_coverage(UVM_CVR_REG_BITS)"));
    assert!(sv.contains("status.add_hdl_path_slice(\"rtl.status.done_q\", 0, 1, 1'b1);"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h0, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(regs.default_map, `UVM_REG_ADDR_WIDTH'h0);"));

    let view_output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-uvmreg-view.sv",
        std::process::id()
    ));
    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--view"),
        OsString::from("gate"),
        OsString::from("-o"),
        OsString::from(&view_output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(view_output.as_path()));
    let sv = fs::read_to_string(&view_output).unwrap();
    assert!(sv.contains("status.add_hdl_path_slice(\"gate.status.gate_done\", 0, 1, 1'b1);"));

    let mode_output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-uvmreg-mode.sv",
        std::process::id()
    ));
    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--mode"),
        OsString::from("diagnostic"),
        OsString::from("-o"),
        OsString::from(&mode_output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(mode_output.as_path()));
    let sv = fs::read_to_string(&mode_output).unwrap();
    assert!(sv.contains("done.configure(this, 1, 0, \"WO\""));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h0, \"WO\");"));

    let coverage_output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-uvmreg-cov.sv",
        std::process::id()
    ));
    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--coverage"),
        OsString::from("-o"),
        OsString::from(&coverage_output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(coverage_output.as_path()));
    let sv = fs::read_to_string(&coverage_output).unwrap();
    assert!(sv.contains("covergroup cg_bits();"));
    assert!(sv.contains("super.new(name, 32, build_coverage(UVM_CVR_REG_BITS));"));
    assert!(sv.contains("add_coverage(build_coverage(UVM_CVR_REG_BITS));"));
    assert!(sv.contains("virtual function void sample(uvm_reg_data_t data,"));
    let _ = fs::remove_file(input);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(view_output);
    let _ = fs::remove_file(mode_output);
    let _ = fs::remove_file(coverage_output);
}

#[test]
fn generates_uvm_reg_by_block_from_ipxact_subcommand() {
    let input = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-uvmreg-blocks.xml",
        std::process::id()
    ));
    let output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-uvmreg-blocks",
        std::process::id()
    ));
    let _ = fs::remove_file(&input);
    let _ = fs::remove_dir_all(&output);
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>example.com</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>demo</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>demo</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>ctrl</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>enable</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>stat</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>done</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#,
    )
    .unwrap();

    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--file-layout"),
        OsString::from("blocks"),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output.as_path()));
    let top = normalize_newlines(fs::read_to_string(output.join("ral_demo.sv")).unwrap());
    let cfg = normalize_newlines(fs::read_to_string(output.join("ral_block_cfg.sv")).unwrap());
    let stat = normalize_newlines(fs::read_to_string(output.join("ral_block_stat.sv")).unwrap());
    assert!(top.contains(
        "`include \"uvm_macros.svh\"\n`include \"ral_block_cfg.sv\"\n`include \"ral_block_stat.sv\"\n\nclass"
    ));
    assert!(top.contains("`include \"ral_block_cfg.sv\""));
    assert!(top.contains("`include \"ral_block_stat.sv\""));
    assert!(top.contains("class ral_sys_demo extends uvm_reg_block;"));
    assert!(!top.contains("\n\n\n"));
    assert!(cfg.contains("class ral_block_cfg extends uvm_reg_block;"));
    assert!(cfg.contains("class ral_reg_cfg_ctrl extends uvm_reg;"));
    assert!(!cfg.contains("class ral_block_stat extends uvm_reg_block;"));
    assert!(!cfg.contains("\n\n\n"));
    assert!(stat.contains("class ral_block_stat extends uvm_reg_block;"));
    assert!(stat.contains("class ral_reg_stat_done extends uvm_reg;"));
    assert!(!stat.contains("\n\n\n"));

    let _ = fs::remove_file(input);
    let _ = fs::remove_dir_all(output);
}

#[test]
fn ipxact_subcommand_resolves_external_type_definitions_from_input_directory() {
    let dir = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-external-types",
        std::process::id()
    ));
    let input = dir.join("top.xml");
    let external = dir.join("common_types.xml");
    let output = dir.join("ral_external_top.sv");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        &external,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:typeDefinitions xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>types</ipxact:library>
  <ipxact:name>common_regs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:addressBlockDefinitions>
    <ipxact:addressBlockDefinition>
      <ipxact:name>shared_block</ipxact:name>
      <ipxact:description>External CLI block</ipxact:description>
      <ipxact:range>0x20</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:register>
        <ipxact:name>status</ipxact:name>
        <ipxact:addressOffset>0x4</ipxact:addressOffset>
        <ipxact:size>32</ipxact:size>
        <ipxact:field>
          <ipxact:name>ready</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:bitWidth>1</ipxact:bitWidth>
          <ipxact:access>read-only</ipxact:access>
        </ipxact:field>
      </ipxact:register>
    </ipxact:addressBlockDefinition>
  </ipxact:addressBlockDefinitions>
</ipxact:typeDefinitions>"#,
    )
    .unwrap();
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>external_top</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:externalTypeDefinitions>
      <ipxact:name>common_types</ipxact:name>
      <ipxact:typeDefinitionsRef vendor="acme" library="types" name="common_regs" version="1.0"/>
    </ipxact:externalTypeDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x80</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="common_types">shared_block</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#,
    )
    .unwrap();

    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output.as_path()));
    let sv = fs::read_to_string(&output).unwrap();
    assert!(sv.contains("`ifndef RAL_EXTERNAL_TOP_SV"));
    assert!(sv.contains("class ral_sys_external_top extends uvm_reg_block;"));
    assert!(!sv.contains("localparam"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h80);"));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn ipxact_subcommand_resolves_external_type_definitions_from_catalog_library_path() {
    let dir = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-catalog-types",
        std::process::id()
    ));
    let input_dir = dir.join("input");
    let library_dir = dir.join("library");
    let input = input_dir.join("top.xml");
    let catalog = library_dir.join("catalog.xml");
    let external = library_dir.join("defs/common_types.xml");
    let output = dir.join("ral_catalog_top.sv");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(external.parent().unwrap()).unwrap();
    fs::create_dir_all(&input_dir).unwrap();
    fs::write(
        &external,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:typeDefinitions xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>catalog_types</ipxact:library>
  <ipxact:name>shared_defs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:addressBlockDefinitions>
    <ipxact:addressBlockDefinition>
      <ipxact:name>catalog_block</ipxact:name>
      <ipxact:range>0x20</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:register>
        <ipxact:name>mode</ipxact:name>
        <ipxact:addressOffset>0x8</ipxact:addressOffset>
        <ipxact:size>32</ipxact:size>
        <ipxact:field>
          <ipxact:name>enable</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:bitWidth>1</ipxact:bitWidth>
          <ipxact:access>read-write</ipxact:access>
        </ipxact:field>
      </ipxact:register>
    </ipxact:addressBlockDefinition>
  </ipxact:addressBlockDefinitions>
</ipxact:typeDefinitions>"#,
    )
    .unwrap();
    fs::write(
        &catalog,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:catalog xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>catalog</ipxact:library>
  <ipxact:name>catalog</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:ipxactFile>
      <ipxact:vlnv vendor="acme" library="catalog_types" name="shared_defs" version="1.0"/>
      <ipxact:name>defs/common_types.xml</ipxact:name>
    </ipxact:ipxactFile>
  </ipxact:typeDefinitions>
</ipxact:catalog>"#,
    )
    .unwrap();
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>catalog_top</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:externalTypeDefinitions>
      <ipxact:name>catalog_types</ipxact:name>
      <ipxact:typeDefinitionsRef vendor="acme" library="catalog_types" name="shared_defs" version="1.0"/>
    </ipxact:externalTypeDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x40</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="catalog_types">catalog_block</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#,
    )
    .unwrap();

    let result = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--library-path"),
        OsString::from(&library_dir),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output.as_path()));
    let sv = fs::read_to_string(&output).unwrap();
    assert!(sv.contains("class ral_sys_catalog_top extends uvm_reg_block;"));
    assert!(sv.contains("default_map.add_reg(mode, `UVM_REG_ADDR_WIDTH'h8, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h40);"));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn ipxact_subcommand_reports_external_type_definition_search_paths() {
    let dir = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-missing-external",
        std::process::id()
    ));
    let input_dir = dir.join("input");
    let library_dir = dir.join("library");
    let input = input_dir.join("top.xml");
    let output = dir.join("ral_missing.sv");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&input_dir).unwrap();
    fs::create_dir_all(&library_dir).unwrap();
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_top</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:externalTypeDefinitions>
      <ipxact:name>missing_types</ipxact:name>
      <ipxact:typeDefinitionsRef vendor="acme" library="missing" name="defs" version="1.0"/>
    </ipxact:externalTypeDefinitions>
  </ipxact:typeDefinitions>
</ipxact:component>"#,
    )
    .unwrap();

    let error = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--library-path"),
        OsString::from(&library_dir),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap_err();

    let error = error.to_string();
    assert!(error.contains("acme:missing:defs:1.0"));
    assert!(error.contains("searched:"));
    assert!(error.contains(&input_dir.display().to_string()));
    assert!(error.contains(&library_dir.display().to_string()));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn ipxact_subcommand_reports_ambiguous_external_type_definitions() {
    let dir = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-ambiguous-external",
        std::process::id()
    ));
    let input_dir = dir.join("input");
    let library_dir = dir.join("library");
    let input = input_dir.join("top.xml");
    let first = library_dir.join("defs_a.xml");
    let second = library_dir.join("defs_b.xml");
    let output = dir.join("ral_ambiguous.sv");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&input_dir).unwrap();
    fs::create_dir_all(&library_dir).unwrap();
    let external_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:typeDefinitions xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>dup_types</ipxact:library>
  <ipxact:name>defs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
</ipxact:typeDefinitions>"#;
    fs::write(&first, external_xml).unwrap();
    fs::write(&second, external_xml).unwrap();
    fs::write(
        &input,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>ambiguous_top</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:externalTypeDefinitions>
      <ipxact:name>dup_types</ipxact:name>
      <ipxact:typeDefinitionsRef vendor="acme" library="dup_types" name="defs" version="1.0"/>
    </ipxact:externalTypeDefinitions>
  </ipxact:typeDefinitions>
</ipxact:component>"#,
    )
    .unwrap();

    let error = run([
        OsString::from("ip-xact"),
        OsString::from(&input),
        OsString::from("--library-path"),
        OsString::from(&library_dir),
        OsString::from("-o"),
        OsString::from(&output),
    ]
    .into_iter())
    .unwrap_err();

    let error = error.to_string();
    assert!(error.contains("ambiguous"));
    assert!(error.contains("acme:dup_types:defs:1.0"));
    assert!(error.contains(&first.display().to_string()));
    assert!(error.contains(&second.display().to_string()));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn generates_html_output() {
    let input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let output_dir = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-example-html",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&output_dir);

    let result = run([
        OsString::from("snapsheet"),
        OsString::from(input),
        OsString::from("--format"),
        OsString::from("html"),
        OsString::from("-o"),
        OsString::from(&output_dir),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output_dir.as_path()));
    let html = fs::read_to_string(output_dir.join("index.html")).unwrap();
    assert!(html.contains("<!doctype html>"));
    assert!(!html.contains("Print or Save PDF"));
    assert!(html.contains("register-search-index"));
    assert!(!html.contains("Fields for Register: status"));
    let block_page = fs::read_to_string(output_dir.join("block-regs.html")).unwrap();
    assert!(block_page.contains("href=\"index.html\""));
    assert!(!block_page.contains("href=\"../index.html\""));
    assert!(block_page.contains("href=\"block-regs/register-regs-status.html\""));
    assert!(!block_page.contains("Fields for Register: status"));
    let register_page =
        fs::read_to_string(output_dir.join("block-regs/register-regs-status.html")).unwrap();
    assert!(register_page.contains("href=\"../index.html\""));
    assert!(!register_page.contains("href=\"../../index.html\""));
    assert!(register_page.contains("Fields for Register: status"));
    assert!(register_page.contains("<strong>Value After Reset:</strong> 0"));
    assert!(output_dir.join("assets/register_reference.css").is_file());
    assert!(output_dir.join("assets/register_reference.js").is_file());
    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn generates_all_outputs() {
    let input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let output_dir =
        std::env::temp_dir().join(format!("irgen-cli-test-{}-example-all", std::process::id()));
    let _ = fs::remove_dir_all(&output_dir);

    let result = run([
        OsString::from("snapsheet"),
        OsString::from(input),
        OsString::from("--format"),
        OsString::from("all"),
        OsString::from("-o"),
        OsString::from(&output_dir),
    ]
    .into_iter())
    .unwrap();

    assert_eq!(result.as_deref(), Some(output_dir.as_path()));
    let ipxact_1_4 =
        fs::read_to_string(output_dir.join("example_simple-ip-xact-spirit-1.4.xml")).unwrap();
    let ipxact_1_5 =
        fs::read_to_string(output_dir.join("example_simple-ip-xact-spirit-1.5.xml")).unwrap();
    let ipxact_2009 =
        fs::read_to_string(output_dir.join("example_simple-ip-xact-ieee-1685-2009.xml")).unwrap();
    let ipxact_2014 =
        fs::read_to_string(output_dir.join("example_simple-ip-xact-ieee-1685-2014.xml")).unwrap();
    let ipxact_2022 =
        fs::read_to_string(output_dir.join("example_simple-ip-xact-ieee-1685-2022.xml")).unwrap();
    let ralf = fs::read_to_string(output_dir.join("example_simple.ralf")).unwrap();
    let rdl = fs::read_to_string(output_dir.join("example_simple.rdl")).unwrap();
    let html = fs::read_to_string(output_dir.join("html/index.html")).unwrap();
    assert!(ipxact_1_4.contains("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.4"));
    assert!(ipxact_1_5.contains("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.5"));
    assert!(ipxact_2009.contains("http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009"));
    assert!(ipxact_2014.contains("http://www.accellera.org/XMLSchema/IPXACT/1685-2014"));
    assert!(ipxact_2022.contains("http://www.accellera.org/XMLSchema/IPXACT/1685-2022"));
    assert!(ralf.contains("block regs {"));
    assert!(rdl.contains("addrmap example_simple {"));
    assert!(html.contains("<!doctype html>"));
    assert!(
        output_dir
            .join("html/assets/register_reference.css")
            .is_file()
    );
    let _ = fs::remove_dir_all(output_dir);
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
    let input = root.join("examples/example.xlsx");
    let spec = root.join("snapsheet.toml");
    let schema = root.join("crates/ipxact/schema/1685-2014/index.xsd");
    let output =
        std::env::temp_dir().join(format!("irgen-cli-test-{}-example.xml", std::process::id()));
    let _ = fs::remove_file(&output);

    let result = run([
        OsString::from("snapsheet"),
        OsString::from(input),
        OsString::from("--config"),
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
    let compact = compact_xml(&xml);
    let reg1_start = compact
        .find("<ipxact:register><ipxact:name>reg1</ipxact:name>")
        .expect("generated IP-XACT should contain reg1");
    let reg2_offset = compact[reg1_start..]
        .find("<ipxact:register><ipxact:name>reg2</ipxact:name>")
        .expect("generated IP-XACT should contain reg2 after reg1");
    let reg1_xml = &compact[reg1_start..reg1_start + reg2_offset];
    assert_eq!(reg1_xml.matches("<ipxact:field>").count(), 4);
    let _ = fs::remove_file(output);
}

#[test]
fn generates_and_validates_complex_ipxact_1_4_output() {
    generate_and_validate_complex_ipxact(
        "spirit-1.4",
        "crates/ipxact/schema/1.4/index.xsd",
        "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.4",
        "<spirit:register><spirit:name>reg1</spirit:name>",
    );
}

#[test]
fn generates_and_validates_complex_ipxact_1_5_output() {
    generate_and_validate_complex_ipxact(
        "spirit-1.5",
        "crates/ipxact/schema/1.5/index.xsd",
        "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.5",
        "<spirit:register><spirit:name>reg1</spirit:name>",
    );
}

#[test]
fn generates_and_validates_complex_ipxact_2009_output() {
    generate_and_validate_complex_ipxact(
        "ieee-1685-2009",
        "crates/ipxact/schema/1685-2009/index.xsd",
        "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009",
        "<spirit:register><spirit:name>reg1</spirit:name>",
    );
}

#[test]
fn generates_and_validates_complex_ipxact_2022_output() {
    generate_and_validate_complex_ipxact(
        "ieee-1685-2022",
        "crates/ipxact/schema/1685-2022/index.xsd",
        "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        "<ipxact:register><ipxact:name>reg1</ipxact:name>",
    );
}

fn generate_and_validate_complex_ipxact(
    standard_arg: &str,
    schema_rel: &str,
    namespace: &str,
    register_needle: &str,
) {
    if ProcessCommand::new("xmllint")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!(
            "skipping CLI IP-XACT {standard_arg} validation because xmllint is not installed"
        );
        return;
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let input = root.join("examples/example.xlsx");
    let spec = root.join("snapsheet.toml");
    let schema = root.join(schema_rel);
    let output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-complex-{standard_arg}.xml",
        std::process::id()
    ));
    let _ = fs::remove_file(&output);

    let result = run([
        OsString::from("snapsheet"),
        OsString::from(input),
        OsString::from("--config"),
        OsString::from(spec),
        OsString::from("--standard"),
        OsString::from(standard_arg),
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
    assert!(compact_xml(&xml).contains(register_needle));
    let _ = fs::remove_file(output);
}

#[test]
fn accepts_explicit_xsd_validation() {
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "input.xlsx",
        "--validate",
        "schema/index.xsd",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.validate_xsd, Some(PathBuf::from("schema/index.xsd")));
}

#[test]
fn rejects_validation_for_non_ipxact_before_loading_workbook() {
    let error = run(snapsheet_args(&[
        "this-file-does-not-exist.xlsx",
        "--format",
        "ralf",
        "--validate",
        "schema/index.xsd",
    ]))
    .unwrap_err();

    assert_eq!(
        error.to_string(),
        "--validate can only be used with --format ip-xact"
    );
}

#[test]
fn rejects_validation_for_all_before_loading_workbook() {
    let error = run(snapsheet_args(&[
        "this-file-does-not-exist.xlsx",
        "--format",
        "all",
        "--validate",
        "schema/index.xsd",
    ]))
    .unwrap_err();

    assert_eq!(
        error.to_string(),
        "--validate can only be used with --format ip-xact"
    );
}

#[test]
fn rejects_missing_validation_schema_before_loading_workbook() {
    let missing_schema = PathBuf::from("schema/does-not-exist.xsd");
    let error = run([
        OsString::from("snapsheet"),
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
    let input =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let output = std::env::temp_dir().join(format!(
        "irgen-cli-test-{}-missing-schema.xml",
        std::process::id()
    ));
    let missing_schema = PathBuf::from("schema/does-not-exist.xsd");
    let _ = fs::remove_file(&output);

    let error = run([
        OsString::from("snapsheet"),
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
    let Command::Convert(parsed) = parse_args(snapsheet_args(&[
        "input.xlsx",
        "--config",
        "snapsheet.toml",
    ]))
    .unwrap() else {
        panic!("expected conversion command");
    };

    assert_eq!(parsed.snapsheet_spec, Some(PathBuf::from("snapsheet.toml")));
}

#[test]
fn rejects_missing_snapsheet_spec_path() {
    assert_snapsheet_parse_error_contains(
        &["input.xlsx", "--config"],
        &["a value is required", "--config"],
    );
}

#[test]
fn rejects_duplicate_snapsheet_spec() {
    assert_snapsheet_parse_error_contains(
        &[
            "input.xlsx",
            "--config",
            "first.toml",
            "--config",
            "second.toml",
        ],
        &["cannot be used multiple times", "--config"],
    );
}

#[test]
fn rejects_regvue_format() {
    assert_snapsheet_parse_error_contains(
        &["input.xlsx", "--format", "regvue"],
        &["invalid value", "regvue", "ip-xact"],
    );
}

#[test]
fn rejects_missing_ipxact_standard() {
    assert_snapsheet_parse_error_contains(
        &["input.xlsx", "--standard"],
        &["a value is required", "--standard"],
    );
}

#[test]
fn rejects_duplicate_ipxact_standard() {
    assert_snapsheet_parse_error_contains(
        &[
            "input.xlsx",
            "--standard",
            "ieee-1685-2014",
            "--standard",
            "ieee-1685-2014",
        ],
        &["cannot be used multiple times", "--standard"],
    );
}

#[test]
fn rejects_unsupported_ipxact_standard() {
    assert_snapsheet_parse_error_contains(
        &["input.xlsx", "--standard", "2020"],
        &[
            "invalid value",
            "2020",
            "spirit-1.4",
            "spirit-1.5",
            "ieee-1685-2009",
            "ieee-1685-2014",
            "ieee-1685-2022",
        ],
    );
}

#[test]
fn rejects_ipxact_standard_for_non_ipxact_format() {
    assert_eq!(
        parse_args(snapsheet_args(&[
            "input.xlsx",
            "--format",
            "systemrdl",
            "--standard",
            "ieee-1685-2014",
        ]))
        .err()
        .as_deref(),
        Some("--standard can only be used with --format ip-xact")
    );
}

#[test]
fn rejects_ipxact_standard_for_all_format() {
    assert_eq!(
        parse_args(snapsheet_args(&[
            "input.xlsx",
            "--format",
            "all",
            "--standard",
            "ieee-1685-2014",
        ]))
        .err()
        .as_deref(),
        Some("--standard can only be used with --format ip-xact")
    );
}

#[test]
fn reports_failing_spreadsheet_conversion() {
    let error = run(snapsheet_args(&["this-file-does-not-exist.xlsx"])).unwrap_err();

    assert!(error.to_string().contains("Workbook error"));
}
