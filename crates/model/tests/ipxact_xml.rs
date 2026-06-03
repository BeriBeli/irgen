use irgen_model::base::{Block, Component, Field, Register};

#[test]
fn emits_field_elements_in_ipxact_2014_order() {
    let component = Component::new(
        "example.com".into(),
        "ip".into(),
        "example".into(),
        "1.0".into(),
        vec![Block::new(
            "regs".into(),
            "0x0".into(),
            "0x4".into(),
            "32".into(),
            vec![Register::new(
                "status".into(),
                "0x0".into(),
                "32".into(),
                vec![Field::new(
                    "done".into(),
                    "0".into(),
                    "1".into(),
                    "W1C".into(),
                    "0".into(),
                    "Completion flag".into(),
                )],
            )],
        )],
    );

    let xml = irgen_model::serialize_ipxact_xml(&component).expect("component should serialize");

    let bit_offset = xml.find("<ipxact:bitOffset>").unwrap();
    let resets = xml.find("<ipxact:resets>").unwrap();
    let bit_width = xml.find("<ipxact:bitWidth>").unwrap();
    let access = xml.find("<ipxact:access>").unwrap();
    let modified_write = xml.find("<ipxact:modifiedWriteValue>").unwrap();

    assert!(bit_offset < resets);
    assert!(resets < bit_width);
    assert!(bit_width < access);
    assert!(access < modified_write);
    assert!(xml.contains("<ipxact:modifiedWriteValue>oneToClear</ipxact:modifiedWriteValue>"));
}
