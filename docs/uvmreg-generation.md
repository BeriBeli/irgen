# UVM Register Model Generation

## Status

`irgen ip-xact` generates UVM IEEE 2020 register model SystemVerilog directly
from IP-XACT component XML. This path is intentionally separate from the
snapsheet-to-IR conversion flow: IP-XACT input is parsed into the `uvmreg`
crate's own register model and then rendered through a SystemVerilog template.

When `-o/--output` is omitted, the generated file is named:

```text
ral_<component-name>.sv
```

The first practical milestone is a common engineering usable generator, not a
complete implementation of every IP-XACT schema branch. In this document,
"common engineering usable" means the generator should handle real register
component XML that uses memory maps, address blocks, banks, registers,
register files, arrays, fields, resets, access metadata, descriptions,
enumerations, local type definitions, common 2022 type-definition reuse, and
basic HDL backdoor paths.

## CLI

Generate UVM RAL from an IP-XACT component XML file:

```sh
cargo run -p irgen-cli -- ip-xact path/to/component.xml
cargo run -p irgen-cli -- ip-xact path/to/component.xml -o ral_component.sv
```

For IEEE 1685-2022 `externalTypeDefinitions`, the CLI scans XML files in the
input file's directory and matches the referenced document by VLNV. This covers
the common project layout where a top component XML and shared type-definition
XML files live together. It is not yet a full catalog-backed VLNV library
resolver.

## Current Coverage

The current generator covers these UVM register-model constructs:

- Include-style SystemVerilog class files targeting UVM IEEE 2020. The file can
  be included inside an existing testbench module, matching many legacy RAL
  environments.
- `uvm_reg_block` / `uvm_reg` classes plus a default top-level naming alias
  class named `ral_sys_<component-name>`.
- One or more IP-XACT `memoryMap` elements mapped to UVM maps.
- `addressUnitBits`, including non-byte-addressed maps where representable by
  UVM map settings.
- `addressBlock` elements as real child `uvm_reg_block` classes. The top
  component block connects them with `add_submap`.
- `usage=memory` blocks as `uvm_mem` instances inside their address-block
  classes.
- `register`, `field`, and `alternateRegister`.
- Scalar and multidimensional `dim` / `array` forms for registers and register
  files.
- `registerFile` hierarchy using generated `uvm_reg_file` subclasses, so common
  test sequences can use natural paths such as `model.block0.regfile_0[i].rega`.
- Top-level and banked `bank` structures, flattened into address blocks while
  preserving deterministic generated names.
- IEEE 1685-2022 `addressSpaces/addressSpace/localMemoryMap` mapped into
  child `uvm_reg_block` classes and connected with `add_submap` when a
  `subspaceMap` can be resolved through an initiator `addressSpaceRef`.
- `memoryRemap` contents as generated registers plus metadata for remap names
  and modes.
- Resets, including reset type names and additional reset values through
  `set_reset`.
- Access policies and field access policies, including mode references as
  generated metadata.
- Common UVM access strings for read-only, write-only, read-write, write-one,
  write-zero, read-clear, and related field side-effect forms.
- Field enumerated values as SystemVerilog enum typedefs.
- Descriptions as string metadata on blocks, register files, registers,
  alternate registers, and fields.
- Field write-value constraints, testable/reserved flags, access restrictions,
  and broadcasts as metadata.
- HDL backdoor paths from IP-XACT `accessHandles`:
  - field slices through `add_hdl_path_slice`
  - whole-register paths when field slices are absent
  - memory block HDL paths through `uvm_mem.configure`
- Local 2022 `typeDefinitions` and scoped `typeDefinitions="..."` references
  for:
  - `memoryMapDefinitionRef`
  - `bankDefinitionRef`
  - `remapDefinitionRef`
  - `addressBlockDefinitionRef`
  - `registerDefinitionRef`
  - `registerFileDefinitionRef`
  - `fieldDefinitionRef`
  - `enumerationDefinitionRef`
  - `fieldAccessPolicyDefinitionRef`
- Same-directory external 2022 `typeDefinitionsRef` resolution by VLNV.

## Five-Version Usability Assessment

The parser is namespace-agnostic and has tests for SPIRIT 1.4, SPIRIT 1.5,
IEEE 1685-2009, IEEE 1685-2014, and IEEE 1685-2022 component roots. For common
engineering register XML, the practical status is:

| Input version | Common engineering usability | Notes |
| --- | --- | --- |
| SPIRIT 1.4 | Partial but useful | Direct memory maps, address blocks, registers, fields, resets, and access metadata are expected to work. Older schema shape lacks standard modern register-model constructs such as 2014/2022-style `accessHandles` and `registerFile`, so arrays/hierarchy may already be flattened or represented differently in source XML. |
| SPIRIT 1.5 | Partial but useful | Similar to 1.4, with better register-file support in the schema. Common direct register maps should parse, but newer 2022 type-definition reuse and address-space submap features do not exist in this version. |
| IEEE 1685-2009 | Partial but useful | Common component register maps should parse. Modern 2014/2022 access-handle and type-definition patterns are not expected in this version. |
| IEEE 1685-2014 | Good for common register maps | Direct memory maps, registers, register files, banks, fields, resets, access metadata, and standard access handles are in the expected usable path. Some 2022-only type-definition structures are not present. |
| IEEE 1685-2022 | Best current coverage | This is the primary target. The generator supports common 2022 type definitions, external type-definition references, memory-map definitions, bank definitions, remap definitions, address spaces, local memory maps, submaps, and access handles. |

The UVM output target is currently the same for all input versions:
UVM IEEE 2020-style SystemVerilog. Version differences affect what IP-XACT
source XML can express and therefore what the parser can recover.

## Known Limitations

The generator is not yet complete IP-XACT RAL coverage. Important limitations:

- No simulator compile gate is run by default. Generated SV has Rust tests,
  real-sample generation tests, and has been manually compiled in the
  `../ral_demo` VCS/UVM IEEE 2020 environment, but CI still needs a repeatable
  simulator smoke gate before this should be called production-stable.
- No full catalog-backed VLNV resolver. Same-directory external
  `typeDefinitionsRef` is supported, but IP-XACT catalog traversal and search
  paths are not.
- `segmentRef` is preserved as metadata, but submap offset/range clipping based
  on address-space segments is not implemented yet.
- `memoryRemap` is generated as remap registers plus metadata. Mode-dependent
  runtime map switching is not modeled yet.
- Mode/view/reset-type semantics are mostly metadata. The generated model does
  not yet synthesize dynamic mode-aware access behavior.
- Parameter and expression evaluation is minimal. Numeric fields used for
  offsets, widths, ranges, dimensions, and resets must be parseable as concrete
  numbers in common decimal, hex, or binary forms.
- Configurable element values, choices, assertions, vendor extensions, and
  complex `isPresent` conditions are not interpreted.
- Only the first relevant `accessHandle/pathSegments` path is used in several
  cases; multiple abstraction-specific backdoor paths still need richer
  selection.
- If an IP-XACT HDL path segment is written as a SystemVerilog macro, the
  consuming build must define that macro as a string expression when the path
  is used by UVM APIs. For example, VCS needs escaped quotes:
  `+define+BLOCK0_HDL_PATH=\"dut.u_regs\"`.
- Address spaces referenced through other components are not resolved. The
  implemented submap path handles local address spaces in the same component.
- Generated metadata localparams can become large for very large register
  files; downstream style or simulator limits have not been characterized.

## Remaining Work

Recommended next implementation order for the common engineering usable
milestone:

1. Add a repeatable VCS/UVM IEEE 2020 smoke target or script that can run the
   `ahb_slave` demo compile outside CI when proprietary tools are available.
2. Add a small simulator-independent SV syntax fixture, or a lightweight open
   simulator gate for the subset it supports.
3. Add deterministic duplicate-name stress tests for blocks, maps, fields,
   enum values, register files, submaps, and metadata localparams.
4. Improve HDL backdoor handling:
   - multiple access handles
   - abstraction-specific selection
   - clearer handling for arrayed HDL paths
5. Implement `segmentRef` offset/range behavior for submaps.
6. Add optional catalog/search-path based VLNV resolution for external IP-XACT
   documents.
7. Add more real-world fixtures for each supported input version.
8. Add a public compatibility matrix in release notes once each version has at
   least one realistic passing fixture.

Work after the common engineering usable milestone:

- Parameter/configurable expression evaluation.
- Mode-aware generated access behavior.
- View-aware inclusion/exclusion behavior.
- More complete vendor-extension preservation or selected vendor-extension
  mapping.
- Cross-component address-space resolution.
- Stronger SystemVerilog style controls for teams with local UVM coding
  conventions.

## Verification Gates

Useful local gates:

```text
cargo fmt --all
cargo test -p irgen-uvmreg -p irgen-cli
cargo run -q -p irgen-cli -- ip-xact ahb_slave/ahb_slave-ipxact-2022.xml -o /tmp/ral_ahb_slave.sv
cargo run -q -p irgen-cli -- ip-xact nic/nic-ipxact-2022.xml -o /tmp/ral_nic_uvmreg.sv
```

The `ahb_slave` sample has also been manually checked against the sibling
`../ral_demo` environment by copying the generated file over the demo's
included RAL file:

```text
cp /tmp/ral_ahb_slave.sv /tmp/ral_demo_irgen.<id>/ral_ahb_slave_ralf.sv
make compile \
  EXTRA_VCS_ARGS='+define+BLOCK0_HDL_PATH=\"dut_wrapper.ahb_svt_dut.u_ahb_slave_reg\" +define+BLOCK1_HDL_PATH=\"dut_wrapper.ahb_svt_dut.u_ahb_slave_reg\"'
```

That compile completed successfully with VCS `-ntb_opts uvm-ieee-2020-2.0`.
Running `ral_reg_write_read_test` and `ral_backdoor_access_test` reached
simulation startup, then stopped at time 0 because the local Synopsys VIP
license session could not be initialized. That is an environment/licensing
blocker rather than a generated-RAL compile issue.
