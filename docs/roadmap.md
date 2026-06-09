# irgen Roadmap

## Current Direction

`irgen` stays CLI-first. Each output format should live behind a narrow crate
and an explicit command path.

Current user flows:

- workbook snapsheet -> IP-XACT 2022 / RALF / SystemRDL
- IP-XACT component XML -> UVM RAL SystemVerilog
- IP-XACT component XML -> static HTML docs

Snapsheet input should not depend on documentation output directly. HTML is
generated from IP-XACT input with `irgen ip-xact --format html`.

## Crate Boundaries

- `crates/cli`: command-line entry point.
- `crates/snapsheet`: workbook parsing and validation.
- `crates/ipxact`: IEEE 1685-2022 exporter from the snapsheet model.
- `crates/ipxact-codegen`: local schema-code generator.
- `crates/ralf`: RALF model and serializer.
- `crates/systemrdl`: SystemRDL model and serializer.
- `crates/uvmreg`: direct IP-XACT XML to UVM RAL generator.
- `crates/docs`: static documentation generator used by `irgen ip-xact`.

Active dependency direction:

```text
cli -> snapsheet
cli -> ipxact -> snapsheet
cli -> ralf -> snapsheet
cli -> systemrdl -> snapsheet
cli -> uvmreg
cli -> docs
```

Do not add a generic facade crate unless there is a concrete shared API that
needs it.

## Current Capability

- Reads `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, and `.ods` workbooks.
- Emits IEEE 1685-2022 IP-XACT, RALF, and SystemRDL from snapsheets.
- Emits single repeated registers as register arrays, and groups matching
  repeated register windows as registerFile arrays.
- Splits wide spreadsheet registers by configured bus width.
- Supports `--bus-bytes`, `--backdoor`, `--config`, and IP-XACT `--validate`.
- Treats `RESET = -` as no reset and no compare.
- Marks reserved fields in IP-XACT and suppresses their backdoor paths.
- Generates UVM RAL from IP-XACT, including split-file output and optional
  coverage hooks.
- Generates static HTML documentation only from IP-XACT input.

## Near-Term Work

1. Keep the IEEE 1685-2022 exporter small and reproducible from
   `crates/ipxact-codegen`.
2. Add realistic IP-XACT fixtures for the UVM RAL and HTML paths.
3. Improve diagnostics for unsupported IP-XACT features in `uvmreg`.
4. Add more snapsheet fixtures for bus-width splitting, reserved fields, and
   reset/no-compare behavior.
5. Consider plain Markdown documentation output after the HTML path stabilizes.

## Out Of Scope For Now

- Restoring older IP-XACT output standards.
- Emitting vendor-specific IP-XACT extensions from snapsheet metadata.
- Generating HTML directly from snapsheets.
- Generating RTL register files.
- Full multi-root IP-XACT library management.

## Verification

Useful local gates:

```text
cargo fmt --all
cargo test -p irgen-snapsheet -p irgen-ip-xact -p irgen-ralf -p irgen-systemrdl -p irgen-cli
cargo test -p irgen-uvmreg
cargo check -p ipxact-codegen
cargo check --workspace
git diff --check
```

Release smoke examples:

```text
cargo build --release --locked --bin irgen
target/release/irgen snapsheet examples/example.xlsx --config snapsheet.toml -o /tmp/irgen-example.xml
target/release/irgen snapsheet examples/example.xlsx --config snapsheet.toml --validate crates/ipxact/schema/1685-2022/index.xsd
target/release/irgen ip-xact /tmp/irgen-example.xml --format html -o /tmp/irgen-example-html
```
