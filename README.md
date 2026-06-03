# irgen

irgen (IP-XACT Register Generator) is a CLI tool that converts structured
snapsheets into register model files. It can emit IEEE 1685-2014 IP-XACT XML,
native Synopsys RALF, or SystemRDL.

## CLI

Convert a snapsheet into IP-XACT XML:

```sh
cargo run -p irgen-cli -- example_simple.xlsx
```

Convert a snapsheet into RALF:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format ralf
```

Convert a snapsheet into SystemRDL:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format systemrdl
```

Write to a specific path:

```sh
cargo run -p irgen-cli -- example_simple.xlsx -o output.xml
```

Validate generated IP-XACT XML with an installed `xmllint` and an explicitly
supplied XSD. Validation is only available for `--format ipxact`:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --validate path/to/index.xsd
```

Without a TOML file, `irgen` uses a simple default parser. Each register row
must provide its own `ADDR`, `REG`, and `FIELD`; array syntax, inherited cells,
blank field names that become register names, and reserved-name matching are not
enabled.

Use a TOML snapsheet specification to enable richer parser rules:

```sh
cargo run -p irgen-cli -- example.xlsx --snapsheet-spec snapsheet.toml
```

See [`docs/snapsheet-spec.example.toml`](docs/snapsheet-spec.example.toml)
or [`snapsheet.toml`](snapsheet.toml) for the complex example specification.
See [`docs/ralf-generation.md`](docs/ralf-generation.md) for RALF output
coverage and current limitations.
See [`docs/systemrdl-generation.md`](docs/systemrdl-generation.md) for
SystemRDL output coverage and current limitations.

```toml
[workbook.sheets]
version = "version"
address_map = "address_map"
register_sheet = "block_name"

[columns.version]
vendor = "VENDOR"
library = "LIBRARY"
name = "NAME"
version = "VERSION"

[columns.address_block]
name = "BLOCK"
offset = "OFFSET"
range = "RANGE"

[columns.register]
address = "ADDR"
register = "REG"
field = "FIELD"
bit = "BIT"
width = "WIDTH"
access = "ATTRIBUTE"
reset = "DEFAULT"
description = "DESCRIPTION"

[register]
inherit_address = true
inherit_register = true
default_description = "No Description"
default_array_step_bytes = "0x4"
max_array_elements = 1000000
register_size = "infer_from_fields"
require_byte_aligned = true
blank_field_name = "register_name"

[register.array]
enabled = true
syntax = "range"
pattern = "{name}{n}, n=range({start?}, {end}, {step?})"

[validation]
reject_duplicate_blocks = true
reject_overlapping_blocks = true
reject_duplicate_registers = true
reject_overlapping_registers = true
reject_duplicate_fields = true
reject_overlapping_fields = true
check_bit_range_matches_width = true
check_reset_fits_width = true

[reserved]
enabled = true
patterns = ["^reserved[0-9]+$", "^rsvd[0-9]+$"]
```

## Snapsheet Format

> [!important]
>
> 1. ***Addresses, ranges, reset values, and `range(...)` arguments accept decimal numbers or hexadecimal numbers prefixed with 0x.***
> 2. ***`range(start?, end, step?)` creates an IP-XACT `registerFile` array. `dim` is `end - start`, and `step` becomes the registerFile `range`/byte stride between adjacent elements. The default `step` is `0x4`.***
> 3. ***see example_simple.xlsx for the no-TOML default format, and example.xlsx with snapsheet.toml for the richer format***

1. Version/Vendor sheet (sheet name: `version`)

| VENDOR      | LIBRARY | NAME    | VERSION | DESCRIPTION |
| ----------- | ------- | ------- | ------- | ----------- |
| example.com | example | example | 1.0.0   | initial     |
|             |         |         |         | ......      |

  - `VENDOR`

    Same as `component.vendor` in IP-XACT.

  - `LIBRARY`

    Same as `component.library` in IP-XACT.

  - `NAME`

    Same as `component.name` in IP-XACT.

  - `VERSION`

    Same as `component.version` in IP-XACT.

  - `DESCRIPTION`

    This field is reserved for version update notes.

2. Address Block Allocation sheet (sheet name: `address_map`)

| BLOCK   | OFFSET | RANGE  | DESCRIPTION      |
| ------- | ------ | ------ | ---------------- |
| noc_reg | 0x8000 | 0x8000 | reg block of noc |
| ......  | ...... | ...... | ......           |

  - `BLOCK`

    This field specifies the name of the address block.

  - `OFFSET`

    This field specifies the offset address of the address block.

  - `RANGE`

    This field specifies the size (in bytes) of the address block.

  - `DESCRIPTION`

    This field describes the module’s functionality. Since no software interface is provided for this section, it may be left blank.

3. Register Description sheets under each address block (sheet name must match the address block name)

| ADDR   | REG                     | FIELD   | BIT     | WIDTH  | ATTRIBUTE | DEFAULT    | DESCRIPTION |
| ------ | ----------------------- | ------- | ------- | ------ | --------- | ---------- | ----------- |
| 0x0    | noc_version             | version | [31:0]  | 32     | RO        | 0x20250101 | noc_version |
| 0x4    | noc_config              | config  | [31:0]  | 32     | RW        | 0x1        | noc_config  |
| 0x1000 | reg{n}, n=range(0, 10, 0x4) | field1  | [31:24] | 8      | RW        | 0x0        | example     |
|        |                           | rsvd1   | [23:16] | 8      | RO        | 0x0        |             |
|        |                           | field0  | [15:8]  | 8      | RW        | 0x0        |             |
|        |                           | rsvd0   | [7:0]   | 8      | RO        | 0x0        |             |
| ...... | ......                  | ......  | ......  | ...... | ......    | ......     | ......      |

  - `ADDR`

    > [!important]
    >
    > - ***When `REG` uses `{n}` with `n=range(start?, end, step?)`, fill in the base address of the generated registerFile.***
    > - ***`step` becomes the generated registerFile `range` and defaults to `0x4`.***
    > - ***Register and registerFile address calculations use 64-bit arithmetic.***

    This field indicates the register’s base address offset relative to the address block.

  - `REG`

    > [!important]
    >
    > - ***Must be unique within the address block. When a register contains multiple fields, merge the corresponding cells.***
    >
    > - ***For repeated register structures, use `reg{n}, n=range(start?, end, step?)`. This emits one IP-XACT registerFile named `reg`.***
    > - ***The generated registerFile has `dim = end - start`; `start` is accepted by the syntax but is not emitted as a suffix.***

    This field specifies the register name.

  - `FIELD`

    > [!important]
    >
    > - ***Must be unique within the register.***
    > - ***If left blank, the field name defaults to the register name. For `{n}` forms, this is the child register name inside the generated registerFile.***
    > - ***Reserved fields must be named using `reserved` or `rsvd` followed by a number (e.g., `reserved1`, `rsvd2`).***

    This field specifies the field name within the register.

  - `BIT`

    This field specifies the bit range of the field, e.g., `[31:0]`, `[20]`.

  - `WIDTH`

    > [!important]
    >
    > - ***The register size is inferred from the highest field bit plus one, matching IP-XACT `register/size`. It may be 32, 64, 128, etc., and must be byte-aligned.***

    This field indicates the number of bits occupied by the field.

  - `ATTRIBUTE`

    This field specifies the field’s access type, e.g., `RW`, `RO`, `W1C`.

  - `DEFAULT`

    This field specifies the field’s reset value.

  - `DESCRIPTION`

    This field describes the field. If left blank, the tool will automatically populate it with "No Description".
