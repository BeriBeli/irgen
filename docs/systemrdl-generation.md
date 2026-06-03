# SystemRDL Generation Support

## Conclusion

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

The default output extension for `--format systemrdl` is `.rdl`. IP-XACT XSD
validation remains scoped to `--format ipxact`; `--validate` is rejected for
SystemRDL output.

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

The crate is split into focused modules:

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
- `Field` maps to a `field` instance with a bit range and reset value.
- Field access attributes map to `sw`, `hw`, and when needed `onread` or
  `onwrite` properties.
- Field descriptions map to `desc`.

## Completed

- Added the `crates/systemrdl` workspace member.
- Added a public SystemRDL AST/model with separate modules for model types,
  conversion, serialization, helpers, writer, errors, and tests.
- Added `irgen_systemrdl::serialize_systemrdl(&irgen_model::base::Component)`.
- Added serializer tests for core SystemRDL declarations, components,
  constraints, arrays, bit ranges, register-file arrays, and base conversion.
- Added `--format systemrdl` CLI support and `.rdl` default output extension.
- Added CLI coverage for explicit SystemRDL format parsing and example export.

## Current Limitations

The SystemRDL model can represent more than the current spreadsheet flow can
populate. The snapsheet-to-SystemRDL conversion does not yet populate:

- memories and signals
- user-defined properties
- parameterized component definitions
- explicit nested addrmap/subsystem composition beyond address blocks
- constraints
- external component references
- HDL path or tool-specific properties

Those features should be enabled in CLI output by first extending the snapsheet
schema and `irgen_model::base`, then mapping the new data into the existing
SystemRDL AST.
