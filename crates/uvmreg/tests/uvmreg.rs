use irgen_ipxact_parser::{
    ParseOptions, parse_ipxact, parse_ipxact_with_options, parse_ipxact_with_resolver,
};
use irgen_uvmreg::{
    FileType, RenderOptions, serialize_uvm_reg, serialize_uvm_reg_by_block_with_options,
    serialize_uvm_reg_with_options,
};

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

fn parse_and_serialize_uvm_reg(xml: &str) -> Result<String, String> {
    let component = parse_ipxact(xml).map_err(|error| error.to_string())?;
    serialize_uvm_reg(&component).map_err(|error| error.to_string())
}

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
  <ipxact:busInterfaces>
    <ipxact:busInterface>
      <ipxact:name>dma_init</ipxact:name>
      <ipxact:busType vendor="example.com" library="bus" name="axi" version="1.0"/>
      <ipxact:initiator>
        <ipxact:addressSpaceRef addressSpaceRef="dma_space"/>
      </ipxact:initiator>
    </ipxact:busInterface>
    <ipxact:busInterface>
      <ipxact:name>lp_init</ipxact:name>
      <ipxact:busType vendor="example.com" library="bus" name="axi" version="1.0"/>
      <ipxact:initiator>
        <ipxact:addressSpaceRef addressSpaceRef="lp_space"/>
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
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>kick</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>write-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
    <ipxact:addressSpace>
      <ipxact:name>lp_space</ipxact:name>
      <ipxact:range>0x20</ipxact:range>
      <ipxact:width>32</ipxact:width>
      <ipxact:addressUnitBits>8</ipxact:addressUnitBits>
      <ipxact:localMemoryMap>
        <ipxact:name>lp_local</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>lp_regs</ipxact:name>
          <ipxact:baseAddress>0</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
          <ipxact:register>
            <ipxact:name>retention</ipxact:name>
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>enabled</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>1</ipxact:bitWidth>
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
  </ipxact:addressSpaces>
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
              <ipxact:reset resetTypeRef="SOFT">
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
          </ipxact:register>
        </ipxact:addressBlock>
      </ipxact:bank>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

#[test]
fn renders_uvm_ieee_2022_register_model() {
    let sv = parse_and_serialize_uvm_reg(IPXACT_COMMON).unwrap();

    assert_demo_uvm_golden_patterns("ieee-2022-register-model", &sv);
}

#[test]
fn generated_uvm_systemverilog_passes_structural_gate() {
    let common_sv = parse_and_serialize_uvm_reg(IPXACT_COMMON).unwrap();
    assert_generated_sv_structural_gate("common-golden", &common_sv);
    assert_demo_uvm_golden_patterns("common-golden", &common_sv);

    let component = parse_ipxact(IPXACT_2022).unwrap();
    let sv = serialize_uvm_reg_with_options(&component, RenderOptions::default()).unwrap();
    assert_generated_sv_structural_gate("single-file", &sv);
    assert_contains_before(&sv, "package ral_demo_pkg;", "import uvm_pkg::*;");
    assert_contains_before(&sv, "endpackage", "`endif");

    let sv = serialize_uvm_reg_with_options(
        &component,
        RenderOptions {
            coverage: true,
            ..RenderOptions::default()
        },
    )
    .unwrap();
    assert_generated_sv_structural_gate("coverage", &sv);

    let sv = serialize_uvm_reg_with_options(
        &component,
        RenderOptions {
            file_type: FileType::Header,
            ..RenderOptions::default()
        },
    )
    .unwrap();
    assert_generated_sv_structural_gate("single-file-header", &sv);
    assert!(
        !sv.contains("package ral_demo_pkg;"),
        "header file type should not wrap classes in a package"
    );

    let files = serialize_uvm_reg_by_block_with_options(
        &component,
        RenderOptions {
            coverage: true,
            ..RenderOptions::default()
        },
    )
    .unwrap();
    assert!(files.len() > 1);
    for file in &files {
        if file.path == "ral_demo_pkg.sv" {
            assert!(file.content.contains("package ral_demo_pkg;"));
            assert_contains_before(&file.content, "import uvm_pkg::*;", "`include \"ral_block_");
            assert_contains_before(&file.content, "`include \"ral_demo.sv\"", "endpackage");
        } else {
            assert_generated_include_sv_structural_gate(&file.path, &file.content);
            assert!(
                !file.content.contains("import uvm_pkg::*;"),
                "{}: package member files should not import uvm_pkg",
                file.path
            );
            assert!(
                !file.content.contains("`include \"uvm_macros.svh\""),
                "{}: package member files should not include uvm_macros",
                file.path
            );
        }
    }

    let files = serialize_uvm_reg_by_block_with_options(
        &component,
        RenderOptions {
            file_type: FileType::Header,
            ..RenderOptions::default()
        },
    )
    .unwrap();
    let top = files
        .iter()
        .find(|file| file.path == "ral_demo.sv")
        .expect("missing top RAL file");
    assert_contains_before(
        &top.content,
        "`include \"uvm_macros.svh\"",
        "`include \"ral_block_",
    );
    assert_contains_before(&top.content, "`include \"ral_block_", "class ral_sys_demo");
    assert!(
        !files.iter().any(|file| file.path == "ral_demo_pkg.sv"),
        "header block layout should not emit a package wrapper"
    );
}

#[test]
fn avoids_generated_systemverilog_member_name_collisions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>member_names</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>bus</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>build</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x100</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>default_map</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>build</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>cg_bits</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
        <ipxact:registerFile>
          <ipxact:name>map</ipxact:name>
          <ipxact:addressOffset>0x10</ipxact:addressOffset>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:register>
            <ipxact:name>i</ipxact:name>
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
    <ipxact:memoryMap>
      <ipxact:name>default</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>default_map</ipxact:name>
        <ipxact:baseAddress>0x100</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let sv = serialize_uvm_reg_with_options(
        &component,
        RenderOptions {
            coverage: true,
            ..RenderOptions::default()
        },
    )
    .unwrap();

    assert!(sv.contains("uvm_reg_map default_map_1;"));
    assert!(!sv.contains("uvm_reg_map default_map;\n"));
    assert!(!sv.contains("default_map = create_map(\"default\", 0, 4, UVM_LITTLE_ENDIAN, 1);"));
    assert_create_map(&sv, "default_map_1", "\"default\"", "4", "1");
    assert!(sv.contains("rand ral_block_build build_1;"));
    assert!(sv.contains("rand ral_block_default_map default_map_2;"));
    assert!(sv.contains("rand ral_reg_build_default_map default_map_1;"));
    assert!(sv.contains("ral_regfile_build_map map;"));
    assert!(sv.contains("rand uvm_reg_field build_1;"));
    assert!(sv.contains("rand uvm_reg_field cg_bits_1;"));
    assert!(sv.contains("rand ral_reg_build_map_i i_1[2];"));
    assert!(sv.contains("for (int unsigned i = 0; i < 2; i++) begin"));
    assert!(sv.contains("i_1[i] = ral_reg_build_map_i::type_id::create($sformatf(\"i_%0d\", i));"));
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>fallback</ipxact:name>
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:viewRef>rtl</ipxact:viewRef>
              <ipxact:pathSegments><ipxact:pathSegment>rtl_only.fallback</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
            <ipxact:accessHandle>
              <ipxact:pathSegments><ipxact:pathSegment>rtl.fallback</ipxact:pathSegment></ipxact:pathSegments>
            </ipxact:accessHandle>
          </ipxact:accessHandles>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_hdl_path_slice(&sv, "status", "\"rtl.status.ready_q\"", "0", "1", "1");
    assert_hdl_path_slice(&sv, "fallback", "\"rtl.fallback\"", "-1", "-1", "1");
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
    assert_hdl_path_slice(
        &gate_sv,
        "status",
        "\"gate.status.gate_ready\"",
        "0",
        "1",
        "1",
    );
    assert_hdl_path_slice(&gate_sv, "fallback", "\"rtl.fallback\"", "-1", "-1", "1");
    assert!(!gate_sv.contains("rtl_only.fallback"));
}

#[test]
fn skips_selected_access_handles_without_paths_when_later_handle_has_path() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>path_bearing_handles</ipxact:name>
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
            <ipxact:accessHandle/>
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
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:viewRef>gate</ipxact:viewRef>
                <ipxact:slices>
                  <ipxact:slice>
                    <ipxact:pathSegments><ipxact:pathSegment>gate_ready_q</ipxact:pathSegment></ipxact:pathSegments>
                  </ipxact:slice>
                </ipxact:slices>
              </ipxact:accessHandle>
              <ipxact:accessHandle>
                <ipxact:slices>
                  <ipxact:slice>
                    <ipxact:pathSegments><ipxact:pathSegment>ready_q</ipxact:pathSegment></ipxact:pathSegments>
                  </ipxact:slice>
                </ipxact:slices>
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

    let gate_component = parse_ipxact_with_options(
        xml,
        ParseOptions {
            preferred_view: Some("gate".into()),
            ..ParseOptions::default()
        },
    )
    .unwrap();
    assert_eq!(
        gate_component.blocks[0].registers[0].fields[0]
            .hdl_path
            .as_deref(),
        Some("gate_ready_q")
    );

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_hdl_path_slice(&sv, "status", "\"rtl.status.ready_q\"", "0", "1", "1");
    let gate_sv =
        serialize_uvm_reg_with_options(&gate_component, RenderOptions::default()).unwrap();
    assert_hdl_path_slice(
        &gate_sv,
        "status",
        "\"rtl.status.gate_ready_q\"",
        "0",
        "1",
        "1",
    );
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(contains_named_call(
        &sv,
        "status.add_hdl_path_slice",
        &[
            ("name", "\"top.u_regs.status.payload_hi[3:0]\""),
            ("offset", "12"),
            ("size", "4"),
            ("first", "1"),
        ],
    ));
    assert!(contains_named_call(
        &sv,
        "status.add_hdl_path_slice",
        &[
            ("name", "\"top.u_regs.status.payload_lo[3:0]\""),
            ("offset", "8"),
            ("size", "4"),
            ("first", "0"),
        ],
    ));
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate generated SystemVerilog class name `ral_reg_cfg_status_flag`"),
        "{error}"
    );
}

#[test]
fn normalizes_generated_systemverilog_identifiers_for_stress_names() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>123 top-unit with a very very long component name 0123456789</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>default</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>class</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>123 status!</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>1st-field</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>default</ipxact:name>
                <ipxact:value>0</ipxact:value>
              </ipxact:enumeratedValue>
              <ipxact:enumeratedValue>
                <ipxact:name>2-way</ipxact:name>
                <ipxact:value>1</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>bit</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>this is a very very long field-name with symbols !@# and digits 0123456789</ipxact:name>
            <ipxact:bitOffset>2</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>for</ipxact:name>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>this</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();

    assert_generated_sv_structural_gate("identifier stress", &sv);
    assert!(sv.contains("class ral_sys__123_top_unit_with_a_very_very_long_component_name_0123456789 extends uvm_reg_block;"));
    assert!(sv.contains("class ral_block__class extends uvm_reg_block;"));
    assert!(sv.contains("class ral_reg__class__123_status extends uvm_reg;"));
    assert!(sv.contains("class ral_reg__class__for extends uvm_reg;"));
    assert!(sv.contains("rand uvm_reg_field _1st_field;"));
    assert!(sv.contains("rand uvm_reg_field _bit;"));
    assert!(sv.contains(
        "rand uvm_reg_field this_is_a_very_very_long_field_name_with_symbols_and_digits_0123456789;"
    ));
    assert!(sv.contains("_1ST_FIELD_DEFAULT = 1'h0,"));
    assert!(sv.contains("_1ST_FIELD_2_WAY = 1'h1"));
    assert!(sv.contains("rand ral_reg__class__123_status _123_status;"));
    assert!(sv.contains("rand ral_reg__class__for _for;"));
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains(&field_configure_access("flag", "1", "0", "RO")));
    assert!(sv.contains(&field_configure_access("value", "3", "1", "RO")));
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RO\"",
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
    assert!(sv.contains(&field_configure_access("flag", "1", "0", "WO")));
    assert!(sv.contains(&field_configure_access("value", "3", "1", "RW")));
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
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
    assert!(sv.contains(&field_configure_access("flag", "1", "0", "WO")));
    assert!(sv.contains(&field_configure_access("value", "3", "1", "RW")));
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
}

#[test]
fn optionally_renders_register_bit_coverage() {
    let component = parse_ipxact(IPXACT_COMMON).unwrap();
    let sv = serialize_uvm_reg_with_options(
        &component,
        RenderOptions {
            coverage: true,
            ..RenderOptions::default()
        },
    )
    .unwrap();

    assert!(sv.contains("local uvm_reg_data_t m_data;"));
    assert!(sv.contains("covergroup cg_bits();"));
    assert!(sv.contains("done_bits: coverpoint {m_data[0:0], m_is_read} iff (m_be);"));
    assert_super_new(
        &sv,
        &[
            ("name", "name"),
            ("n_bits", "32"),
            ("has_coverage", "build_coverage(UVM_CVR_REG_BITS)"),
        ],
    );
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
    assert_eq!(state.reserved.as_deref(), Some("true"));

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();

    assert!(sv.contains(&field_configure_call(
        "clear",
        "1",
        "0",
        "W1C",
        field_args("0", "1'h0", "1", "1")
    )));
    assert_set_reset(&sv, "clear", "1'h1", "\"SOFT\"");
    assert!(!sv.contains("clear.set_compare"));
    assert!(sv.contains("state.set_compare(UVM_NO_CHECK);"));
    assert_hdl_path_slice(&sv, "irq", "{`IRQ_HDL_PATH, \".clear_q\"}", "0", "1", "1");
}

#[test]
fn applies_ipxact_reset_masks_to_generated_uvm_resets() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>masked_resets</ipxact:name>
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
            <ipxact:name>mode</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>4</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>4'hf</ipxact:value>
                <ipxact:mask>4'h5</ipxact:mask>
              </ipxact:reset>
              <ipxact:reset resetTypeRef="SOFT">
                <ipxact:value>4'ha</ipxact:value>
                <ipxact:mask>4'h3</ipxact:mask>
              </ipxact:reset>
            </ipxact:resets>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>undefined</ipxact:name>
            <ipxact:bitOffset>4</ipxact:bitOffset>
            <ipxact:bitWidth>4</ipxact:bitWidth>
            <ipxact:resets>
              <ipxact:reset>
                <ipxact:value>4'hf</ipxact:value>
                <ipxact:mask>4'h0</ipxact:mask>
              </ipxact:reset>
              <ipxact:reset resetTypeRef="SOFT">
                <ipxact:value>4'ha</ipxact:value>
                <ipxact:mask>4'h0</ipxact:mask>
              </ipxact:reset>
            </ipxact:resets>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let field = &component.blocks[0].registers[0].fields[0];

    assert_eq!(field.resets[0].value, "4'hf");
    assert_eq!(field.resets[0].mask.as_deref(), Some("4'h5"));
    assert_eq!(field.resets[1].value, "4'ha");
    assert_eq!(field.resets[1].mask.as_deref(), Some("4'h3"));

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains(&field_configure_call(
        "mode",
        "4",
        "0",
        "RW",
        field_args("0", "4'h5", "1", "1")
    )));
    assert_set_reset(&sv, "mode", "4'h2", "\"SOFT\"");
    assert!(sv.contains(&field_configure_call(
        "undefined",
        "4",
        "4",
        "RW",
        field_args("0", "4'h0", "0", "1")
    )));
    assert!(!sv.contains("undefined.set_reset"));
}

#[test]
fn maps_ipxact_read_write_side_effects_to_ieee_1800_2_access_strings() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>access_matrix</ipxact:name>
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
          <ipxact:name>effects</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>rw_clear</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                <ipxact:readAction>clear</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>rw_set</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                <ipxact:readAction>set</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>rw_once</ipxact:name>
            <ipxact:bitOffset>2</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-writeOnce</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>wo_once</ipxact:name>
            <ipxact:bitOffset>3</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>writeOnce</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w_set_r_clear</ipxact:name>
            <ipxact:bitOffset>4</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>set</ipxact:modifiedWriteValue>
                <ipxact:readAction>clear</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w_clear_r_set</ipxact:name>
            <ipxact:bitOffset>5</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>clear</ipxact:modifiedWriteValue>
                <ipxact:readAction>set</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>disabled</ipxact:name>
            <ipxact:bitOffset>6</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>no-access</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>write_only_effects</ipxact:name>
          <ipxact:addressOffset>4</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>woc</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>write-only</ipxact:access>
                <ipxact:modifiedWriteValue>clear</ipxact:modifiedWriteValue>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>wos</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>write-only</ipxact:access>
                <ipxact:modifiedWriteValue>set</ipxact:modifiedWriteValue>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w1src</ipxact:name>
            <ipxact:bitOffset>2</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>oneToSet</ipxact:modifiedWriteValue>
                <ipxact:readAction>clear</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w1crs</ipxact:name>
            <ipxact:bitOffset>3</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>oneToClear</ipxact:modifiedWriteValue>
                <ipxact:readAction>set</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w0src</ipxact:name>
            <ipxact:bitOffset>4</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>zeroToSet</ipxact:modifiedWriteValue>
                <ipxact:readAction>clear</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>w0crs</ipxact:name>
            <ipxact:bitOffset>5</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:modifiedWriteValue>zeroToClear</ipxact:modifiedWriteValue>
                <ipxact:readAction>set</ipxact:readAction>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();

    assert!(sv.contains(&field_configure_access("rw_clear", "1", "0", "WRC")));
    assert!(sv.contains(&field_configure_access("rw_set", "1", "1", "WRS")));
    assert!(sv.contains(&field_configure_access("rw_once", "1", "2", "W1")));
    assert!(sv.contains(&field_configure_access("wo_once", "1", "3", "WO1")));
    assert!(sv.contains(&field_configure_access("w_set_r_clear", "1", "4", "WSRC")));
    assert!(sv.contains(&field_configure_access("w_clear_r_set", "1", "5", "WCRS")));
    assert!(sv.contains(&field_configure_access("disabled", "1", "6", "NOACCESS")));
    assert!(sv.contains(&field_configure_access("woc", "1", "0", "WOC")));
    assert!(sv.contains(&field_configure_access("wos", "1", "1", "WOS")));
    assert!(sv.contains(&field_configure_access("w1src", "1", "2", "W1SRC")));
    assert!(sv.contains(&field_configure_access("w1crs", "1", "3", "W1CRS")));
    assert!(sv.contains(&field_configure_access("w0src", "1", "4", "W0SRC")));
    assert!(sv.contains(&field_configure_access("w0crs", "1", "5", "W0CRS")));
    assert_add_reg(
        &sv,
        "default_map",
        "effects",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
    assert!(contains_sv_call(
        &sv,
        "default_map.add_reg",
        &[
            ("rg", "write_only_effects"),
            ("offset", "`UVM_REG_ADDR_WIDTH'h4"),
            ("rights", "\"RW\""),
        ],
    ));
}

#[test]
fn reports_unmapped_ipxact_access_side_effects() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bad_access</ipxact:name>
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
            <ipxact:name>custom</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                <ipxact:modifiedWriteValue>modify</ipxact:modifiedWriteValue>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "unsupported IP-XACT access policy for field `custom`: access=`read-write`, modifiedWriteValue=modify, readAction=<none>"
        ),
        "{error}"
    );
}

#[test]
fn reports_unsupported_field_write_value_constraints() {
    let xml = unsupported_field_feature_xml(
        r#"<ipxact:writeValueConstraint><ipxact:minimum>0</ipxact:minimum><ipxact:maximum>3</ipxact:maximum></ipxact:writeValueConstraint>"#,
    );

    let error = parse_and_serialize_uvm_reg(&xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `writeValueConstraint` on field `limited`"),
        "{error}"
    );
}

#[test]
fn reports_unsupported_field_broadcasts() {
    let xml = unsupported_field_feature_xml(
        r#"<ipxact:broadcasts><ipxact:broadcastTo><ipxact:fieldRef fieldRef="other"/></ipxact:broadcastTo></ipxact:broadcasts>"#,
    );

    let error = parse_and_serialize_uvm_reg(&xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `broadcasts` on field `limited`"),
        "{error}"
    );
}

#[test]
fn reports_unsupported_field_access_restrictions() {
    let xml = unsupported_field_feature_xml(
        r#"<ipxact:accessRestrictions><ipxact:accessRestriction><ipxact:readAccessMask>0x1</ipxact:readAccessMask></ipxact:accessRestriction></ipxact:accessRestrictions>"#,
    );

    let error = parse_and_serialize_uvm_reg(&xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `accessRestrictions` on field `limited`"),
        "{error}"
    );
}

#[test]
fn reports_unsupported_register_access_restrictions() {
    let xml = unsupported_register_access_restrictions_xml();

    let error = parse_and_serialize_uvm_reg(&xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `accessRestrictions` on register `status`"),
        "{error}"
    );
}

#[test]
fn reports_unsupported_address_block_access_restrictions() {
    let xml = unsupported_address_block_access_restrictions_xml();

    let error = parse_and_serialize_uvm_reg(&xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT feature `accessRestrictions` on addressBlock `cfg`"),
        "{error}"
    );
}

fn unsupported_field_feature_xml(feature_xml: &str) -> String {
    format!(
        r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>unsupported_field_feature</ipxact:name>
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
            <ipxact:name>limited</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>2</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
                {feature_xml}
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#
    )
}

fn unsupported_register_access_restrictions_xml() -> String {
    r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>unsupported_register_feature</ipxact:name>
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
          <ipxact:accessPolicies>
            <ipxact:accessPolicy>
              <ipxact:access>read-write</ipxact:access>
              <ipxact:accessRestrictions>
                <ipxact:accessRestriction>
                  <ipxact:readAccessMask>0xff</ipxact:readAccessMask>
                </ipxact:accessRestriction>
              </ipxact:accessRestrictions>
            </ipxact:accessPolicy>
          </ipxact:accessPolicies>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>8</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#
        .into()
}

fn unsupported_address_block_access_restrictions_xml() -> String {
    r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>unsupported_block_feature</ipxact:name>
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
            <ipxact:access>read-write</ipxact:access>
            <ipxact:accessRestrictions>
              <ipxact:accessRestriction>
                <ipxact:readAccessMask>0xffff</ipxact:readAccessMask>
              </ipxact:accessRestriction>
            </ipxact:accessRestrictions>
          </ipxact:accessPolicy>
        </ipxact:accessPolicies>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>8</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#
        .into()
}

#[test]
fn renders_ipxact_alternate_registers() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let alternate = &component.blocks[0].registers[0].alternate_registers[0];

    assert_eq!(alternate.name, "debug_irq");
    assert_eq!(alternate.access.as_deref(), Some("read-only"));
    assert_eq!(alternate.fields[0].name, "raw");

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_regs_irq_debug_irq extends uvm_reg;"));
    assert!(sv.contains(&field_configure_call(
        "raw",
        "8",
        "0",
        "RO",
        field_args("0", "8'h0", "0", "0")
    )));
    assert!(sv.contains("rand ral_reg_regs_irq_debug_irq debug_irq;"));
    assert_add_reg(
        &sv,
        "default_map",
        "debug_irq",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RO\"",
    );
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

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains(&field_configure_call(
        "state",
        "4",
        "0",
        "RO",
        field_args("0", "4'h0", "0", "0")
    )));
    assert!(sv.contains("typedef enum bit [3:0] {"));
    assert!(sv.contains("STATE_IDLE = 4'h0,"));
    assert!(sv.contains("STATE_BUSY = 4'h1"));
    assert!(sv.contains("} state_e;"));
    assert_add_reg(
        &sv,
        "default_map",
        "block_status",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert!(sv.contains(&field_configure_call(
        "doorbell",
        "1",
        "0",
        "WO",
        field_args("0", "1'h0", "0", "1")
    )));
    assert_add_reg(
        &sv,
        "default_map",
        "gate",
        "`UVM_REG_ADDR_WIDTH'h20",
        "\"WO\"",
    );
}

#[test]
fn renders_ipxact_memory_blocks_as_uvm_mem() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let memory = &component.blocks[1];

    assert_eq!(memory.name, "packet_mem");
    assert_eq!(memory.usage.as_deref(), Some("memory"));
    assert_eq!(memory.access.as_deref(), Some("read-only"));
    assert_eq!(memory.hdl_path.as_deref(), Some("`PKT_MEM_HDL_PATH"));

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("uvm_mem packet_mem;"));
    assert_new(
        &sv,
        "packet_mem",
        &[
            ("name", "\"packet_mem\""),
            ("size", "64"),
            ("n_bits", "32"),
            ("access", "\"RO\""),
            ("has_coverage", "UVM_NO_COVERAGE"),
        ],
    );
    assert_configure(
        &sv,
        "packet_mem",
        &[("parent", "this"), ("hdl_path", "`PKT_MEM_HDL_PATH")],
    );
    assert_add_mem(
        &sv,
        "default_map",
        "packet_mem",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "packet_mem.default_map",
        "`UVM_REG_ADDR_WIDTH'h2000",
    );
}

#[test]
fn renders_read_write_ipxact_memory_blocks_as_rw_uvm_mem() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>rw_memory</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>buffer</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:usage>memory</ipxact:usage>
        <ipxact:access>read-write</ipxact:access>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();

    assert_new(
        &sv,
        "buffer",
        &[
            ("name", "\"buffer\""),
            ("size", "8"),
            ("n_bits", "32"),
            ("access", "\"RW\""),
            ("has_coverage", "UVM_NO_COVERAGE"),
        ],
    );
    assert_add_mem(
        &sv,
        "default_map",
        "buffer",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
}

#[test]
fn reports_ipxact_memory_access_that_uvm_mem_cannot_represent() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>wo_memory</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>sink</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:usage>memory</ipxact:usage>
        <ipxact:access>write-only</ipxact:access>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("unsupported IP-XACT memory access for memory block `sink`: `write-only`"),
        "{error}"
    );
}

#[test]
fn reports_zero_address_block_width_before_generating_uvm_map() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>zero_width_block</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>0</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for addressBlock width: `0`"),
        "{error}"
    );
}

#[test]
fn reports_zero_memory_block_range_before_generating_uvm_mem() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>zero_memory_range</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>buffer</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:usage>memory</ipxact:usage>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for addressBlock range: `0`"),
        "{error}"
    );
}

#[test]
fn reports_zero_memory_map_address_unit_bits() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>zero_aub</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressUnitBits>0</ipxact:addressUnitBits>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for memoryMap addressUnitBits: `0`"),
        "{error}"
    );
}

#[test]
fn optionally_renders_memory_address_coverage() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let sv = serialize_uvm_reg_with_options(
        &component,
        RenderOptions {
            coverage: true,
            ..RenderOptions::default()
        },
    )
    .unwrap();

    assert!(sv.contains("class ral_mem_packet_mem extends uvm_mem;"));
    assert!(sv.contains("`uvm_object_utils(ral_mem_packet_mem)"));
    assert!(sv.contains("covergroup cg_addr();"));
    assert!(sv.contains("offset: coverpoint m_offset;"));
    assert!(sv.contains("access: coverpoint m_is_read;"));
    assert_super_new(
        &sv,
        &[
            ("name", "name"),
            ("size", "64"),
            ("n_bits", "32"),
            ("access", "\"RO\""),
            ("has_coverage", "build_coverage(UVM_CVR_ADDR_MAP)"),
        ],
    );
    assert!(sv.contains("add_coverage(build_coverage(UVM_CVR_ADDR_MAP));"));
    assert!(sv.contains("if (get_coverage(UVM_CVR_ADDR_MAP)) begin"));
    assert!(sv.contains("cg_addr.sample();"));
    assert!(sv.contains("ral_mem_packet_mem packet_mem;"));
    assert_new(&sv, "packet_mem", &[("name", "\"packet_mem\"")]);
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

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_from_definition_status_from_def extends uvm_reg;"));
    assert!(sv.contains("READY_NOT_READY = 2'h0,"));
    assert!(sv.contains("READY_READY = 2'h1"));
    assert!(sv.contains(&field_configure_call(
        "ready",
        "2",
        "0",
        "RC",
        field_args("1", "2'h1", "1", "0")
    )));
    assert_add_reg(
        &sv,
        "default_map",
        "status_from_def",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "from_definition.default_map",
        "`UVM_REG_ADDR_WIDTH'h2400",
    );
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

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_banked_ctl_mode extends uvm_reg;"));
    assert_add_reg(
        &sv,
        "default_map",
        "mode",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
    assert_add_reg(
        &sv,
        "default_map",
        "value",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "banked_ctl.default_map",
        "`UVM_REG_ADDR_WIDTH'h3000",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "banked_stat.default_map",
        "`UVM_REG_ADDR_WIDTH'h3010",
    );
}

#[test]
fn resolves_top_level_subspace_maps_without_metadata_output() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let subspace = &component.subspace_maps[0];

    assert_eq!(subspace.name, "dma_window");
    assert_eq!(subspace.base_address, "0x2800");
    assert_eq!(subspace.segment_ref.as_deref(), Some("cfg_seg"));

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(!sv.contains("localparam"));
}

#[test]
fn preserves_memory_remaps_and_generates_their_registers() {
    let component = parse_ipxact(IPXACT_2022).unwrap();
    let remap = &component.memory_remaps[0];

    assert_eq!(remap.name, "low_power");
    assert_eq!(remap.blocks[0].name, "low_power_lp_regs");
    assert_eq!(remap.subspace_maps[0].name, "low_power_lp_window");

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains("class ral_reg_low_power_lp_regs_wake extends uvm_reg;"));
    assert!(sv.contains("rand ral_reg_low_power_lp_regs_wake low_power_lp_regs_wake;"));
    assert_add_reg(
        &sv,
        "default_map",
        "low_power_lp_regs_wake",
        "`UVM_REG_ADDR_WIDTH'h4000",
        "\"RO\"",
    );
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
    assert_add_reg(
        &sv,
        "default_map",
        "debug_dbg_regs_ctrl",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
    assert_add_reg(
        &sv,
        "default_map",
        "common_common_regs_ctrl",
        "`UVM_REG_ADDR_WIDTH'hc",
        "\"RW\"",
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

    let sv = parse_and_serialize_uvm_reg(IPXACT_2022).unwrap();
    assert!(sv.contains(&field_configure_call(
        "value",
        "32",
        "0",
        "RW",
        field_args("1", "32'h0", "0", "1")
    )));
    assert!(sv.contains("rand ral_reg_regs_counter counter[2][1];"));
    assert!(sv.contains("for (int unsigned i0 = 0; i0 < 2; i0++) begin"));
    assert!(sv.contains("for (int unsigned i1 = 0; i1 < 1; i1++) begin"));
    assert!(sv.contains("counter[i0][i1] = ral_reg_regs_counter::type_id::create($sformatf(\"counter_%0d_%0d\", i0, i1));"));
    assert_add_reg(
        &sv,
        "default_map",
        "counter[i0][i1]",
        "`UVM_REG_ADDR_WIDTH'h10 + (i0 * 1 + i1) * `UVM_REG_ADDR_WIDTH'h8",
        "\"RW\"",
    );
}

#[test]
fn reports_zero_register_array_dimensions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>zero_reg_dim</ipxact:name>
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
          <ipxact:array>
            <ipxact:dim>0</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for register dim: `0`"),
        "{error}"
    );
}

#[test]
fn reports_zero_register_file_array_dimensions() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>zero_regfile_dim</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:array>
            <ipxact:dim>0</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>4</ipxact:range>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for registerFile dim: `0`"),
        "{error}"
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("if (i == 0) begin"));
    assert_hdl_path_slice(
        &sv,
        "counter[i]",
        "\"top.u_regs.counter0\"",
        "-1",
        "-1",
        "1",
    );
    assert!(sv.contains("if (i == 1) begin"));
    assert_hdl_path_slice(
        &sv,
        "counter[i]",
        "\"top.u_regs.counter1\"",
        "-1",
        "-1",
        "1",
    );
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("if (i == 0) begin"));
    assert_hdl_path_slice(&sv, "status[i]", "\"top.u0.ready_q\"", "0", "1", "1");
    assert_hdl_path_slice(&sv, "status[i]", "\"top.u0.error_q\"", "1", "1", "0");
    assert!(sv.contains("if (i == 1) begin"));
    assert_hdl_path_slice(&sv, "status[i]", "\"top.u1.ready_q\"", "0", "1", "1");
    assert_hdl_path_slice(&sv, "status[i]", "\"top.u1.error_q\"", "1", "1", "0");
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle indices for register `status.ready` have 1 dimensions, expected 2"
        ),
        "{error}"
    );
}

#[test]
fn reports_out_of_range_indexed_register_access_handle_indices() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bad_array_path_index</ipxact:name>
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
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:indices><ipxact:index>2</ipxact:index></ipxact:indices>
              <ipxact:pathSegments><ipxact:pathSegment>top.u2.status_q</ipxact:pathSegment></ipxact:pathSegments>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle index 2 for `status` dimension 1 is outside register array dimension size 2"
        ),
        "{error}"
    );
}

#[test]
fn reports_out_of_range_indexed_field_access_handle_indices() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bad_field_array_path_index</ipxact:name>
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
            <ipxact:dim>3</ipxact:dim>
            <ipxact:stride>4</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:indices>
                  <ipxact:index>1</ipxact:index>
                  <ipxact:index>3</ipxact:index>
                </ipxact:indices>
                <ipxact:pathSegments><ipxact:pathSegment>top.u1.ready_q</ipxact:pathSegment></ipxact:pathSegments>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT accessHandle index 3 for `status.ready` dimension 2 is outside register array dimension size 3"
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("duplicate IP-XACT accessHandle indices for `status.ready`: `0`"),
        "{error}"
    );
}

#[test]
fn reports_indexed_access_handles_missing_paths() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_indexed_path</ipxact:name>
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
          <ipxact:accessHandles>
            <ipxact:accessHandle>
              <ipxact:indices><ipxact:index>0</ipxact:index></ipxact:indices>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT indexed accessHandle for `status` is missing a path"),
        "{error}"
    );
}

#[test]
fn reports_indexed_access_handles_missing_indices() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_indexed_indices</ipxact:name>
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
                <ipxact:indices/>
                <ipxact:pathSegments><ipxact:pathSegment>top.ready_q</ipxact:pathSegment></ipxact:pathSegments>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT indexed accessHandle for `status.ready` is missing indices"),
        "{error}"
    );
}

#[test]
fn reports_non_indexed_access_handles_missing_paths() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_path</ipxact:name>
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
          <ipxact:accessHandles>
            <ipxact:accessHandle/>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT accessHandle for `status` is missing a path"),
        "{error}"
    );
}

#[test]
fn reports_access_handle_slices_missing_paths() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_slice_path</ipxact:name>
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
            <ipxact:accessHandles>
              <ipxact:accessHandle>
                <ipxact:slices>
                  <ipxact:slice/>
                </ipxact:slices>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("IP-XACT accessHandle for `status.ready` is missing a path"),
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_create_map(&sv, "default_map", "\"default_map\"", "4", "0");
    assert_add_reg(
        &sv,
        "default_map",
        "ctrl",
        "`UVM_REG_ADDR_WIDTH'h2",
        "\"RW\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "cfg.default_map",
        "`UVM_REG_ADDR_WIDTH'h20",
    );
    assert_new(
        &sv,
        "ram",
        &[
            ("name", "\"ram\""),
            ("size", "4"),
            ("n_bits", "32"),
            ("access", "\"RW\""),
            ("has_coverage", "UVM_NO_COVERAGE"),
        ],
    );
    assert_add_mem(
        &sv,
        "default_map",
        "ram",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "ram.default_map",
        "`UVM_REG_ADDR_WIDTH'h40",
    );
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
        <ipxact:baseAddress>(32'sh80 &lt;&lt; 1) | 16</ipxact:baseAddress>
        <ipxact:range>'h80 &gt;&gt; 1</ipxact:range>
        <ipxact:width>8 &lt;&lt; 2</ipxact:width>
        <ipxact:register>
          <ipxact:name>sample</ipxact:name>
          <ipxact:array>
            <ipxact:dim>(1 &lt;&lt; 2) &gt;&gt; 1</ipxact:dim>
            <ipxact:stride>16 % 6</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>(4'h1 &lt;&lt; 2) | (3 &amp; 0)</ipxact:addressOffset>
          <ipxact:size>32'd16 &lt;&lt; 1</ipxact:size>
          <ipxact:field>
            <ipxact:name>mode</ipxact:name>
            <ipxact:bitOffset>(1 &lt;&lt; 2) &gt;&gt; 1</ipxact:bitOffset>
            <ipxact:bitWidth>7 &amp; 3</ipxact:bitWidth>
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
                <ipxact:value>(1 &lt;&lt; 2) ^ 3</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].base_address, "(32'sh80<<1) | 16");
    assert_eq!(component.blocks[0].registers[0].dim, "2");

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_reg_cfg_sample extends uvm_reg;"));
    assert!(sv.contains("typedef enum bit [2:0] {"));
    assert!(sv.contains("MODE_MAX = 3'h7"));
    assert!(sv.contains(&field_configure_call(
        "mode",
        "3",
        "2",
        "RW",
        field_args("0", "3'h3", "1", "1")
    )));
    assert!(sv.contains("rand ral_reg_cfg_sample sample[2];"));
    assert_add_reg(
        &sv,
        "default_map",
        "sample[i]",
        "`UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "cfg.default_map",
        "`UVM_REG_ADDR_WIDTH'h110",
    );
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
            <ipxact:enumeratedValues>
              <ipxact:enumeratedValue>
                <ipxact:name>busy</ipxact:name>
                <ipxact:value>ENUM_BUSY</ipxact:value>
              </ipxact:enumeratedValue>
            </ipxact:enumeratedValues>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("MODE_BUSY = 2'h3"));
    assert!(sv.contains(&field_configure_call(
        "mode",
        "2",
        "2",
        "RW",
        field_args("0", "2'h1", "1", "1")
    )));
    assert!(sv.contains("rand ral_reg_cfg_sample sample[2];"));
    assert_add_reg(
        &sv,
        "default_map",
        "sample[i]",
        "`UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "cfg.default_map",
        "`UVM_REG_ADDR_WIDTH'h90",
    );
}

#[test]
fn evaluates_parameterized_ipxact_boolean_metadata() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>bool_metadata</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:parameters>
    <ipxact:parameter parameterId="BLOCK_VOL">
      <ipxact:name>BLOCK_VOL</ipxact:name>
      <ipxact:value>0</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="REG_VOL">
      <ipxact:name>REG_VOL</ipxact:name>
      <ipxact:value>1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FIELD_VOL">
      <ipxact:name>FIELD_VOL</ipxact:name>
      <ipxact:value>1</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="FIELD_TESTABLE">
      <ipxact:name>FIELD_TESTABLE</ipxact:name>
      <ipxact:value>0</ipxact:value>
    </ipxact:parameter>
    <ipxact:parameter parameterId="RESERVED_FLAG">
      <ipxact:name>RESERVED_FLAG</ipxact:name>
      <ipxact:value>1</ipxact:value>
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
        <ipxact:volatile>BLOCK_VOL</ipxact:volatile>
        <ipxact:register>
          <ipxact:name>status</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:volatile>REG_VOL != 0</ipxact:volatile>
          <ipxact:field>
            <ipxact:name>ready</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:volatile>FIELD_VOL</ipxact:volatile>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:testable>FIELD_TESTABLE</ipxact:testable>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>reserved_bit</ipxact:name>
            <ipxact:bitOffset>1</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:reserved>RESERVED_FLAG</ipxact:reserved>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();
    let register = &component.blocks[0].registers[0];

    assert_eq!(component.blocks[0].volatile.as_deref(), Some("false"));
    assert_eq!(register.volatile.as_deref(), Some("true"));
    assert_eq!(register.fields[0].volatile.as_deref(), Some("true"));
    assert_eq!(register.fields[0].testable.as_deref(), Some("false"));
    assert_eq!(register.fields[1].reserved.as_deref(), Some("true"));

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains(&field_configure_call(
        "ready",
        "1",
        "0",
        "RW",
        field_args("1", "1'h0", "0", "1")
    )));
    assert!(sv.contains("ready.set_compare(UVM_NO_CHECK);"));
    assert!(sv.contains(&field_configure_call(
        "reserved_bit",
        "1",
        "1",
        "RW",
        field_args("1", "1'h0", "0", "1")
    )));
    assert!(sv.contains("reserved_bit.set_compare(UVM_NO_CHECK);"));
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_create_map(&sv, "default_map", "\"default_map\"", "8", "1");
    assert_super_new(
        &sv,
        &[
            ("name", "name"),
            ("n_bits", "64"),
            ("has_coverage", "UVM_NO_COVERAGE"),
        ],
    );
    assert!(sv.contains(&field_configure_call(
        "ready",
        "4",
        "0",
        "RW",
        field_args("0", "4'ha", "1", "1")
    )));
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h8",
        "\"RW\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "cfg.default_map",
        "`UVM_REG_ADDR_WIDTH'h100",
    );
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-write</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let component = parse_ipxact(xml).unwrap();

    assert_eq!(component.blocks[0].map_name, "cfg");
    assert_eq!(component.blocks[1].map_name, "status");

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("uvm_reg_map status_map;"));
    assert_create_map(&sv, "default_map", "\"default_map\"", "4", "1");
    assert_create_map(&sv, "status_map", "\"status\"", "4", "0");
    assert_add_reg(
        &sv,
        "default_map",
        "enable",
        "`UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
    assert_add_reg(
        &sv,
        "default_map",
        "count",
        "`UVM_REG_ADDR_WIDTH'h2",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "ctrls.default_map",
        "`UVM_REG_ADDR_WIDTH'h100",
    );
    assert_add_submap(
        &sv,
        "status_map",
        "stats.default_map",
        "`UVM_REG_ADDR_WIDTH'h10",
    );
    assert!(!sv.contains("default_map.add_submap(stats.default_map"));
}

#[test]
fn reports_field_ranges_that_exceed_register_size() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>field_too_wide</ipxact:name>
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
          <ipxact:size>8</ipxact:size>
          <ipxact:field>
            <ipxact:name>upper</ipxact:name>
            <ipxact:bitOffset>6</ipxact:bitOffset>
            <ipxact:bitWidth>4</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT field `upper` in register `status` spans bits 6..9, beyond register size 8"
        ),
        "{error}"
    );
}

#[test]
fn reports_zero_register_size_before_generating_uvm_reg() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>zero_reg_size</ipxact:name>
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
          <ipxact:size>0</ipxact:size>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for register size: `0`"),
        "{error}"
    );
}

#[test]
fn reports_zero_field_bit_width_before_generating_uvm_field() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>zero_field_width</ipxact:name>
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
            <ipxact:name>empty</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>0</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for field bitWidth: `0`"),
        "{error}"
    );
}

#[test]
fn reports_overlapping_field_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>field_overlap</ipxact:name>
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
          <ipxact:size>16</ipxact:size>
          <ipxact:field>
            <ipxact:name>low</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>8</ipxact:bitWidth>
          </ipxact:field>
          <ipxact:field>
            <ipxact:name>middle</ipxact:name>
            <ipxact:bitOffset>4</ipxact:bitOffset>
            <ipxact:bitWidth>4</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT fields `middle` and `low` overlap in register `status` at bits 4..7"
        ),
        "{error}"
    );
}

#[test]
fn reports_overlapping_register_address_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>register_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
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
          </ipxact:field>
        </ipxact:register>
        <ipxact:register>
          <ipxact:name>control</ipxact:name>
          <ipxact:addressOffset>2</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>enable</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `control` and `status` overlap in addressBlock `regs` at offsets 2..3"
        ),
        "{error}"
    );
}

#[test]
fn reports_self_overlapping_register_array_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>array_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x20</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:register>
          <ipxact:name>counter</ipxact:name>
          <ipxact:array>
            <ipxact:dim>2</ipxact:dim>
            <ipxact:stride>2</ipxact:stride>
          </ipxact:array>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:size>32</ipxact:size>
          <ipxact:field>
            <ipxact:name>value</ipxact:name>
            <ipxact:bitOffset>0</ipxact:bitOffset>
            <ipxact:bitWidth>1</ipxact:bitWidth>
          </ipxact:field>
        </ipxact:register>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `counter` and `counter` overlap in addressBlock `regs` at offsets 2..3"
        ),
        "{error}"
    );
}

#[test]
fn reports_overlapping_top_level_address_block_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>map_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs0</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:addressBlock>
        <ipxact:name>regs1</ipxact:name>
        <ipxact:baseAddress>0x8</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `regs1` and `regs0` overlap in memoryMap `cfg` at offsets 8..15"
        ),
        "{error}"
    );
}

#[test]
fn reports_overlapping_memory_remap_address_block_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>remap_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:memoryRemap>
        <ipxact:name>alt</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>debug0</ipxact:name>
          <ipxact:baseAddress>0x20</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
        <ipxact:addressBlock>
          <ipxact:name>debug1</ipxact:name>
          <ipxact:baseAddress>0x28</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:memoryRemap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `alt_debug1` and `alt_debug0` overlap in memoryMap `cfg` at offsets 40..47"
        ),
        "{error}"
    );
}

#[test]
fn reports_overlapping_subspace_map_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>subspace_overlap</ipxact:name>
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
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
  </ipxact:addressSpaces>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>cfg</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>regs</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x10</ipxact:range>
        <ipxact:width>32</ipxact:width>
      </ipxact:addressBlock>
      <ipxact:subspaceMap initiatorRef="dma_init" segmentRef="cfg_seg">
        <ipxact:name>dma_window</ipxact:name>
        <ipxact:baseAddress>0x28</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `dma_window` and `regs` overlap in memoryMap `cfg` at offsets 8..15"
        ),
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
            <ipxact:alternateRegisters>
              <ipxact:alternateRegister>
                <ipxact:name>shadow</ipxact:name>
                <ipxact:field>
                  <ipxact:name>raw</ipxact:name>
                  <ipxact:bitOffset>0</ipxact:bitOffset>
                  <ipxact:bitWidth>8</ipxact:bitWidth>
                  <ipxact:fieldAccessPolicies>
                    <ipxact:fieldAccessPolicy>
                      <ipxact:access>read-write</ipxact:access>
                    </ipxact:fieldAccessPolicy>
                  </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_regfile_cfg_local extends uvm_reg_file;"));
    assert!(sv.contains("ral_regfile_cfg_local local;"));
    assert!(sv.contains("local = ral_regfile_cfg_local::type_id::create(\"local\");"));
    assert_configure(
        &sv,
        "local",
        &[
            ("blk_parent", "this"),
            ("regfile_parent", "null"),
            ("hdl_path", "\"\""),
        ],
    );
    assert!(sv.contains("rand ral_reg_cfg_local_status status;"));
    assert!(sv.contains("status = ral_reg_cfg_local_status::type_id::create(\"status\");"));
    assert_configure(
        &sv,
        "status",
        &[("blk_parent", "get_block()"), ("regfile_parent", "this")],
    );
    assert_add_reg(
        &sv,
        "mp",
        "status",
        "offset + `UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert!(sv.contains("rand ral_reg_cfg_local_status_shadow shadow;"));
    assert_add_reg(
        &sv,
        "mp",
        "shadow",
        "offset + `UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();

    assert!(sv.contains("class ral_regfile_cfg_local extends uvm_reg_file;"));
    assert!(sv.contains("rand ral_reg_cfg_local_counter counter[2];"));
    assert!(sv.contains(
        "counter[i] = ral_reg_cfg_local_counter::type_id::create($sformatf(\"counter_%0d\", i));"
    ));
    assert_configure(
        &sv,
        "counter[i]",
        &[("blk_parent", "get_block()"), ("regfile_parent", "this")],
    );
    assert_add_reg(
        &sv,
        "mp",
        "counter[i]",
        "offset + `UVM_REG_ADDR_WIDTH'h4 + i * `UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
    assert!(sv.contains("ral_regfile_cfg_lane lane[2];"));
    assert!(
        sv.contains("lane[i] = ral_regfile_cfg_lane::type_id::create($sformatf(\"lane_%0d\", i));")
    );
    assert_configure(
        &sv,
        "lane[i]",
        &[
            ("blk_parent", "this"),
            ("regfile_parent", "null"),
            ("hdl_path", "\"\""),
        ],
    );
    assert!(sv.contains("rand ral_reg_cfg_lane_sample sample[3];"));
    assert!(sv.contains(
        "sample[i] = ral_reg_cfg_lane_sample::type_id::create($sformatf(\"sample_%0d\", i));"
    ));
    assert_configure(
        &sv,
        "sample[i]",
        &[("blk_parent", "get_block()"), ("regfile_parent", "this")],
    );
    assert_add_reg(
        &sv,
        "mp",
        "sample[i]",
        "offset + `UVM_REG_ADDR_WIDTH'h8 + i * `UVM_REG_ADDR_WIDTH'h4",
        "\"RW\"",
    );
}

#[test]
fn reports_overlapping_register_file_member_address_ranges() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x40</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>0x20</ipxact:range>
          <ipxact:register>
            <ipxact:name>status</ipxact:name>
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>value</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>32</ipxact:bitWidth>
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
            </ipxact:field>
          </ipxact:register>
          <ipxact:register>
            <ipxact:name>control</ipxact:name>
            <ipxact:addressOffset>2</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>enable</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `control` and `status` overlap in registerFile `cluster` at offsets 2..3"
        ),
        "{error}"
    );
}

#[test]
fn reports_self_overlapping_register_file_member_arrays() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_array_overlap</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x40</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>0x20</ipxact:range>
          <ipxact:register>
            <ipxact:name>counter</ipxact:name>
            <ipxact:array>
              <ipxact:dim>2</ipxact:dim>
              <ipxact:stride>2</ipxact:stride>
            </ipxact:array>
            <ipxact:addressOffset>0</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>value</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
              <ipxact:bitWidth>32</ipxact:bitWidth>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address ranges `counter` and `counter` overlap in registerFile `cluster` at offsets 2..3"
        ),
        "{error}"
    );
}

#[test]
fn reports_register_file_members_that_exceed_register_file_range() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_range</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>0x40</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>4</ipxact:range>
          <ipxact:register>
            <ipxact:name>control</ipxact:name>
            <ipxact:addressOffset>4</ipxact:addressOffset>
            <ipxact:size>32</ipxact:size>
            <ipxact:field>
              <ipxact:name>enable</ipxact:name>
              <ipxact:bitOffset>0</ipxact:bitOffset>
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

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "IP-XACT address range `control` in registerFile `cluster` ends at offset 7, beyond registerFile range 4"
        ),
        "{error}"
    );
}

#[test]
fn reports_zero_register_file_range_before_generating_uvm_reg_file() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>lib</ipxact:library>
  <ipxact:name>rf_zero_range</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>regs</ipxact:name>
      <ipxact:addressBlock>
        <ipxact:name>cfg</ipxact:name>
        <ipxact:baseAddress>0</ipxact:baseAddress>
        <ipxact:range>4</ipxact:range>
        <ipxact:width>32</ipxact:width>
        <ipxact:registerFile>
          <ipxact:name>cluster</ipxact:name>
          <ipxact:addressOffset>0</ipxact:addressOffset>
          <ipxact:range>0</ipxact:range>
        </ipxact:registerFile>
      </ipxact:addressBlock>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains("invalid IP-XACT number for registerFile range: `0`"),
        "{error}"
    );
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
        <ipxact:fieldAccessPolicies>
          <ipxact:fieldAccessPolicy>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:fieldAccessPolicy>
        </ipxact:fieldAccessPolicies>
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
                <ipxact:fieldAccessPolicies>
                  <ipxact:fieldAccessPolicy>
                    <ipxact:access>read-write</ipxact:access>
                  </ipxact:fieldAccessPolicy>
                </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(!sv.contains("localparam"));
    assert!(sv.contains(&field_configure_call(
        "ready",
        "1",
        "0",
        "RO",
        field_args("0", "1'h0", "0", "0")
    )));
    assert!(sv.contains(&field_configure_call(
        "raw",
        "8",
        "0",
        "RW",
        field_args("0", "8'h0", "0", "1")
    )));
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
        <ipxact:fieldAccessPolicies>
          <ipxact:fieldAccessPolicy>
            <ipxact:access>read-only</ipxact:access>
          </ipxact:fieldAccessPolicy>
        </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("STATE_B_VALUE = 2'h3"));
    assert!(sv.contains(&field_configure_call(
        "state",
        "2",
        "0",
        "RW",
        field_args("0", "2'h0", "0", "1")
    )));
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
          <ipxact:fieldAccessPolicies>
            <ipxact:fieldAccessPolicy>
              <ipxact:access>read-only</ipxact:access>
            </ipxact:fieldAccessPolicy>
          </ipxact:fieldAccessPolicies>
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
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "cfg.default_map",
        "`UVM_REG_ADDR_WIDTH'h40",
    );
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>write-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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
            <ipxact:fieldAccessPolicies>
              <ipxact:fieldAccessPolicy>
                <ipxact:access>read-only</ipxact:access>
              </ipxact:fieldAccessPolicy>
            </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert!(sv.contains("class ral_block_bridge_dma_space_dma_regs extends uvm_reg_block;"));
    assert!(sv.contains("class ral_sys_bridge_dma_space extends uvm_reg_block;"));
    assert!(sv.contains("class ral_sys_bridge extends uvm_reg_block;"));
    assert!(sv.contains("rand ral_reg_bridge_dma_space_dma_regs_doorbell doorbell;"));
    assert_add_reg(
        &sv,
        "default_map",
        "doorbell",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"WO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "dma_regs.default_map",
        "`UVM_REG_ADDR_WIDTH'h20",
    );
    assert!(sv.contains("rand ral_sys_bridge_dma_space dma_window;"));
    assert!(sv.contains("dma_window = ral_sys_bridge_dma_space::type_id::create(\"dma_window\");"));
    assert!(contains_sv_call(
        &sv,
        "default_map.add_submap",
        &[
            ("child_map", "dma_window.default_map"),
            ("offset", "`UVM_REG_ADDR_WIDTH'hfe0"),
        ],
    ));
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
fn reports_unresolved_subspace_map_address_spaces() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_subspace_address_space</ipxact:name>
  <ipxact:version>1.0</ipxact:version>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>host</ipxact:name>
      <ipxact:subspaceMap initiatorRef="missing_init">
        <ipxact:name>external_window</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "subspaceMap `external_window` initiatorRef `missing_init` does not resolve to a local addressSpace"
        ),
        "{error}"
    );
}

#[test]
fn reports_missing_subspace_map_segment_refs() {
    let xml = r#"
<ipxact:component xmlns:ipxact="http://www.accellera.org/XMLSchema/IPXACT/1685-2022">
  <ipxact:vendor>acme</ipxact:vendor>
  <ipxact:library>ip</ipxact:library>
  <ipxact:name>missing_subspace_segment</ipxact:name>
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
      <ipxact:localMemoryMap>
        <ipxact:name>dma_local</ipxact:name>
        <ipxact:addressBlock>
          <ipxact:name>dma_regs</ipxact:name>
          <ipxact:baseAddress>0</ipxact:baseAddress>
          <ipxact:range>0x10</ipxact:range>
          <ipxact:width>32</ipxact:width>
        </ipxact:addressBlock>
      </ipxact:localMemoryMap>
    </ipxact:addressSpace>
  </ipxact:addressSpaces>
  <ipxact:memoryMaps>
    <ipxact:memoryMap>
      <ipxact:name>host</ipxact:name>
      <ipxact:subspaceMap initiatorRef="dma_init" segmentRef="cfg_seg">
        <ipxact:name>dma_window</ipxact:name>
        <ipxact:baseAddress>0x1000</ipxact:baseAddress>
      </ipxact:subspaceMap>
    </ipxact:memoryMap>
  </ipxact:memoryMaps>
</ipxact:component>"#;

    let error = parse_and_serialize_uvm_reg(xml).unwrap_err().to_string();

    assert!(
        error.contains(
            "subspaceMap `dma_window` segmentRef `cfg_seg` was not found in addressSpace `dma_space`"
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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
                <ipxact:fieldAccessPolicies>
                  <ipxact:fieldAccessPolicy>
                    <ipxact:access>read-write</ipxact:access>
                  </ipxact:fieldAccessPolicy>
                </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_create_map(&sv, "default_map", "\"default_map\"", "4", "0");
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h1",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "b_regs.default_map",
        "`UVM_REG_ADDR_WIDTH'h2",
    );
    assert!(contains_sv_call(
        &sv,
        "default_map.add_reg",
        &[
            ("rg", "debug_dbg_regs_ctrl"),
            ("offset", "`UVM_REG_ADDR_WIDTH'h8"),
            ("rights", "\"RW\""),
        ],
    ));
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-only</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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
              <ipxact:fieldAccessPolicies>
                <ipxact:fieldAccessPolicy>
                  <ipxact:access>read-write</ipxact:access>
                </ipxact:fieldAccessPolicy>
              </ipxact:fieldAccessPolicies>
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

    let sv = parse_and_serialize_uvm_reg(xml).unwrap();
    assert_add_reg(
        &sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert_add_submap(
        &sv,
        "default_map",
        "banked_def_regs.default_map",
        "`UVM_REG_ADDR_WIDTH'h100",
    );
    assert_add_reg(
        &sv,
        "default_map",
        "lowpower_lp_regs_ctrl",
        "`UVM_REG_ADDR_WIDTH'h200",
        "\"RW\"",
    );
    assert!(!sv.contains("a_bank_block"));
    assert!(!sv.contains("a_remap_block"));
}

fn assert_demo_uvm_golden_patterns(label: &str, sv: &str) {
    let patterns = [
        "`ifndef RAL_DEMO_PKG_SV",
        "class ral_reg_regs_status extends uvm_reg;",
        "class ral_regfile_regs_lane extends uvm_reg_file;",
        "class ral_block_regs extends uvm_reg_block;",
        "class ral_sys_demo extends uvm_reg_block;",
        "rand ral_block_regs regs;",
        "rand ral_reg_regs_lane_ctrl ctrl;",
    ];

    for pattern in patterns {
        assert!(
            sv.contains(pattern),
            "{label}: missing generated UVM golden pattern `{pattern}`"
        );
    }
    let done_configure =
        field_configure_call("done", "1", "0", "RO", field_args("0", "1'h1", "1", "0"));
    assert!(
        sv.contains(&done_configure),
        "{label}: missing generated UVM golden pattern `{done_configure}`"
    );
    assert_create_map(sv, "default_map", "\"default_map\"", "4", "1");
    assert_hdl_path_slice(sv, "status", "{`REGS_HDL_PATH, \".done_q\"}", "0", "1", "1");
    assert_add_reg(
        sv,
        "default_map",
        "status",
        "`UVM_REG_ADDR_WIDTH'h4",
        "\"RO\"",
    );
    assert_add_submap(
        sv,
        "default_map",
        "regs.default_map",
        "`UVM_REG_ADDR_WIDTH'h1000",
    );
    assert_hdl_path_slice(sv, "ctrl", "\"top.u_regs.lane.enable_q\"", "0", "1", "1");
    assert_add_reg(
        sv,
        "mp",
        "ctrl",
        "offset + `UVM_REG_ADDR_WIDTH'h0",
        "\"RW\"",
    );
    assert_sv_call(
        sv,
        "lane[i].map",
        &[
            ("mp", "default_map"),
            (
                "offset",
                "`UVM_REG_ADDR_WIDTH'h20 + i * `UVM_REG_ADDR_WIDTH'h10",
            ),
        ],
    );
}

fn field_configure_access(name: &str, width: &str, lsb: &str, access: &str) -> String {
    format!(
        "{name}.configure(\n        .parent(this),\n        .size({width}),\n        .lsb_pos({lsb}),\n        .access(\"{access}\")"
    )
}

#[derive(Clone, Copy)]
struct FieldConfigureArgs<'a> {
    volatile: &'a str,
    reset: &'a str,
    has_reset: &'a str,
    is_rand: &'a str,
}

fn field_args<'a>(
    volatile: &'a str,
    reset: &'a str,
    has_reset: &'a str,
    is_rand: &'a str,
) -> FieldConfigureArgs<'a> {
    FieldConfigureArgs {
        volatile,
        reset,
        has_reset,
        is_rand,
    }
}

fn field_configure_call(
    name: &str,
    width: &str,
    lsb: &str,
    access: &str,
    args: FieldConfigureArgs<'_>,
) -> String {
    let FieldConfigureArgs {
        volatile,
        reset,
        has_reset,
        is_rand,
    } = args;
    format!(
        "{name}.configure(\n        .parent(this),\n        .size({width}),\n        .lsb_pos({lsb}),\n        .access(\"{access}\"),\n        .volatile({volatile}),\n        .reset({reset}),\n        .has_reset({has_reset}),\n        .is_rand({is_rand}),\n        .individually_accessible(1)\n      );"
    )
}

fn contains_named_call(sv: &str, callee: &str, args: &[(&str, &str)]) -> bool {
    let mut search_from = 0;
    while let Some(relative_start) = sv[search_from..].find(callee) {
        let start = search_from + relative_start;
        let after_callee = start + callee.len();
        if sv[after_callee..].starts_with('(') {
            let open = after_callee;
            if let Some(args_text) = call_args_text(sv, open) {
                let actual = compact_sv_text(args_text);
                if args.iter().all(|(name, value)| {
                    let expected = compact_sv_text(&format!(".{name}({value})"));
                    actual.contains(&expected)
                }) {
                    return true;
                }
            }
        }
        search_from = after_callee;
    }
    false
}

fn contains_positional_call(sv: &str, callee: &str, args: &[(&str, &str)]) -> bool {
    let expected = compact_sv_text(
        &args
            .iter()
            .map(|(_, value)| *value)
            .collect::<Vec<_>>()
            .join(", "),
    );
    let mut search_from = 0;
    while let Some(relative_start) = sv[search_from..].find(callee) {
        let start = search_from + relative_start;
        let after_callee = start + callee.len();
        if sv[after_callee..].starts_with('(') {
            let open = after_callee;
            if let Some(args_text) = call_args_text(sv, open)
                && compact_sv_text(args_text) == expected
            {
                return true;
            }
        }
        search_from = after_callee;
    }
    false
}

fn contains_sv_call(sv: &str, callee: &str, args: &[(&str, &str)]) -> bool {
    if args.len() <= 3 {
        contains_positional_call(sv, callee, args)
    } else {
        contains_named_call(sv, callee, args)
    }
}

fn assert_sv_call(sv: &str, callee: &str, args: &[(&str, &str)]) {
    assert!(
        contains_sv_call(sv, callee, args),
        "missing SystemVerilog call `{callee}` with args {args:?}"
    );
}

fn assert_named_call(sv: &str, callee: &str, args: &[(&str, &str)]) {
    assert!(
        contains_named_call(sv, callee, args),
        "missing named call `{callee}` with args {args:?}"
    );
}

fn assert_create_map(sv: &str, var_name: &str, name: &str, n_bytes: &str, byte_addressing: &str) {
    assert_named_call(
        sv,
        &format!("{var_name} = create_map"),
        &[
            ("name", name),
            ("base_addr", "0"),
            ("n_bytes", n_bytes),
            ("endian", "UVM_LITTLE_ENDIAN"),
            ("byte_addressing", byte_addressing),
        ],
    );
}

fn assert_add_reg(sv: &str, map: &str, rg: &str, offset: &str, rights: &str) {
    assert_sv_call(
        sv,
        &format!("{map}.add_reg"),
        &[("rg", rg), ("offset", offset), ("rights", rights)],
    );
}

fn assert_add_mem(sv: &str, map: &str, mem: &str, offset: &str, rights: &str) {
    assert_sv_call(
        sv,
        &format!("{map}.add_mem"),
        &[("mem", mem), ("offset", offset), ("rights", rights)],
    );
}

fn assert_add_submap(sv: &str, map: &str, child_map: &str, offset: &str) {
    assert_sv_call(
        sv,
        &format!("{map}.add_submap"),
        &[("child_map", child_map), ("offset", offset)],
    );
}

fn assert_hdl_path_slice(
    sv: &str,
    receiver: &str,
    name: &str,
    offset: &str,
    size: &str,
    first: &str,
) {
    assert_named_call(
        sv,
        &format!("{receiver}.add_hdl_path_slice"),
        &[
            ("name", name),
            ("offset", offset),
            ("size", size),
            ("first", first),
        ],
    );
}

fn assert_set_reset(sv: &str, field: &str, value: &str, kind: &str) {
    assert_sv_call(
        sv,
        &format!("{field}.set_reset"),
        &[("value", value), ("kind", kind)],
    );
}

fn assert_configure(sv: &str, target: &str, args: &[(&str, &str)]) {
    assert_sv_call(sv, &format!("{target}.configure"), args);
}

fn assert_super_new(sv: &str, args: &[(&str, &str)]) {
    assert_sv_call(sv, "super.new", args);
}

fn assert_new(sv: &str, target: &str, args: &[(&str, &str)]) {
    assert_sv_call(sv, &format!("{target} = new"), args);
}

fn compact_sv_text(text: &str) -> String {
    let mut out = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for ch in text.chars() {
        if in_string {
            out.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            out.push(ch);
        } else if !ch.is_ascii_whitespace() {
            out.push(ch);
        }
    }

    out
}

fn assert_generated_sv_structural_gate(label: &str, sv: &str) {
    assert!(
        sv.contains("import uvm_pkg::*;"),
        "{label}: missing uvm_pkg import"
    );
    assert!(
        sv.contains("`include \"uvm_macros.svh\""),
        "{label}: missing uvm_macros include"
    );
    assert_generated_include_sv_structural_gate(label, sv);
}

fn assert_generated_include_sv_structural_gate(label: &str, sv: &str) {
    assert!(sv.contains("`ifndef "), "{label}: missing include guard");
    assert!(
        sv.contains("`define "),
        "{label}: missing include guard define"
    );
    assert!(
        sv.trim_end().ends_with("`endif"),
        "{label}: missing final endif"
    );
    assert!(
        !sv.contains("uvm_reg_map default_map;\n"),
        "{label}: generated code redeclares uvm_reg_block::default_map"
    );
    assert_generated_uvm_call_shapes(label, sv);

    let structurally_active_sv = sv_assuming_protected_sample_macros(sv);
    let words = sv_words_outside_strings_and_comments(&structurally_active_sv);
    assert_generated_sv_identifiers(label, &words, sv);
    assert_balanced_word(label, &words, "class", "endclass");
    assert_balanced_word(label, &words, "function", "endfunction");
    assert_balanced_word(label, &words, "covergroup", "endgroup");
    assert_balanced_word(label, &words, "begin", "end");

    let class_count = words.iter().filter(|word| word.as_str() == "class").count();
    assert!(class_count > 0, "{label}: no generated classes");
    assert_eq!(
        class_count,
        sv.matches(" extends ").count(),
        "{label}: every generated class should declare an extends type"
    );
    assert_eq!(
        class_count,
        sv.matches("`uvm_object_utils(").count(),
        "{label}: every generated class should be factory registered"
    );
    assert!(
        sv.contains("virtual function void build();") || sv.contains("function new("),
        "{label}: generated classes should expose UVM construction/build methods"
    );
}

fn assert_contains_before(haystack: &str, needle: &str, marker: &str) {
    let needle_index = haystack
        .find(needle)
        .unwrap_or_else(|| panic!("missing `{needle}` in generated output"));
    let marker_index = haystack
        .find(marker)
        .unwrap_or_else(|| panic!("missing `{marker}` in generated output"));
    assert!(
        needle_index < marker_index,
        "expected `{needle}` to appear before `{marker}`"
    );
}

fn assert_generated_uvm_call_shapes(label: &str, sv: &str) {
    assert!(
        !sv.contains("create_map(\""),
        "{label}: create_map has more than three arguments and should use named argument association"
    );
    assert!(
        !sv.contains(".add_hdl_path_slice(") || sv.contains(".first("),
        "{label}: add_hdl_path_slice has more than three arguments and should use named argument association"
    );
}

fn call_args_text(line: &str, open: usize) -> Option<&str> {
    let mut paren_depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let args_start = open + 1;

    for (offset, ch) in line[open..].char_indices() {
        let index = open + offset;
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => paren_depth += 1,
            ')' => {
                paren_depth = paren_depth.checked_sub(1)?;
                if paren_depth == 0 {
                    return Some(&line[args_start..index]);
                }
            }
            _ => {}
        }
    }

    None
}

fn assert_generated_sv_identifiers(label: &str, words: &[String], sv: &str) {
    for pair in words.windows(2) {
        if pair[0] == "class" {
            assert!(
                is_valid_sv_identifier(&pair[1]),
                "{label}: invalid generated class identifier `{}`",
                pair[1]
            );
        }
    }

    let mut remaining = sv;
    while let Some(start) = remaining.find("`uvm_object_utils(") {
        let after_start = &remaining[start + "`uvm_object_utils(".len()..];
        let Some(end) = after_start.find(')') else {
            panic!("{label}: unterminated uvm_object_utils macro");
        };
        let name = after_start[..end].trim();
        assert!(
            is_valid_sv_identifier(name),
            "{label}: invalid uvm_object_utils identifier `{name}`"
        );
        remaining = &after_start[end + 1..];
    }
}

fn is_valid_sv_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    if !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()) {
        return false;
    }
    !is_sv_keyword_for_test(value)
}

fn is_sv_keyword_for_test(value: &str) -> bool {
    matches!(
        value,
        "class"
            | "endclass"
            | "function"
            | "endfunction"
            | "package"
            | "endpackage"
            | "rand"
            | "int"
            | "bit"
            | "begin"
            | "end"
            | "default"
            | "for"
            | "if"
            | "else"
            | "this"
            | "super"
            | "null"
    )
}

fn assert_balanced_word(label: &str, words: &[String], open: &str, close: &str) {
    let open_count = words.iter().filter(|word| word.as_str() == open).count();
    let close_count = words.iter().filter(|word| word.as_str() == close).count();
    assert_eq!(
        open_count, close_count,
        "{label}: unbalanced {open}/{close} tokens"
    );
}

fn sv_words_outside_strings_and_comments(sv: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = sv.chars().peekable();
    let mut in_string = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }

        match ch {
            '"' => {
                push_sv_word(&mut words, &mut current);
                in_string = true;
            }
            '/' if chars.peek() == Some(&'/') => {
                push_sv_word(&mut words, &mut current);
                chars.next();
                in_line_comment = true;
            }
            '/' if chars.peek() == Some(&'*') => {
                push_sv_word(&mut words, &mut current);
                chars.next();
                in_block_comment = true;
            }
            ch if ch.is_ascii_alphanumeric() || ch == '_' => current.push(ch),
            _ => push_sv_word(&mut words, &mut current),
        }
    }
    push_sv_word(&mut words, &mut current);
    words
}

fn push_sv_word(words: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        words.push(std::mem::take(current));
    }
}

fn sv_assuming_protected_sample_macros(sv: &str) -> String {
    let mut out = Vec::new();
    let mut in_sample_ifdef = false;
    let mut skipping_else_branch = false;

    for line in sv.lines() {
        let trimmed = line.trim();
        if matches!(
            trimmed,
            "`ifdef UVM_REG_PROTECTED_SAMPLE" | "`ifdef UVM_MEM_PROTECTED_SAMPLE"
        ) {
            in_sample_ifdef = true;
            skipping_else_branch = false;
            continue;
        }
        if in_sample_ifdef && trimmed == "`else" {
            skipping_else_branch = true;
            continue;
        }
        if in_sample_ifdef && trimmed == "`endif" {
            in_sample_ifdef = false;
            skipping_else_branch = false;
            continue;
        }
        if !skipping_else_branch {
            out.push(line);
        }
    }

    out.join("\n")
}
