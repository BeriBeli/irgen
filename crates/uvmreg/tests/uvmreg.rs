use irgen_uvmreg::{
    ParseOptions, RenderOptions, ipxact_to_uvm_reg, parse_ipxact, parse_ipxact_with_options,
    parse_ipxact_with_resolver, serialize_uvm_reg_with_options,
};

const IPXACT_2014: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2014">
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
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment><ipxact:pathSegmentName>`REGS_HDL_PATH</ipxact:pathSegmentName></ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:addressOffset>0x4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>done</ipxact:name>
            <ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment><ipxact:pathSegmentName>done_q</ipxact:pathSegmentName></ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:resets><ipxact:reset><ipxact:value>0x1</ipxact:value></ipxact:reset></ipxact:resets>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
        <ipxact:registerFile>
          <ipxact:name>lane</ipxact:name>
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment><ipxact:pathSegmentName>top.u_regs.lane</ipxact:pathSegmentName></ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:dim>4</ipxact:dim>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:register>
            <ipxact:name>ctrl</ipxact:name>
            <ipxact:addressOffset>0x0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>enable</ipxact:name>
              <ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment><ipxact:pathSegmentName>enable_q</ipxact:pathSegmentName></ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:resets><ipxact:reset><ipxact:value>0</ipxact:value></ipxact:reset></ipxact:resets>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

const IPXACT_2022: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x30</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:accessPolicies>
          <ipxact:accessPolicy>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:accessPolicy>
        </ipxact:accessPolicies>
        <ipxact:register>
          <ipxact:name>irq</ipxact:name>
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>`IRQ_HDL_PATH</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>clear</ipxact:name>
            <ipxact:accessHandles><ipxact:accessHandle><ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>clear_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices></ipxact:accessHandle></ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>0</ipxact:value>
                <ipxact:mask>1</ipxact:mask>
              </ipxact:reset>
              <ipxact:reset>
                <ipxact:resetTypeRef>SOFT</ipxact:resetTypeRef>
                <ipxact:value>1</ipxact:value>
                <ipxact:mask>1</ipxact:mask>
              </ipxact:reset>
            </ipxact:resets>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modeRef priority="1">diagnostic</ipxact:modeRef>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                <ipxact:modifiedWriteValue>oneToClear</ipxact:modifiedWriteValue>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:alternateRegisters>
            <ipxact:alternateRegister>
              <ipxact:name>debug_irq</ipxact:name>
              <ipxact:modeRef priority="1">diagnostic</ipxact:modeRef>
              <ipxact:volatile>false</ipxact:volatile>
              <ipxact:accessPolicies>
                <ipxact:accessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:accessPolicy>
              </ipxact:accessPolicies>
              <ipxact:field>
                <ipxact:name>raw</ipxact:name>
                <ipxact:bitOffset>0</ipxact:bitOffset>
                <ipxact:bitWidth>8</ipxact:bitWidth>
              </ipxact:field>
            </ipxact:alternateRegister>
          </ipxact:alternateRegisters>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>block_status</ipxact:name>
          <ipxact:addressOffset>0x4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>state</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>4</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:writeValueConstraint>
                  <ipxact:useEnumeratedValues>true</ipxact:useEnumeratedValues>
                </ipxact:writeValueConstraint>
                <ipxact:broadcasts>
                  <ipxact:broadcastTo>
                    <ipxact:memoryMapRef memoryMapRef="demo"/>
                    <ipxact:addressBlockRef addressBlockRef="regs"/>
                    <ipxact:registerRef registerRef="gate"/>
                    <ipxact:fieldRef fieldRef="doorbell"/>
                  </ipxact:broadcastTo>
                </ipxact:broadcasts>
                <ipxact:accessRestrictions>
                  <ipxact:accessRestriction>
                    <ipxact:modeRef priority="0">diagnostic</ipxact:modeRef>
                    <ipxact:readAccessMask>0xf</ipxact:readAccessMask>
                    <ipxact:writeAccessMask>0x3</ipxact:writeAccessMask>
                  </ipxact:accessRestriction>
                </ipxact:accessRestrictions>
                <ipxact:testable testConstraint="readOnly">false</ipxact:testable>
                <ipxact:reserved>1</ipxact:reserved>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue usage="read">
                <ipxact:name>idle</ipxact:name>
                <ipxact:value>0</ipxact:value>
              </ipxact:enumeratedValue>
              <ipxact:enumeratedValue usage="read-write">
                <ipxact:name>busy</ipxact:name>
                <ipxact:value>0b1</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>counter</ipxact:name>
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:dim>1</ipxact:dim>
            <ipxact:stride>8</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0x10</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:volatile>true</ipxact:volatile>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                <ipxact:writeValueConstraint>
                  <ipxact:minimum>0x2</ipxact:minimum>
                  <ipxact:maximum>0xf</ipxact:maximum>
                </ipxact:writeValueConstraint>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>gate</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:accessPolicies>
            <ipxact:accessPolicy>
              <ipxact:modeRef priority="1">diagnostic</ipxact:modeRef>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:accessPolicy>
            <ipxact:accessPolicy>
              <ipxact:access>write-only</ipxact:access>
            </ipxact:accessPolicy>
          </ipxact:accessPolicies>
          <ipxact:field>
            <ipxact:name>doorbell</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>packet_mem</ipxact:name>
        <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>`PKT_MEM_HDL_PATH</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
        <ipxact:baseAddress>0x2000</ipxact:baseAddress>
        <ipxact:range>0x100</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:usage>memory</ipxact:usage>
        <ipxact:accessPolicies>
          <ipxact:accessPolicy>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:accessPolicy>
        </ipxact:accessPolicies>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>from_definition</ipxact:name>
        <ipxact:baseAddress>0x2400</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="local_types">common_block_def</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
      <ipxact:subspaceMap initiatorRef="dma_init" segmentRef="cfg_seg">
        <ipxact:name>dma_window</ipxact:name>
        <ipxact:baseAddress>0x2800</ipxact:baseAddress>
      </ipxact:subspaceMap>
      <ipxact:memoryRemap>
        <ipxact:name>low_power</ipxact:name>
        <ipxact:modeRef priority="0">sleep</ipxact:modeRef>
        <ipxact:addressBlock>
          <ipxact:name>lp_regs</ipxact:name>
          <ipxact:baseAddress>0x4000</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>wake</ipxact:name>
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>cause</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>4</ipxact:bitWidth>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
        <ipxact:subspaceMap initiatorRef="lp_init">
          <ipxact:name>lp_window</ipxact:name>
          <ipxact:baseAddress>0x5000</ipxact:baseAddress>
        </ipxact:subspaceMap>
      </ipxact:memoryRemap>
      <ipxact:bank bankAlignment="serial">
        <ipxact:name>banked</ipxact:name>
        <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>top.u_banked</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
        <ipxact:baseAddress>0x3000</ipxact:baseAddress>
        <ipxact:addressBlock>
          <ipxact:name>ctl</ipxact:name>
          <ipxact:accessHandles><ipxact:accessHandle><ipxact:pathSegments><ipxact:pathSegment>ctl_regs</ipxact:pathSegment></ipxact:pathSegments></ipxact:accessHandle></ipxact:accessHandles>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>mode</ipxact:name>
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>en</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
        <ipxact:addressBlock>
          <ipxact:name>stat</ipxact:name>
          <ipxact:range>0x20</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>value</ipxact:name>
            <ipxact:addressOffset>4</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>code</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>8</ipxact:bitWidth>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:bank>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

#[test]
fn parses_ipxact_register_subset() {
    let component = parse_ipxact(IPXACT_2014).unwrap();

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
fn parses_all_supported_ipxact_namespaces() {
    let variants = [
        (
            "spirit",
            "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.4",
        ),
        (
            "spirit",
            "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1.5",
        ),
        (
            "spirit",
            "http://www.spiritconsortium.org/XMLSchema/SPIRIT/1685-2009",
        ),
        (
            "ipxact",
            "http://www.accellera.org/XMLSchema/IPXACT/1685-2014",
        ),
        (
            "ipxact",
            "http://www.accellera.org/XMLSchema/IPXACT/1685-2022",
        ),
    ];

    for (prefix, namespace) in variants {
        let xml = IPXACT_2014.replace("ipxact", prefix).replace(
            "http://www.accellera.org/XMLSchema/IPXACT/1685-2014",
            namespace,
        );
        let component = parse_ipxact(&xml).unwrap();
        assert_eq!(component.name, "demo");
    }
}

#[test]
fn renders_uvm_ieee_2020_register_model() {
    let sv = ipxact_to_uvm_reg(IPXACT_2014).unwrap();

    assert!(sv.contains("`ifndef RAL_DEMO_SV"));
    assert!(sv.contains("class ral_reg_regs_status extends uvm_reg;"));
    assert!(sv.contains("class ral_regfile_regs_lane extends uvm_reg_file;"));
    assert!(sv.contains("class ral_block_regs extends uvm_reg_block;"));
    assert!(sv.contains("class ral_sys_demo extends uvm_reg_block;"));
    assert!(sv.contains("rand ral_block_regs regs;"));
    assert!(sv.contains("done.configure(this, 1, 0, \"RO\", 1'b0, 1'h1, 1'b1, 1'b0, 1);"));
    assert!(sv.contains("status.add_hdl_path_slice({`REGS_HDL_PATH, \".done_q\"}, 0, 1, 1'b1);"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(regs.default_map, `UVM_REG_ADDR_WIDTH'h1000);"));
    assert!(sv.contains("rand ral_reg_regs_lane_ctrl ctrl;"));
    assert!(sv.contains("ctrl.add_hdl_path_slice(\"top.u_regs.lane.enable_q\", 0, 1, 1'b1);"));
    assert!(sv.contains("mp.add_reg(ctrl, offset + `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
    assert!(sv.contains(
        "lane[i].map(default_map, `UVM_REG_ADDR_WIDTH'h20 + i * `UVM_REG_ADDR_WIDTH'h10);"
    ));
}

#[test]
fn prefers_generic_access_handle_over_view_specific_paths() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>view_paths</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
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
            <ipxact:name>ready</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:viewRef>gate</ipxact:viewRef>
                <ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>gate_ready</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices>
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:slices><ipxact:slice><ipxact:pathSegments><ipxact:pathSegment>ready_q</ipxact:pathSegment></ipxact:pathSegments></ipxact:slice></ipxact:slices>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>fallback</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:viewRef>rtl</ipxact:viewRef>
              <ipxact:pathSegments><ipxact:pathSegment>rtl.fallback</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(
        component.blocks[0].registers[0].hdl_path.as_deref(),
        Some("rtl.status")
    );
    assert_eq!(
        component.blocks[0].registers[0].fields[0]
            .hdl_path
            .as_deref(),
        Some("ready_q")
    );
    assert_eq!(
        component.blocks[0].registers[1].hdl_path.as_deref(),
        Some("rtl.fallback")
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("status.add_hdl_path_slice(\"rtl.status.ready_q\", 0, 1, 1'b1);"));
    assert!(sv.contains("fallback.add_hdl_path_slice(\"rtl.fallback\", -1, -1, 1'b1);"));
    assert!(!sv.contains("gate.status"));
    assert!(!sv.contains("gate_ready"));

    let gate_component = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_view: Some("gate".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap();
    assert_eq!(
        gate_component.blocks[0].registers[0].hdl_path.as_deref(),
        Some("gate.status")
    );
    assert_eq!(
        gate_component.blocks[0].registers[0].fields[0]
            .hdl_path
            .as_deref(),
        Some("gate_ready")
    );

    let gate_sv =
        serialize_uvm_reg_with_options(&gate_component, RenderOptions::default()).unwrap();
    assert!(gate_sv.contains("status.add_hdl_path_slice(\"gate.status.gate_ready\", 0, 1, 1'b1);"));
}

#[test]
fn renders_multiple_hdl_slices_for_split_fields() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>split_paths</ipxact:name>
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
              <ipxact:pathSegments><ipxact:pathSegment>top.u_regs.status</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>payload</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:slices>
                  <ipxact:slice>
                    <ipxact:pathSegments><ipxact:pathSegment>payload_hi</ipxact:pathSegment></ipxact:pathSegments>
                    <ipxact:range><ipxact:left>3</ipxact:left><ipxact:right>0</ipxact:right></ipxact:range>
                  </ipxact:slice>
                  <ipxact:slice>
                    <ipxact:pathSegments><ipxact:pathSegment>payload_lo</ipxact:pathSegment></ipxact:pathSegments>
                    <ipxact:range><ipxact:left>3</ipxact:left><ipxact:right>0</ipxact:right></ipxact:range>
                  </ipxact:slice>
                </ipxact:slices>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>8</ipxact:bitOffset>
            <ipxact:bitWidth>8</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let field = &component.blocks[0].registers[0].fields[0];

    assert_eq!(field.hdl_path_slices.len(), 2);
    assert_eq!(field.hdl_path_slices[0].path, "payload_hi");
    assert_eq!(field.hdl_path_slices[1].right.as_deref(), Some("0"));

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains(
        "status.add_hdl_path_slice(\"top.u_regs.status.payload_hi[3:0]\", 12, 4, 1'b1);"
    ));
    assert!(
        sv.contains(
            "status.add_hdl_path_slice(\"top.u_regs.status.payload_lo[3:0]\", 8, 4, 1'b0);"
        )
    );
}

#[test]
fn reports_duplicate_generated_class_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>dupe_classes</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>8</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status-flag</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>status_flag</ipxact:name>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = ipxact_to_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate generated SystemVerilog class name `ral_reg_cfg_status_flag`"),
        "{error}"
    );
}

#[test]
fn selects_mode_specific_access_policies_when_requested() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>mode_policies</ipxact:name>
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
              <ipxact:modeRef>diagnostic</ipxact:modeRef>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:accessPolicy>
            <ipxact:accessPolicy>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:accessPolicy>
          </ipxact:accessPolicies>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>flag</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modeRef>diagnostic</ipxact:modeRef>
                <ipxact:access>write-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>3</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let register = &component.blocks[0].registers[0];

    assert_eq!(register.access.as_deref(), Some("read-only"));
    assert_eq!(register.fields[0].access.as_deref(), Some("read-only"));

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("flag.configure(this, 1, 0, \"RO\""));
    assert!(sv.contains("value.configure(this, 3, 1, \"RO\""));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h0, \"RO\");"));

    let diagnostic = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("diagnostic".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap();
    let register = &diagnostic.blocks[0].registers[0];

    assert_eq!(register.access.as_deref(), Some("read-write"));
    assert_eq!(register.fields[0].access.as_deref(), Some("write-only"));

    let sv = serialize_uvm_reg_with_options(&diagnostic, RenderOptions::default()).unwrap();
    assert!(sv.contains("flag.configure(this, 1, 0, \"WO\""));
    assert!(sv.contains("value.configure(this, 3, 1, \"RW\""));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
}

#[test]
fn selects_lowest_priority_mode_access_policy() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>priority_mode_policies</ipxact:name>
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
              <ipxact:modeRef priority="5">diagnostic</ipxact:modeRef>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:accessPolicy>
            <ipxact:accessPolicy>
              <ipxact:modeRef priority="1">diagnostic</ipxact:modeRef>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:accessPolicy>
            <ipxact:accessPolicy>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:accessPolicy>
          </ipxact:accessPolicies>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>flag</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modeRef priority="3">diagnostic</ipxact:modeRef>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
              <ipxact:fieldAccessPolicy>
                <ipxact:modeRef priority="0">diagnostic</ipxact:modeRef>
                <ipxact:access>write-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>3</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let generic = parse_ipxact(xml).unwrap();
    assert_eq!(
        generic.blocks[0].registers[0].access.as_deref(),
        Some("read-only")
    );
    assert_eq!(
        generic.blocks[0].registers[0].fields[0].access.as_deref(),
        Some("read-only")
    );

    let diagnostic = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("diagnostic".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap();
    let register = &diagnostic.blocks[0].registers[0];

    assert_eq!(register.access.as_deref(), Some("read-write"));
    assert_eq!(register.fields[0].access.as_deref(), Some("write-only"));

    let sv = serialize_uvm_reg_with_options(&diagnostic, RenderOptions::default()).unwrap();
    assert!(sv.contains("flag.configure(this, 1, 0, \"WO\""));
    assert!(sv.contains("value.configure(this, 3, 1, \"RW\""));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
}

#[test]
fn optionally_renders_register_bit_coverage() {
    let component = parse_ipxact(IPXACT_2014).unwrap();
    let sv = serialize_uvm_reg_with_options(&component, RenderOptions { coverage: true }).unwrap();

    assert!(sv.contains("local uvm_reg_data_t m_data;"));
    assert!(sv.contains("covergroup cg_bits();"));
    assert!(sv.contains("done_bits: coverpoint {m_data[0:0], m_is_read} iff (m_be);"));
    assert!(sv.contains("super.new(name, 32, build_coverage(UVM_CVR_REG_BITS));"));
    assert!(sv.contains("add_coverage(build_coverage(UVM_CVR_REG_BITS));"));
    assert!(sv.contains("if (get_coverage(UVM_CVR_REG_BITS)) begin"));
    assert!(sv.contains("cg_bits.sample();"));
}

#[test]
fn renders_ipxact_2022_field_access_policies() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let clear = &component.blocks[0].registers[0].fields[0];
    let state = &component.blocks[0].registers[1].fields[0];

    assert_eq!(clear.reset.as_deref(), Some("0"));
    assert_eq!(clear.resets.len(), 2);
    assert_eq!(clear.resets[1].reset_type.as_deref(), Some("SOFT"));
    assert_eq!(clear.resets[1].value, "1");
    assert_eq!(clear.access.as_deref(), Some("read-write"));
    assert_eq!(state.testable.as_deref(), Some("false"));
    assert_eq!(state.reserved.as_deref(), Some("1"));

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();

    assert!(sv.contains("clear.configure(this, 1, 0, \"W1C\", 1'b0, 1'h0, 1'b1, 1'b1, 1);"));
    assert!(sv.contains("clear.set_reset(1'h1, \"SOFT\");"));
    assert!(!sv.contains("clear.set_compare"));
    assert!(sv.contains("state.set_compare(UVM_NO_CHECK);"));
    assert!(sv.contains("irq.add_hdl_path_slice({`IRQ_HDL_PATH, \".clear_q\"}, 0, 1, 1'b1);"));
}

#[test]
fn renders_ipxact_alternate_registers() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let alternate = &component.blocks[0].registers[0].alternate_registers[0];

    assert_eq!(alternate.name, "debug_irq");
    assert_eq!(alternate.access.as_deref(), Some("read-only"));
    assert_eq!(alternate.fields[0].name, "raw");

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_regs_irq_debug_irq extends uvm_reg;"));
    assert!(sv.contains("raw.configure(this, 8, 0, \"RO\", 1'b0, 8'h0, 1'b0, 1'b0, 1);"));
    assert!(sv.contains("rand ral_reg_regs_irq_debug_irq debug_irq;"));
    assert!(sv.contains("default_map.add_reg(debug_irq, `UVM_REG_ADDR_WIDTH'h0, \"RO\");"));
}

#[test]
fn inherits_block_and_register_access_policies() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let block = &component.blocks[0];
    let block_status = &block.registers[1];
    let gate = &block.registers[3];

    assert_eq!(block.access.as_deref(), Some("read-only"));
    assert_eq!(block_status.fields[0].access.as_deref(), None);
    assert_eq!(block_status.fields[0].enumerated_values.len(), 2);
    assert_eq!(gate.access.as_deref(), Some("write-only"));
    assert_eq!(gate.fields[0].access.as_deref(), None);

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("state.configure(this, 4, 0, \"RO\", 1'b0, 4'h0, 1'b0, 1'b0, 1);"));
    assert!(sv.contains("typedef enum bit [3:0] {"));
    assert!(sv.contains("STATE_IDLE = 4'h0,"));
    assert!(sv.contains("STATE_BUSY = 4'h1"));
    assert!(sv.contains("} state_e;"));
    assert!(sv.contains("default_map.add_reg(block_status, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains("doorbell.configure(this, 1, 0, \"WO\", 1'b0, 1'h0, 1'b0, 1'b1, 1);"));
    assert!(sv.contains("default_map.add_reg(gate, `UVM_REG_ADDR_WIDTH'h20, \"WO\");"));
}

#[test]
fn renders_ipxact_memory_blocks_as_uvm_mem() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let memory = &component.blocks[1];

    assert_eq!(memory.name, "packet_mem");
    assert_eq!(memory.usage.as_deref(), Some("memory"));
    assert_eq!(memory.access.as_deref(), Some("read-only"));
    assert_eq!(memory.hdl_path.as_deref(), Some("`PKT_MEM_HDL_PATH"));

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("uvm_mem packet_mem;"));
    assert!(sv.contains("packet_mem = new(\"packet_mem\", 64, 32, \"RO\", UVM_NO_COVERAGE);"));
    assert!(sv.contains("packet_mem.configure(this, `PKT_MEM_HDL_PATH);"));
    assert!(sv.contains("default_map.add_mem(packet_mem, `UVM_REG_ADDR_WIDTH'h0, \"RO\");"));
    assert!(
        sv.contains("default_map.add_submap(packet_mem.default_map, `UVM_REG_ADDR_WIDTH'h2000);")
    );
}

#[test]
fn optionally_renders_memory_address_coverage() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let sv = serialize_uvm_reg_with_options(&component, RenderOptions { coverage: true }).unwrap();

    assert!(sv.contains("class ral_mem_packet_mem extends uvm_mem;"));
    assert!(sv.contains("`uvm_object_utils(ral_mem_packet_mem)"));
    assert!(sv.contains("covergroup cg_addr();"));
    assert!(sv.contains("offset: coverpoint m_offset;"));
    assert!(sv.contains("access: coverpoint m_is_read;"));
    assert!(sv.contains("super.new(name, 64, 32, \"RO\", build_coverage(UVM_CVR_ADDR_MAP));"));
    assert!(sv.contains("add_coverage(build_coverage(UVM_CVR_ADDR_MAP));"));
    assert!(sv.contains("if (get_coverage(UVM_CVR_ADDR_MAP)) begin"));
    assert!(sv.contains("cg_addr.sample();"));
    assert!(sv.contains("ral_mem_packet_mem packet_mem;"));
    assert!(sv.contains("packet_mem = new(\"packet_mem\");"));
    assert!(!sv.contains("packet_mem = new(\"packet_mem\", 64, 32, \"RO\", UVM_NO_COVERAGE);"));
}

#[test]
fn expands_local_address_block_and_register_definitions() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let block = &component.blocks[2];
    let register = &block.registers[0];

    assert_eq!(block.name, "from_definition");
    assert_eq!(block.base_address, "0x2400");
    assert_eq!(block.range, "0x10");
    assert_eq!(register.name, "status_from_def");
    assert_eq!(register.size, "32");
    assert_eq!(register.access.as_deref(), Some("read-only"));
    assert_eq!(register.fields[0].name, "ready");
    assert_eq!(register.fields[0].bit_width, "2");
    assert_eq!(register.fields[0].volatile.as_deref(), Some("true"));
    assert_eq!(register.fields[0].read_action.as_deref(), Some("clear"));
    assert_eq!(register.fields[0].reset.as_deref(), Some("1"));
    assert_eq!(register.fields[0].enumerated_values.len(), 2);

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_from_definition_status_from_def extends uvm_reg;"));
    assert!(sv.contains("READY_NOT_READY = 2'h0,"));
    assert!(sv.contains("READY_READY = 2'h1"));
    assert!(sv.contains("ready.configure(this, 2, 0, \"RC\", 1'b1, 2'h1, 1'b1, 1'b0, 1);"));
    assert!(sv.contains("default_map.add_reg(status_from_def, `UVM_REG_ADDR_WIDTH'h0, \"RO\");"));
    assert!(sv.contains(
        "default_map.add_submap(from_definition.default_map, `UVM_REG_ADDR_WIDTH'h2400);"
    ));
}

#[test]
fn flattens_serial_banks_into_address_blocks() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let ctl = &component.blocks[3];
    let stat = &component.blocks[4];

    assert_eq!(ctl.name, "banked_ctl");
    assert_eq!(ctl.base_address, "0x3000");
    assert_eq!(ctl.hdl_path.as_deref(), Some("top.u_banked.ctl_regs"));
    assert_eq!(stat.name, "banked_stat");
    assert_eq!(stat.base_address, "0x3010");

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_banked_ctl_mode extends uvm_reg;"));
    assert!(sv.contains("default_map.add_reg(mode, `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
    assert!(sv.contains("default_map.add_reg(value, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(
        sv.contains("default_map.add_submap(banked_ctl.default_map, `UVM_REG_ADDR_WIDTH'h3000);")
    );
    assert!(
        sv.contains("default_map.add_submap(banked_stat.default_map, `UVM_REG_ADDR_WIDTH'h3010);")
    );
}

#[test]
fn resolves_top_level_subspace_maps_without_metadata_output() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let subspace = &component.subspace_maps[0];

    assert_eq!(subspace.name, "dma_window");
    assert_eq!(subspace.base_address, "0x2800");
    assert_eq!(subspace.segment_ref.as_deref(), Some("cfg_seg"));

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(!sv.contains("localparam"));
}

#[test]
fn preserves_memory_remaps_and_generates_their_registers() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let remap = &component.memory_remaps[0];

    assert_eq!(remap.name, "low_power");
    assert_eq!(remap.blocks[0].name, "low_power_lp_regs");
    assert_eq!(remap.subspace_maps[0].name, "low_power_lp_window");

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_low_power_lp_regs_wake extends uvm_reg;"));
    assert!(sv.contains("rand ral_reg_low_power_lp_regs_wake low_power_lp_regs_wake;"));
    assert!(sv.contains(
        "default_map.add_reg(low_power_lp_regs_wake, `UVM_REG_ADDR_WIDTH'h4000, \"RO\");"
    ));
}

#[test]
fn filters_memory_remaps_by_requested_mode() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>remap_modes</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:memoryRemap>
        <ipxact:name>debug</ipxact:name>
        <ipxact:modeRef>debug</ipxact:modeRef>
        <ipxact:addressBlock>
          <ipxact:name>dbg_regs</ipxact:name>
          <ipxact:baseAddress>0x4</ipxact:baseAddress>
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
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:memoryRemap>
      <ipxact:memoryRemap>
        <ipxact:name>sleep</ipxact:name>
        <ipxact:modeRef>sleep</ipxact:modeRef>
        <ipxact:addressBlock>
          <ipxact:name>sleep_regs</ipxact:name>
          <ipxact:baseAddress>0x8</ipxact:baseAddress>
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
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:memoryRemap>
      <ipxact:memoryRemap>
        <ipxact:name>common</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>common_regs</ipxact:name>
          <ipxact:baseAddress>0xc</ipxact:baseAddress>
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
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:memoryRemap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    assert_eq!(component.memory_remaps.len(), 3);

    let debug_component = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_mode: Some("debug".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap();

    assert_eq!(debug_component.memory_remaps.len(), 2);
    assert_eq!(debug_component.memory_remaps[0].name, "debug");
    assert_eq!(debug_component.memory_remaps[1].name, "common");

    let sv = serialize_uvm_reg_with_options(&debug_component, RenderOptions::default()).unwrap();
    assert!(
        sv.contains("default_map.add_reg(debug_dbg_regs_ctrl, `UVM_REG_ADDR_WIDTH'h4, \"RW\");")
    );
    assert!(
        sv.contains(
            "default_map.add_reg(common_common_regs_ctrl, `UVM_REG_ADDR_WIDTH'hc, \"RW\");"
        )
    );
    assert!(!sv.contains("sleep_sleep_regs_ctrl"));
}

#[test]
fn renders_ipxact_register_arrays() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let counter = &component.blocks[0].registers[2];

    assert_eq!(counter.name, "counter");
    assert_eq!(counter.dim, "2");
    assert_eq!(counter.dims, vec!["2".to_string(), "1".to_string()]);
    assert_eq!(counter.stride.as_deref(), Some("8"));
    assert_eq!(counter.volatile.as_deref(), Some("true"));

    let sv = ipxact_to_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("value.configure(this, 32, 0, \"RW\", 1'b1, 32'h0, 1'b0, 1'b1, 1);"));
    assert!(sv.contains("rand ral_reg_regs_counter counter[2][1];"));
    assert!(sv.contains("for (int unsigned i0 = 0; i0 < 2; i0++) begin"));
    assert!(sv.contains("for (int unsigned i1 = 0; i1 < 1; i1++) begin"));
    assert!(sv.contains("counter[i0][i1] = ral_reg_regs_counter::type_id::create($sformatf(\"counter_%0d_%0d\", i0, i1));"));
    assert!(
        sv.contains(
            "default_map.add_reg(counter[i0][i1], `UVM_REG_ADDR_WIDTH'h10 + (i0 * 1 + i1) * `UVM_REG_ADDR_WIDTH'h8, \"RW\");"
        )
    );
}

#[test]
fn renders_indexed_access_handles_for_register_arrays() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>array_paths</ipxact:name>
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
          <ipxact:name>counter</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
              <ipxact:pathSegments><ipxact:pathSegment>top.u_regs.counter0</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
            <ipxact:accessHandle>
              <ipxact:indices><ipxact:index>1</ipxact:index></ipxact:indices>
              <ipxact:pathSegments><ipxact:pathSegment>top.u_regs.counter1</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].registers[0].hdl_path, None);
    assert_eq!(component.blocks[0].registers[0].indexed_hdl_paths.len(), 2);
    assert_eq!(
        component.blocks[0].registers[0].indexed_hdl_paths[1].path,
        "top.u_regs.counter1"
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("if (i == 0) begin"));
    assert!(sv.contains("counter[i].add_hdl_path_slice(\"top.u_regs.counter0\", -1, -1, 1'b1);"));
    assert!(sv.contains("if (i == 1) begin"));
    assert!(sv.contains("counter[i].add_hdl_path_slice(\"top.u_regs.counter1\", -1, -1, 1'b1);"));
}

#[test]
fn renders_indexed_field_access_handles_for_register_arrays() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>array_field_paths</ipxact:name>
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
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u0.ready_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>1</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u1.ready_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>error</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u0.error_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>1</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u1.error_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(
        component.blocks[0].registers[0].fields[0]
            .indexed_hdl_paths
            .len(),
        2
    );
    assert_eq!(
        component.blocks[0].registers[0].fields[1].indexed_hdl_paths[1].path,
        "top.u1.error_q"
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("if (i == 0) begin"));
    assert!(sv.contains("status[i].add_hdl_path_slice(\"top.u0.ready_q\", 0, 1, 1'b1);"));
    assert!(sv.contains("status[i].add_hdl_path_slice(\"top.u0.error_q\", 1, 1, 1'b0);"));
    assert!(sv.contains("if (i == 1) begin"));
    assert!(sv.contains("status[i].add_hdl_path_slice(\"top.u1.ready_q\", 0, 1, 1'b1);"));
    assert!(sv.contains("status[i].add_hdl_path_slice(\"top.u1.error_q\", 1, 1, 1'b0);"));
}

#[test]
fn reports_indexed_access_handle_dimension_mismatch() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bad_array_paths</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x40</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u0.ready_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = ipxact_to_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle indices for register `status.ready` have 1 dimensions, expected 2"
        ),
        "{error}"
    );
}

#[test]
fn reports_duplicate_indexed_access_handle_indices() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>duplicate_indexed_paths</ipxact:name>
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
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u0.ready_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u0.ready_shadow_q</ipxact:pathSegment></ipxact:pathSegments>
              </ipxact:accessHandle>
            </ipxact:accessHandles>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = ipxact_to_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT accessHandle indices for `status.ready`: `0`"),
        "{error}"
    );
}

#[test]
fn respects_memory_map_address_unit_bits() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>word_addr</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressUnitBits>32</ipxact:addressUnitBits>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x20</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>ctrl</ipxact:name>
          <ipxact:addressOffset>0x2</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>en</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>ram</ipxact:name>
        <ipxact:baseAddress>0x40</ipxact:baseAddress>
        <ipxact:range>0x4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:usage>memory</ipxact:usage>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].address_unit_bits, "32");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("default_map = create_map(\"default_map\", 0, 4, UVM_LITTLE_ENDIAN);"));
    assert!(sv.contains("default_map.add_reg(ctrl, `UVM_REG_ADDR_WIDTH'h2, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h20);"));
    assert!(sv.contains("ram = new(\"ram\", 4, 32, \"RW\", UVM_NO_COVERAGE);"));
    assert!(sv.contains("default_map.add_mem(ram, `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(ram.default_map, `UVM_REG_ADDR_WIDTH'h40);"));
}

#[test]
fn evaluates_common_ipxact_constant_expressions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>expr_regs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>32'sh100 + 16</ipxact:baseAddress>
        <ipxact:range>'h40</ipxact:range>
        <ipxact:width>8 * 4</ipxact:width>
        <ipxact:register>
          <ipxact:name>sample</ipxact:name>
          <ipxact:array>
            <ipxact:dim>1 + 1</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>4'h4</ipxact:addressOffset>
          <ipxact:size>32'd32</ipxact:size>
          <ipxact:field>
            <ipxact:name>mode</ipxact:name>
            <ipxact:bitOffset>1 + 1</ipxact:bitOffset>
            <ipxact:bitWidth>4'b0010</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>8'h3</ipxact:value>
              </ipxact:reset>
            </ipxact:resets>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>max</ipxact:name>
                <ipxact:value>(2 * 3) + 1</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].base_address, "32'sh100 + 16");
    assert_eq!(component.blocks[0].registers[0].dim, "2");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_reg_cfg_sample extends uvm_reg;"));
    assert!(sv.contains("typedef enum bit [1:0] {"));
    assert!(sv.contains("MODE_MAX = 2'h7"));
    assert!(sv.contains("mode.configure(this, 2, 2, \"RW\", 1'b0, 2'h3, 1'b1, 1'b1, 1);"));
    assert!(sv.contains("rand ral_reg_cfg_sample sample[2];"));
    assert!(sv.contains("default_map.add_reg(sample[i], `UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h110);"));
}

#[test]
fn evaluates_ipxact_parameters_and_configurable_values() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>param_regs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="REG_COUNT">
      <ipxact:name>REG_COUNT</ipxact:name>
      <ipxact:value>2</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="REG_STRIDE">
      <ipxact:name>REG_STRIDE</ipxact:name>
      <ipxact:value>4</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="REG_WIDTH">
      <ipxact:name>REG_WIDTH</ipxact:name>
      <ipxact:value>32</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FIELD_LSB">
      <ipxact:name>FIELD_LSB</ipxact:name>
      <ipxact:value>1 + 1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FIELD_WIDTH">
      <ipxact:name>FIELD_WIDTH</ipxact:name>
      <ipxact:value>2</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="RESET_VALUE">
      <ipxact:name>RESET_VALUE</ipxact:name>
      <ipxact:value>1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="ENUM_BUSY">
      <ipxact:name>ENUM_BUSY</ipxact:name>
      <ipxact:value>RESET_VALUE + 2</ipxact:value>
    </ipxact:parameter>
  </ipxact:parameters>
  <ipxact:configurableElementValues>
    <ipxact:configurableElementValue referenceId="BASE_ADDR">16'h80</ipxact:configurableElementValue>
    <ipxact:configurableElementValue referenceId="REG_OFFSET">REG_STRIDE</ipxact:configurableElementValue>
  </ipxact:configurableElementValues>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>BASE_ADDR + REG_COUNT * REG_STRIDE * 2</ipxact:baseAddress>
        <ipxact:range>REG_COUNT * REG_STRIDE</ipxact:range>
        <ipxact:width>REG_WIDTH</ipxact:width>
        <ipxact:register>
          <ipxact:name>sample</ipxact:name>
          <ipxact:array>
            <ipxact:dim>REG_COUNT</ipxact:dim>
            <ipxact:stride>REG_STRIDE</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>REG_OFFSET</ipxact:addressOffset>
          <ipxact:size>REG_WIDTH</ipxact:size>
          <ipxact:field>
            <ipxact:name>mode</ipxact:name>
            <ipxact:bitOffset>FIELD_LSB</ipxact:bitOffset>
            <ipxact:bitWidth>FIELD_WIDTH</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>RESET_VALUE</ipxact:value>
              </ipxact:reset>
            </ipxact:resets>
            <ipxact:access>read-write</ipxact:access>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>busy</ipxact:name>
                <ipxact:value>ENUM_BUSY</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].base_address, "144");
    assert_eq!(component.blocks[0].range, "8");
    assert_eq!(component.blocks[0].width, "32");
    assert_eq!(component.blocks[0].registers[0].address_offset, "4");
    assert_eq!(component.blocks[0].registers[0].dim, "2");
    assert_eq!(
        component.blocks[0].registers[0].stride.as_deref(),
        Some("4")
    );
    assert_eq!(component.blocks[0].registers[0].fields[0].bit_offset, "2");
    assert_eq!(component.blocks[0].registers[0].fields[0].bit_width, "2");
    assert_eq!(
        component.blocks[0].registers[0].fields[0].reset.as_deref(),
        Some("1")
    );
    assert_eq!(
        component.blocks[0].registers[0].fields[0].enumerated_values[0].value,
        "3"
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("MODE_BUSY = 2'h3"));
    assert!(sv.contains("mode.configure(this, 2, 2, \"RW\", 1'b0, 2'h1, 1'b1, 1'b1, 1);"));
    assert!(sv.contains("rand ral_reg_cfg_sample sample[2];"));
    assert!(sv.contains("default_map.add_reg(sample[i], `UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h90);"));
}

#[test]
fn applies_ipxact_definition_instance_parameter_overrides() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>override_regs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:addressBlockDefinitions>
      <ipxact:addressBlockDefinition>
        <ipxact:name>templated_block</ipxact:name>
        <ipxact:parameters>
          <ipxact:parameter parameterId="BLOCK_RANGE">
            <ipxact:name>BLOCK_RANGE</ipxact:name>
            <ipxact:value>0x10</ipxact:value>
          </ipxact:parameter>
          <ipxact:parameter parameterId="REG_WIDTH">
            <ipxact:name>REG_WIDTH</ipxact:name>
            <ipxact:value>32</ipxact:value>
          </ipxact:parameter>
          <ipxact:parameter parameterId="REG_OFFSET">
            <ipxact:name>REG_OFFSET</ipxact:name>
            <ipxact:value>0</ipxact:value>
          </ipxact:parameter>
          <ipxact:parameter parameterId="FIELD_WIDTH">
            <ipxact:name>FIELD_WIDTH</ipxact:name>
            <ipxact:value>1</ipxact:value>
          </ipxact:parameter>
          <ipxact:parameter parameterId="RESET_VALUE">
            <ipxact:name>RESET_VALUE</ipxact:name>
            <ipxact:value>0</ipxact:value>
          </ipxact:parameter>
        </ipxact:parameters>
        <ipxact:range>BLOCK_RANGE</ipxact:range>
        <ipxact:width>REG_WIDTH</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>REG_OFFSET</ipxact:addressOffset>
          <ipxact:size>REG_WIDTH</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>FIELD_WIDTH</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>RESET_VALUE</ipxact:value>
              </ipxact:reset>
            </ipxact:resets>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlockDefinition>
    </ipxact:addressBlockDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:addressBlockDefinitionRef>templated_block</ipxact:addressBlockDefinitionRef>
        <ipxact:baseAddress>BASE_ADDR</ipxact:baseAddress>
        <ipxact:configurableElementValues>
          <ipxact:configurableElementValue referenceId="BASE_ADDR">0x100</ipxact:configurableElementValue>
          <ipxact:configurableElementValue referenceId="BLOCK_RANGE">0x20</ipxact:configurableElementValue>
          <ipxact:configurableElementValue referenceId="REG_WIDTH">64</ipxact:configurableElementValue>
          <ipxact:configurableElementValue referenceId="REG_OFFSET">8</ipxact:configurableElementValue>
          <ipxact:configurableElementValue referenceId="FIELD_WIDTH">4</ipxact:configurableElementValue>
          <ipxact:configurableElementValue referenceId="RESET_VALUE">0xa</ipxact:configurableElementValue>
        </ipxact:configurableElementValues>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let block = &component.blocks[0];
    let register = &block.registers[0];
    let field = &register.fields[0];

    assert_eq!(block.base_address, "256");
    assert_eq!(block.range, "32");
    assert_eq!(block.width, "64");
    assert_eq!(register.address_offset, "8");
    assert_eq!(register.size, "64");
    assert_eq!(field.bit_width, "4");
    assert_eq!(field.reset.as_deref(), Some("10"));

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("default_map = create_map(\"default_map\", 0, 8, UVM_LITTLE_ENDIAN);"));
    assert!(sv.contains("super.new(name, 64, UVM_NO_COVERAGE);"));
    assert!(sv.contains("ready.configure(this, 4, 0, \"RW\", 1'b0, 4'ha, 1'b1, 1'b1, 1);"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h8, \"RW\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h100);"));
}

#[test]
fn evaluates_static_ipxact_is_present_flags() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>present_flags</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="ENABLE_STATUS">
      <ipxact:name>ENABLE_STATUS</ipxact:name>
      <ipxact:value>1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="DISABLE_EXTRA">
      <ipxact:name>DISABLE_EXTRA</ipxact:name>
      <ipxact:value>0</ipxact:value>
    </ipxact:parameter>
  </ipxact:parameters>
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
          <ipxact:isPresent>ENABLE_STATUS</ipxact:isPresent>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>enable</ipxact:name>
            <ipxact:isPresent>true</ipxact:isPresent>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>on</ipxact:name>
                <ipxact:isPresent>1</ipxact:isPresent>
                <ipxact:value>1</ipxact:value>
              </ipxact:enumeratedValue>
              <ipxact:enumeratedValue>
                <ipxact:name>off</ipxact:name>
                <ipxact:isPresent>false</ipxact:isPresent>
                <ipxact:value>0</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>hidden_field</ipxact:name>
            <ipxact:isPresent>DISABLE_EXTRA</ipxact:isPresent>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>hidden_reg</ipxact:name>
          <ipxact:isPresent>0</ipxact:isPresent>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
        <ipxact:registerFile>
          <ipxact:name>hidden_file</ipxact:name>
          <ipxact:isPresent>false</ipxact:isPresent>
          <ipxact:addressOffset>8</ipxact:addressOffset>
          <ipxact:range>4</ipxact:range>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].registers.len(), 1);
    assert!(component.blocks[0].register_files.is_empty());
    assert_eq!(component.blocks[0].registers[0].fields.len(), 1);
    assert_eq!(
        component.blocks[0].registers[0].fields[0]
            .enumerated_values
            .len(),
        1
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_reg_cfg_status extends uvm_reg;"));
    assert!(sv.contains("ENABLE_ON = 1'h1"));
    assert!(!sv.contains("hidden_reg"));
    assert!(!sv.contains("hidden_field"));
    assert!(!sv.contains("hidden_file"));
    assert!(!sv.contains("ENABLE_OFF"));
}

#[test]
fn evaluates_static_ipxact_is_present_boolean_expressions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>present_exprs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="REV">
      <ipxact:name>REV</ipxact:name>
      <ipxact:value>3</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FEATURE_A">
      <ipxact:name>FEATURE_A</ipxact:name>
      <ipxact:value>1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FEATURE_B">
      <ipxact:name>FEATURE_B</ipxact:name>
      <ipxact:value>0</ipxact:value>
    </ipxact:parameter>
  </ipxact:parameters>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>visible</ipxact:name>
          <ipxact:isPresent>(REV + 1) &gt;= 4 &amp;&amp; (FEATURE_A == 1 || FEATURE_B == 1)</ipxact:isPresent>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>enabled</ipxact:name>
            <ipxact:isPresent>FEATURE_A != 0 &amp;&amp; !FEATURE_B</ipxact:isPresent>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>supported</ipxact:name>
                <ipxact:isPresent>REV &lt; 4</ipxact:isPresent>
                <ipxact:value>1</ipxact:value>
              </ipxact:enumeratedValue>
              <ipxact:enumeratedValue>
                <ipxact:name>future</ipxact:name>
                <ipxact:isPresent>REV &gt; 4 || FEATURE_B</ipxact:isPresent>
                <ipxact:value>0</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>disabled</ipxact:name>
            <ipxact:isPresent>FEATURE_B || REV == 0</ipxact:isPresent>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>hidden</ipxact:name>
          <ipxact:isPresent>REV &lt; 2 || FEATURE_B</ipxact:isPresent>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let register = &component.blocks[0].registers[0];

    assert_eq!(component.blocks[0].registers.len(), 1);
    assert_eq!(register.name, "visible");
    assert_eq!(register.fields.len(), 1);
    assert_eq!(register.fields[0].name, "enabled");
    assert_eq!(register.fields[0].enumerated_values.len(), 1);
    assert_eq!(register.fields[0].enumerated_values[0].name, "supported");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_reg_cfg_visible extends uvm_reg;"));
    assert!(sv.contains("ENABLED_SUPPORTED = 1'h1"));
    assert!(!sv.contains("hidden"));
    assert!(!sv.contains("disabled"));
    assert!(!sv.contains("future"));
}

#[test]
fn renders_multiple_memory_maps() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>multi_map</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>ctrls</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>enable</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>bit0</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:access>read-write</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
    <ipxact:memoryMap>
      <ipxact:name>status</ipxact:name>
      <ipxact:addressUnitBits>32</ipxact:addressUnitBits>
      <ipxact:addressBlock>
        <ipxact:name>stats</ipxact:name>
        <ipxact:baseAddress>0x10</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>count</ipxact:name>
          <ipxact:addressOffset>0x2</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>32</ipxact:bitWidth>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].map_name, "cfg");
    assert_eq!(component.blocks[1].map_name, "status");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("uvm_reg_map status_map;"));
    assert!(sv.contains("default_map = create_map(\"default_map\", 0, 4, UVM_LITTLE_ENDIAN);"));
    assert!(sv.contains("status_map = create_map(\"status\", 0, 4, UVM_LITTLE_ENDIAN);"));
    assert!(sv.contains("default_map.add_reg(enable, `UVM_REG_ADDR_WIDTH'h0, \"RW\");"));
    assert!(sv.contains("default_map.add_reg(count, `UVM_REG_ADDR_WIDTH'h2, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(ctrls.default_map, `UVM_REG_ADDR_WIDTH'h100);"));
    assert!(sv.contains("status_map.add_submap(stats.default_map, `UVM_REG_ADDR_WIDTH'h10);"));
    assert!(!sv.contains("default_map.add_submap(stats.default_map"));
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
fn renders_scalar_register_files_without_array_loop() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_scalar</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:range>0x80</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>local</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
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
            <ipxact:alternateRegisters>
              <ipxact:alternateRegister>
                <ipxact:name>shadow</ipxact:name>
                <ipxact:field>
                  <ipxact:name>raw</ipxact:name>
                  <ipxact:bitOffset>0</ipxact:bitOffset>
                  <ipxact:bitWidth>8</ipxact:bitWidth>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:field>
              </ipxact:alternateRegister>
            </ipxact:alternateRegisters>
          </ipxact:register>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].register_files[0].dim, "1");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_regfile_cfg_local extends uvm_reg_file;"));
    assert!(sv.contains("ral_regfile_cfg_local local;"));
    assert!(sv.contains("local = ral_regfile_cfg_local::type_id::create(\"local\");"));
    assert!(sv.contains("local.configure(this, null, \"\");"));
    assert!(sv.contains("rand ral_reg_cfg_local_status status;"));
    assert!(sv.contains("status = ral_reg_cfg_local_status::type_id::create(\"status\");"));
    assert!(sv.contains("status.configure(get_block(), this);"));
    assert!(sv.contains("mp.add_reg(status, offset + `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains("rand ral_reg_cfg_local_status_shadow shadow;"));
    assert!(sv.contains("mp.add_reg(shadow, offset + `UVM_REG_ADDR_WIDTH'h4, \"RW\");"));
    assert!(!sv.contains("status[1]"));
    assert!(!sv.contains("$sformatf(\"status_%0d\""));
}

#[test]
fn renders_register_arrays_inside_register_files() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_reg_arrays</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:range>0x100</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>local</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x20</ipxact:range>
          <ipxact:register>
            <ipxact:name>counter</ipxact:name>
            <ipxact:array>
              <ipxact:dim>2</ipxact:dim>
              <ipxact:stride>4</ipxact:stride>
            </ipxact:array>
            <ipxact:addressOffset>0x4</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>value</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>32</ipxact:bitWidth>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:registerFile>
        <ipxact:registerFile>
          <ipxact:name>lane</ipxact:name>
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>0x20</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0x60</ipxact:addressOffset>
          <ipxact:range>0x20</ipxact:range>
          <ipxact:register>
            <ipxact:name>sample</ipxact:name>
            <ipxact:array>
              <ipxact:dim>3</ipxact:dim>
              <ipxact:stride>4</ipxact:stride>
            </ipxact:array>
            <ipxact:addressOffset>0x8</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>value</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>32</ipxact:bitWidth>
              <ipxact:access>read-write</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let sv = ipxact_to_uvm_reg(xml).unwrap();

    assert!(sv.contains("class ral_regfile_cfg_local extends uvm_reg_file;"));
    assert!(sv.contains("rand ral_reg_cfg_local_counter counter[2];"));
    assert!(sv.contains(
        "counter[i] = ral_reg_cfg_local_counter::type_id::create($sformatf(\"counter_%0d\", i));"
    ));
    assert!(sv.contains("counter[i].configure(get_block(), this);"));
    assert!(sv.contains("mp.add_reg(counter[i], offset + `UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4, \"RW\");"));
    assert!(sv.contains("ral_regfile_cfg_lane lane[2];"));
    assert!(
        sv.contains("lane[i] = ral_regfile_cfg_lane::type_id::create($sformatf(\"lane_%0d\", i));")
    );
    assert!(sv.contains("lane[i].configure(this, null, \"\");"));
    assert!(sv.contains("rand ral_reg_cfg_lane_sample sample[3];"));
    assert!(sv.contains(
        "sample[i] = ral_reg_cfg_lane_sample::type_id::create($sformatf(\"sample_%0d\", i));"
    ));
    assert!(sv.contains("sample[i].configure(get_block(), this);"));
    assert!(sv.contains("mp.add_reg(sample[i], offset + `UVM_REG_ADDR_WIDTH'h8 + i * `UVM_REG_ADDR_WIDTH'h4, \"RW\");"));
}

#[test]
fn ignores_retained_descriptions_while_generating_ral() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>desc_meta</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>local_types</ipxact:name>
    <ipxact:fieldDefinitions>
      <ipxact:fieldDefinition>
        <ipxact:name>ready_field_def</ipxact:name>
        <ipxact:description>Definition field description</ipxact:description>
        <ipxact:bitWidth>1</ipxact:bitWidth>
        <ipxact:access>read-only</ipxact:access>
      </ipxact:fieldDefinition>
    </ipxact:fieldDefinitions>
    <ipxact:registerDefinitions>
      <ipxact:registerDefinition>
        <ipxact:name>status_reg_def</ipxact:name>
        <ipxact:description>Definition register description</ipxact:description>
        <ipxact:size>32</ipxact:size>
        <ipxact:field>
          <ipxact:name>ready</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:fieldDefinitionRef typeDefinitions="local_types">ready_field_def</ipxact:fieldDefinitionRef>
        </ipxact:field>
      </ipxact:registerDefinition>
    </ipxact:registerDefinitions>
    <ipxact:registerFileDefinitions>
      <ipxact:registerFileDefinition>
        <ipxact:name>status_file_def</ipxact:name>
        <ipxact:description>Definition register file description</ipxact:description>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:register>
          <ipxact:name>status_from_file</ipxact:name>
          <ipxact:addressOffset>0x4</ipxact:addressOffset>
          <ipxact:registerDefinitionRef typeDefinitions="local_types">status_reg_def</ipxact:registerDefinitionRef>
        </ipxact:register>
      </ipxact:registerFileDefinition>
    </ipxact:registerFileDefinitions>
    <ipxact:addressBlockDefinitions>
      <ipxact:addressBlockDefinition>
        <ipxact:name>cfg_block_def</ipxact:name>
        <ipxact:description>Definition block description
with "quotes"</ipxact:description>
        <ipxact:range>0x80</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:description>Instance register description</ipxact:description>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:registerDefinitionRef typeDefinitions="local_types">status_reg_def</ipxact:registerDefinitionRef>
          <ipxact:alternateRegisters>
            <ipxact:alternateRegister>
              <ipxact:name>debug_status</ipxact:name>
              <ipxact:description>Alternate register description</ipxact:description>
              <ipxact:field>
                <ipxact:name>raw</ipxact:name>
                <ipxact:description>Alternate field description</ipxact:description>
                <ipxact:bitOffset>0</ipxact:bitOffset>
                <ipxact:bitWidth>8</ipxact:bitWidth>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:field>
            </ipxact:alternateRegister>
          </ipxact:alternateRegisters>
        </ipxact:register>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:registerFileDefinitionRef typeDefinitions="local_types">status_file_def</ipxact:registerFileDefinitionRef>
        </ipxact:registerFile>
      </ipxact:addressBlockDefinition>
    </ipxact:addressBlockDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="local_types">cfg_block_def</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let block = &component.blocks[0];

    assert_eq!(block.name, "cfg");
    assert_eq!(block.register_files[0].name, "cluster");
    assert_eq!(block.registers[0].name, "status");
    assert_eq!(block.registers[0].fields[0].name, "ready");
    assert_eq!(
        block.registers[0].alternate_registers[0].name,
        "debug_status"
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(!sv.contains("localparam"));
    assert!(sv.contains("ready.configure(this, 1, 0, \"RO\", 1'b0, 1'h0, 1'b0, 1'b0, 1);"));
    assert!(sv.contains("raw.configure(this, 8, 0, \"RW\", 1'b0, 8'h0, 1'b0, 1'b1, 1);"));
}

#[test]
fn resolves_definition_refs_by_type_definitions_scope() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>scoped_defs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>types_a</ipxact:name>
    <ipxact:fieldAccessPolicyDefinitions>
      <ipxact:fieldAccessPolicyDefinition>
        <ipxact:name>shared_policy</ipxact:name>
        <ipxact:access>read-only</ipxact:access>
      </ipxact:fieldAccessPolicyDefinition>
    </ipxact:fieldAccessPolicyDefinitions>
    <ipxact:enumerationDefinitions>
      <ipxact:enumerationDefinition>
        <ipxact:name>shared_enum</ipxact:name>
        <ipxact:width>1</ipxact:width>
        <ipxact:enumeratedValue>
          <ipxact:name>a_value</ipxact:name>
          <ipxact:value>0</ipxact:value>
        </ipxact:enumeratedValue>
      </ipxact:enumerationDefinition>
    </ipxact:enumerationDefinitions>
    <ipxact:fieldDefinitions>
      <ipxact:fieldDefinition>
        <ipxact:name>shared_field</ipxact:name>
        <ipxact:bitWidth>1</ipxact:bitWidth>
        <ipxact:description>A field</ipxact:description>
        <ipxact:access>read-only</ipxact:access>
      </ipxact:fieldDefinition>
    </ipxact:fieldDefinitions>
    <ipxact:registerDefinitions>
      <ipxact:registerDefinition>
        <ipxact:name>shared_reg</ipxact:name>
        <ipxact:size>16</ipxact:size>
        <ipxact:description>A register</ipxact:description>
        <ipxact:field>
          <ipxact:name>state</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:fieldDefinitionRef typeDefinitions="types_a">shared_field</ipxact:fieldDefinitionRef>
        </ipxact:field>
      </ipxact:registerDefinition>
    </ipxact:registerDefinitions>
    <ipxact:addressBlockDefinitions>
      <ipxact:addressBlockDefinition>
        <ipxact:name>shared_block</ipxact:name>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>16</ipxact:width>
        <ipxact:description>A block</ipxact:description>
      </ipxact:addressBlockDefinition>
    </ipxact:addressBlockDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:typeDefinitions>
    <ipxact:name>types_b</ipxact:name>
    <ipxact:fieldAccessPolicyDefinitions>
      <ipxact:fieldAccessPolicyDefinition>
        <ipxact:name>shared_policy</ipxact:name>
        <ipxact:access>read-write</ipxact:access>
      </ipxact:fieldAccessPolicyDefinition>
    </ipxact:fieldAccessPolicyDefinitions>
    <ipxact:enumerationDefinitions>
      <ipxact:enumerationDefinition>
        <ipxact:name>shared_enum</ipxact:name>
        <ipxact:width>2</ipxact:width>
        <ipxact:enumeratedValue usage="read">
          <ipxact:name>b_value</ipxact:name>
          <ipxact:value>3</ipxact:value>
        </ipxact:enumeratedValue>
      </ipxact:enumerationDefinition>
    </ipxact:enumerationDefinitions>
    <ipxact:fieldDefinitions>
      <ipxact:fieldDefinition>
        <ipxact:name>shared_field</ipxact:name>
        <ipxact:bitWidth>2</ipxact:bitWidth>
        <ipxact:description>B field</ipxact:description>
        <ipxact:fieldAccessPolicies>
          <ipxact:fieldAccessPolicy>
            <ipxact:fieldAccessPolicyDefinitionRef typeDefinitions="types_b">shared_policy</ipxact:fieldAccessPolicyDefinitionRef>
          </ipxact:fieldAccessPolicy>
        </ipxact:fieldAccessPolicies>
        <ipxact:enumeratedValues>
          <ipxact:enumerationDefinitionRef typeDefinitions="types_b">shared_enum</ipxact:enumerationDefinitionRef>
        </ipxact:enumeratedValues>
      </ipxact:fieldDefinition>
    </ipxact:fieldDefinitions>
    <ipxact:registerDefinitions>
      <ipxact:registerDefinition>
        <ipxact:name>shared_reg</ipxact:name>
        <ipxact:size>32</ipxact:size>
        <ipxact:description>B register</ipxact:description>
        <ipxact:field>
          <ipxact:name>state</ipxact:name>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:fieldDefinitionRef typeDefinitions="types_b">shared_field</ipxact:fieldDefinitionRef>
        </ipxact:field>
      </ipxact:registerDefinition>
    </ipxact:registerDefinitions>
    <ipxact:addressBlockDefinitions>
      <ipxact:addressBlockDefinition>
        <ipxact:name>shared_block</ipxact:name>
        <ipxact:range>0x40</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:description>B block</ipxact:description>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:registerDefinitionRef typeDefinitions="types_b">shared_reg</ipxact:registerDefinitionRef>
        </ipxact:register>
      </ipxact:addressBlockDefinition>
    </ipxact:addressBlockDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="types_b">shared_block</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let block = &component.blocks[0];
    let register = &block.registers[0];
    let field = &register.fields[0];

    assert_eq!(block.range, "0x40");
    assert_eq!(block.width, "32");
    assert_eq!(register.size, "32");
    assert_eq!(field.bit_width, "2");
    assert_eq!(field.access.as_deref(), Some("read-write"));
    assert_eq!(field.enumerated_values[0].name, "b_value");
    assert_eq!(field.enumerated_values[0].value, "3");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("STATE_B_VALUE = 2'h3"));
    assert!(sv.contains("state.configure(this, 2, 0, \"RW\", 1'b0, 2'h0, 1'b0, 1'b1, 1);"));
    assert!(!sv.contains("A block"));
    assert!(!sv.contains("A register"));
    assert!(!sv.contains("A field"));
    assert!(!sv.contains("STATE_A_VALUE"));
}

#[test]
fn resolves_external_type_definitions_with_resolver() {
    let top = r#"
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
        <ipxact:baseAddress>0x40</ipxact:baseAddress>
        <ipxact:addressBlockDefinitionRef typeDefinitions="common_types">shared_block</ipxact:addressBlockDefinitionRef>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;
    let external = r#"
<ipxact:typeDefinitions xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>types</ipxact:library>
  <ipxact:name>common_regs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:addressBlockDefinitions>
    <ipxact:addressBlockDefinition>
      <ipxact:name>shared_block</ipxact:name>
      <ipxact:description>External block description</ipxact:description>
      <ipxact:range>0x20</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:register>
        <ipxact:name>status</ipxact:name>
        <ipxact:description>External register description</ipxact:description>
        <ipxact:addressOffset>0x4</ipxact:addressOffset>
        <ipxact:size>32</ipxact:size>
        <ipxact:field>
          <ipxact:name>ready</ipxact:name>
          <ipxact:description>External field description</ipxact:description>
          <ipxact:bitOffset>0</ipxact:bitOffset>
          <ipxact:bitWidth>1</ipxact:bitWidth>
          <ipxact:access>read-only</ipxact:access>
        </ipxact:field>
      </ipxact:register>
    </ipxact:addressBlockDefinition>
  </ipxact:addressBlockDefinitions>
</ipxact:typeDefinitions>"#;

    let component = parse_ipxact_with_resolver(top, |reference| {
        assert_eq!(reference.vendor, "acme");
        assert_eq!(reference.library, "types");
        assert_eq!(reference.name, "common_regs");
        assert_eq!(reference.version, "1.0");
        Ok(Some(external.into()))
    })
    .unwrap();

    assert_eq!(component.blocks[0].name, "cfg");
    assert_eq!(component.blocks[0].registers[0].name, "status");
    assert_eq!(component.blocks[0].registers[0].fields[0].name, "ready");

    let sv = irgen_uvmreg::serialize_uvm_reg(&component).unwrap();
    assert!(!sv.contains("localparam"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(cfg.default_map, `UVM_REG_ADDR_WIDTH'h40);"));
}

#[test]
fn renders_address_space_local_memory_map_as_uvm_submap() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bridge</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:busInterfaces>
    <ipxact:busInterface>
      <ipxact:name>dma_init</ipxact:name>
      <ipxact:busType vendor="acme" library="bus" name="axi" version="1.0"/>
      <ipxact:abstractionTypes>
        <ipxact:abstractionType>
          <ipxact:abstractionRef vendor="acme" library="bus" name="axi_rtl" version="1.0"/>
        </ipxact:abstractionType>
      </ipxact:abstractionTypes>
      <ipxact:initiator>
        <ipxact:addressSpaceRef addressSpaceRef="dma_space"/>
      </ipxact:initiator>
    </ipxact:busInterface>
  </ipxact:busInterfaces>
  <ipxact:addressSpaces>
    <ipxact:addressSpace>
      <ipxact:name>dma_space</ipxact:name>
      <ipxact:range>0x100</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:addressUnitBits>8</ipxact:addressUnitBits>
      <ipxact:segments>
        <ipxact:segment>
          <ipxact:name>cfg_seg</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
        </ipxact:segment>
      </ipxact:segments>
      <ipxact:localMemoryMap>
        <ipxact:name>dma_local</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>dma_regs</ipxact:name>
          <ipxact:baseAddress>0x20</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>doorbell</ipxact:name>
            <ipxact:addressOffset>0x4</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>kick</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:access>write-only</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
  </ipxact:addressSpaces>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>host</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0x0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
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
      <ipxact:subspaceMap initiatorRef="dma_init" segmentRef="cfg_seg">
        <ipxact:name>dma_window</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.address_spaces[0].name, "dma_space");
    assert_eq!(component.address_spaces[0].segments[0].name, "cfg_seg");
    assert_eq!(
        component.address_spaces[0].segments[0].address_offset,
        "0x20"
    );
    assert_eq!(component.address_spaces[0].segments[0].range, "0x10");
    assert_eq!(component.address_spaces[0].blocks[0].name, "dma_regs");
    assert_eq!(
        component.subspace_maps[0].address_space_ref.as_deref(),
        Some("dma_space")
    );

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_block_bridge_dma_space_dma_regs extends uvm_reg_block;"));
    assert!(sv.contains("class ral_sys_bridge_dma_space extends uvm_reg_block;"));
    assert!(sv.contains("class ral_sys_bridge extends uvm_reg_block;"));
    assert!(sv.contains("rand ral_reg_bridge_dma_space_dma_regs_doorbell doorbell;"));
    assert!(sv.contains("default_map.add_reg(doorbell, `UVM_REG_ADDR_WIDTH'h4, \"WO\");"));
    assert!(sv.contains("default_map.add_submap(dma_regs.default_map, `UVM_REG_ADDR_WIDTH'h20);"));
    assert!(sv.contains("rand ral_sys_bridge_dma_space dma_window;"));
    assert!(sv.contains("dma_window = ral_sys_bridge_dma_space::type_id::create(\"dma_window\");"));
    assert!(
        sv.contains("default_map.add_submap(dma_window.default_map, `UVM_REG_ADDR_WIDTH'hfe0);")
    );
}

#[test]
fn reports_subspace_segment_range_violations() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bridge</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:busInterfaces>
    <ipxact:busInterface>
      <ipxact:name>dma_init</ipxact:name>
      <ipxact:busType vendor="acme" library="bus" name="axi" version="1.0"/>
      <ipxact:initiator>
        <ipxact:addressSpaceRef addressSpaceRef="dma_space"/>
      </ipxact:initiator>
    </ipxact:busInterface>
  </ipxact:busInterfaces>
  <ipxact:addressSpaces>
    <ipxact:addressSpace>
      <ipxact:name>dma_space</ipxact:name>
      <ipxact:range>0x100</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:addressUnitBits>8</ipxact:addressUnitBits>
      <ipxact:segments>
        <ipxact:segment>
          <ipxact:name>cfg_seg</ipxact:name>
          <ipxact:addressOffset>0x20</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
        </ipxact:segment>
      </ipxact:segments>
      <ipxact:localMemoryMap>
        <ipxact:name>dma_local</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>dma_regs</ipxact:name>
          <ipxact:baseAddress>0x20</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
        <ipxact:addressBlock>
          <ipxact:name>stray_regs</ipxact:name>
          <ipxact:baseAddress>0x40</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
  </ipxact:addressSpaces>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>host</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:subspaceMap initiatorRef="dma_init" segmentRef="cfg_seg">
        <ipxact:name>dma_window</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let error = serialize_uvm_reg_with_options(&component, RenderOptions::default())
        .unwrap_err()
        .to_string();

    assert!(
        error.contains(
            "subspaceMap `dma_window` segmentRef `cfg_seg` does not cover addressBlock `stray_regs`"
        ),
        "{error}"
    );
}

#[test]
fn expands_scoped_memory_map_definitions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>map_defs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>types_a</ipxact:name>
    <ipxact:memoryMapDefinitions>
      <ipxact:memoryMapDefinition>
        <ipxact:name>shared_map</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>a_regs</ipxact:name>
          <ipxact:baseAddress>0</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>16</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:memoryMapDefinition>
    </ipxact:memoryMapDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:typeDefinitions>
    <ipxact:name>types_b</ipxact:name>
    <ipxact:memoryMapDefinitions>
      <ipxact:memoryMapDefinition>
        <ipxact:name>shared_map</ipxact:name>
        <ipxact:addressUnitBits>32</ipxact:addressUnitBits>
        <ipxact:addressBlock>
          <ipxact:name>b_regs</ipxact:name>
          <ipxact:description>Scoped memory map block</ipxact:description>
          <ipxact:baseAddress>0x2</ipxact:baseAddress>
          <ipxact:range>0x4</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>status</ipxact:name>
            <ipxact:addressOffset>0x1</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>ready</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
        <ipxact:memoryRemap>
          <ipxact:name>debug</ipxact:name>
          <ipxact:modeRef>dbg</ipxact:modeRef>
          <ipxact:addressBlock>
            <ipxact:name>dbg_regs</ipxact:name>
            <ipxact:baseAddress>0x8</ipxact:baseAddress>
            <ipxact:range>0x4</ipxact:range>
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
        </ipxact:memoryRemap>
      </ipxact:memoryMapDefinition>
    </ipxact:memoryMapDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:memoryMapDefinitionRef typeDefinitions="types_b">shared_map</ipxact:memoryMapDefinitionRef>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks.len(), 1);
    assert_eq!(component.blocks[0].name, "b_regs");
    assert_eq!(component.blocks[0].map_name, "cfg");
    assert_eq!(component.blocks[0].address_unit_bits, "32");
    assert_eq!(component.memory_remaps[0].name, "debug");
    assert_eq!(component.memory_remaps[0].blocks[0].name, "debug_dbg_regs");
    assert_eq!(component.memory_remaps[0].blocks[0].map_name, "cfg");
    assert_eq!(component.memory_remaps[0].blocks[0].address_unit_bits, "32");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("default_map = create_map(\"default_map\", 0, 4, UVM_LITTLE_ENDIAN);"));
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h1, \"RO\");"));
    assert!(sv.contains("default_map.add_submap(b_regs.default_map, `UVM_REG_ADDR_WIDTH'h2);"));
    assert!(
        sv.contains("default_map.add_reg(debug_dbg_regs_ctrl, `UVM_REG_ADDR_WIDTH'h8, \"RW\");")
    );
    assert!(!sv.contains("a_regs"));
}

#[test]
fn expands_scoped_bank_and_memory_remap_definitions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bank_defs</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:typeDefinitions>
    <ipxact:name>types_a</ipxact:name>
    <ipxact:bankDefinitions>
      <ipxact:bankDefinition>
        <ipxact:name>shared_bank</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>a_bank_block</ipxact:name>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:bankDefinition>
    </ipxact:bankDefinitions>
    <ipxact:memoryRemapDefinitions>
      <ipxact:memoryRemapDefinition>
        <ipxact:name>shared_remap</ipxact:name>
        <ipxact:modeRef priority="0">a_mode</ipxact:modeRef>
        <ipxact:addressBlock>
          <ipxact:name>a_remap_block</ipxact:name>
          <ipxact:baseAddress>0</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:memoryRemapDefinition>
    </ipxact:memoryRemapDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:typeDefinitions>
    <ipxact:name>types_b</ipxact:name>
    <ipxact:bankDefinitions>
      <ipxact:bankDefinition>
        <ipxact:name>shared_bank</ipxact:name>
        <ipxact:addressUnitBits>8</ipxact:addressUnitBits>
        <ipxact:addressBlock>
          <ipxact:name>def_regs</ipxact:name>
          <ipxact:description>Scoped bank block</ipxact:description>
          <ipxact:range>0x10</ipxact:range>
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
        </ipxact:addressBlock>
      </ipxact:bankDefinition>
    </ipxact:bankDefinitions>
    <ipxact:memoryRemapDefinitions>
      <ipxact:memoryRemapDefinition>
        <ipxact:name>shared_remap</ipxact:name>
        <ipxact:modeRef priority="0">definition_mode</ipxact:modeRef>
        <ipxact:addressBlock>
          <ipxact:name>lp_regs</ipxact:name>
          <ipxact:baseAddress>0x200</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
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
      </ipxact:memoryRemapDefinition>
    </ipxact:memoryRemapDefinitions>
  </ipxact:typeDefinitions>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:bank bankAlignment="serial">
        <ipxact:name>banked</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:bankDefinitionRef typeDefinitions="types_b">shared_bank</ipxact:bankDefinitionRef>
      </ipxact:bank>
      <ipxact:memoryRemap>
        <ipxact:name>lowpower</ipxact:name>
        <ipxact:modeRef priority="1">sleep</ipxact:modeRef>
        <ipxact:remapDefinitionRef typeDefinitions="types_b">shared_remap</ipxact:remapDefinitionRef>
      </ipxact:memoryRemap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].name, "banked_def_regs");
    assert_eq!(component.blocks[0].base_address, "0x100");
    assert_eq!(component.memory_remaps[0].name, "lowpower");
    assert_eq!(
        component.memory_remaps[0].blocks[0].name,
        "lowpower_lp_regs"
    );
    assert_eq!(component.memory_remaps[0].blocks[0].base_address, "0x200");

    let sv = ipxact_to_uvm_reg(xml).unwrap();
    assert!(sv.contains("default_map.add_reg(status, `UVM_REG_ADDR_WIDTH'h4, \"RO\");"));
    assert!(sv.contains(
        "default_map.add_submap(banked_def_regs.default_map, `UVM_REG_ADDR_WIDTH'h100);"
    ));
    assert!(
        sv.contains(
            "default_map.add_reg(lowpower_lp_regs_ctrl, `UVM_REG_ADDR_WIDTH'h200, \"RW\");"
        )
    );
    assert!(!sv.contains("a_bank_block"));
    assert!(!sv.contains("a_remap_block"));
}
