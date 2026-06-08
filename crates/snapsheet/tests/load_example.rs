use std::fs;
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
fn loads_macro_enabled_workbook_extension() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/example_simple.xlsx");
    let copy = std::env::temp_dir().join(format!(
        "irgen-snapsheet-test-{}-example_simple.xlsm",
        std::process::id()
    ));
    let _ = fs::remove_file(&copy);
    fs::copy(&source, &copy).unwrap();

    let loaded = irgen_snapsheet::load_excel(&copy)
        .expect("OOXML workbook should parse with an .xlsm extension");

    assert_eq!(loaded.compo.name(), "example_simple");
    assert_eq!(loaded.compo.blks().len(), 1);
    let _ = fs::remove_file(copy);
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
    assert_eq!(loaded.compo.blks()[0].regs().len(), 4);
    assert_eq!(loaded.compo.blks()[0].register_files().len(), 2);
    assert_eq!(loaded.compo.blks()[1].regs().len(), 4);
    assert_eq!(loaded.compo.blks()[1].register_files().len(), 5);
}
