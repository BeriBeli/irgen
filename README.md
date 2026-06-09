# irgen

`irgen` is a CLI-first register generation toolkit.

It currently has two user-facing flows:

- `irgen snapsheet`: convert workbook register tables into IEEE 1685-2022
  IP-XACT, RALF, SystemRDL, or all three text outputs.
- `irgen ip-xact`: generate UVM RAL SystemVerilog or static HTML register
  documentation from an IP-XACT component XML file.

## Quick Start

Build the CLI:

```sh
cargo build --release
```

Generate IEEE 1685-2022 IP-XACT from a workbook:

```sh
./target/release/irgen snapsheet examples/example_simple.xlsx
```

Generate RALF or SystemRDL:

```sh
./target/release/irgen snapsheet examples/example_simple.xlsx --format ralf
./target/release/irgen snapsheet examples/example_simple.xlsx --format systemrdl
```

Generate all snapsheet text outputs:

```sh
./target/release/irgen snapsheet examples/example_simple.xlsx --format all -o generated
```

The `all` output directory contains:

```text
<component>-ip-xact-ieee-1685-2022.xml
<component>.ralf
<component>.rdl
```

Generate UVM RAL from IP-XACT:

```sh
./target/release/irgen ip-xact path/to/component.xml
./target/release/irgen ip-xact path/to/component.xml --coverage
./target/release/irgen ip-xact path/to/component.xml --file-layout blocks -o ral_component
```

Generate HTML documentation from IP-XACT:

```sh
./target/release/irgen ip-xact path/to/component.xml --format html -o docs-html
```

`snapsheet` does not generate HTML directly. Convert the workbook to IP-XACT
first, then run `irgen ip-xact --format html` if documentation is needed.

## Snapsheet Inputs

Workbook input may be `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, or `.ods`.

Default parsing works with `examples/example_simple.xlsx`. Use
`--config snapsheet.toml` for custom sheet/column names, inherited cells,
register arrays, reserved-field checks, and optional backdoor paths:

```sh
./target/release/irgen snapsheet examples/example.xlsx --config snapsheet.toml
```

Useful snapsheet options:

```sh
./target/release/irgen snapsheet input.xlsx --bus-bytes 8
./target/release/irgen snapsheet input.xlsx --backdoor
./target/release/irgen snapsheet input.xlsx --validate crates/ipxact/schema/1685-2022/index.xsd
```

`--validate` requires `xmllint` and is only valid with `--format ip-xact`.
`--standard` currently accepts only `ieee-1685-2022`.

## Output Defaults

When `-o/--output` is omitted:

- `snapsheet --format ip-xact` writes `<component>.xml`
- `snapsheet --format ralf` writes `<component>.ralf`
- `snapsheet --format systemrdl` writes `<component>.rdl`
- `snapsheet --format all` writes `<component>/`
- `ip-xact` writes `ral_<component>.sv`
- `ip-xact --file-layout blocks` writes `ral_<component>/`
- `ip-xact --format html` writes `<component>.html/`

## Documentation

- [Snapsheet format](docs/snapsheet-format.md)
- [IP-XACT generation](docs/ipxact-generation.md)
- [RALF generation](docs/ralf-generation.md)
- [SystemRDL generation](docs/systemrdl-generation.md)
- [UVM RAL and HTML from IP-XACT](docs/uvmreg-generation.md)
- [Roadmap](docs/roadmap.md)

## Examples

- `examples/example_simple.xlsx`: default workbook layout.
- `examples/example.xlsx` with `snapsheet.toml`: configured workbook layout.
- `.xlsm` and `.ods` variants mirror the workbook examples.
