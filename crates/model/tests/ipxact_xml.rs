use irgen_model::base::{Block, Component, Field, Register, RegisterFile};

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

#[test]
fn emits_hdl_path_for_all_ipxact_versions() {
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

    let snps_field_path = r#"<snps:field xmlns:snps="http://www.synopsys.com"><snps:hdl_path>u_status.done_q</snps:hdl_path></snps:field>"#;
    let snps_block_path = r#"<snps:addressBlock xmlns:snps="http://www.synopsys.com"><snps:hdl_path>`REGS_HDL_PATH</snps:hdl_path></snps:addressBlock>"#;
    let access_handle_2014 = r#"<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment><ipxact:pathSegmentName>u_status.done_q</ipxact:pathSegmentName></ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>"#;
    let block_access_handle_2014 =
        r#"<ipxact:pathSegmentName>`REGS_HDL_PATH</ipxact:pathSegmentName>"#;
    let access_handle_2022 = r#"<ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>u_status.done_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>"#;
    let block_access_handle_2022 = r#"<ipxact:pathSegment>`REGS_HDL_PATH</ipxact:pathSegment>"#;
    let reserved_snps_path = "<snps:hdl_path>reserved0</snps:hdl_path>";
    let disabled_snps_path = "<snps:hdl_path>no_path</snps:hdl_path>";
    let reserved_access_path_2014 = "<ipxact:pathSegmentName>reserved0</ipxact:pathSegmentName>";
    let disabled_access_path_2014 = "<ipxact:pathSegmentName>no_path</ipxact:pathSegmentName>";
    let reserved_access_path_2022 = "<ipxact:pathSegment>reserved0</ipxact:pathSegment>";
    let disabled_access_path_2022 = "<ipxact:pathSegment>no_path</ipxact:pathSegment>";

    let ipxact_2009 =
        irgen_model::serialize_ipxact_2009_xml(&component).expect("component should serialize");
    let ipxact_2014 =
        irgen_model::serialize_ipxact_xml(&component).expect("component should serialize");
    let ipxact_2022 =
        irgen_model::serialize_ipxact_2022_xml(&component).expect("component should serialize");

    assert!(ipxact_2009.contains(snps_field_path));
    assert!(ipxact_2009.contains(snps_block_path));
    assert!(ipxact_2014.contains(access_handle_2014));
    assert!(ipxact_2014.contains(block_access_handle_2014));
    assert!(ipxact_2022.contains(access_handle_2022));
    assert!(ipxact_2022.contains(block_access_handle_2022));
    assert!(!ipxact_2014.contains("snps:hdl_path"));
    assert!(!ipxact_2022.contains("snps:hdl_path"));
    assert!(!ipxact_2009.contains(reserved_snps_path));
    assert!(!ipxact_2009.contains(disabled_snps_path));
    assert!(!ipxact_2014.contains(reserved_access_path_2014));
    assert!(!ipxact_2014.contains(disabled_access_path_2014));
    assert!(!ipxact_2022.contains(reserved_access_path_2022));
    assert!(!ipxact_2022.contains(disabled_access_path_2022));
}

#[test]
fn emits_register_csr_setting_vendor_extension_for_all_ipxact_versions() {
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
            vec![
                Register::new_with_description_and_csr_setting(
                    "skip".into(),
                    "0x0".into(),
                    "32".into(),
                    String::new(),
                    Some("NO_CSR_TEST".into()),
                    vec![Field::new(
                        "value".into(),
                        "0".into(),
                        "32".into(),
                        "RW".into(),
                        "0".into(),
                        String::new(),
                    )],
                ),
                Register::new(
                    "normal".into(),
                    "0x4".into(),
                    "32".into(),
                    vec![Field::new(
                        "value".into(),
                        "0".into(),
                        "32".into(),
                        "RW".into(),
                        "0".into(),
                        String::new(),
                    )],
                ),
            ],
        )],
    );

    let expected = r#"<snps:register xmlns:snps="http://www.synopsys.com"><snps:csrSetting>NO_CSR_TEST</snps:csrSetting></snps:register>"#;

    let ipxact_2009 =
        irgen_model::serialize_ipxact_2009_xml(&component).expect("component should serialize");
    let ipxact_2014 =
        irgen_model::serialize_ipxact_xml(&component).expect("component should serialize");
    let ipxact_2022 =
        irgen_model::serialize_ipxact_2022_xml(&component).expect("component should serialize");

    assert!(ipxact_2009.contains(expected));
    assert!(ipxact_2014.contains(expected));
    assert!(ipxact_2022.contains(expected));
    assert_eq!(ipxact_2009.matches("<snps:csrSetting>").count(), 1);
    assert_eq!(ipxact_2014.matches("<snps:csrSetting>").count(), 1);
    assert_eq!(ipxact_2022.matches("<snps:csrSetting>").count(), 1);
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

    let xml = irgen_model::serialize_ipxact_xml(&component).expect("component should serialize");

    assert!(xml.contains("<ipxact:registerFile>"));
    assert!(xml.contains("<ipxact:dim>4</ipxact:dim>"));
    assert!(xml.contains("<ipxact:addressOffset>0x10</ipxact:addressOffset>"));
    assert!(xml.contains("<ipxact:range>0x10</ipxact:range>"));
    assert!(xml.contains("<ipxact:register><ipxact:name>lane</ipxact:name>"));
}
