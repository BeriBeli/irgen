use std::path::Path;

#[test]
fn loads_workspace_example() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../example.xlsx");
    let loaded = irgen_snapsheet::load_excel(&example).expect("example workbook should parse");

    assert_eq!(loaded.compo.vendor(), "example.com");
    assert_eq!(loaded.compo.library(), "IP");
    assert_eq!(loaded.compo.name(), "example");
    assert_eq!(loaded.compo.version(), "1.0");
    assert_eq!(loaded.compo.blks().len(), 2);
    assert_eq!(loaded.compo.blks()[0].regs().len(), 9);
    assert_eq!(loaded.compo.blks()[1].regs().len(), 13);
}
