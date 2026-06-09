# IP-XACT Generation

`irgen snapsheet` emits register-oriented IEEE 1685-2022 component XML from
the snapsheet model.

Only IEEE 1685-2022 is supported on the snapsheet output path. Keeping one
standard avoids divergent generated RAL metadata.

## CLI

```sh
cargo run -p irgen-cli -- snapsheet examples/example_simple.xlsx
cargo run -p irgen-cli -- snapsheet examples/example.xlsx --config snapsheet.toml
cargo run -p irgen-cli -- snapsheet examples/example.xlsx --standard ieee-1685-2022
```

`--format ip-xact` is the default. When `-o/--output` is omitted, output is
written as `<component>.xml`.

`--format all` writes one IP-XACT file:

```text
<component>-ip-xact-ieee-1685-2022.xml
```

## Coverage

The exporter covers the register-oriented subset produced by snapsheets:

- component VLNV
- memory maps and address blocks
- registers, register arrays, and registerFile arrays
- fields, access policies, resets, and descriptions
- field HDL paths through standard `accessHandles`
- `testable=false` and `reserved=true` field metadata

It is not a general IP-XACT authoring library for every root document or schema
feature.

Generated IP-XACT does not emit vendor-specific `snps:*` extensions.

## Backdoor Paths

Snapsheet `PATH` cells are treated as complete HDL paths. They are used only
when `--backdoor` is passed or `register.backdoor = true` is set in
`snapsheet.toml`.

Reserved fields and blank or `-` paths do not emit backdoor metadata.

## Validation

XSD validation is opt-in and requires `xmllint`:

```sh
cargo run -p irgen-cli -- snapsheet examples/example.xlsx \
  --config snapsheet.toml \
  --validate crates/ipxact/schema/1685-2022/index.xsd
```

Useful local gates:

```text
cargo fmt --all
cargo test -p irgen-ip-xact --test ipxact_xml
cargo test -p irgen-cli
cargo check --workspace
```

## Crates

- `crates/ipxact`: IEEE 1685-2022 XML exporter.
- `crates/snapsheet`: workbook parser and register model used by snapsheet
  exporters.
- `crates/ipxact-codegen`: local `xsd-parser` based schema generator.
- `crates/ipxact/schema/1685-2022`: third-party XSD files used for generation
  and validation.
