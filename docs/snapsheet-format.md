# Snapsheet Format

`irgen snapsheet` converts workbook register tables into IP-XACT, RALF, and
SystemRDL.

Supported workbook formats: `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, and `.ods`.

## Modes

Default mode parses `examples/example_simple.xlsx` without a TOML file.

Configured mode uses `snapsheet.toml`:

```sh
cargo run -p irgen-cli -- snapsheet examples/example.xlsx --config snapsheet.toml
```

Configured mode enables custom sheet/column names, inherited `ADDR` / `REG`
cells, register arrays, reserved-field validation, and optional backdoor paths.

## Sheets

Configured workbooks use:

- `version`: component identity
- `address_map`: address block list
- one register sheet per address block

### Version

| VENDOR | LIBRARY | NAME | VERSION | DESC |
| --- | --- | --- | --- | --- |
| example.com | example | example | 1.0.0 | initial |

`VENDOR`, `LIBRARY`, `NAME`, and `VERSION` map to the generated component VLNV.
`DESC` is informational.

### Address Map

| BLOCK | OFFSET | RANGE | DESC |
| --- | --- | --- | --- |
| regs | 0x0 | 0x1000 | control registers |

- `BLOCK`: address block name and matching register sheet name.
- `OFFSET`: block base address.
- `RANGE`: validation bound in bytes.
- `DESC`: optional description.

`RANGE` is contracted during conversion to the used register span, so unused
tail space is not emitted.

### Register Sheets

| ADDR | REG | REG_DESC | FIELD | BIT | ATTR | RESET | FIELD_DESC | PATH |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x0 | ctrl | Control register | enable | [0] | RW | 0x0 | enable bit | dut.ctrl.enable |
| | | | reserved0 | [31:1] | RO | - | | - |

Columns:

- `ADDR`: register address relative to the block.
- `REG`: register name.
- `REG_DESC`: optional register description.
- `FIELD`: field name. Configured mode can default blank names to `REG`.
- `BIT`: bit range, such as `[31:0]` or `[0]`.
- `ATTR`: access attribute, such as `RW`, `RO`, or `W1C`.
- `RESET`: reset value. `-` means no reset and no compare.
- `FIELD_DESC`: optional field description.
- `PATH`: complete HDL backdoor path, used only when backdoor output is enabled.

Legacy `WIDTH` columns may still appear and are checked against `BIT` when
present.

## CLI Options

```sh
cargo run -p irgen-cli -- snapsheet input.xlsx --format ip-xact
cargo run -p irgen-cli -- snapsheet input.xlsx --format ralf
cargo run -p irgen-cli -- snapsheet input.xlsx --format systemrdl
cargo run -p irgen-cli -- snapsheet input.xlsx --format all
```

Other useful options:

```sh
cargo run -p irgen-cli -- snapsheet input.xlsx --bus-bytes 8
cargo run -p irgen-cli -- snapsheet input.xlsx --backdoor
cargo run -p irgen-cli -- snapsheet input.xlsx --validate crates/ipxact/schema/1685-2022/index.xsd
```

`--bus-bytes` accepts `1`, `2`, `4`, `8`, or `16`. Wide spreadsheet registers
are split into physical registers at this bus width before output generation.

`--backdoor` enables `PATH` consumption. Without it, the `PATH` column is
ignored.

HTML is not a snapsheet output. Generate IP-XACT first, then run:

```sh
cargo run -p irgen-cli -- ip-xact component.xml --format html -o docs-html
```

## Register Arrays

Configured array syntax:

```text
{name}{n}, n=range({start?}, {end}, {step?})
```

Example:

```text
channel{n}, n=range(0, 4, 0x10)
```

This creates a register-file array with `dim = end - start` and byte stride
`step`. The default stride is `register.default_array_step_bytes`.

## TOML Shape

Config files only need to contain overrides; omitted settings use built-in
defaults. The root `snapsheet.toml` keeps just the options needed by
`examples/example.xlsx`:

```toml
[register]
inherit_address = true
inherit_register = true
blank_field_name = "register_name"

[register.array]
enabled = true

[reserved]
enabled = true
```

Add `[columns.*]` or `[workbook.sheets]` entries only when a workbook uses
different sheet or column names from the defaults shown above.

Reserved field names must match `reserved[0-9]+` or `rsvd[0-9]+` when reserved
validation is enabled.

## Validation

The parser rejects common workbook mistakes before output generation:

- duplicate or overlapping address blocks
- duplicate or overlapping registers
- duplicate or overlapping fields
- malformed arrays
- invalid access attributes
- register widths that are not byte-aligned
- invalid bus widths
- reset values that do not fit inside the field width
- register arrays that exceed `max_array_elements`
