use irgen_ipxact_parser::{ParseOptions, parse_ipxact, parse_ipxact_with_options};

const IPXACT_COMMON: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>example.com</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>demo</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:fieldAccessPolicyDefinitions>
      <ipxact:fieldAccessPolicyDefinition>
        <ipxact:name>ro_clear_policy</ipxact:name>
        <ipxact:access>read-only</ipxact:access>
        <ipxact:readAction>clear</ipxact:readAction>
      </ipxact:fieldAccessPolicyDefinition>
    </ipxact:fieldAccessPolicyDefinitions>
    <ipxact:enumerationDefinitions>
      <ipxact:enumerationDefinition>
        <ipxact:name>ready_values</ipxact:name>
        <ipxact:width>2</ipxact:width>
        <ipxact:enumeratedValue usage="read">
          <ipxact:name>not_ready</ipxact:name>
          <ipxact:value>0</ipxact:value>
        </ipxact:enumeratedValue>
        <ipxact:enumeratedValue usage="read">
          <ipxact:name>ready</ipxact:name>
          <ipxact:value>1</ipxact:value>
        </ipxact:enumeratedValue>
      </ipxact:enumerationDefinition>
    </ipxact:enumerationDefinitions>
    <ipxact:fieldDefinitions>
      <ipxact:fieldDefinition>
        <ipxact:name>ready_field_def</ipxact:name>
        <ipxact:bitWidth>2</ipxact:bitWidth>
        <ipxact:volatile>true</ipxact:volatile>
        <ipxact:resets><ipxact:reset><ipxact:value>1</ipxact:value></ipxact:reset></ipxact:resets>
        <ipxact:fieldAccessPolicies>
          <ipxact:fieldAccessPolicy>
            <ipxact:fieldAccessPolicyDefinitionRef typeDefinitions="local_types">ro_clear_policy</ipxact:fieldAccessPolicyDefinitionRef>
          </ipxact:fieldAccessPolicy>
        </ipxact:fieldAccessPolicies>
        <ipxact:enumeratedValues>
          <ipxact:enumerationDefinitionRef typeDefinitions="local_types">ready_values</ipxact:enumerationDefinitionRef>
        </ipxact:enumeratedValues>
      </ipxact:fieldDefinition>
    </ipxact:fieldDefinitions>
    <ipxact:registerDefinitions>
      <ipxact:registerDefinition>
        <ipxact:name>common_status_def</ipxact:name>
        <ipxact:size>32</ipxact:size>
        <ipxact:accessPolicies>
          <ipxact:accessPolicy>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:accessPolicy>
        </ipxact:accessPolicies>
        <ipxact:field>
          <ipxact:name>ready</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:fieldDefinitionRef typeDefinitions="local_types">ready_field_def</ipxact:fieldDefinitionRef>
        </ipxact:field>
      </ipxact:registerDefinition>
    </ipxact:registerDefinitions>
    <ipxact:addressBlockDefinitions>
      <ipxact:addressBlockDefinition>
        <ipxact:name>common_block_def</ipxact:name>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status_from_def</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:registerDefinitionRef typeDefinitions="local_types">common_status_def</ipxact:registerDefinitionRef>
        </ipxact:register>
      </ipxact:addressBlockDefinition>
    </ipxact:addressBlockDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>demo</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
        <ipxact:range>0x100</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>`REGS_HDL_PATH</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:addressOffset>0x4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>done</ipxact:name>
            <ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>done_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:resets><ipxact:reset><ipxact:value>0x1</ipxact:value></ipxact:reset></ipxact:resets>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
        <ipxact:registerFile>
          <ipxact:name>lane</ipxact:name>
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>top.u_regs.lane</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:array>
            <ipxact:dim>4</ipxact:dim>
          </ipxact:array>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:register>
            <ipxact:name>ctrl</ipxact:name>
            <ipxact:addressOffset>0x0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>enable</ipxact:name>
              <ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>enable_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:resets><ipxact:reset><ipxact:value>0</ipxact:value></ipxact:reset></ipxact:resets>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
          </ipxact:register>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

#[test]
fn parses_ipxact_register_subset() {
    let component = parse_ipxact(IPXACT_COMMON).unwrap();

    assert_eq!(component.name, "demo");
    assert_eq!(component.blocks[0].registers[0].name, "status");
    assert_eq!(
        component.blocks[0].registers[0].hdl_path.as_deref(),
        Some("`REGS_HDL_PATH")
    );
    assert_eq!(
        component.blocks[0].registers[0].fields[0]
            .hdl_path
            .as_deref(),
        Some("done_q")
    );
    assert_eq!(component.blocks[0].register_files[0].dim, "4");
}

#[test]
fn rejects_non_2022_ipxact_namespaces() {
    let xml = IPXACT_COMMON.replace(
        "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        "urn:unsupported-ipxact-namespace",
    );
    let error = parse_ipxact(&xml).unwrap_err().to_string();

    assert!(error.contains("unsupported IP-XACT namespace"), "{error}");
}

#[test]
fn rejects_pre_2022_direct_register_dims() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_dim</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:dim>2</ipxact:dim>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `direct dim` on register `status`"),
        "{error}"
    );
}

#[test]
fn rejects_pre_2022_direct_register_file_dims() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_regfile_dim</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:dim>2</ipxact:dim>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `direct dim` on registerFile `cluster`"),
        "{error}"
    );
}

#[test]
fn rejects_pre_2022_direct_field_access_elements() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_field_access</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `access` on field `ready`"),
        "{error}"
    );
}

#[test]
fn reports_requested_access_handle_view_without_matching_or_generic_path() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_view_path</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
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
          </ipxact:accessHandles>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_view: Some("rtl".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap_err()
    .to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle for `status` does not define requested view `rtl` and has no generic fallback"
        ),
        "{error}"
    );
}

#[test]
fn reports_quoted_access_handle_path_segments() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>quoted_path</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:pathSegments>
                <ipxact:pathSegment>"rtl.status"</ipxact:pathSegment>
              </ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle pathSegment for `status` must not include SystemVerilog string quotes: `\"rtl.status\"`"
        ),
        "{error}"
    );
}

#[test]
fn rejects_pre_2022_path_segment_name_elements() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_path_segment</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:pathSegments>
                <ipxact:pathSegment><ipxact:pathSegmentName>rtl.status</ipxact:pathSegmentName></ipxact:pathSegment>
              </ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT element `pathSegmentName` in IEEE 1685-2022 input"),
        "{error}"
    );
}

#[test]
fn reports_requested_register_access_policy_mode_without_matching_or_generic_policy() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_register_mode_policy</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:accessPolicies>
            <ipxact:accessPolicy>
              <ipxact:modeRef>sleep</ipxact:modeRef>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:accessPolicy>
          </ipxact:accessPolicies>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("diagnostic".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap_err()
    .to_string();

    assert!(
        error.contains(
            "IP-XACT register `status` access policy does not define requested mode `diagnostic` and has no generic fallback"
        ),
        "{error}"
    );
}

#[test]
fn reports_requested_address_block_access_policy_mode_without_matching_or_generic_policy() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_block_mode_policy</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:accessPolicies>
          <ipxact:accessPolicy>
            <ipxact:modeRef>sleep</ipxact:modeRef>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:accessPolicy>
        </ipxact:accessPolicies>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("diagnostic".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap_err()
    .to_string();

    assert!(
        error.contains(
            "IP-XACT addressBlock `cfg` access policy does not define requested mode `diagnostic` and has no generic fallback"
        ),
        "{error}"
    );
}

#[test]
fn reports_requested_field_access_policy_mode_without_matching_or_generic_policy() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_field_mode_policy</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modeRef>sleep</ipxact:modeRef>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("diagnostic".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap_err()
    .to_string();

    assert!(
        error.contains(
            "IP-XACT field `ready` access policy does not define requested mode `diagnostic` and has no generic fallback"
        ),
        "{error}"
    );
}

#[test]
fn rejects_pre_2022_reset_type_ref_elements() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_reset_type_ref</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:resetTypeRef>SOFT</ipxact:resetTypeRef>
                <ipxact:value>0</ipxact:value>
              </ipxact:reset>
            </ipxact:resets>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT element `resetTypeRef` in IEEE 1685-2022 input"),
        "{error}"
    );
}

#[test]
fn reports_invalid_or_unsupported_ipxact_boolean_metadata() {
    let invalid = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>invalid_bool</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:reserved>maybe</ipxact:reserved>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;
    let error = parse_ipxact(invalid).unwrap_err();
    assert_eq!(
        error.to_string(),
        "invalid IP-XACT boolean for field reserved: `maybe`"
    );

    let unsupported = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>unsupported_bool</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="BAD_BOOL">
      <ipxact:name>BAD_BOOL</ipxact:name>
      <ipxact:value>UNSUPPORTED_FUNC(1)</ipxact:value>
    </ipxact:parameter>
  </ipxact:parameters>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:volatile>BAD_BOOL != 0</ipxact:volatile>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;
    let error = parse_ipxact(unsupported).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unsupported IP-XACT parameter expression `BAD_BOOL` used in field volatile: `UNSUPPORTED_FUNC(1)`"
    );
}

#[test]
fn reports_unsupported_parameter_expressions_when_used_by_numeric_fields() {
    let unused = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>unused_bad_param</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="BAD_EXPR">
      <ipxact:name>BAD_EXPR</ipxact:name>
      <ipxact:value>UNSUPPORTED_FUNC(4)</ipxact:value>
    </ipxact:parameter>
  </ipxact:parameters>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;
    assert!(parse_ipxact(unused).is_ok());

    let used_parameter = unused
        .replace(
            "<ipxact:name>unused_bad_param</ipxact:name>",
            "<ipxact:name>used_bad_param</ipxact:name>",
        )
        .replace(
            "<ipxact:baseAddress>0</ipxact:baseAddress>",
            "<ipxact:baseAddress>BAD_EXPR + 4</ipxact:baseAddress>",
        );
    let error = parse_ipxact(&used_parameter).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unsupported IP-XACT parameter expression `BAD_EXPR` used in addressBlock baseAddress: `UNSUPPORTED_FUNC(4)`"
    );

    let used_configurable = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bad_configurable</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:configurableElementValues>
    <ipxact:configurableElementValue referenceId="BAD_BASE">UNSUPPORTED_FUNC(8)</ipxact:configurableElementValue>
  </ipxact:configurableElementValues>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>BAD_BASE + 4</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;
    let error = parse_ipxact(used_configurable).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unsupported IP-XACT parameter expression `BAD_BASE` used in addressBlock baseAddress: `UNSUPPORTED_FUNC(8)`"
    );
}

#[test]
fn rejects_unsupported_ipxact_is_present_elements() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>bad_present</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>conditional</ipxact:name>
          <ipxact:isPresent>false</ipxact:isPresent>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>bit0</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err();
    assert_eq!(
        error.to_string(),
        "unsupported IP-XACT element `isPresent` in IEEE 1685-2022 input"
    );
}

#[test]
fn reports_duplicate_memory_map_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>duplicate_maps</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>ctrls</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>stats</ipxact:name>
        <ipxact:baseAddress>4</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT memoryMap name `cfg`"),
        "{error}"
    );
}

#[test]
fn reports_duplicate_address_block_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>duplicate_blocks</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>4</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT addressBlock name `regs` under memoryMap `cfg`"),
        "{error}"
    );
}

#[test]
fn reports_missing_internal_type_definition_refs() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>missing_refs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>defs</ipxact:name>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:addressBlockDefinitionRef typeDefinitions="defs">missing_block</ipxact:addressBlockDefinitionRef>
        <ipxact:baseAddress>0</ipxact:baseAddress>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT addressBlockDefinition not found: `defs::missing_block`"),
        "{error}"
    );
}

#[test]
fn reports_missing_field_access_policy_definition_refs() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>missing_policy</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>defs</ipxact:name>
    <ipxact:fieldAccessPolicyDefinitions/>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:fieldAccessPolicyDefinitionRef typeDefinitions="defs">missing_policy</ipxact:fieldAccessPolicyDefinitionRef>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT fieldAccessPolicyDefinition not found: `defs::missing_policy`"),
        "{error}"
    );
}

#[test]
fn reports_duplicate_field_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>duplicate_fields</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT field name `ready` under register `status`"),
        "{error}"
    );
}

#[test]
fn reports_duplicate_enumerated_value_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>duplicate_enums</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>ctrl</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>mode</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>2</ipxact:bitWidth>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>idle</ipxact:name>
                <ipxact:value>0</ipxact:value>
              </ipxact:enumeratedValue>
              <ipxact:enumeratedValue>
                <ipxact:name>idle</ipxact:name>
                <ipxact:value>1</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT enumeratedValue name `idle` under field `mode`"),
        "{error}"
    );
}

#[test]
fn rejects_pre_2022_subspace_map_master_ref() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>legacy_master_ref</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>host</ipxact:name>
      <ipxact:subspaceMap masterRef="cpu">
        <ipxact:name>external_window</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_ipxact(xml).unwrap_err().to_string();

    assert!(
        error.contains("missing required IP-XACT element `initiatorRef`"),
        "{error}"
    );
}
