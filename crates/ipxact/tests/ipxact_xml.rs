use irgen_snapsheet::model::{
    Block, Component, Field, FieldOptions, Register, RegisterArray, RegisterFile,
};

fn compact_xml(xml: &str) -> String {
    xml.split_whitespace().collect()
}

#[test]
fn emits_field_elements_in_ipxact_order() {
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

    let xml = ip_xact::serialize(&component).expect("component should serialize");

    let bit_offset = xml.find("<ipxact:bitOffset>").unwrap();
    let bit_width = xml.find("<ipxact:bitWidth>").unwrap();
    let resets = xml.find("<ipxact:resets>").unwrap();
    let access = xml.find("<ipxact:access>").unwrap();
    let modified_write = xml.find("<ipxact:modifiedWriteValue>").unwrap();

    assert!(bit_offset < resets);
    assert!(bit_offset < bit_width);
    assert!(bit_width < resets);
    assert!(resets < access);
    assert!(access < modified_write);
    assert!(xml.contains("<ipxact:modifiedWriteValue>oneToClear</ipxact:modifiedWriteValue>"));
}

#[test]
fn emits_pretty_printed_ipxact_xml() {
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
                    "RW".into(),
                    "0".into(),
                    String::new(),
                )],
            )],
        )],
    );

    let xml = ip_xact::serialize(&component).expect("component should serialize");

    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"));
    assert!(xml.contains("\n  <ipxact:vendor>example.com</ipxact:vendor>\n"));
    assert!(xml.contains("\n      <ipxact:addressBlock>\n"));
    assert!(xml.ends_with('\n'));
}

#[test]
fn emits_standard_hdl_paths_for_ipxact_versions_that_support_them() {
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
                vec![
                    Field::new_with_hdl_path(
                        "done".into(),
                        "0".into(),
                        "1".into(),
                        "RW".into(),
                        "0".into(),
                        "Completion flag".into(),
                        Some("u_status.done_q".into()),
                    ),
                    Field::new(
                        "reserved0".into(),
                        "1".into(),
                        "1".into(),
                        "RO".into(),
                        "0".into(),
                        String::new(),
                    ),
                    Field::new_with_hdl_path(
                        "no_path".into(),
                        "2".into(),
                        "1".into(),
                        "RO".into(),
                        "0".into(),
                        String::new(),
                        None,
                    ),
                ],
            )],
        )],
    );

    let access_handle = r#"<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>u_status.done_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>"#;
    let reserved_access_path = "<ipxact:pathSegment>reserved0</ipxact:pathSegment>";
    let disabled_access_path = "<ipxact:pathSegment>no_path</ipxact:pathSegment>";

    let ipxact = ip_xact::serialize(&component).expect("component should serialize");
    let compact = compact_xml(&ipxact);

    assert!(compact.contains(access_handle));
    assert!(!ipxact.contains(reserved_access_path));
    assert!(!ipxact.contains(disabled_access_path));
}

#[test]
fn emits_register_arrays() {
    let component = Component::new(
        "example.com".into(),
        "ip".into(),
        "example".into(),
        "1.0".into(),
        vec![Block::new(
            "regs".into(),
            "0x0".into(),
            "0x8".into(),
            "32".into(),
            vec![Register::new_arrayed(
                "status".into(),
                "0x0".into(),
                "32".into(),
                String::new(),
                RegisterArray::new(vec!["4".into()], Some("0x4".into())),
                vec![Field::new(
                    "value".into(),
                    "0".into(),
                    "32".into(),
                    "RW".into(),
                    "0".into(),
                    String::new(),
                )],
            )],
        )],
    );

    let ipxact = ip_xact::serialize(&component).expect("component should serialize");

    assert!(ipxact.contains("<ipxact:array>"));
    assert!(ipxact.contains("<ipxact:dim>4</ipxact:dim>"));
    assert!(ipxact.contains("<ipxact:stride>0x4</ipxact:stride>"));
}

#[test]
fn emits_register_file_arrays() {
    let component = Component::new(
        "example.com".into(),
        "ip".into(),
        "example".into(),
        "1.0".into(),
        vec![Block::new_with_register_files(
            "regs".into(),
            "0x0".into(),
            "0x40".into(),
            "32".into(),
            vec![],
            vec![RegisterFile::new(
                "lane".into(),
                "0x10".into(),
                "0x10".into(),
                "4".into(),
                vec![Register::new(
                    "lane".into(),
                    "0x0".into(),
                    "32".into(),
                    vec![Field::new(
                        "enable".into(),
                        "0".into(),
                        "1".into(),
                        "RW".into(),
                        "0".into(),
                        "Enable".into(),
                    )],
                )],
            )],
        )],
    );

    let xml = ip_xact::serialize(&component).expect("component should serialize");

    assert!(xml.contains("<ipxact:registerFile>"));
    assert!(xml.contains("<ipxact:dim>4</ipxact:dim>"));
    assert!(xml.contains("<ipxact:addressOffset>0x10</ipxact:addressOffset>"));
    assert!(xml.contains("<ipxact:range>0x10</ipxact:range>"));
    assert!(compact_xml(&xml).contains("<ipxact:register><ipxact:name>lane</ipxact:name>"));
}

#[test]
fn emits_testable_and_reserved_field_access_policy() {
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
                vec![
                    Field::new_with_options(FieldOptions {
                        name: "skip_compare".into(),
                        offset: "0".into(),
                        width: "1".into(),
                        attr: "RW".into(),
                        reset: String::new(),
                        desc: String::new(),
                        hdl_path: None,
                        testable: Some(false),
                        reserved: false,
                    }),
                    Field::new(
                        "reserved0".into(),
                        "1".into(),
                        "1".into(),
                        "RO".into(),
                        "0".into(),
                        String::new(),
                    ),
                ],
            )],
        )],
    );

    let xml = ip_xact::serialize(&component).expect("component should serialize");

    assert!(xml.contains("<ipxact:testable>false</ipxact:testable>"));
    assert!(xml.contains("<ipxact:reserved>true</ipxact:reserved>"));
    assert!(!xml.contains("<ipxact:value></ipxact:value>"));
}
