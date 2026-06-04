# irgen Roadmap

## Direction

The current product goal is a CLI-first register spreadsheet converter that
emits IEEE 1685-2014 IP-XACT and related text formats. UI work and UI
dependencies have been removed from the active path.

IEEE 1685-2014 compliance is the active milestone. IEEE 1685-2009 and IEEE
1685-2022 are P2 multi-version work.

## Crate Boundaries

- `crates/cli`: command-line entry point.
- `crates/snapsheet`: spreadsheet loading, row validation, array expansion, and
  register aggregation.
- `crates/model`: lightweight register IR plus conversion into
  `ip_xact::v2014`.
- `crates/ipxact`: broader IP-XACT schema model, currently focused on 2014
  compliance.
- `crates/ralf`: RALF model and serializer.
- `crates/systemrdl`: SystemRDL model and serializer.

Active dependency direction:

```text
cli -> snapsheet -> model -> ipxact
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

- Converts `.xlsx` input into IP-XACT 2014 XML, RALF, and SystemRDL.
- Supports `--format ipxact|ralf|systemrdl`.
- Supports `--ipxact-version 2014`, with 2014 as the default for IP-XACT
  output.
- Supports `--snapsheet-spec <snapsheet.toml>`.
- Supports opt-in `--validate <schema.xsd>` for IP-XACT XML via `xmllint`.
- Validates common workbook failures before conversion, including duplicate
  fields, overlapping bit ranges, malformed arrays, invalid attributes,
  address collisions, out-of-range registers, and reset values that do not fit.
- Emits register arrays as IP-XACT `registerFile` arrays.
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

## P1: Cleanup

- Replace remaining placeholder or stringly typed 2014 nested structures that
  are still on active paths.
- Continue auditing overbroad expression wrapper types where the schema
  disallows extension attributes.
- Add new `.xlsx` fixtures as parser failure modes are discovered.
- Decide whether XML schema validation should remain opt-in CLI behavior or
  become part of release verification only.

## P2: Multi-Version And Future Formats

- Expand official schema validation and namespace-aware serializers before
  claiming IEEE 1685-2009 or IEEE 1685-2022 compliance.
- Review generated 2022 serde names before promoting 2022 support.
- If C header, UVM RAL, SystemVerilog RTL, or HTML outputs return, add
  them as explicit CLI formats or separate crates rather than broadening the
  current register IR prematurely.

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
