# irgen CLI Roadmap

This document tracks follow-up work after splitting the CLI, snapsheet parser,
and lightweight model crates.

## Current Structure

- `crates/cli`: command-line entry point for spreadsheet to IP-XACT conversion.
- `crates/snapsheet`: spreadsheet loading, register expansion, and model
  assembly.
- `crates/model`: lightweight register IR and conversion into the IP-XACT 2014
  model.
- `crates/ipxact`: broader IP-XACT model library; its IEEE 1685-2014 types now
  back the CLI's emitted model.

The active dependency direction is `cli -> snapsheet -> model -> ipxact`, with
`cli` also depending on `model` for serialization.

The CLI can convert `example.xlsx` into IEEE 1685-2014 XML. The generated XML
has been validated against the official Accellera IEEE 1685-2014 XSD.

## Register Grouping

`crates/snapsheet/src/register.rs` intentionally groups rows by `REG`. A register
may contain multiple fields, with one spreadsheet row per field, so these rows
must be aggregated into one register model.

The parser validates ambiguous inputs before expansion and aggregation.
Grouping remains intentional.

## Completed

- Replaced the Polars expression pipeline with an explicit row parser. Only
  `ADDR` and `REG` are inherited across merged cells; empty rows are ignored.
- Added validation for conflicting register definitions, duplicate fields,
  overlapping bit ranges, `BIT` / `WIDTH` mismatches, invalid attributes,
  reset values that do not fit their field, 32-bit register width rules,
  byte alignment, out-of-range registers, duplicate address block names, and
  overlapping address blocks.
- Replaced `UInt32` address arithmetic with checked `u64` arithmetic.
- Standardized numeric input: addresses, ranges, reset values, and
  `range(...)` arguments accept decimal or `0x`-prefixed hexadecimal values.
- Added array validation for malformed expressions, invalid argument counts,
  zero steps, empty expansions, overflow, excessive expansion size, generated
  name collisions, and generated address collisions.
- Changed array suffixes to use the actual `n` value. For example,
  `reg{n}, n=range(1, 3)` generates `reg_1` and `reg_2`.
- Changed the optional third `range(...)` argument to the byte offset between
  adjacent expanded registers. The default offset is `0x4`.
- Added parser diagnostics with sheet name, row number, column name, register
  name, and block name where available.
- Added CLI tests for missing input, unknown options, output path handling,
  explicit IP-XACT selection, and failing spreadsheet conversion.
- Added row-level invalid input tests for duplicate registers, duplicate
  fields, overlapping fields, invalid attributes, malformed ranges,
  out-of-range offsets, and trailing empty rows.
- Removed RegVue output support; the CLI now emits IP-XACT only.
- Added generated `.xlsx` fixtures that exercise invalid workbook handling
  through the public loader.
- Added `--validate <xsd>` support for explicitly supplied IEEE 1685-2014
  schemas. The CLI invokes an installed `xmllint` only when validation is
  requested.
- Vendored an unmodified copy of the official Accellera IEEE 1685-2014 XSD for
  repeatable CI validation on Linux.
- Added schema-valid IEEE 1685-2014 component, basic bus-interface,
  register-oriented memory-map, remap-state, memory-remap, nested-bank, subspace-map,
  address-space, segment, local-memory-map, local-bank, register-file,
  alternate-register, field-data, model/ports-core, remap-port-reference, and
  model-view presence/instantiation-reference, and catalog models to `crates/ipxact`,
  with Linux CI validation against the official XSD. Remap-port index/value
  expressions now use the schema's unsigned integer-expression shape.
- Added schema-valid IEEE 1685-2014 system, mirrored-slave, mirrored-master,
  mirrored-system, and monitor bus-interface modes. Bus-interface `bitsInLau`
  now uses the schema's unsigned positive longint-expression shape.
- Added schema-valid IEEE 1685-2014 bus-interface abstraction types and
  logical-to-physical port maps, including view and physical-port keyref
  validation.
- Added schema-valid IEEE 1685-2014 mirrored-interface channels with
  bus-interface keyref validation. Channel and channel bus-interface-reference
  presence expressions now use the schema's unsigned bit-expression shape.
- Added schema-valid IEEE 1685-2014 indirect interfaces with field-ID and
  memory-map keyref validation, plus schema-ordered parameters and
  QName-preserving vendor extensions. Indirect-interface endianness now uses
  the schema enum instead of a stringly typed value, and `bitsInLau` uses the
  schema's unsigned positive longint-expression shape.
- Added schema-valid IEEE 1685-2014 component-level parameters and choices
  with choice-reference keyref validation.
- Added schema-valid IEEE 1685-2014 basic component file sets and
  instantiation file-set references with keyref validation. File and file-set
  reference presence expressions now use the schema's unsigned bit-expression
  shape.
- Added schema-valid IEEE 1685-2014 file and file-set build metadata,
  dependencies, include metadata, logical and exported names, image types,
  file-level and file-set-level QName-preserving vendor extensions, and
  file-set functions with `fileRef` keyref validation.
- Added schema-valid IEEE 1685-2014 file defines and typed file-set function
  arguments using dedicated name-value-pair models. File defines now retain
  schema-ordered vendor extensions.
- Added schema-valid IEEE 1685-2014 component CPUs and address-space
  executable images with address-space and file-set keyref validation.
- Added schema-valid IEEE 1685-2014 executable-image language tools with
  builders, linker flags, and linker command-file configuration.
- Added schema-valid IEEE 1685-2014 component generators with linker-command
  `generatorRef` keyref validation.
- Expanded schema-valid IEEE 1685-2014 component instantiations with strict
  language matching, module parameters, builders, file-set references, and
  parameters. Added design-configuration-instantiation parameters.
- Added schema-valid IEEE 1685-2014 wire-port constraint sets,
  component-instantiation constraint-set references, component whitebox
  elements, and instantiation whitebox HDL paths. The schema-valid structure is
  covered without claiming whitebox keyref enforcement because the vendored
  schema selector and model placement differ.
- Expanded schema-valid wire-port constraint sets with vector slices, typed
  drive/load cell specifications, and timing constraints. Added
  multi-dimensional indices to whitebox HDL path segments.
- Added schema-valid wire type definitions with constrained type names,
  definition paths, and per-view references. Added default-value, clock, and
  single-shot wire driver choices, including clock units.
- Expanded schema-valid bus interfaces with presence expressions,
  required-connection flags, LAU widths, bit steering, endianness, and
  parameters.
- Replaced the placeholder configurable-array model with schema-shaped
  multi-dimensional bounds. Added schema-valid port presence expressions,
  arrays, pointer/reference access selection, and indexed HDL access handles.
  Configurable-array `left`/`right` bounds now use the schema's unsigned
  integer expression shape.
- Expanded schema-valid component and module parameters with vector and
  multi-dimensional array metadata.
- Replaced the text-only 2014 vendor-extension placeholder with a structured
  recursive tree. Namespaced elements, attributes, text, and nested children
  now serialize without raw XML injection and pass official XSD validation on
  representative component, bus-interface, parameter, module-parameter, and
  port paths.
- Split 2014 Serde mappings into prefixed serialization names and local-name
  deserialization aliases. Register-oriented components and catalogs now
  roundtrip through generated 2014 XML. Added an explicit QName-preserving
  parser for isolated vendor-extension containers.
- Added QName-preserving `Component::from_xml_str` and `Catalog::from_xml_str`
  entry points for full-document imports. A component read-modify-write cycle
  retains vendor-extension QNames across root, bus-interface, parameter,
  module-parameter, and port attachment points and revalidates against the
  official XSD.
- Replaced the public 2009 `BusDefinition` re-export with a dedicated 2014 root
  model. Required connection flags, inheritance, capacity expressions, system
  groups, parameters, assertions, QName-preserving vendor extensions, and
  read-modify-write behavior pass official XSD validation.
- Replaced the public 2009 `Design` re-export with a dedicated 2014 root model.
  Component instances with configurable overrides, active and monitor
  interconnections, ad-hoc internal and external port references, range
  selections, parameters, assertions, QName-preserving vendor extensions, and
  read-modify-write behavior pass official XSD validation.
- Replaced the generated `DesignConfiguration` placeholder with a dedicated
  2014 root model. Design references, generator-chain overrides,
  interconnection abstractor chains, broadcast endpoint selection, active-view
  overrides, parameters, assertions, QName-preserving vendor extensions, and
  read-modify-write behavior pass official XSD validation.
- Replaced the public 2009 `AbstractionDefinition` re-export with a dedicated
  2014 root model. Wire and transactional logical ports, qualifiers,
  system/master/slave mode constraints, driver requirements, timing and cell
  constraints, protocols, payloads, parameters, assertions, QName-preserving
  vendor extensions, and read-modify-write behavior pass official XSD
  validation.
- Replaced the generated `GeneratorChain` placeholder with a dedicated 2014
  root model. Ordered group, VLNV, and component-generator selectors, embedded
  generators, chain groups, phase/API/transport metadata, executable URIs,
  choices, parameters, assertions, QName-preserving vendor extensions, and
  read-modify-write behavior pass official XSD validation.
- Replaced the generated `Abstractor` placeholder with a dedicated 2014 root
  model. Exactly-two interface construction, abstraction port maps, restricted
  model views and instantiations, restricted physical ports, generators,
  parameters, assertions, QName-preserving vendor extensions, and
  read-modify-write behavior pass official XSD validation. Added the missing
  generator-level vendor-extension attachment point shared by component and
  abstractor generators.
- Removed the remaining IEEE 1685-2009 re-exports and direct dependencies from
  the 2014 module. The generated compatibility layer now uses 2014 whitebox,
  CPU, and transactional-port protocol types instead of silently importing
  older schema models.
- Expanded dedicated transactional component ports with protocol, payload, and
  type-definition metadata, including custom protocol names, mandatory payload
  extensions, nested service types, typed parameters, view references,
  QName-preserving vendor extensions, and read-modify-write validation.
- Added dedicated component-root clock drivers, reset types, and assertions.
  Independent clock waveforms, field-reset policy keyrefs, QName-preserving
  reset-type vendor extensions, and read-modify-write behavior pass official
  XSD validation.
- Added dedicated vendor-extension attachment points across the
  memory-map/register path. Memory maps, address blocks, banks, local banks,
  nested banks, banked subspace maps, register files, registers, alternate
  registers, fields, and enumerated values now preserve QName-based extensions
  and pass official XSD validation on representative paths.
- Added dedicated register-path parameter attachment points. Address blocks,
  banks, local banks, nested banks, subspace maps, banked subspace maps,
  register files, registers, alternate registers, and fields now serialize
  schema-ordered `parameters` and pass official XSD validation on representative
  paths.
- Added register-path access/presence/array surface. Address blocks and fields
  now carry non-indexed HDL access handles; banks and local banks carry simple
  HDL access handles; register files, registers, and alternate registers carry
  indexed access handles. Address blocks, banks, local banks, subspace maps,
  register files, registers, alternate registers, and fields now serialize
  schema-ordered `isPresent`, while register files and registers also serialize
  schema-ordered `dim` arrays. Memory maps and memory remaps now serialize
  schema-ordered `isPresent`. Address spaces, segments, and local memory maps
  now serialize schema-ordered `isPresent`; address spaces and segments also
  retain QName-based vendor extensions. Address-space, address-block, and
  banked-address-block `range`/`width` block-size fields now use the schema's
  unsigned positive long integer and unsigned integer expression shapes.
  Memory-map and address-space `addressUnitBits` now use the schema's unsigned
  positive long integer expression shape. Address-block, bank, subspace-map,
  and local-bank `baseAddress` now use the schema's unsigned long integer
  expression shape. Master address-space-reference `baseAddress` now uses the
  schema's signed long integer expression shape. Segment and register-file
  `addressOffset`/`range`, plus register `addressOffset`/`size`, now use their
  schema-specific unsigned expression shapes. Field `bitOffset`, `bitWidth`,
  and `reserved` now use the schema's unsigned integer, unsigned positive
  integer, and unsigned bit-expression shapes. Enumerated field values, field
  reset values and masks, and field write-constraint bounds now use the
  schema's unsigned bit-vector expression shape. Register files now include
  `typeIdentifier`.
- Normalized schema-specific port width/count expressions. Abstraction
  wire-mode `width`, abstraction transactional-mode `busWidth`, load
  constraint `count`, component transactional-port `busWidth`, and component
  transactional connection bounds now use the corresponding unsigned integer
  expression shapes. Component port vectors, driver/constraint-set ranges,
  design `partSelect` ranges, and HDL path segment indices now use the
  schema's unsigned integer expression shape as well. Component wire driver
  default, clock-pulse, and single-shot values, plus abstraction wire default
  values, now use the schema's unsigned bit-vector expression shape.
  Bus-interface port-map `logicalTieOff` values now use the schema's unsigned
  positive integer expression shape, and bus-interface port-map `isPresent`
  attachment points now serialize in schema order and roundtrip through XSD
  coverage. Assertion `assert` expressions now use the schema's unsigned
  bit-expression shape. Memory-map and local-memory-map
  bank `bankAlignment` attributes now use the schema enum instead of raw
  strings. Field `modifiedWriteValue`, `readAction`, and
  `testable@testConstraint` now use schema enums instead of raw strings.
  Memory-map/register/field `access` values now use the schema `accessType`
  enum instead of raw strings. Memory-map bank/address-block `usage` values
  and enumerated-value `usage` attributes now use dedicated schema enums
  instead of raw strings. Memory-map `shared` values now use the schema
  `sharedType` enum instead of raw strings. Field `writeValueConstraint` now
  uses a schema-choice model instead of independent optional child fields.
  Indirect-interface targets now use a schema-choice model for either a
  memory-map reference or one-or-more transparent bridges. Slave bus-interface
  targets now use the optional schema choice for either a memory-map reference
  or one-or-more transparent bridges. Component bus interfaces, abstractor bus
  interfaces, bus-interface `bitSteering`, design ad-hoc tied values,
  component parameters, module parameters, parameter values, configurable
  element values, and files now retain QName-preserving `any.att` extension
  attributes, while file defines retain schema-ordered vendor extensions. These
  paths pass official XSD validation.
- Added executable-image build metadata extension points. Executable images,
  language file builders, and linker command files now retain QName-based
  vendor extensions and pass official XSD validation on the address-space
  executable-image path.
- Added file-set extension points. Files and file sets now retain QName-based
  vendor extensions and pass official XSD validation and roundtrip coverage on
  the file-set build metadata path.
- Added model instantiation extension points. Design instantiations and
  design-configuration instantiations now retain QName-based vendor extensions
  and pass official XSD validation and roundtrip coverage on the model-view
  instantiation path.
- Added model-view presence expressions. Component model views now serialize
  schema-ordered `isPresent` and pass official XSD validation and roundtrip
  coverage on the model-view instantiation path.
- Added official-XSD validation and roundtrip coverage for all eight IEEE
  1685-2014 root documents listed by the vendored `index.xsd`: component,
  catalog, bus definition, abstraction definition, abstractor, design,
  design configuration, and generator chain.

## P1: Remaining Validation

- Add more `.xlsx` fixtures as new workbook-level failure modes are discovered.

## P2: Architecture

- Keep `crates/model` focused on snapsheet conversion while using
  `ip_xact::v2014` as the emitted IP-XACT model. The previous lightweight
  IP-XACT and RegVue model files are retained for reference but are no longer
  included from the model crate root.
- If C header, UVM RAL, SystemVerilog RTL, or HTML exports return, add them as
  explicit CLI formats backed by dedicated output crates instead of restoring
  a generic `core` facade.
- Decide whether XML schema validation should remain an opt-in CLI feature or
  become the default for release verification workflows.

## Verification Notes

- `cargo check --workspace --offline` passes.
- `cargo clippy --workspace --all-targets --all-features --offline -- -D warnings`
  passes.
- `cargo test --workspace --offline` passes.
- `cargo build --release --locked --bin irgen --offline` passes.
- `target/release/irgen example.xlsx -o /tmp/irgen-example.xml` generates XML.
- `target/release/irgen example.xlsx --validate
  crates/model/tests/fixtures/ipxact-1685-2014/index.xsd` generates and
  validates XML when `xmllint` is installed.
- The generated sample validates against the official Accellera IEEE 1685-2014
  XSD.
