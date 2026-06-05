# irgen

`irgen` is a CLI-first register spreadsheet converter. It reads structured
Excel snapsheets and emits register-oriented IEEE 1685 IP-XACT XML, Synopsys
RALF, SystemRDL, or HTML register documentation.

## Quick Start

Generate IP-XACT XML:

```sh
cargo run -p irgen-cli -- example_simple.xlsx
```

When `-o/--output` is omitted, outputs are written in the current directory
using the input file stem: `example_simple.xml`, `example_simple.ralf`,
`example_simple.rdl`, the `example_simple/` HTML directory, or the
`example_simple/` all-output directory.

IP-XACT output defaults to IEEE 1685-2014. IEEE 1685-2009 and IEEE 1685-2022
can also be specified explicitly for the current register-oriented subset:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --ipxact-version 2009
cargo run -p irgen-cli -- example_simple.xlsx --ipxact-version 2014
cargo run -p irgen-cli -- example_simple.xlsx --ipxact-version 2022
```

2009 and 2022 support is intentionally narrower than the 2014 path today: the
CLI emits the component, memory map, register, register-file, field, reset, and
access-policy structures needed by snapsheet register tables, but it is not yet
a complete model for every IP-XACT root document or schema feature.

Generate RALF or SystemRDL:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format ralf
cargo run -p irgen-cli -- example_simple.xlsx --format systemrdl
```

Generate DWC-style HTML register documentation. HTML output is a directory with
an `index.html`, shared assets, block index pages, and register detail pages:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format html
```

Generate every supported output format at once:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format all
```

The all-output directory contains `ipxact-2009.xml`, `ipxact-2014.xml`,
`ipxact-2022.xml`, `ralf.ralf`, `systemrdl.rdl`, and an `html/` documentation
directory.

Write to a specific output path:

```sh
cargo run -p irgen-cli -- example_simple.xlsx -o output.xml
cargo run -p irgen-cli -- example_simple.xlsx --format html -o docs-html
cargo run -p irgen-cli -- example_simple.xlsx --format all -o generated
```

Use a TOML snapsheet specification for custom sheet names, column names, array
syntax, inherited cells, and stricter validation:

```sh
cargo run -p irgen-cli -- example.xlsx --snapsheet-spec snapsheet.toml
```

Validate generated IP-XACT XML with `xmllint` and an explicit XSD:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --validate path/to/index.xsd
```

`--validate` and `--ipxact-version` are only available with `--format ipxact`.

## Documentation

- [Snapsheet format](docs/snapsheet-format.md): workbook layout, TOML parser
  configuration, arrays, reserved fields, and validation behavior.
- [RALF generation](docs/ralf-generation.md): RALF model coverage, snapsheet
  mapping, and current limitations.
- [SystemRDL generation](docs/systemrdl-generation.md): SystemRDL model
  coverage, snapsheet mapping, and current limitations.
- [IP-XACT 2014 compliance](docs/ipxact-2014-compliance.md): active IEEE
  1685-2014 compliance status and verification evidence.
- [Roadmap](docs/roadmap.md): crate boundaries, milestones, and useful
  verification gates.

## Examples

- `example_simple.xlsx` uses the default no-TOML workbook format.
- `example.xlsx` plus `snapsheet.toml` shows the richer configurable format.
