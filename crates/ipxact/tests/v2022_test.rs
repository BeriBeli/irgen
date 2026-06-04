//! Official-schema smoke tests for IEEE 1685-2022 fixtures.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

const MINIMAL_2022_COMPONENT_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022"
                  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                  xsi:schemaLocation="http://www.accellera.org/XMLSchema/IPXACT/1685-2022 http://www.accellera.org/XMLSchema/IPXACT/1685-2022/index.xsd">
  <ipxact:vendor>example.org</ipxact:vendor>
  <ipxact:library>peripherals</ipxact:library>
  <ipxact:name>timer</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
</ipxact:component>"#;

#[test]
fn minimal_component_validates_against_official_2022_xsd() {
    validate_xml("minimal-component", MINIMAL_2022_COMPONENT_XML);
}

fn validate_xml(name: &str, xml: &str) {
    if Command::new("xmllint").arg("--version").output().is_err() {
        eprintln!("skipping official 2022 XSD validation because xmllint is not installed");
        return;
    }

    let output = temp_xml_path(name);
    fs::write(&output, xml).expect("temporary XML should be writable");

    let validation = Command::new("xmllint")
        .args(["--noout", "--schema"])
        .arg(schema_path())
        .arg(&output)
        .output()
        .expect("xmllint should run");

    fs::remove_file(&output).expect("temporary XML should be removable");

    assert!(
        validation.status.success(),
        "official 2022 schema validation failed:\n{}",
        String::from_utf8_lossy(&validation.stderr)
    );
}

fn schema_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/schemas/1685-2022/index.xsd")
}

fn temp_xml_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ip-xact-v2022-{name}-{}.xml", std::process::id()))
}
