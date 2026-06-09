# SystemRDL Generation

`irgen snapsheet --format systemrdl` emits SystemRDL from the shared snapsheet
register model.

```sh
cargo run -p irgen-cli -- snapsheet examples/example_simple.xlsx --format systemrdl
```

When `-o/--output` is omitted, output is written as `<component>.rdl`.

## Mapping

- `Component` maps to a top-level `addrmap`.
- Each address block maps to a nested `addrmap` instance.
- Registers map to `reg` instances.
- Register-file arrays map to `regfile` array instances with byte stride.
- Fields map to `field` instances with bit ranges.
- Access attributes map to `sw`, `hw`, `onread`, and `onwrite` properties.
- Non-empty reset values emit as reset properties.
- `RESET = -` does not emit reset data.
- Field HDL paths emit as `hdl_path_slice` when backdoor paths are present.
  Reserved fields do not emit HDL paths.

Snapsheet address-block `RANGE` values are validation bounds. SystemRDL output
does not emit explicit block ranges; tools infer size from addressable
contents.

## Current Limits

The SystemRDL crate can represent more than the snapsheet model currently
populates. Snapsheet output does not yet model:

- memories and signals
- user-defined properties
- parameterized components
- nested subsystems beyond address blocks
- constraints
- external component references

Add those features first to the snapsheet model, then map them into the
SystemRDL AST.

## Useful Gates

```text
cargo test -p irgen-systemrdl
cargo test -p irgen-cli
```
