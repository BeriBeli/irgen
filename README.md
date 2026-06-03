# irgen

irgen (IP-XACT Register Generator) is a CLI tool that converts structured
snapsheets into IP-XACT XML files using the IEEE 1685-2014 format.

## CLI

Convert a snapsheet into IP-XACT XML:

```sh
cargo run -p irgen-cli -- example.xlsx
```

Write to a specific path:

```sh
cargo run -p irgen-cli -- example.xlsx -o output.xml
```

Validate generated IP-XACT XML with an installed `xmllint` and an explicitly
supplied XSD:

```sh
cargo run -p irgen-cli -- example.xlsx --validate path/to/index.xsd
```

## Snapsheet Format

> [!important]
>
> 1. ***Addresses, ranges, reset values, and `range(...)` arguments accept decimal numbers or hexadecimal numbers prefixed with 0x.***
> 2. ***`range(start?, end, step?)` expands register suffixes from `start` (inclusive) to `end` (exclusive). `step` is the byte offset between adjacent expanded registers and defaults to `0x4`. For example, to generate 10 registers incrementing by `0x10`, write `range(0, 10, 0x10)`.***
> 3. ***see example.xlsx as an example***

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
    > - ***When the register is named `reg{n}` with `n=range(start?, end, step?)`, only fill in the base address of the first expanded register.***
    > - ***`step` is the byte offset between adjacent expanded registers and defaults to `0x4`.***
    > - ***Expanded register addresses use 64-bit arithmetic.***

    This field indicates the register’s base address offset relative to the address block.

  - `REG`

    > [!important]
    >
    > - ***Must be unique within the address block. When a register contains multiple fields, merge the corresponding cells.***
    >
    > - ***For repeated registers, use `reg{n}, n=range(start?, end, step?)` to represent the array.***
    > - ***The generated suffix is the actual `n` value. For example, `reg{n}, n=range(1, 3)` generates `reg_1` and `reg_2`.***

    This field specifies the register name.

  - `FIELD`

    > [!important]
    >
    > - ***Must be unique within the register.***
    > - ***If left blank, the field name defaults to the register name after array expansion.***
    > - ***Reserved fields must be named using `reserved` or `rsvd` followed by a number (e.g., `reserved1`, `rsvd2`).***

    This field specifies the field name within the register.

  - `BIT`

    This field specifies the bit range of the field, e.g., `[31:0]`, `[20]`.

  - `WIDTH`

    > [!important]
    >
    > - ***The register width is calculated by summing the widths of all its fields. Ensure the total equals 32.***

    This field indicates the number of bits occupied by the field.

  - `ATTRIBUTE`

    This field specifies the field’s access type, e.g., `RW`, `RO`, `W1C`.

  - `DEFAULT`

    This field specifies the field’s reset value.

  - `DESCRIPTION`

    This field describes the field. If left blank, the tool will automatically populate it with "No Description".
