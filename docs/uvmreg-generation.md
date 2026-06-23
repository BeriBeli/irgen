# UVM RAL And HTML From IP-XACT

`irgen ip-xact` reads an IP-XACT component XML file. It can generate either
UVM RAL SystemVerilog or static HTML register documentation.

This path is separate from snapsheet parsing. IP-XACT input is parsed by
`crates/ipxact-parser` into the shared `crates/ipxact-model` representation.
UVM output is rendered by `crates/uvmreg`; HTML output converts the same parsed
model into the documentation view used by `crates/docs`.

## CLI

Generate UVM RAL:

```sh
cargo run -p irgen-cli -- ip-xact path/to/component.xml
cargo run -p irgen-cli -- ip-xact path/to/component.xml -o ral_component.sv
cargo run -p irgen-cli -- ip-xact path/to/component.xml --coverage
cargo run -p irgen-cli -- ip-xact path/to/component.xml --file-type header
cargo run -p irgen-cli -- ip-xact path/to/component.xml --file-layout blocks -o ral_component
```

Generate HTML docs:

```sh
cargo run -p irgen-cli -- ip-xact path/to/component.xml --format html -o docs-html
```

Other useful options:

```sh
cargo run -p irgen-cli -- ip-xact path/to/component.xml --view rtl
cargo run -p irgen-cli -- ip-xact path/to/component.xml --mode diagnostic
cargo run -p irgen-cli -- ip-xact path/to/component.xml --library-path path/to/ipxact/library
```

`--format uvm-reg` is the default. `--file-type package` is the default and
wraps generated RAL in a SystemVerilog package named `ral_<component>_pkg`.
`--file-type header` writes include-guarded SystemVerilog without a package.
`--file-layout single` writes one SystemVerilog file. `--file-layout blocks`
writes a directory containing a top-level file plus one file per address block.
In package blocks layout, `ral_<component>_pkg.sv` imports `uvm_pkg`, includes
`uvm_macros.svh`, and includes all generated class files. In header blocks
layout, only the top-level file imports `uvm_pkg` and includes `uvm_macros.svh`;
per-block files are include headers consumed by the top-level file.

For IEEE 1685-2022 `externalTypeDefinitions`, the CLI scans the input file's
directory and any `--library-path` directories. It matches XML files by VLNV
and follows IP-XACT catalog files.

## UVM Support

The generator supports common register-map XML:

- memory maps, address blocks, banks, register files, registers, fields, and
  memories
- scalar and multidimensional arrays
- reset values and reset masks
- common access and side-effect combinations
- enumerated values as SystemVerilog enum typedefs
- static `testable=false` and `reserved=true` metadata as
  `set_compare(UVM_NO_CHECK)`
- selected mode/view access policies
- HDL backdoor access handles
- optional register bit and memory address-map coverage
- common IEEE 1685-2022 type-definition reuse

Generated classes use conventional names such as `ral_sys_*`, `ral_block_*`,
`ral_regfile_*`, and `ral_reg_*`.

## HTML Support

`--format html` writes a static site directory with:

- `index.html`
- shared CSS/JS assets
- one block page per address block
- one register detail page per register

HTML generation is available only from `irgen ip-xact`, not directly from
`irgen snapsheet`.

## Current Limits

- No simulator compile gate is run by default.
- Catalog lookup is focused on common 2022 type-definition references.
- Parameter and configurable expression evaluation is static and intentionally
  limited.
- Runtime mode switching is not generated.
- Coverage output still depends on the consuming testbench enabling UVM RAL
  coverage before model construction and enabling simulator coverage
  collection.
- Vendor extensions are not preserved unless they already map to supported
  generated behavior.

## Useful Gates

```text
cargo test -p irgen-ipxact-parser -p irgen-uvmreg -p irgen-docs -p irgen-cli
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml -o ral_component.sv
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --file-type header -o ral_component.sv
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --file-layout blocks -o ral_component
cargo run -q -p irgen-cli -- ip-xact path/to/component.xml --format html -o docs-html
```
