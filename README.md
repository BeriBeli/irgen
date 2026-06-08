# irgen

`irgen` is a CLI-first register generation toolkit. It converts structured
workbook snapsheets into register-oriented SPIRIT/IP-XACT XML, RALF, SystemRDL,
or HTML register documentation, and it generates UVM RAL SystemVerilog from
IP-XACT component XML.

## Quick Start

`irgen` has two subcommands:

- `snapsheet`: convert workbook snapsheets into IP-XACT, RALF, SystemRDL, HTML,
  or all supported outputs.
- `ip-xact`: generate UVM register model SystemVerilog from an IP-XACT
  component XML file.

Generate IP-XACT XML:

```sh
./irgen snapsheet examples/example_simple.xlsx
```

Snapsheet input can be `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, or `.ods` as long as
the workbook uses the expected sheet and column layout.

When `-o/--output` is omitted, IP-XACT, RALF, SystemRDL, and all-output paths
are written in the current directory using the component name:
`<component>.xml`, `<component>.ralf`, `<component>.rdl`, or `<component>/`.
HTML output still defaults to the input file stem, such as `example_simple/`.

IP-XACT output defaults to IEEE 1685-2014. SPIRIT 1.4, SPIRIT 1.5,
IEEE 1685-2009, IEEE 1685-2014, and IEEE 1685-2022 can also be specified
explicitly for the current register-oriented subset:

```sh
./irgen snapsheet examples/example_simple.xlsx --standard spirit-1.4
./irgen snapsheet examples/example_simple.xlsx --standard spirit-1.5
./irgen snapsheet examples/example_simple.xlsx --standard ieee-1685-2009
./irgen snapsheet examples/example_simple.xlsx --standard ieee-1685-2014
./irgen snapsheet examples/example_simple.xlsx --standard ieee-1685-2022
```

The IP-XACT emitters cover the register-oriented component subset produced by
snapsheets: memory maps, address blocks, registers, register-file arrays where
the target schema supports them, fields, resets, and field access metadata.
They are not complete models for every IP-XACT root document or schema feature.
SPIRIT 1.4 does not define `registerFile`, so register-file arrays are flattened
into ordinary registers in 1.4 output.

Field HDL backdoor paths in IEEE 1685-2014 and IEEE 1685-2022 are emitted
through standard `accessHandles`. The SPIRIT 1.4, SPIRIT 1.5, and IEEE
1685-2009 emitters do not carry HDL paths because those standards do not
provide the same standard register-model access-handle structure. Generated
IP-XACT does not add macro-backed block HDL paths or emit Synopsys `snps:*`
vendor extensions.

Generate RALF or SystemRDL:

```sh
./irgen snapsheet examples/example_simple.xlsx --format ralf
./irgen snapsheet examples/example_simple.xlsx --format systemrdl
```

Generate DWC-style HTML register documentation. HTML output is a directory with
an `index.html`, shared assets, block index pages, and register detail pages:

```sh
./irgen snapsheet examples/example_simple.xlsx --format html
```

Generate every supported output format at once:

```sh
./irgen snapsheet examples/example_simple.xlsx --format all
```

The all-output directory contains `<component>-ip-xact-spirit-1.4.xml`,
`<component>-ip-xact-spirit-1.5.xml`,
`<component>-ip-xact-ieee-1685-2009.xml`,
`<component>-ip-xact-ieee-1685-2014.xml`,
`<component>-ip-xact-ieee-1685-2022.xml`, `<component>.ralf`,
`<component>.rdl`, and an `html/` documentation directory.

Write to a specific output path:

```sh
./irgen snapsheet examples/example_simple.xlsx -o output.xml
./irgen snapsheet examples/example_simple.xlsx --format html -o docs-html
./irgen snapsheet examples/example_simple.xlsx --format all -o generated
```

Use a TOML snapsheet specification for custom sheet names, column names, array
syntax, inherited cells, and stricter validation:

```sh
./irgen snapsheet examples/example.xlsx --config snapsheet.toml
```

Validate generated IP-XACT XML with `xmllint` and an explicit XSD:

```sh
./irgen snapsheet examples/example_simple.xlsx --standard spirit-1.5 --validate crates/ipxact/schema/1.5/index.xsd
```

`--validate` and `--standard` are only available with `--format ip-xact`.

Generate UVM RAL SystemVerilog from an IP-XACT component XML file:

```sh
./irgen ip-xact path/to/component.xml
./irgen ip-xact path/to/component.xml -o ral_component.sv
./irgen ip-xact path/to/component.xml --coverage
./irgen ip-xact path/to/component.xml --file-layout blocks -o ral_component
```

When `-o/--output` is omitted, the `ip-xact` subcommand writes
`ral_<component-name>.sv` in the current directory. Pass `--file-layout blocks`
to write a directory instead: a top-level `ral_<component-name>.sv` that
includes one `ral_block_<block-name>.sv` file per address block. With block
layout and no `-o/--output`, the output directory defaults to
`ral_<component-name>`.

For IEEE 1685-2022 `externalTypeDefinitions`, the CLI scans XML files in the
input file's directory and any `--library-path` directories. It matches
referenced documents by VLNV and follows IP-XACT `catalog` files.

Pass `--coverage` to emit UVM `UVM_CVR_REG_BITS` register bit coverage support
in the generated register classes. The consuming testbench and simulator run
must still enable UVM RAL coverage collection for coverage to appear in reports.

## Documentation

- [Snapsheet format](docs/snapsheet-format.md): workbook layout, TOML parser
  configuration, arrays, reserved fields, and validation behavior.
- [RALF generation](docs/ralf-generation.md): RALF model coverage, snapsheet
  mapping, and current limitations.
- [SystemRDL generation](docs/systemrdl-generation.md): SystemRDL model
  coverage, snapsheet mapping, and current limitations.
- [IP-XACT generation](docs/ipxact-generation.md): supported schema standards,
  crate layout, and current register-oriented coverage.
- [UVM register model generation](docs/uvmreg-generation.md): IP-XACT input to
  UVM IEEE 2020 RAL coverage, five-version usability assessment, and remaining
  work.
- [Roadmap](docs/roadmap.md): crate boundaries, milestones, and useful
  verification gates.

## Examples

- `examples/example_simple.xlsx` uses the default no-TOML workbook format.
- `examples/example.xlsx` plus `snapsheet.toml` shows the richer configurable
  format.
- `examples/example_simple.xlsm` and `examples/example_simple.ods` mirror the
  default workbook in additional supported formats.
- `examples/example.xlsm` and `examples/example.ods` mirror the configured
  workbook in additional supported formats.
