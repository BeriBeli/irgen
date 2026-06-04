# IP-XACT Crate Compliance Audit

## Conclusion

`crates/ipxact` is not a schema-compliant general-purpose IP-XACT library yet.
The IEEE 1685-2014 component, register-oriented memory-map, remap-state,
memory-remap, nested-bank, subspace-map, address-space, local-memory-map,
bus-interface-mode, register-file, alternate-register, field-data, model/ports
core, model views and instantiations, and catalog paths now pass official XSD
validation. Bus-interface abstraction port maps also pass official view and
physical-port keyref validation. Mirrored-interface channels pass interface
keyref validation and retain channel/channel-reference metadata. Indirect
interfaces pass field-ID and memory-map keyref
validation. Component-level configurable parameters and choices pass
choice-reference keyref validation. Indirect-interface parameters, vendor
extensions, schema-enum endianness, and schema-shaped `bitsInLau` now pass
along with field-ID and memory-map keyref validation.
Component file sets, build metadata, functions, file/file-set vendor
extensions, and instantiation file-set references pass file-set and
file-reference keyref validation. Component CPUs and address-space executable
images pass address-space, file-set, and component-generator keyref validation,
with CPU presence, parameters, vendor extensions, generator-reference `xml:id`,
and component-generator group `xml:id` covered by roundtrip tests. Wire-port
constraint sets, drive/load/timing constraint details,
component-instantiation constraint references, indexed whitebox HDL paths,
wire type definitions, whitebox-element presence/driveability/parameters/vendor
extensions, and default/clock/single-shot wire drivers also pass official XSD
structural validation. The dedicated 2014 bus-definition root now passes official XSD
validation and roundtrip coverage, including required connectivity flags,
inheritance, capacity expressions, system groups, parameters, assertions, and
vendor extensions. The dedicated 2014 design root now passes official XSD
validation and roundtrip coverage for component instances with configurable
overrides, normal and monitor interconnections, ad-hoc port references, range
selections, parameters, assertions, and vendor extensions. The dedicated 2014
design-configuration root now passes official XSD validation and roundtrip
coverage for design references, generator-chain overrides, interconnection
abstractor chains, broadcast endpoints, active-view overrides, parameters,
assertions, and vendor extensions. The dedicated 2014 abstraction-definition
root now passes official XSD validation and roundtrip coverage for wire and
transactional logical ports, qualifiers, mode constraints, driver requirements,
timing and cell constraints, protocols, payloads, parameters, assertions, and
vendor extensions. The dedicated 2014 generator-chain root now passes official
XSD validation and roundtrip coverage for ordered selectors, embedded
generators, chain groups, execution metadata, choices, parameters, assertions,
and vendor extensions. The dedicated 2014 abstractor root now passes official
XSD validation and roundtrip coverage for exactly two interfaces, abstraction
port maps, restricted model views, component instantiations, restricted physical
ports, generators, parameters, assertions, and vendor extensions. Common
bus-interface optional data, including presence, connection, LAU width, bit
steering, endianness, parameters, and port-map presence expressions, passes as
well.
Port-level presence expressions, multi-dimensional arrays, and HDL
access handles also pass. Component and module parameter vectors and arrays
pass as well. Transactional component-port protocols, payload metadata, type
definitions, nested service types, typed parameters, and view references pass
as well. Component-root independent clock drivers, clock-driver `xml:id`, clock
time-expression bounds/units/extension attributes, reset types with
`xml:id`/display metadata, reset-policy keyrefs, and assertions with
`xml:id`/display metadata and assert-expression extension attributes pass as
well. Structured vendor-extension trees with arbitrary namespaced elements,
attributes, text, and nested children pass on
representative component, bus-interface, parameter, module-parameter, port,
memory-map, bank, local-bank, register-file, register, alternate-register,
field, and enumerated-value paths. Register-path parameter attachment points
also pass on address blocks, banks, local banks, subspace maps, register files,
registers, alternate registers, and fields. Register-path access handles,
presence expressions, register arrays, and type identifiers now pass on
representative address-block, bank, local-bank, subspace-map, register-file,
register, alternate-register, and field paths. Register-path
`volatile` values now pass on representative address-block, bank, nested-bank,
banked-address-block, register, alternate-register, and field paths. Memory-map
and memory-remap presence expressions pass as well. Register-path `access`
attachment points and memory-block `usage` values now use 2014 schema enums
instead of raw strings. Memory-map `shared` values and enumerated-value
`usage` attributes also use their dedicated schema enums. Field
`writeValueConstraint` now preserves the schema choice among `writeAsRead`,
`useEnumeratedValues`, and `minimum`/`maximum` range branches.
Indirect-interface targets now preserve the schema choice between a memory-map
reference and one-or-more transparent bridges. Slave bus-interface targets now
preserve the optional schema choice with the same alternatives, and retain
schema-ordered `fileSetRefGroup` entries for slave-associated file sets.
Executable-image `languageTools` now preserves the linker branch choice between
`linkerFlags` with optional `linkerCommandFile` and a required
`linkerCommandFile`, while rejecting linker-only configurations.
Address-space, segment, and local-memory-map presence expressions pass as well,
with QName-preserving vendor extensions on address spaces and segments. Component
bus interfaces, abstractor bus interfaces, bus-interface `bitSteering`, design
ad-hoc tied values, component parameters, module parameters, parameter values,
configurable element values, choice enumerations, unsigned bit expressions,
unsigned bit-vector expressions, unsigned integer expressions, unsigned positive
integer expressions, signed long integer expressions, unsigned long integer
expressions, unsigned positive long integer expressions, real expressions, clock
time expressions, string expressions, URI string expressions, file build flags,
and files now retain QName-preserving `any.att` extension attributes. File
defines and file-set function arguments now retain schema-ordered vendor
extensions.
Executable-image build metadata now preserves QName-based
vendor extensions on executable images, language file builders, and linker
command files. Design instantiations and design-configuration instantiations
now preserve QName-based vendor extensions and roundtrip through the model-view
instantiation path, with component instantiations covered as well. Component
model views now preserve schema-ordered presence expressions on the same path.

The focused `crates/model` conversion layer now uses `ip_xact::v2014` as the
emitted IP-XACT 2014 model for the current snapsheet-to-IP-XACT workflow.

## Verified Behavior

- `cargo test -p ip-xact --offline` passes: 43 unit tests and 49 integration
  tests.
- The 2009 integration coverage exercises component parsing, construction,
  serialization, and self-roundtrip behavior. The 2014 integration coverage
  exercises forty-five serialization, deserialization, roundtrip, and
  official-XSD validation cases.
- `v2014::Component` and `v2014::Catalog` now emit the IEEE 1685-2014 namespace
  and schema location.
- `v2014::Component` samples containing memory maps, address blocks, nested
  banks, subspace maps, master and slave bus interfaces, memory-map
  references, transparent bridges, memory remaps, register files, registers,
  alternate registers, fields, resets, enumerations, write constraints,
  remap states, address spaces, segments, master address-space references,
  local memory maps, local banks, wire ports, port vectors, transactional-port
  core fields, protocols, payloads, type definitions, service type definitions,
  type parameters, remap-port references, model views, environment identifiers,
  component/design/design-configuration instantiation references, system,
  mirrored, and monitor bus-interface modes, abstraction types, and logical
  to physical port maps, mirrored-interface channels with bit-expression
  presence controls and channel/channel-reference metadata, indirect interfaces
  with parameters, vendor extensions,
  schema-enum endianness, and schema-shaped `bitsInLau`, root-level component
  descriptions, and component-level parameters and choices, file sets, file and
  file-set-reference bit-expression presence controls, file build metadata,
  file/file-set vendor extensions, file-set functions, and instantiation file-set references, CPUs, and
  address-space executable images, wire-port constraint sets with vector, drive, load, and timing
  details, instantiation constraint references, indexed whitebox HDL paths,
  wire type definitions, and default, clock, and single-shot wire drivers,
  bus-interface presence, required-connection flags, LAU widths, bit steering,
  endianness, and parameters, port presence expressions, multi-dimensional
  arrays, and HDL access handles, component and module parameter vectors and
  arrays, and structured vendor-extension trees on representative component,
  bus-interface, parameter, module-parameter, port, memory-map, bank,
  local-bank/register-file/register/field, and enumerated-value paths,
  plus register-path parameter attachment points,
  simple-content attributes, `xml:id`, and numeric expression attributes pass
  official XSD validation.
- A `v2014::Catalog` containing an `ipxactFile`, attribute-based VLNV, and URI
  expression passes official XSD validation.
- The CLI test path converts the complex `example.xlsx` workbook with
  `snapsheet.toml`, validates the emitted IP-XACT against the official 2014
  XSD when `xmllint` is installed, and asserts that repeated `REG` rows are
  aggregated into one register containing multiple fields.
- Linux CI runs the `v2014_test` schema checks after installing `xmllint`.

## Completed: 2014 Component Slice

- Added dedicated 2014 `Component`, basic bus-interface, `MemoryMaps`,
  `MemoryMap`, remap-state, memory-remap, `AddressBlock`, nested-bank, subspace-map,
  `RegisterFile`, `Register`, alternate-register, `Field`, field-data, reset,
  and expression types. Remap-port `portIndex` and `value` now use the schema's
  unsigned integer-expression shape. Remap states now retain display metadata,
  and remap-port value extension attributes roundtrip through official XSD
  coverage. Memory remaps now retain `xml:id` and display metadata.
- Added dedicated 2014 `AddressSpaces`, `AddressSpace`, segment,
  master-address-space-reference, local-memory-map, and recursive local-bank
  types.
- Added dedicated 2014 `Model`, `Ports`, wire-port, vector,
  transactional-port-core, and remap-port types. Validated the
  `remapPort@portRef` keyref against a physical component port.
- Added dedicated model views and component/design/design-configuration
  instantiations. Validated model-view `isPresent` plus all three
  view-reference keyrefs.
- Added dedicated system, mirrored-slave, mirrored-master, mirrored-system,
  and monitor bus-interface modes, including mirrored-slave remap addresses.
  Mirrored-slave base-address remaps now validate and roundtrip `state`,
  `xml:id`, value, and range through official 2014 XSD coverage.
- Added dedicated abstraction types and logical-to-physical port maps.
  Validated abstraction view references and physical-port keyrefs.
- Added dedicated mirrored-interface channels. Validated channel interface
  references against component bus interfaces, plus schema-shaped unsigned
  bit-expression presence controls on channels and channel bus-interface
  references, channel `xml:id`/display metadata, and channel
  bus-interface-reference `xml:id` values. Validated schema-shaped unsigned
  positive longint-expression `bitsInLau` on bus interfaces.
- Added dedicated indirect interfaces. Validated address/data `fieldID`
  references, text-based memory-map references, schema-enum endianness,
  schema-shaped unsigned positive longint-expression `bitsInLau`,
  schema-ordered parameters, and QName-preserving vendor extensions.
- Added dedicated component-level parameters and choices. Validated parameter
  attributes and choice-reference keyrefs.
- Added dedicated basic component file sets, files, standard file types, and
  instantiation file-set references. Validated file-set-reference keyrefs and
  schema-shaped unsigned bit-expression presence controls on files and
  file-set references.
- Added dedicated file and file-set build metadata, dependencies, include
  metadata, logical and exported names, image types, and file-set functions.
  Validated QName-preserving vendor extensions on files and file sets, plus
  function `fileRef` keyrefs against `file@fileId`.
- Added dedicated 2014 name-value pairs for file defines and typed file-set
  function arguments, including schema-ordered vendor extensions for both
  `nameValuePairType` paths.
- Added dedicated component CPUs and address-space executable images,
  including CPU address-space references, CPU presence controls, CPU parameters,
  CPU vendor extensions, image parameters, and image file-set-reference groups.
- Added dedicated executable-image language tools, including file builders,
  linker flags, and linker command-file configuration.
- Added dedicated component generators, including scope, phase, parameters,
  API selection, transport methods, and groups. Validated linker-command-file
  `generatorRef` keyrefs.
- Expanded component instantiations with strict language matching, typed
  module parameters, default file builders, file-set references, and
  parameters. Component instantiation default builders, file-set references, and
  vendor extensions now have roundtrip and official XSD coverage. Added
  design-configuration-instantiation parameters.
- Added wire-port constraint sets, component-instantiation constraint-set
  references, component whitebox elements, and instantiation whitebox HDL
  paths. Their structure passes the official XSD. The vendored schema's
  whitebox keyref selector targets view-level references while its model
  definition places them inside component instantiations, so whitebox keyref
  enforcement is not claimed. Whitebox-element presence, driveability,
  parameters, and QName-preserving vendor extensions now roundtrip through the
  same official-XSD path.
- Expanded wire-port constraint sets with vector slices, typed drive/load cell
  specifications, timing constraints, and typed cell function, class,
  strength, edge, and delay enums. Added multi-dimensional indices to whitebox
  HDL path segments.
- Added dedicated wire type definitions with constrained type names,
  definition paths, per-view references, and `xml:id` roundtrip coverage for
  wire type definitions, type-definition paths, and view references. Added typed
  wire drivers covering default values, clock waveforms with units, and
  single-shot waveforms.
- Expanded dedicated bus interfaces with typed presence expressions,
  required-connection flags, LAU widths, bit-steering expressions, endianness,
  and parameters.
- Replaced the placeholder configurable-array model with schema-shaped
  multi-dimensional `array(left, right)` bounds. Expanded component ports with
  presence expressions, arrays, pointer/reference access selection, and HDL
  access handles including per-view refs, indices, slices, and path segments.
- Expanded component and module parameters with reusable vector and
  multi-dimensional array metadata.
- Expanded register-path parameter attachment points on address blocks, banks,
  local banks, nested banks, subspace maps, banked subspace maps, register
  files, registers, alternate registers, and fields.
- Expanded register-path access/presence/array structures. Added dedicated
  simple, indexed, and non-indexed HDL access-handle containers, bank and local
  bank `isPresent`, subspace-map `isPresent`, register-file/register/alternate
  register/field `isPresent`, register-file/register `dim`, and register-file
  `typeIdentifier` coverage. Added memory-map and memory-remap `isPresent`
  coverage. Added address-space, segment, and local-memory-map `isPresent`
  coverage plus address-space and segment vendor-extension attachment points.
  Added executable-image, language-file-builder, and linker-command-file
  vendor-extension attachment points. Added design-instantiation and
  design-configuration-instantiation vendor-extension attachment points.
- Replaced the text-only vendor-extension placeholder with a structured,
  recursive XML tree model. Arbitrary namespaced elements, attributes, text,
  and nested children now serialize without raw XML injection. Added extension
  attachment points for the component root, bus interfaces, CPUs, whitebox
  elements, component parameters, module parameters, ports, memory maps,
  address blocks, banks, local banks, nested banks, banked subspace maps,
  register files, registers, alternate registers, fields, and enumerated values.
- Split the 2014 Serde element mapping into prefixed serialization names and
  local-name deserialization aliases. Modeled component and catalog roots now
  deserialize their generated XML; a register-oriented component roundtrip and
  a catalog roundtrip are covered.
- Added `VendorExtensions::from_xml_str` for isolated extension containers that
  need QName-preserving reads. It retains qualified element and attribute names
  while decoding text, CDATA, predefined entities, and character references.
- Added QName-preserving `Component::from_xml_str` and `Catalog::from_xml_str`
  import entry points. They protect arbitrary qualified names only inside
  `vendorExtensions` before derived deserialization and decode them recursively
  afterward. A full component read-modify-write cycle retains extension QNames
  at the root, bus-interface, component-parameter, module-parameter, and port
  levels and revalidates against the official XSD.
- Replaced the public 2009 `BusDefinition` re-export with a dedicated 2014 root
  model and QName-preserving import entry point. Required direct-connection and
  addressable flags, optional broadcast, inheritance, capacity expressions,
  system-group names, parameters, assertions, `xml:id`, and vendor extensions
  validate and roundtrip. Renamed the generated placeholder to
  `LegacyBusDefinition` so it cannot silently win the public API.
- Replaced the public 2009 `Design` re-export with a dedicated 2014 root model
  and QName-preserving import entry point. Added typed component instances with
  configurable element overrides, normal and monitor interconnections,
  interface exclusions, ad-hoc internal and external port references, tied
  values, and range selections. The generated placeholder is now
  `LegacyDesign`.
- Replaced the generated `DesignConfiguration` placeholder with a dedicated
  2014 root model and QName-preserving import entry point. Added typed design
  references, generator-chain overrides, interconnection abstractor chains,
  broadcast endpoint references, and active-view overrides. The generated
  placeholder is now `LegacyDesignConfiguration`.
- Replaced the public 2009 `AbstractionDefinition` re-export with a dedicated
  2014 root model and QName-preserving import entry point. Added typed wire and
  transactional logical ports, qualifiers, system/master/slave mode
  constraints, driver requirements, timing and cell constraints, protocols,
  and payloads. The generated placeholder is now
  `LegacyAbstractionDefinition`.
- Replaced the generated `GeneratorChain` placeholder with a dedicated 2014
  root model and QName-preserving import entry point. Added ordered group,
  configurable VLNV, and component-generator selectors, embedded generators,
  chain groups, phase/API/transport metadata, executable URIs, choices,
  parameters, and assertions. The generated placeholder is now
  `LegacyGeneratorChain`.
- Replaced the generated `Abstractor` placeholder with a dedicated 2014 root
  model and QName-preserving import entry point. Added exactly-two interface
  construction, abstraction port maps, restricted model views and component
  instantiations, restricted physical ports, generators, parameters, and
  assertions. The generated placeholder and related helper placeholders now use
  `Legacy*` names. Added the missing generator-level vendor-extension attachment
  point shared by component and abstractor generators.
- Replaced the incompatible 2009 re-exports for the 2014 register path.
- Fixed memory-map choice serialization: address blocks, banks, and subspace
  maps retain their schema element names.
- Added required register `addressOffset` and `size` elements.
- Encoded `xml:id`, VLNV fields, numeric expression bounds, and reset type
  references as XML attributes.
- Added official XSD validation tests for representative 2014 register
  component and catalog documents.
- Removed all remaining 2009 public re-exports and direct source dependencies
  from the 2014 module. The generated compatibility layer now uses dedicated
  2014 whitebox, CPU, and port-protocol structures.
- Expanded dedicated transactional component ports with protocol, payload, and
  type-definition metadata, including custom protocol names, mandatory payload
  extensions, nested service types, typed parameters, type-parameter presence
  controls and extension points, service/type-definition `xml:id` attributes,
  view references, and QName-preserving vendor extensions.
- Added dedicated component-root independent clock drivers, reset types, and
  assertions. Validated full clock waveforms, clock-driver `xml:id`, clock
  time-expression bounds/units/extension attributes, field-reset policy keyrefs,
  reset-type `xml:id`/display metadata, QName-preserving reset-type extensions,
  assertion `xml:id`/display metadata, assert-expression extension attributes,
  and read-modify-write behavior.
- Added dedicated memory-map/register vendor-extension attachment points.
  Validated QName-preserving extensions across memory maps, address blocks,
  banks, local banks, nested banks, banked subspace maps, register files,
  registers, alternate registers, fields, and enumerated values.
- Added dedicated memory-map/register parameter attachment points. Validated
  schema-ordered parameters across address blocks, banks, local banks, nested
  banks, subspace maps, banked subspace maps, register files, registers,
  alternate registers, and fields.
- Added dedicated memory-map/register access, presence, and array attachment
  points. Validated schema-ordered simple access handles on banks/local banks,
  non-indexed access handles on address blocks/fields, indexed access handles
  on register files/registers/alternate registers, `isPresent` across the
  register path and memory-remap path, register-file/register `dim`,
  `typeIdentifier` across representative address-block, register-file,
  register, alternate-register, and field paths, and `volatile` values across
  representative memory-block, register, alternate-register, and field paths.
- Added dedicated address-space/local-memory-map presence and extension
  attachment points. Validated schema-ordered `isPresent` on address spaces,
  segments, and local memory maps, plus QName-preserving vendor extensions on
  address spaces and segments.
- Added dedicated executable-image build metadata extension points. Validated
  QName-preserving vendor extensions on executable images, language file
  builders, and linker command files. Linker command-file generator references
  and component-generator groups now retain `xml:id` values on the same
  validated component-generator keyref path.
- Added dedicated file-set extension points. Validated QName-preserving vendor
  extensions on files and file sets, plus schema-ordered vendor extensions on
  typed file-set function arguments.
- Added dedicated model-view presence expressions. Validated schema-ordered
  `isPresent` on component model views.
- Added component-instantiation extension-point coverage. Validated
  schema-ordered default file builders, file-set references, and QName-preserving
  vendor extensions on component instantiations.
- Added component-root CPU and whitebox optional-structure coverage. Validated
  CPU presence controls, CPU vendor extensions, and whitebox-element presence,
  driveability, parameters, and QName-preserving vendor extensions.
- Added dedicated indirect-interface parameter and extension attachment
  points. Validated schema-ordered parameters and QName-preserving vendor
  extensions on indirect interfaces.
- Normalized dedicated indirect-interface `endianness` to the schema enum
  instead of a stringly typed value.
- Normalized dedicated `bitsInLau` attachment points on bus interfaces and
  indirect interfaces to the schema's unsigned positive longint-expression
  shape instead of the broader numeric-expression shape.
- Normalized dedicated address-space, address-block, and banked-address-block
  `range`/`width` block-size fields to the schema's unsigned positive long
  integer-expression and unsigned integer-expression shapes instead of the
  broader numeric-expression shape.
- Normalized dedicated memory-map and address-space `addressUnitBits` fields
  to the schema's unsigned positive long integer-expression shape instead of
  the broader numeric-expression shape.
- Normalized dedicated address-block, bank, subspace-map, and local-bank
  `baseAddress` fields to the schema's unsigned long integer-expression shape
  instead of the broader numeric-expression shape.
- Normalized dedicated master address-space-reference `baseAddress` fields to
  the schema's signed long integer-expression shape instead of the broader
  numeric-expression shape.
- Normalized dedicated segment and register-file `addressOffset`/`range`
  fields to the schema's unsigned long integer-expression and unsigned positive
  long integer-expression shapes instead of the broader numeric-expression
  shape.
- Normalized dedicated register `addressOffset` and `size` attachment points
  to the schema's unsigned long integer-expression and unsigned positive
  integer-expression shapes instead of the broader numeric-expression shape.
- Normalized dedicated field `bitOffset` and `bitWidth` attachment points to
  the schema's unsigned integer-expression and unsigned positive
  integer-expression shapes instead of the broader numeric-expression shape.
- Normalized dedicated enumerated field values, field reset values/masks, and
  write-constraint bounds to the schema's unsigned bit-vector expression shape
  instead of the broader numeric-expression shape. Field resets now retain
  `xml:id` plus reset value/mask extension attributes.
- Normalized dedicated abstraction wire-mode `width`, abstraction
  transactional-mode `busWidth`, load-constraint `count`, component
  transactional-port `busWidth`, and component transactional connection bounds
  to the corresponding schema unsigned integer-expression shapes instead of
  the broader numeric-expression shape.
- Normalized dedicated component port vectors, driver ranges,
  constraint-set ranges, design `partSelect` ranges, and HDL path segment
  indices to the schema's unsigned integer-expression shape instead of the
  broader numeric-expression shape.
- Normalized shared configurable-array `left`/`right` bounds to the schema's
  unsigned integer-expression shape instead of the broader numeric-expression
  shape.
- Normalized dedicated component wire driver values and abstraction wire
  default values to the schema's unsigned bit-vector expression shape instead
  of the broader numeric-expression shape.
- Normalized dedicated bus-interface port-map `logicalTieOff` values to the
  schema's unsigned positive integer-expression shape instead of the broader
  numeric-expression shape.
- Added schema-ordered bus-interface port-map `isPresent` attachment points
  with roundtrip and official XSD coverage.
- Normalized assertion `assert` values to the schema's unsigned
  bit-expression shape instead of a raw string.
- Normalized memory-map and local-memory-map bank `bankAlignment` attributes
  to the schema enum instead of raw strings.
- Normalized field `modifiedWriteValue`, `readAction`, and
  `testable@testConstraint` values to schema enums instead of raw strings.
- Normalized memory-map/register/field `access` values to the schema
  `accessType` enum instead of raw strings.
- Normalized memory-block `usage` values and enumerated-value `usage`
  attributes to dedicated schema enums instead of raw strings.
- Normalized memory-map `shared` values to the schema `sharedType` enum
  instead of raw strings.
- Normalized field `writeValueConstraint` to a schema choice, preventing
  independent optional child fields from representing invalid multi-branch
  constraints.
- Normalized indirect-interface targets to a schema choice, preventing a
  memory-map reference and transparent bridges from being represented together.
- Normalized slave bus-interface targets to an optional schema choice,
  preventing a memory-map reference and transparent bridges from being
  represented together.
- Added schema-ordered slave bus-interface `fileSetRefGroup` support with
  roundtrip and official XSD coverage.
- Normalized executable-image `languageTools` linker metadata to the schema
  choice between `linkerFlags` and `linkerCommandFile`, with roundtrip, official
  XSD coverage, and a negative linker-only deserialization test.
- Added QName-preserving `any.att` extension attributes for component bus
  interfaces, abstractor bus interfaces, bus-interface `bitSteering`, design
  ad-hoc tied values, component parameters, module parameters, parameter values,
  configurable element values, choice enumerations, unsigned bit expressions,
  unsigned bit-vector expressions, unsigned integer expressions including
  remap-port values, unsigned positive integer expressions, signed long integer
  expressions, unsigned long
  integer expressions, unsigned positive long integer expressions, real
  expressions, clock time expressions, string expressions, URI string
  expressions, file build flags, and files with roundtrip and official XSD
  coverage.
- Added schema-ordered vendor extensions for file `define` and file-set
  function `argument` `nameValuePairType` values with roundtrip and official
  XSD coverage.
- Normalized dedicated field `reserved` attachment points to the schema's
  unsigned bit-expression shape instead of the broader numeric-expression
  shape.
- Normalized dedicated `isPresent` attachment points for files, file-set
  references, mirrored-interface channels, and channel bus-interface
  references to the schema's unsigned bit-expression shape instead of the
  broader numeric-expression shape.
- Normalized dedicated remap-port `portIndex` and `value` attachment points to
  the schema's unsigned integer-expression shape instead of the broader
  numeric-expression shape.
- Added official-XSD validation and roundtrip coverage for all eight 2014 root
  documents listed by the vendored `index.xsd`.

## P0: Remaining Blockers

- Complete the dedicated IEEE 1685-2014 component surface: audit remaining
  vendor-extension attachment points, extension attributes, and omitted
  optional component structures.
- Add namespace-aware serializers and official XSD validation for 2009 and
  2022 before claiming multi-version compliance.

## P1: Structural Cleanup

- Replace placeholder structs and remaining stringly typed nested structures
  with typed schema models.
- Review generated serde names. The 2022 access restriction model currently
  contains a non-schema element rename for `lower_bound`.
- Add 2022 official XSD fixtures and conformance tests before claiming 2022
  support.
- Add official 2009 SPIRIT schema fixtures and validate 2009 output rather than
  relying only on serde self-roundtrips.

## Schema Fixture Note

The vendored IEEE 1685-2014 fixture is sufficient for the current CLI
component validation path. Before treating it as a complete mirror, refresh it
from the current Accellera schema bundle: the official online 2014 directory
also lists `configurable.xsd`, which is not present in the local fixture.

## External References

- Accellera IEEE 1685-2014 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2014/>
- Accellera IEEE 1685-2022 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2022/>
- Accellera IP-XACT downloads:
  <https://www.accellera.org/downloads/standards/ip-xact>
