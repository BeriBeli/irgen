# SystemRDL Generation Support

## Status

`crates/systemrdl` provides a native SystemRDL model and serializer. It is a
typed model layer, not a string-only exporter, so future input sources can build
SystemRDL declarations directly.

The current snapsheet workflow maps through `crates/model::base`, so CLI output
is limited to the register/address-map data that the spreadsheet-derived model
can represent.

The CLI supports SystemRDL output with:

```sh
cargo run -p irgen-cli -- example_simple.xlsx --format systemrdl
```

When `-o/--output` is omitted, SystemRDL output is written in the current
directory as `<component>.rdl`. IP-XACT XSD validation remains scoped to
`--format ipxact`; `--validate` is rejected for SystemRDL output.

## Model Coverage

The public model in `crates/systemrdl/src/ast.rs` covers the core SystemRDL 2.x
register-description surface:

- package and import declarations
- enum, struct, user property, raw, and component declarations
- `addrmap`, `regfile`, `reg`, `field`, `mem`, and `signal` component kinds
- parameterized component definitions
- component instances with arrays, bit ranges, addresses, strides, resets, and
  instance property overrides
- generic property assignments for built-in or user-defined properties
- constraints
- typed expression values for identifiers, numbers, strings, booleans, enum
  references, arrays, structs, and raw expressions
- typed software and hardware access helpers for the current snapsheet mapping

The crate is split across focused modules:

- `ast.rs`: SystemRDL model types
- `convert.rs`: current `irgen_model::base` to SystemRDL model conversion
- `serialize.rs`: SystemRDL text emission
- `util.rs`: numeric/access/string helpers
- `writer.rs`: indentation writer
- `error.rs`: conversion errors

## Snapsheet Mapping

- `Component` maps to a top-level `addrmap`.
- Each address block maps to a nested `addrmap` instance at the block offset.
- `Register` maps to a `reg` instance at the register offset.
- `Register::size` maps to `regwidth` and `accesswidth`.
- `RegisterFile` maps to a `regfile` array instance with `dim` and byte stride.
  Sparse arrays are represented directly with `+=` stride; SystemRDL does not
  have an IP-XACT-style explicit range property for `addrmap` or `regfile`.
- `Field` maps to a `field` instance with a bit range and reset value.
- Field access attributes map to `sw`, `hw`, and when needed `onread` or
  `onwrite` properties.
- Descriptions are intentionally not emitted in generated SystemRDL.
- HDL backdoor paths use standard SystemRDL properties. Address-block
  instances receive a built-in `hdl_path` assignment with a SystemVerilog macro
  placeholder such as `` `REGS_HDL_PATH``. Fields receive `hdl_path_slice` with
  their configured path, since the standard `hdl_path` property is not valid on
  field components. Reserved fields do not receive `hdl_path_slice`.

## Address Ranges and Sparse Arrays

Snapsheet address-block `RANGE` values are used by the parser as validation
bounds and are contracted in the base model to the actual occupied register
span. SystemRDL output does not emit that contracted block range because
SystemRDL has no IP-XACT-style explicit `addrmap` or `regfile` range property.
Toolchains are expected to infer component size from addressable contents.

Sparse register-file arrays are emitted with the standard SystemRDL stride
syntax:

```systemrdl
regfile_0[512] @ 0x10 += 0x100;
```

This means each element starts `0x100` bytes after the previous one. If the
last element uses less than a full stride, that unused tail space is not modeled
as a separate register or block. `irgen` intentionally does not split the last
array element into a scalar instance, because that would change the regular
array model into a tool-specific workaround.

`bridge` is also not used for this case. In SystemRDL, `bridge` describes
multiple address-map views from a root map; it is not a range or overlap
escape hatch for a single CSR address space. Use `bridge` only when the design
really has multiple bus-visible views.

## Implementation Notes

- `crates/systemrdl` is a workspace member.
- `irgen_systemrdl::serialize_systemrdl(&irgen_model::base::Component)`
  converts through the SystemRDL AST before serialization.
- Serializer tests cover core SystemRDL declarations, components, constraints,
  arrays, bit ranges, register-file arrays, and base conversion.
- CLI coverage includes explicit `--format systemrdl` parsing, example export,
  and current-directory `.rdl` default output behavior.
- CI validates generated `.rdl` examples with the Python
  `systemrdl-compiler` package by running compile and elaboration.

## Current Limitations

The SystemRDL model can represent more than the current spreadsheet flow can
populate. The snapsheet-to-SystemRDL conversion does not yet populate:

- memories and signals
- user-defined properties
- parameterized component definitions
- explicit nested addrmap/subsystem composition beyond address blocks
- constraints
- external component references

Those features should be enabled in CLI output by first extending the snapsheet
schema and `irgen_model::base`, then mapping the new data into the existing
SystemRDL AST.
