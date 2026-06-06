# irgen Roadmap

## Direction

The current product goal is a CLI-first register spreadsheet converter that
emits register-oriented IEEE 1685 IP-XACT, RALF, SystemRDL, and static register
documentation. Future generated artifacts should stay CLI-first as well:
additional documentation formats, verification-facing HDL backdoor metadata,
software-facing headers, and hardware-facing register files should be added as
explicit output formats or narrow crates rather than by introducing UI runtime
dependencies.

IEEE 1685-2014 compliance is the active milestone. IEEE 1685-2009 and IEEE
1685-2022 are available for the current snapsheet register-table subset, but
their broader schema coverage is still incomplete.

## Crate Boundaries

- `crates/cli`: command-line entry point.
- `crates/snapsheet`: spreadsheet loading, row validation, array expansion, and
  register aggregation.
- `crates/model`: lightweight register IR plus conversion into versioned
  `ip_xact` component models.
- `crates/ipxact`: broader IP-XACT schema model, currently focused on 2014
  compliance with narrower 2009 and 2022 register-oriented emitters.
- `crates/docs`: static register documentation view and HTML site generator.
- `crates/ralf`: RALF model and serializer.
- `crates/systemrdl`: SystemRDL model and serializer.

Active dependency direction:

```text
cli -> snapsheet -> model -> ipxact
cli -> docs -> model
cli -> ralf
cli -> systemrdl
ralf/systemrdl -> model
```

Do not restore a generic `core` facade unless there is a concrete shared API
that needs it.

## Register Grouping

`crates/snapsheet/src/register.rs` intentionally groups rows by `REG`.

A single register may contain multiple fields, with one spreadsheet row per
field, so rows with the same register identity must aggregate into one register
model. This behavior is required for correct IP-XACT field emission.

## Current Capability

- Converts `.xlsx` input into IP-XACT XML, RALF, and SystemRDL.
- Supports `--format ipxact|ralf|systemrdl|html|all`.
- Supports `--ipxact-version 2009|2014|2022`, with 2014 as the default for
  IP-XACT output.
- Supports `--snapsheet-spec <snapsheet.toml>`.
- Supports opt-in `--validate <schema.xsd>` for IP-XACT XML via `xmllint`.
- Generates a static HTML register documentation site with shared assets, block
  index pages, register detail pages, deterministic anchors, and search data.
- Validates common workbook failures before conversion, including duplicate
  fields, overlapping bit ranges, malformed arrays, invalid attributes,
  address collisions, out-of-range registers, and reset values that do not fit.
- Emits register arrays as IP-XACT `registerFile` arrays.
- Carries field-level HDL backdoor paths from the optional `PATH` column through
  IP-XACT, RALF, and SystemRDL output. Reserved fields and explicit `-` values
  suppress field HDL paths.
- Uses checked `u64` arithmetic for addresses and array expansion.

## Documentation Map

- `docs/snapsheet-format.md`: workbook layout, TOML configuration, array
  rules, and parser validation behavior.
- `docs/ralf-generation.md`: RALF model coverage, snapsheet mapping, and
  limitations.
- `docs/systemrdl-generation.md`: SystemRDL model coverage, snapsheet mapping,
  and limitations.
- `docs/ipxact-2014-compliance.md`: IEEE 1685-2014 compliance status and
  verification evidence.

## P0: 2014 IP-XACT Compliance

Closed for the current component milestone.

Current state:

- All eight IEEE 1685-2014 root documents listed by the vendored `index.xsd`
  have official-XSD validation coverage.
- The `componentType` top-level sequence has no known omitted optional
  structure.
- Included-schema nested attachment points have representative official-XSD
  coverage.

## P1: Stabilization And Cleanup

Status: Active.

- Replace remaining placeholder or stringly typed 2014 nested structures that
  are still on active paths.
- Continue auditing overbroad expression wrapper types where the schema
  disallows extension attributes.
- Add new `.xlsx` fixtures as parser failure modes are discovered.
- Decide whether XML schema validation should remain opt-in CLI behavior or
  become part of release verification only.

## P2: Register Documentation Outputs

Status: Partially shipped.

Implemented:

- `crates/docs` provides a documentation-oriented register view derived from the
  current model: blocks, register-file arrays, registers, fields, offsets, bit
  ranges, reset values, access attributes, and descriptions.
- `--format html` generates a static HTML documentation site with no active web
  server dependency.
- `--format all` includes the HTML documentation site under `html/`.
- HTML output uses deterministic anchors for blocks, registers, and fields.
- HTML and future text documentation have a shared model boundary in
  `crates/docs::view`, so address, field, reset, and access data should not
  diverge across formats.

Remaining:

- Add Markdown or another plain-text register documentation format suitable for
  code review and release artifacts.

Acceptance gates:

- Documentation output must preserve register ordering and array expansion
  semantics from the snapsheet parser.
- HTML and text documentation must share one model so they cannot diverge on
  addresses, bit ranges, reset values, or access modes.
- Generated docs should include deterministic anchors for blocks, registers,
  and fields.

## P3: HDL Backdoor Path Support

Status: Closed for the current field-level scope.

Implemented:

- `irgen_model::base::Field` carries an optional `hdl_path`.
- Snapsheet parsing supports an optional `PATH` column. Blank path cells default
  non-reserved fields to the field name, `-` disables the path, and reserved
  fields do not emit HDL paths.
- IP-XACT 2014 and 2022 emit field and block HDL paths through standard
  `accessHandles`; IP-XACT 2009 does not emit HDL paths because it lacks the
  same standard register-model access-handle structure.
- RALF and SystemRDL outputs preserve field HDL paths; SystemRDL uses
  `hdl_path_slice` for fields.
- Tests cover field-level paths, disabled paths, reserved-field suppression,
  register-file array naming behavior, and IP-XACT version-specific emission.
- Keep backdoor paths as verification metadata only; they must not imply RTL
  implementation, bus behavior, or generated software accessors.

Acceptance gates:

- Backdoor path output must be deterministic and must round-trip through the
  relevant text serializers without changing register addresses or field
  semantics.
- Tests should cover register-file arrays, explicit field-level paths, disabled
  paths, and missing paths for reserved fields.

## P4: Software-Facing Outputs

Status: Not started.

- Add C header generation after the documentation view and HDL backdoor
  metadata are stable.
- Start with conservative constants and macros: base addresses, register
  offsets, field shifts, masks, reset values, and access comments.
- Avoid generated read/write helper functions until address-space ownership,
  volatile access width, endian assumptions, and side-effect semantics are
  explicitly modeled.
- Add naming-policy tests so generated identifiers are deterministic, valid C,
  collision-resistant, and configurable where necessary.

## P5: Register-File And RTL Outputs

Status: Not started.

- Treat register-file generation as higher risk than documentation or C
  headers because it implies hardware behavior, not just register metadata.
- Before adding a register-file output, decide the target explicitly:
  SystemVerilog RTL register file, UVM RAL, a machine-readable manifest, or
  another consumer-specific file.
- If SystemVerilog RTL returns, add it as a dedicated crate with an explicit
  behavior model for clocks, resets, bus protocol, write masks, read data,
  access side effects, reserved fields, and generated assertions.
- If UVM RAL returns, keep it separate from RALF serialization and verify it
  against simulator-compatible examples.

## P6: Multi-Version IP-XACT

Status: Active but intentionally narrow.

- Current CLI support emits and validates the register-oriented component subset
  for IEEE 1685-2009 and IEEE 1685-2022.
- Expand 2009 and 2022 beyond the current register-oriented component subset
  before claiming complete IEEE 1685-2009 or IEEE 1685-2022 coverage.
- Add representative schema validation for non-component root documents and
  optional schema branches in both versions.

## Verification

Useful gates:

```text
cargo fmt --all
cargo test -p ip-xact --test v2014_test --offline -- --nocapture
cargo test --workspace --offline --lib --tests
cargo clippy --workspace --all-targets --all-features --offline -- -D warnings
git diff --check
```

Release-oriented smoke checks:

```text
cargo build --release --locked --bin irgen --offline
target/release/irgen example.xlsx --snapsheet-spec snapsheet.toml -o /tmp/irgen-example.xml
target/release/irgen example.xlsx --snapsheet-spec snapsheet.toml --validate crates/ipxact/tests/fixtures/schemas/1685-2014/index.xsd
```
