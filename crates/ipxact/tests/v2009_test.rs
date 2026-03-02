//! Integration tests for IP-XACT 2009

#[test]
fn test_parse_component_from_xml() {
    use ip_xact::v2009::Component;

    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<component xmlns="http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009"
           xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
           xsi:schemaLocation="http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009 http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009/index.xsd">
  <vendor>vendor.com</vendor>
  <library>peripheral_lib</library>
  <name>uart</name>
  <version>1.0</version>
  <description>UART peripheral component</description>
  <parameters>
    <parameter>
      <name>CLK_FREQ</name>
      <value>100000000</value>
      <format>long</format>
    </parameter>
  </parameters>
</component>"#;

    let _component: Component = serde_xml_rs::from_str(xml).expect("Failed to parse component XML");
}

#[test]
fn test_build_component_programmatically() {
    use ip_xact::v2009::Component;

    // Build a component programmatically
    let component = Component::new(
        "vendor.com".to_string(),
        "peripheral_lib".to_string(),
        "uart".to_string(),
        "1.0".to_string(),
    );

    // Verify the component is correctly constructed
    assert_eq!(component.vendor, "vendor.com");
    assert_eq!(component.library, "peripheral_lib");
    assert_eq!(component.name, "uart");
    assert_eq!(component.version, "1.0");
}

#[test]
fn test_serialize_component_to_xml() {
    use ip_xact::v2009::Component;

    let component = Component::new(
        "test.vendor".to_string(),
        "test_lib".to_string(),
        "test_component".to_string(),
        "1.0".to_string(),
    );

    let xml = serde_xml_rs::to_string(&component).expect("Failed to serialize component");
    assert!(xml.contains("component"));
}

#[test]
fn test_roundtrip_component() {
    use ip_xact::v2009::Component;

    let mut component = Component::new(
        "vendor".to_string(),
        "library".to_string(),
        "component".to_string(),
        "1.0".to_string(),
    );
    component.description = Some("Test component".to_string());

    // Serialize
    let xml = serde_xml_rs::to_string(&component).expect("Failed to serialize");

    // Deserialize
    let parsed: Component = serde_xml_rs::from_str(&xml).expect("Failed to deserialize");

    // Verify
    assert_eq!(parsed.vendor, "vendor");
    assert_eq!(parsed.library, "library");
    assert_eq!(parsed.name, "component");
    assert_eq!(parsed.version, "1.0");
    assert_eq!(parsed.description, Some("Test component".to_string()));
}
