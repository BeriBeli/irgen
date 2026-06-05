# Snapsheet Format

## Status

`irgen` supports two snapsheet modes:

- Default mode parses `example_simple.xlsx` without a TOML specification.
- Configured mode parses `example.xlsx` with `snapsheet.toml` or another
  `--snapsheet-spec <snapsheet.toml>` file.

Default mode expects each register row to provide its own `ADDR`, `REG`, and
`FIELD`. It does not enable array syntax, inherited cells, blank field names
that become register names, or reserved-name matching.

Configured mode enables richer parser rules, including custom sheet and column
names, inherited address/register cells, register-file arrays, reserved-field
matching, and stricter validation.

## CLI Usage

Use the default parser:

```sh
cargo run -p irgen-cli -- example_simple.xlsx
```

Use a TOML snapsheet specification:

```sh
cargo run -p irgen-cli -- example.xlsx --snapsheet-spec snapsheet.toml
```

The root `snapsheet.toml` matches the richer workbook example and can be copied
as a starting point for another workbook.

## Workbook Layout

Configured mode uses three workbook areas:

- A version sheet that provides component identity.
- An address-map sheet that allocates address blocks.
- One register sheet per address block.

The default example uses the same logical columns, but without TOML-driven
renaming or parser options.

## Version Sheet

Default configured sheet name: `version`.

| VENDOR | LIBRARY | NAME | VERSION | DESC |
| --- | --- | --- | --- | --- |
| example.com | example | example | 1.0.0 | initial |

Columns:

- `VENDOR`: maps to `component.vendor` in IP-XACT.
- `LIBRARY`: maps to `component.library` in IP-XACT.
- `NAME`: maps to `component.name` in IP-XACT.
- `VERSION`: maps to `component.version` in IP-XACT.
- `DESC`: reserved for version update notes.

## Address-Map Sheet

Default configured sheet name: `address_map`.

| BLOCK | OFFSET | RANGE | DESC |
| --- | --- | --- | --- |
| noc_reg | 0x8000 | 0x8000 | reg block of noc |

Columns:

- `BLOCK`: address-block name. In configured mode, each register sheet must
  match one address-block name.
- `OFFSET`: address-block base offset.
- `RANGE`: address-block size in bytes.
- `DESC`: optional block description.

## Register Sheets

Each register sheet describes the registers under one address block.

| ADDR | REG | REG_DESC | FIELD | BIT | ATTR | RESET | FIELD_DESC |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0x0 | noc_version | Version information register | version | [31:0] | RO | 0x20250101 | noc_version |
| 0x4 | noc_config | Configuration register | config | [31:0] | RW | 0x1 | noc_config |
| 0x1000 | reg{n}, n=range(0, 10, 0x4) | Repeated example register | field1 | [31:24] | RW | 0x0 | example |
| | | | rsvd1 | [23:16] | RO | 0x0 | |
| | | | field0 | [15:8] | RW | 0x0 | |
| | | | rsvd0 | [7:0] | RO | 0x0 | |

Columns:

- `ADDR`: register base address relative to the address block. For
  `REG` values that use `{n}` with `n=range(...)`, this is the base address of
  the generated register-file array.
- `REG`: register name. It must be unique within the address block unless
  multiple rows describe fields in the same register.
- `REG_DESC`: optional register-level description. If multiple rows
  describe the same register, the first non-empty value is used.
- `FIELD`: field name. It must be unique within the register. In configured
  mode, a blank field name can default to the register name.
- `BIT`: field bit range, such as `[31:0]` or `[20]`. Field width is inferred
  from this range.
- `ATTR`: field access type, such as `RW`, `RO`, or `W1C`.
- `RESET`: field reset value.
- `FIELD_DESC`: optional field description. Blank values remain blank unless a
  non-empty default description is configured.

For backwards compatibility, workbooks may still include the old `WIDTH` field
size column. If present, it is optional and can be checked against `BIT`.

## Number And Array Rules

Addresses, ranges, reset values, and `range(...)` arguments accept decimal
numbers or hexadecimal numbers prefixed with `0x`.

Configured array syntax is:

```text
{name}{n}, n=range({start?}, {end}, {step?})
```

Array behavior:

- `range(start?, end, step?)` creates an IP-XACT `registerFile` array.
- `dim` is `end - start`.
- `step` is the byte stride between adjacent array elements.
- The default `step` is `0x4`.
- Register and register-file address calculations use checked `u64`
  arithmetic.
- `start` is accepted by the syntax but is not emitted as a suffix.

## Register Size

When `register_size = "infer_from_fields"`, the register size is inferred from
the highest field bit plus one. The result may be 32, 64, 128, or another
byte-aligned width.

When `require_byte_aligned = true`, non-byte-aligned register sizes are
rejected.

## TOML Specification

The configured example uses this shape:

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
description = "DESC"

[columns.address_block]
name = "BLOCK"
offset = "OFFSET"
range = "RANGE"
description = "DESC"

[columns.register]
address = "ADDR"
register = "REG"
field = "FIELD"
bit = "BIT"
access = "ATTR"
reset = "RESET"
register_description = "REG_DESC"
description = "FIELD_DESC"

[register]
inherit_address = true
inherit_register = true
# default_description = ""
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
check_reset_fits_width = true

[reserved]
enabled = true
patterns = ["^reserved[0-9]+$", "^rsvd[0-9]+$"]
```

## Validation

Configured mode can reject common workbook issues before output generation:

- duplicate or overlapping address blocks
- duplicate or overlapping registers
- duplicate or overlapping fields
- malformed array syntax
- invalid access attributes
- legacy `WIDTH` values that do not match bit ranges, when the column is present
- reset values that do not fit inside the field width
- register arrays that exceed `max_array_elements`

IP-XACT XSD validation is a separate CLI step enabled with `--validate` and is
only available for `--format ipxact`. IP-XACT output defaults to version 2014;
versions 2009, 2014, and 2022 can be selected with `--ipxact-version`.

The 2009 and 2022 emitters currently cover the register-oriented component
subset produced from snapsheets: memory maps, address blocks, registers,
register-file arrays, fields, resets where supported by the target schema, and
field access metadata. They are not yet complete implementations of every
IEEE 1685-2009 or IEEE 1685-2022 document type or schema feature.
