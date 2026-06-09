# UVM Register Model Generation

## Status

`irgen ip-xact` generates include-style UVM IEEE 1800.2-2020 register model
SystemVerilog directly from IP-XACT component XML. This path is separate from
the snapsheet-to-IR conversion flow: IP-XACT input is parsed into the `uvmreg`
crate's register model and rendered through SystemVerilog templates.

The current milestone is "common engineering usable" generation for real
register-map XML. That means direct memory maps, address blocks, banks,
registers, register files, arrays, fields, resets, access values,
enumerations, common 2022 type-definition reuse, generation-time mode/view
selection, and common HDL backdoor paths.

Simulator validation is intentionally outside the current target. Until a UVM
verification environment is available, correctness is guarded by Rust tests,
generated-SystemVerilog structural checks, and golden-pattern checks.

When `-o/--output` is omitted, the generated file is named:

```text
ral_<component-name>.sv
```

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
component/type-definition XML directly by VLNV and follows IP-XACT `catalog`
files whose `ipxactFile` entries point to matching XML files.

## Supported Generation

| Area | Current support |
| --- | --- |
| UVM output shape | Generates `uvm_reg_block`, `uvm_reg`, `uvm_reg_file`, and `uvm_mem` classes with conventional `ral_sys_*`, `ral_block_*`, `ral_regfile_*`, and `ral_reg_*` names. Generated maps use IEEE UVM `create_map(name, 0, n_bytes, UVM_LITTLE_ENDIAN, byte_addressing)` and avoid redeclaring `uvm_reg_block::default_map`. |
| File layout | Supports single-file output and block-split output. Generated files are include-style SystemVerilog intended to be consumed by an existing UVM testbench/package setup. |
| Memory maps | Supports one or more IP-XACT `memoryMap` elements, `addressUnitBits`, `addressBlock`, `bank`, `memoryRemap`, and local 2022 `addressSpaces/addressSpace/localMemoryMap` through generated child blocks and `add_submap`. |
| Registers and hierarchy | Supports `register`, `alternateRegister`, `registerFile`, scalar and multidimensional register/register-file arrays, top-level and banked structures, and deterministic flattened names where the IP-XACT structure requires flattening. |
| Fields | Supports field construction/configuration, enumerated values as SystemVerilog enum typedefs, `volatile`, `testable=false`, and `reserved=true` metadata when those values are static. `testable=false` and `reserved=true` generate `set_compare(UVM_NO_CHECK)`. |
| Access values | Maps common IP-XACT access and side-effect combinations to IEEE 1800.2 predefined UVM field access strings, including `RW`, `RO`, `WO`, `W1`, `WO1`, `NOACCESS`, `RC`, `RS`, `WC`, `WS`, `W1C`, `W1S`, `W1T`, `W0C`, `W0S`, `W0T`, `WRC`, `WRS`, `WOC`, `WOS`, `WSRC`, `WCRS`, `W1SRC`, `W1CRS`, `W0SRC`, and `W0CRS`. Unsupported combinations are reported instead of being emitted as `RW`. |
| Memories | `usage=memory` address blocks generate `uvm_mem` instances. IP-XACT omitted or `read-write` memory access becomes UVM `"RW"`; `read-only` becomes `"RO"`. Other memory access values are reported because IEEE 1800.2 `uvm_mem::new` only defines `"RW"` and `"RO"`. |
| Resets | Supports reset values, reset masks, reset type names, and additional reset values through `set_reset`. |
| Expressions | Evaluates common static numeric expressions for offsets, ranges, widths, dimensions, reset values, enum values, and static boolean metadata. Supported operators include parentheses, `+`, `-`, `*`, `/`, `%`, `<<`, `>>`, `&`, `|`, `^`, comparisons, `&&`, `||`, and `!`, with checked unsigned arithmetic. Simple parameters and `configurableElementValue referenceId` constants are supported, including local overrides on referenced 2022 type-definition instances. |
| `isPresent` | Filters RAL-relevant nodes when `isPresent` is a static boolean, concrete number, supported parameter/configurable expression, or supported comparison/logical expression. Invalid or unsupported `isPresent` expressions are reported. |
| Mode selection | `--mode <modeRef>` selects matching register/addressBlock access policies, field access policies, and matching `memoryRemap` entries at generation time. Matching policies honor lower numeric `modeRef priority` first. If a requested mode has no matching or generic policy, generation reports a diagnostic instead of using another explicit mode. |
| View selection | `--view <viewRef>` selects matching HDL backdoor `accessHandle` entries with generic handles as the only fallback. Handles for other explicit views are not used as fallback. |
| HDL backdoor paths | Uses IEEE UVM public APIs instead of emitting `uvm_hdl_path_slice` literals directly. Field slices generate `add_hdl_path_slice(path, offset, size, first)`. Whole-register paths use the UVM `offset=-1`, `size=-1` convention. Multi-slice fields, register-array indexed handles, field-level indexed slices, memory HDL paths, and selected path-bearing fallback are supported. |
| Type definitions | Supports local 2022 `typeDefinitions`, scoped `typeDefinitions="..."` references, same-directory external `typeDefinitionsRef`, additional `--library-path` directories, and catalog-backed external 2022 type-definition resolution by VLNV. |
| Coverage | `--coverage` emits register bit coverage with `UVM_CVR_REG_BITS` and memory address-map coverage with `UVM_CVR_ADDR_MAP`, including generated covergroups and sampling hooks. Runtime coverage still depends on the consuming testbench enabling UVM RAL coverage before model construction and enabling simulator coverage collection. |
| Diagnostics | Reports duplicate sibling names, missing type definitions, generated SystemVerilog class/member name collisions, illegal generated identifiers, malformed access handles, unsupported access policies, unsupported behavior-affecting schema features, invalid booleans, zero sizes/ranges, field overlaps, register/register-file/address-block/memory-map address overlaps, unresolved submaps, and unsupported parameter/configurable expressions used by generated RAL. |
| Retained metadata | Descriptions, comments, and enum usage comments are intentionally ignored until they drive generated UVM behavior. |

## IP-XACT Version Status

The parser is namespace-agnostic and has tests for SPIRIT 1.4, SPIRIT 1.5,
IEEE 1685-2009, IEEE 1685-2014, and IEEE 1685-2022 component roots. The same
shared register-map fixture is generated through all five namespaces and
checked by the generated-SV structural/golden gate.

| Input version | Common engineering usability | Notes |
| --- | --- | --- |
| SPIRIT 1.4 | Partial but useful | Direct memory maps, address blocks, registers, fields, resets, and access values are expected to work. Modern schema features such as 2014/2022 access handles and 2022 type definitions do not exist in this version. |
| SPIRIT 1.5 | Partial but useful | Similar to 1.4, with better register-file support in the schema. Newer 2022 type-definition and address-space submap features do not exist. |
| IEEE 1685-2009 | Partial but useful | Common component register maps should parse. Modern 2014/2022 access-handle and type-definition patterns are not expected. |
| IEEE 1685-2014 | Good for common register maps | Direct memory maps, registers, register files, banks, fields, resets, access values, and standard access handles are in the usable path. |
| IEEE 1685-2022 | Best current coverage | Primary target. Supports common type definitions, external references, memory-map definitions, bank/remap definitions, address spaces, local memory maps, submaps, and access handles. |

The UVM output target is the same for all input versions:
UVM IEEE 1800.2-2020-style SystemVerilog. Version differences affect what the
source XML can express and what the parser can recover.

## Readiness

| Target | Estimated readiness | Main gap |
| --- | ---: | --- |
| Common single-project register map generation | ~80-85% | Richer configurable expression semantics, broader unsupported-feature diagnostics, more realistic fixtures, and local coding-style expectations. |
| Common engineering usability across five input versions | ~65-75% | Each version needs realistic version-specific fixtures, not only the shared structural fixture. |
| Broad IP-XACT RAL completeness | ~50-60% | Runtime mode/view behavior, choices/assertions, vendor extensions, cross-component address spaces, complete coverage/backdoor modeling, and project-specific style controls. |

These estimates assume the near-term target is common engineering usability,
not complete IP-XACT schema coverage. The remaining non-simulator work is
mostly fixture breadth, expression/configurable semantics, and unsupported
feature diagnostics.

## Known Limitations

- No simulator compile gate is run by default. Current validation is
  simulator-independent and does not replace compiling the generated UVM RAL in
  a real verification environment.
- Catalog-backed VLNV resolution is focused on external 2022
  `typeDefinitionsRef` documents. It is not a full multi-root IP-XACT library
  manager with precedence controls or all document kinds modeled in generated
  RAL.
- `segmentRef` offset correction is implemented for local address-space
  segments. Generation reports an error when the referenced local address
  space exposes blocks outside the selected segment range, because UVM maps do
  not provide a simple submap clipping primitive.
- `memoryRemap` is selected at generation time. Runtime mode-dependent map
  switching is not modeled.
- Mode/view metadata that does not affect generated access policy or HDL
  backdoor path selection is retained-only and not modeled at runtime.
- Coverage generation currently covers register bits and memory address maps.
  Block-level, cross, memory data, and user-configurable coverage models are
  not generated yet.
- Parameter/configurable evaluation is intentionally static and limited.
  Choices, assertions, richer parameter type semantics, vendor extensions, and
  non-static `isPresent` behavior are not interpreted.
- Access-handle support covers common paths, view selection, indexed register
  arrays, indexed field slices, and multi-slice fields. Richer indexed path
  selection, skipped-handle reporting, and macro/path-expression diagnostics
  still need expansion.
- Address spaces referenced through other components are not resolved. The
  implemented submap path handles local address spaces in the same component.
- Default generated SystemVerilog intentionally omits unused metadata
  localparams.

## Remaining Work

Recommended next implementation order for the common engineering usable
milestone:

1. Add realistic version-specific golden fixtures for all supported IP-XACT
   namespaces, building on the shared generated-SV structural/golden gate.
2. Expand expression/configurable evaluation, especially choices, assertions,
   richer parameter type semantics, and broader unsupported-expression
   diagnostics.
3. Add diagnostics for more ignored or partially modeled IP-XACT features,
   including skipped access handles, retained-only mode/view metadata, and
   unresolved cross-document references beyond the type-definition flow.
4. Broaden duplicate-name and generated-name stress tests across deep
   hierarchy and type-definition expansion.
5. Improve HDL backdoor handling for richer indexed selection, skipped
   non-selected handle reporting, and clearer macro/path-expression
   diagnostics.
6. Expand `segmentRef` and catalog/search-path support if cross-component
   address-space or multi-root library resolution becomes part of the target.
7. Add a public compatibility matrix in release notes once each input version
   has at least one realistic passing fixture.

Later, when a UVM verification environment is available, add simulator smoke
gates for generated UVM IEEE 1800.2-2020 output and coverage collection. The
coverage smoke should enable:

```systemverilog
uvm_reg::include_coverage("*", UVM_CVR_REG_BITS | UVM_CVR_ADDR_MAP);
```

and then run register/memory accesses and check that generated `cg_bits` and
`cg_addr` coverpoints appear in the simulator coverage report.

Work after the common engineering usable milestone:

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
