# UVM Register Model Generation

## Status

`irgen ip-xact` generates UVM IEEE 2020 register model SystemVerilog directly
from IP-XACT component XML. This path is intentionally separate from the
snapsheet-to-IR conversion flow: IP-XACT input is parsed into the `uvmreg`
crate's own register model and then rendered through a SystemVerilog template.
The template is organized as a small top-level include/header template plus
partials for register, register-file, and block classes, following the
same maintainability direction as generators such as PeakRDL-uvm where package
and header emission are template-driven rather than one large hard-coded file.

When `-o/--output` is omitted, the generated file is named:

```text
ral_<component-name>.sv
```

The first practical milestone is a common engineering usable generator, not a
complete implementation of every IP-XACT schema branch. In this document,
"common engineering usable" means the generator should handle real register
component XML that uses memory maps, address blocks, banks, registers,
register files, arrays, fields, resets, access values,
enumerations, local type definitions, common 2022 type-definition reuse, and
basic HDL backdoor paths.

## CLI

Generate UVM RAL from an IP-XACT component XML file:

```sh
cargo run -p irgen-cli -- ip-xact path/to/component.xml
cargo run -p irgen-cli -- ip-xact path/to/component.xml -o ral_component.sv
cargo run -p irgen-cli -- ip-xact path/to/component.xml --coverage
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
- `uvm_reg_block` / `uvm_reg` classes. Generated class names follow a
  conventional RAL style:
  `ral_reg_<path>`, `ral_regfile_<path>`, `ral_block_<block>`, and top-level
  `ral_sys_<component-name>`.
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
- `addressSpace/segments/segment` offset handling. When `subspaceMap segmentRef`
  resolves to a local address-space segment, the generated `add_submap` offset
  subtracts the segment `addressOffset` so the segment's local addresses land
  at the subspace map base address.
- `memoryRemap` contents as generated registers.
- Resets, including reset type names and additional reset values through
  `set_reset`.
- Access policies and field access policies when they define the effective
  access used by generated registers or fields.
- Common UVM access strings for read-only, write-only, read-write, write-one,
  write-zero, read-clear, and related field side-effect forms.
- Field enumerated values as SystemVerilog enum typedefs.
- Optional register coverage with `--coverage`. This emits UVM
  `UVM_CVR_REG_BITS` support in generated `uvm_reg` classes using
  `build_coverage`, `add_coverage`, `get_coverage`, a `cg_bits` covergroup,
  and a `sample()` override compatible with UVM IEEE 2020.
  This option generates the register coverage implementation; it does not by
  itself prove that a simulator run has enabled and reported that coverage.
  A consuming testbench must enable UVM register coverage before model
  construction, for example with `uvm_reg::include_coverage("*",
  UVM_CVR_REG_BITS)`, and the simulator build/run must also enable coverage
  database collection.
- Retained-only metadata such as descriptions, policy mode references,
  testable/reserved flags, access restrictions, broadcasts, reset masks, and
  enum usage comments is intentionally ignored until it drives generated UVM
  behavior.
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
| SPIRIT 1.4 | Partial but useful | Direct memory maps, address blocks, registers, fields, resets, and access values are expected to work. Older schema shape lacks standard modern register-model constructs such as 2014/2022-style `accessHandles` and `registerFile`, so arrays/hierarchy may already be flattened or represented differently in source XML. |
| SPIRIT 1.5 | Partial but useful | Similar to 1.4, with better register-file support in the schema. Common direct register maps should parse, but newer 2022 type-definition reuse and address-space submap features do not exist in this version. |
| IEEE 1685-2009 | Partial but useful | Common component register maps should parse. Modern 2014/2022 access-handle and type-definition patterns are not expected in this version. |
| IEEE 1685-2014 | Good for common register maps | Direct memory maps, registers, register files, banks, fields, resets, access values, and standard access handles are in the expected usable path. Some 2022-only type-definition structures are not present. |
| IEEE 1685-2022 | Best current coverage | This is the primary target. The generator supports common 2022 type definitions, external type-definition references, memory-map definitions, bank definitions, remap definitions, address spaces, local memory maps, submaps, and access handles. |

The UVM output target is currently the same for all input versions:
UVM IEEE 2020-style SystemVerilog. Version differences affect what IP-XACT
source XML can express and therefore what the parser can recover.

## Completion Assessment

Current implementation status, using a practical engineering-readiness scale:

| Target | Estimated readiness | Meaning |
| --- | ---: | --- |
| Common single-project register map generation | ~70% | Core UVM classes, maps, fields, arrays, resets, access values, basic backdoor, and optional register bit coverage are present. The largest remaining risks are simulator regression coverage, expression handling, diagnostics, and richer backdoor/coverage policy. |
| Common engineering usability across five input versions | ~55-65% | All five namespaces have parser entry tests, but each version still needs realistic fixtures that exercise its common schema shape. IEEE 1685-2014 and IEEE 1685-2022 are closer to usable than SPIRIT 1.4, SPIRIT 1.5, and IEEE 1685-2009. |
| Broad IP-XACT RAL completeness | ~30-40% | Full mode/view semantics, catalog traversal, configurable expressions, vendor extensions, cross-component address spaces, and complete coverage/backdoor modeling are not implemented yet. |

The estimate assumes the first milestone is "common engineering usable" rather
than complete IP-XACT schema coverage. A reasonable remaining effort estimate is:

| Workstream | Rough effort | Notes |
| --- | ---: | --- |
| Repeatable simulator and syntax gates | 2-4 days | Add a documented simulator smoke gate for generated UVM IEEE 2020 output and a smaller simulator-independent syntax/golden check where possible. |
| Expression and parameter evaluation | 3-5 days | Evaluate concrete parameter/configurable expressions for offsets, ranges, widths, dimensions, and reset values. |
| Backdoor robustness | 2-4 days | Handle multiple access handles, abstraction-specific paths, arrayed HDL paths, and clearer macro/string path diagnostics. |
| Coverage robustness | 2-3 days | Add a coverage smoke gate, clarify runtime enablement, and decide how much block/memory/cross coverage belongs in the common milestone. |
| Realistic five-version fixture matrix | 3-5 days | Add at least one realistic fixture per input version, preferably with golden generated SV checks. |
| Naming, diagnostics, and stress tests | 2-4 days | Add deterministic duplicate-name, keyword, long-name, and unsupported-feature diagnostics. |

Overall, the remaining work for a common engineering usable version is roughly
two to four engineering weeks. A much more complete IP-XACT RAL generator,
including mode/view behavior, catalog-backed VLNV resolution, rich vendor
extension handling, and cross-component address-space semantics, is more likely
a multi-month effort.

## Known Limitations

The generator is not yet complete IP-XACT RAL coverage. Important limitations:

- No simulator compile gate is run by default. Generated SV has Rust tests and
  real-sample generation tests, but CI still needs a repeatable simulator smoke
  gate before this should be called production-stable.
- No full catalog-backed VLNV resolver. Same-directory external
  `typeDefinitionsRef` is supported, but IP-XACT catalog traversal and search
  paths are not.
- `segmentRef` offset correction is implemented for local address-space
  segments. Strict range clipping based on the segment `range` is not generated
  yet because UVM maps do not directly expose a simple submap clipping primitive.
- `memoryRemap` contents are generated, but mode-dependent runtime map
  switching is not modeled yet.
- Mode/view behavior is not modeled yet. Retained-only metadata is not parsed
  into the model by default.
- Coverage is currently generated for `uvm_reg` classes only. Block-level,
  memory, cross, and user-configurable coverage models are not generated yet.
  The `--coverage` option emits covergroups and sampling hooks; coverage
  reporting still depends on the consuming simulation enabling UVM RAL coverage
  before model construction and enabling simulator coverage collection.
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
- Default generated SV intentionally omits unused metadata localparams.

## Remaining Work

Recommended next implementation order for the common engineering usable
milestone:

1. Add a repeatable simulator smoke gate for generated UVM IEEE 2020 output
   when proprietary tools are available.
2. Add a coverage smoke gate that enables `uvm_reg::include_coverage("*",
   UVM_CVR_REG_BITS)`, runs register accesses, and checks that generated
   `cg_bits` coverpoints appear in the simulator coverage report.
3. Add a simulator-independent generated-SV fixture, or a lightweight open
   simulator gate for the subset it supports.
4. Add expression/configurable evaluation for numeric fields used by offsets,
   ranges, widths, dimensions, and reset values.
5. Add diagnostics for ignored or partially modeled IP-XACT features, including
   mode/view behavior, skipped access handles, unsupported expressions, and
   unresolved external references.
6. Add deterministic duplicate-name stress tests for blocks, maps, fields,
   enum values, register files, and submaps.
7. Improve HDL backdoor handling:
   - multiple access handles
   - abstraction-specific selection
   - clearer handling for arrayed HDL paths
8. Implement stricter `segmentRef` range behavior for submaps, likely through
   generated wrapper maps or explicit diagnostics when a child map exposes
   addresses outside the referenced segment.
9. Add optional catalog/search-path based VLNV resolution for external IP-XACT
   documents.
10. Add more real-world fixtures for each supported input version.
11. Add a public compatibility matrix in release notes once each version has at
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
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml -o ral_component.sv
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --coverage -o ral_component_cov.sv
```

For generated coverage to appear in simulator reports, the consuming
testbench must enable UVM RAL coverage before model construction and the
simulator invocation must enable coverage database collection.
