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
cargo run -p irgen-cli -- ip-xact path/to/component.xml --file-layout blocks -o ral_component
cargo run -p irgen-cli -- ip-xact path/to/component.xml --view rtl
cargo run -p irgen-cli -- ip-xact path/to/component.xml --mode diagnostic
cargo run -p irgen-cli -- ip-xact path/to/component.xml --library-path path/to/ipxact/library
```

`--file-layout single` is the default and writes one SystemVerilog file. With
`--file-layout blocks`, `-o/--output` is treated as an output directory. The
directory contains a top-level `ral_<component-name>.sv` file plus one
`ral_block_<block-name>.sv` file for each generated address-block class; the
top-level file includes the block files. If `-o/--output` is omitted in block
layout, the directory defaults to `ral_<component-name>`.

For IEEE 1685-2022 `externalTypeDefinitions`, the CLI scans XML files in the
input file's directory and any `--library-path` directories. It matches
component/type-definition XML directly by VLNV and also follows IP-XACT
`catalog` files whose `ipxactFile` entries point to matching XML files.

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
  If the referenced local address space exposes blocks outside the segment
  range, generation reports a diagnostic instead of emitting an overbroad
  submap.
- `memoryRemap` contents as generated registers.
- Resets, including reset type names and additional reset values through
  `set_reset`.
- Common constant numeric expressions for offsets, ranges, widths, dimensions,
  reset values, and enum values, including decimal/hex/binary literals,
  SystemVerilog-sized integer literals, underscores, parentheses, `+ - * /`
  arithmetic, simple component/type-definition parameters, and
  `configurableElementValue referenceId` constants. Referenced 2022
  type-definition instances may override definition-local numeric parameters
  through local `configurableElementValues`. XML text entity references such as
  `&lt;`, `&gt;`, and `&amp;` are decoded before expression evaluation.
- Static `isPresent` filtering for RAL-relevant nodes when the condition is a
  boolean, concrete number, supported parameter expression, or simple static
  comparison/logical expression using `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`,
  `||`, and `!`.
- Access policies and field access policies when they define the effective
  access used by generated registers or fields.
- Generation-time `--mode <modeRef>` selection for mode-specific
  `accessPolicy` and `fieldAccessPolicy` entries, plus filtering of
  `memoryRemap` entries by `modeRef`. When multiple policies match the
  requested mode, lower numeric `modeRef priority` values are selected first.
- Field `testable=false` and `reserved=true` metadata as UVM RAL compare
  control through `uvm_reg_field::set_compare(UVM_NO_CHECK)`.
- Common UVM access strings for read-only, write-only, read-write, write-one,
  write-zero, read-clear, and related field side-effect forms.
- Field enumerated values as SystemVerilog enum typedefs.
- Optional register and memory coverage with `--coverage`. This emits UVM
  `UVM_CVR_REG_BITS` support in generated `uvm_reg` classes using
  `build_coverage`, `add_coverage`, `get_coverage`, a `cg_bits` covergroup,
  and a `sample()` override compatible with UVM IEEE 2020. It also emits
  generated `uvm_mem` subclasses with `UVM_CVR_ADDR_MAP` address coverage for
  IP-XACT memory blocks.
  This option generates the coverage implementation; it does not by itself
  prove that a simulator run has enabled and reported that coverage.
  A consuming testbench must enable UVM register coverage before model
  construction, for example with `uvm_reg::include_coverage("*",
  UVM_CVR_REG_BITS | UVM_CVR_ADDR_MAP)`, and the simulator build/run must also
  enable coverage database collection.
- Retained-only metadata such as descriptions, access restrictions, broadcasts,
  reset masks, and enum usage comments is intentionally ignored until it drives
  generated UVM behavior.
- HDL backdoor paths from IP-XACT `accessHandles`:
  - field slices through `add_hdl_path_slice`
  - multi-slice field paths with `slice/range` mapped to multiple
    `add_hdl_path_slice` calls
  - whole-register paths when field slices are absent
  - memory block HDL paths through `uvm_mem.configure`
  - multiple `accessHandle` entries by preferring a generic handle with no
    `viewRef`, then falling back to the first path-bearing handle
  - explicit `--view <viewRef>` selection for view-specific backdoor paths
  - register-array element paths from IP-XACT `accessHandle/indices`, including
    field-level indexed slices
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
- Additional `--library-path` directories and IP-XACT `catalog` files for
  external 2022 `typeDefinitionsRef` resolution by VLNV.
- Diagnostics for generated SystemVerilog class-name collisions after IP-XACT
  names are normalized to legal identifiers.
- Diagnostics for duplicate IP-XACT sibling names before map layout
  generation, including `memoryMap`, `addressSpace`, `segment`,
  `addressBlock`, `subspaceMap`, `memoryRemap`, `register`, `registerFile`,
  `alternateRegister`, `field`, and `enumeratedValue` names.
- Diagnostics for missing internal or scoped 2022 type-definition references,
  including memory-map, remap, bank, address-block, register-file, register,
  field, enumeration, and field-access-policy definitions.
- Diagnostics for malformed IP-XACT `accessHandle` entries, including indexed
  dimension mismatches, duplicate indices on the same register or field, and
  multi-slice field paths with missing or width-mismatched ranges.

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
| Common single-project register map generation | ~75-80% | Core UVM classes, maps, fields, arrays, resets, access values, generation-time mode/view selection, common backdoor paths, diagnostics, and optional register/memory coverage are present. The largest remaining risks are simulator regression coverage, richer configurable expression semantics, unsupported-feature diagnostics, and local coding-style expectations. |
| Common engineering usability across five input versions | ~60-70% | All five namespaces have parser entry tests, and the shared parser covers common direct register-map shapes. Each version still needs realistic fixtures that exercise its common schema shape. IEEE 1685-2014 and IEEE 1685-2022 are closer to usable than SPIRIT 1.4, SPIRIT 1.5, and IEEE 1685-2009. |
| Broad IP-XACT RAL completeness | ~40-50% | Generation-time mode/view selection, catalog-backed type-definition resolution, common configurable constants, diagnostics, and selected coverage/backdoor modeling are implemented. Runtime mode/view behavior, choices/assertions, vendor extensions, cross-component address spaces, and complete coverage/backdoor modeling remain out of scope. |

The estimate assumes the first milestone is "common engineering usable" rather
than complete IP-XACT schema coverage. A reasonable remaining effort estimate is:

| Workstream | Rough effort | Notes |
| --- | ---: | --- |
| Repeatable simulator and syntax gates | 2-4 days | Add a documented simulator smoke gate for generated UVM IEEE 2020 output and a smaller simulator-independent syntax/golden check where possible. |
| Parameter/configurable evaluation | 1-3 days | Expand beyond simple numeric constants and local definition-instance overrides into choices, assertions, richer parameter type semantics, non-static `isPresent` diagnostics, and unsupported-expression reporting. |
| Backdoor robustness | 1-3 days | Indexed register/field access handles, view selection, and multi-slice fields are present. Remaining work is richer indexed selection policy, skipped-handle diagnostics, and clearer macro/string path diagnostics. |
| Coverage robustness | 1-2 days | Register bit and memory address-map coverage are generated. Remaining work is a coverage smoke gate, runtime enablement checks, and an explicit decision on block, memory-data, cross, and user-configurable coverage scope. |
| Realistic five-version fixture matrix | 3-5 days | Add at least one realistic fixture per input version, preferably with golden generated SV checks. |
| Naming, diagnostics, and stress tests | 1-3 days | Common sibling-name, generated-class-name, missing type-definition, segment-range, and malformed indexed-accessHandle diagnostics are present. Remaining work is keyword, long-name, generated-member collision, and broader unsupported-feature diagnostics. |

Overall, the remaining work for a common engineering usable version is roughly
one to three engineering weeks, dominated by fixture breadth and simulator
gates rather than basic parser/render coverage. A much more complete IP-XACT
RAL generator, including runtime mode/view behavior, rich vendor-extension
handling, cross-component address-space semantics, and project-specific style
controls, is still more likely a multi-month effort.

## Known Limitations

The generator is not yet complete IP-XACT RAL coverage. Important limitations:

- No simulator compile gate is run by default. Generated SV has Rust tests and
  real-sample generation tests, but CI still needs a repeatable simulator smoke
  gate before this should be called production-stable.
- Catalog-backed VLNV resolution is currently focused on external 2022
  `typeDefinitionsRef` documents. It scans the input directory and explicit
  `--library-path` directories, follows catalog entries, and resolves matching
  XML files by VLNV. Missing references report the searched paths, and
  duplicate VLNV matches are reported as ambiguous. It is not yet a full
  multi-root library manager with precedence controls or all IP-XACT document
  kinds modeled in generated RAL.
- `segmentRef` offset correction is implemented for local address-space
  segments. Because UVM maps do not directly expose a simple submap clipping
  primitive, generation reports an error when a referenced address-space block
  falls outside the selected segment range.
- `memoryRemap` contents are generated. With `--mode`, remaps with matching
  `modeRef` and remaps without a `modeRef` are generated while other
  mode-specific remaps are skipped. Mode-dependent runtime map switching is not
  modeled yet.
- Mode/view behavior is only partially modeled. `--mode` selects matching
  register and field access policies and filters memory remaps during
  generation, including `modeRef priority` ordering for matching policies, and
  `--view` selects matching HDL backdoor paths. Runtime mode switching and
  retained-only mode/view metadata are not modeled yet.
- Coverage is currently generated for register bit coverage and memory
  address-map coverage. Block-level, cross, memory data, and user-configurable
  coverage models are not generated yet. The `--coverage` option emits
  covergroups and sampling hooks; coverage reporting still depends on the
  consuming simulation enabling UVM RAL coverage before model construction and
  enabling simulator coverage collection.
- Parameter and configurable expression evaluation is intentionally limited.
  Numeric fields used for offsets, widths, ranges, dimensions, and resets may
  use concrete decimal/hex/binary literals, SystemVerilog-sized integer
  literals, underscores, parentheses, `+ - * /` arithmetic, simple
  component/type-definition parameters, and `configurableElementValue
  referenceId` constants. Local `configurableElementValues` on referenced
  type-definition instances override definition-local numeric parameters during
  parsing. XML text entity references such as `&lt;`, `&gt;`, and `&amp;` are
  decoded before expression evaluation. Choices, assertions, vendor extensions,
  richer parameter type semantics, and non-static `isPresent` conditions are
  not interpreted. Static boolean/numeric `isPresent` values, supported
  parameter expressions, and simple comparison/logical conditions are evaluated
  for RAL-relevant nodes before generation.
- Access-handle selection can target a specific IP-XACT `viewRef` through
  `--view`. When no view is requested, generated RAL prefers handles that apply
  to all views and otherwise falls back to the first path-bearing handle.
  Register-array element paths from IP-XACT `accessHandle/indices` are
  supported for register-level paths and field-level slices; richer indexed
  path selection is still limited. Indexed accessHandles whose index count does
  not match the register array dimensions, or whose indices duplicate another
  handle on the same register or field, are reported as generation errors.
  Multi-slice field paths require slice ranges, and the generated slices must
  add up to the field width.
- Address spaces referenced through other components are not resolved. The
  implemented submap path handles local address spaces in the same component.
- Default generated SV intentionally omits unused metadata localparams.

## Remaining Work

Recommended next implementation order for the common engineering usable
milestone:

1. Add a repeatable simulator smoke gate for generated UVM IEEE 2020 output
   when proprietary tools are available.
2. Add a coverage smoke gate that enables `uvm_reg::include_coverage("*",
   UVM_CVR_REG_BITS | UVM_CVR_ADDR_MAP)`, runs register and memory accesses,
   and checks that generated `cg_bits` and `cg_addr` coverpoints appear in the
   simulator coverage report.
3. Add a simulator-independent generated-SV fixture, or a lightweight open
   simulator gate for the subset it supports.
4. Expand expression/configurable evaluation beyond simple constants and local
   definition-instance overrides, including choices, assertions, richer
   parameter type semantics, and clearer diagnostics.
5. Add diagnostics for ignored or partially modeled IP-XACT features, including
   mode/view behavior, skipped access handles, unsupported expressions, and
   unresolved cross-document references beyond the currently modeled
   type-definition flow.
6. Add broader duplicate-name stress tests for generated-name collisions across
   deep hierarchy and type-definition expansion. Common IP-XACT sibling names
   and generated class-name collisions are already reported.
7. Improve HDL backdoor handling:
   - richer indexed path selection
   - clearer macro/string path diagnostics
8. Expand `segmentRef` support beyond local address spaces if cross-component
   address-space resolution is added later.
9. Expand catalog/search-path based VLNV resolution with precedence controls
   and richer multi-root library management.
10. Add more real-world fixtures for each supported input version.
11. Add a public compatibility matrix in release notes once each version has at
   least one realistic passing fixture.

Work after the common engineering usable milestone:

- Richer parameter/configurable expression evaluation.
- Runtime mode-aware generated access behavior.
- Runtime view-aware inclusion/exclusion behavior.
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
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --file-layout blocks -o ral_component
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --coverage -o ral_component_cov.sv
```

For generated coverage to appear in simulator reports, the consuming
testbench must enable UVM RAL coverage before model construction and the
simulator invocation must enable coverage database collection. Register bit
coverage uses `UVM_CVR_REG_BITS`; memory address coverage uses
`UVM_CVR_ADDR_MAP`.
