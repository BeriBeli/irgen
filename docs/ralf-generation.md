# RALF Generation Support

## Status

`crates/ralf` provides a native RALF model and serializer. The crate now has a
public RALF AST instead of a direct string printer, so future input sources can
build full RALF constructs without going through IP-XACT or the current
spreadsheet-only base model.

The current snapsheet workflow still maps through `crates/model::base`, so CLI
output is limited to the constructs that the spreadsheet-derived model can
represent. The RALF crate itself models a broader language surface.

The CLI supports RALF output with:

```sh
cargo run -p irgen-cli -- snapsheet example_simple.xlsx --format ralf
```

When `-o/--output` is omitted, RALF output is written in the current directory
as `<component>.ralf`. IP-XACT XSD validation remains scoped to
`--format ipxact`; `--validate` is rejected for RALF output.

## Model Coverage

The public model in `crates/ralf/src/ast.rs` covers the core RALF construct
summary:

- top-level `source` and raw passthrough items
- standalone `field`, `register`, `regfile`, `memory`, `virtual register`,
  `block`, and `system` definitions
- single-domain and multi-domain `block` / `system` definitions
- register, regfile, memory, virtual-register, block, and system instances
- count and range arrays
- HDL path strings on instances that support them
- address offsets and array increments
- attributes and docs
- access modes, endian modes, memory initial values, register noise, shared
  declarations, cover directives, constraints, field enums, coverpoints, and
  register cross coverage
- `user_code lang=... [(scope)]` on RALF classes that support user code
- `register_cb` and `field_cb_class` callback class definitions
- `add_reg_cb` attachments, including external callback class references
- single-domain block `default_map_name`

The crate is split across focused modules:

- `ast.rs`: RALF model types
- `convert.rs`: current `irgen_model::base` to RALF model conversion
- `serialize.rs`: RALF text emission
- `util.rs`: numeric/access/doc helpers
- `writer.rs`: indentation writer
- `error.rs`: conversion errors

## Snapsheet Mapping

- `Component` with one address block emits one or more standalone `block`
  definitions.
- `Component` with multiple address blocks also emits a top-level `system`
  that instantiates each generated block at the address-block offset.
- `Block` maps to `block <name> { ... }`.
- `Block::size` maps from bit width to RALF `bytes`.
- `Register` maps to `register <name> @<offset> { ... }`.
- `Register::size` maps from bit width to RALF `bytes`.
- `RegisterFile` maps to `regfile <name>[<dim>] @<offset> +<range> { ... }`.
  The `range` value is treated as the byte stride between register-file array
  elements, matching the snapsheet array semantics.
- `Field` maps to `field <name> @<bitOffset> { ... }`.
- `Field::hdl_path` maps to the RALF field instance HDL path, emitted as
  `field <name> (<hdl_path>) @<bitOffset> { ... }`. Reserved fields do not emit
  HDL paths.
- `Field::width` maps to `bits`.
- `Field::attr` maps to RALF access mnemonics such as `rw`, `ro`, `w1c`,
  `wsrc`, and `wo1`.
- `Field::reset` maps to `hard_reset` when non-empty.
- `Field::desc` maps to a compact `doc { ... }` entry when non-empty.

## Numeric Formatting

Decimal literals are emitted as decimal values. `0x`-prefixed literals are
emitted as RALF hexadecimal literals using the `'h...` form. Register and block
sizes must be byte-aligned because RALF `bytes` is the natural representation
for those widths.

## Implementation Notes

- `crates/ralf` is a workspace member.
- `irgen_ralf::serialize_ralf(&irgen_model::base::Component)` converts through
  the RALF AST before serialization.
- Serializer tests cover simple registers, fields, resets, access mapping,
  register-file arrays, memories, virtual registers, systems, domains,
  attributes, enums, constraints, cover directives, and cross coverage.
- Typed model coverage includes `user_code`, callback class definitions,
  `add_reg_cb`, external callback references, and `default_map_name`.
- CLI coverage includes explicit `--format ralf` parsing and current-directory
  `.ralf` default output behavior.

## Current Limitations

The RALF model can represent more than the current spreadsheet flow can
populate. The snapsheet-to-RALF conversion still does not populate:

- multiple physical-interface domains
- memories and virtual registers
- nested sub-block or subsystem composition
- volatile flags
- soft reset types
- enum values
- coverage directives
- constraints
- callbacks and user code

Those features should be enabled in CLI output by first extending the snapsheet
schema and `irgen_model::base`, then mapping the new data into the existing
RALF AST.
