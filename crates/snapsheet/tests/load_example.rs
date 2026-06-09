use std::path::Path;

#[test]
fn loads_workspace_example() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let loaded = irgen_snapsheet::load_excel(&example).expect("example workbook should parse");

    assert_eq!(loaded.compo.vendor(), "example.com");
    assert_eq!(loaded.compo.library(), "IP");
    assert_eq!(loaded.compo.name(), "example_simple");
    assert_eq!(loaded.compo.version(), "1.0");
    assert_eq!(loaded.compo.blks().len(), 1);
    assert_eq!(loaded.compo.blks()[0].regs().len(), 2);
    assert_eq!(loaded.compo.blks()[0].register_files().len(), 0);
}

#[test]
fn loads_macro_enabled_example() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsm");
    let loaded = irgen_snapsheet::load_excel(&example).expect("xlsm example should parse");

    assert_eq!(loaded.compo.name(), "example_simple");
    assert_eq!(loaded.compo.blks().len(), 1);
}

#[test]
fn loads_opendocument_example() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.ods");
    let loaded = irgen_snapsheet::load_excel(&example).expect("ods example should parse");

    assert_eq!(loaded.compo.name(), "example_simple");
    assert_eq!(loaded.compo.blks().len(), 1);
}

#[test]
fn loads_workspace_example_with_root_snapsheet_toml() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example = manifest.join("../../examples/example.xlsx");
    let spec = manifest.join("../../snapsheet.toml");

    let loaded = irgen_snapsheet::load_excel_with_config_file(&example, &spec)
        .expect("example workbook should parse with root snapsheet.toml");

    assert_eq!(loaded.compo.vendor(), "example.com");
    assert_eq!(loaded.compo.blks().len(), 2);
    assert_eq!(loaded.compo.blks()[0].regs().len(), 6);
    assert_eq!(loaded.compo.blks()[0].register_files().len(), 0);
    assert_eq!(loaded.compo.blks()[1].regs().len(), 10);
    assert_eq!(loaded.compo.blks()[1].register_files().len(), 0);
}

#[test]
fn loads_configured_examples_in_additional_formats() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = manifest.join("../../snapsheet.toml");

    for extension in ["xlsm", "ods"] {
        let example = manifest.join(format!("../../examples/example.{extension}"));
        let loaded = irgen_snapsheet::load_excel_with_config_file(&example, &spec)
            .unwrap_or_else(|error| panic!("configured {extension} example should parse: {error}"));

        assert_eq!(loaded.compo.vendor(), "example.com");
        assert_eq!(loaded.compo.blks().len(), 2);
    }
}
