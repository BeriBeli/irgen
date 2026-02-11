# irgen-gui

irgen (IP-XACT Register Generator) is a tool that converts structured Snapsheets into IP-XACT (.xml, IEEE-1685-2014) format. Additionally, it generates JSON configuration files required by [regvue](https://github.com/nasa-jpl/regvue), a modern hardware/software interface documentation tool provided by NASA.

## User Configuration

irgen reads user configuration from a project-local dot-config directory under the user's home:

- Linux/macOS: `$HOME/.config/irgen`
- Windows: `%USERPROFILE%\\.config\\irgen`

Subdirectories:

- `themes/`: custom gpui-component theme JSON files
- `templates/`: custom `.tera` templates
  - `templates/c_header.tera`: custom C header template file
  - `templates/uvm_ral.tera`: custom C source template file
  - `templates/sv_rtl.tera`: custom SystemVerilog RTL template file
  - `templates/html.tera`: custom HTML template file

Configuration file:

- `config.json`: application settings persisted across restarts (including theme mode and last selected light/dark theme names)

Template loading priority:

1. User templates under `templates/`
2. Built-in templates from application assets

This means user templates override built-in templates with the same template name.

### Template Context Variables

Custom `.tera` templates receive one register model context with two aliases:

- `compo`
- `component`

They point to the same object. Top-level fields are also exposed for backward compatibility.

Context shape:

```text
Component:
  vendor: string
  library: string
  name: string
  version: string
  blks: Block[]

Block:
  name: string
  offset: string
  range: string
  size: string
  regs: Register[]

Register:
  name: string
  offset: string
  size: string
  fields: Field[]

Field:
  name: string
  offset: string
  width: string
  attr: string
  reset: string
  desc: string
```

Important notes:

- `offset`, `range`, `size`, `width`, `reset` are strings in template context (for example `0x100`, `32`).
- Block-level and register-level descriptions are not currently provided in context. Field description is available as `field.desc`.
- Built-in templates use standard Tera filters such as `upper`, `lower`, `replace`, and `default`.

Minimal example:

```tera
/* {{ compo.name }} v{{ compo.version }} */
{% for blk in compo.blks %}
/* block {{ blk.name }} @ {{ blk.offset }} */
{% for reg in blk.regs %}
#define {{ compo.name | upper }}_{{ blk.name | upper }}_{{ reg.name | upper }}_OFF {{ reg.offset }}
{% for field in reg.fields %}
/* field {{ field.name }} width={{ field.width }} attr={{ field.attr }} reset={{ field.reset }} */
{% endfor %}
{% endfor %}
{% endfor %}
```

Theme loading behavior:

1. Built-in themes from `assets/themes/` are available in irgen.
2. User themes are loaded from `themes/`.
3. If a user theme has the same `name` and `mode`, user theme takes precedence (same priority model as templates).
4. `gpui-component` built-in themes remain available as additional fallback themes.

## Snapsheet Format

> [!important]
>
> 1. ***All hexadecimal numbers must be prefixed with 0x; decimal numbers must have no prefix.***
> 2. ***range(start?, end, step?) is the syntax for expressing lists, similar to Python’s range() function. It supports range(start, end), range(end), and range(start, end, step). For example, to generate 10 registers incrementing by 0x10, write range(0, 40, 4).***
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

2. Address Block Allocation sheet (sheet name: `address map`)

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
| 0x1000 | reg{n}, range(0, 40, 4) | field1  | [31:24] | 16     | RW        | 0x0        | example     |
|        |                         | rsvd1   | [23:16] | 16     | RO        | 0x0        |             |
|        |                         | field0  | [15:8]  | 16     | RW        | 0x0        |             |
|        |                         | rsvd0   | [7:0]   | 16     | RO        | 0x0        |             |
| ...... | ......                  | ......  | ......  | ...... | ......    | ......     | ......      |

  - `ADDR`

    > [!important]
    >
    > - ***When the register is named `reg{n}` with `n=range(start?, end, step?)`, only fill in the base address of `reg0`.***

    This field indicates the register’s base address offset relative to the address block.

  - `REG`

    > [!important]
    >
    > - ***Must be unique within the address block. When a register contains multiple fields, merge the corresponding cells.***
    >
    > - ***For registers wider than 32 bits, use `reg{n}, n=range(start?, end, step?)` to represent them.***

    This field specifies the register name.

  - `FIELD`

    > [!important]
    >
    > - ***Must be unique within the register.***
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
