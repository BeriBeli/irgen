# RALF Generation

`irgen snapsheet --format ralf` emits RALF from the shared snapsheet register
model.

```sh
cargo run -p irgen-cli -- snapsheet examples/example_simple.xlsx --format ralf
```

When `-o/--output` is omitted, output is written as `<component>.ralf`.

## Mapping

- `Component` with one block emits standalone `block` definitions.
- `Component` with multiple blocks also emits a top-level `system`.
- Address blocks map to RALF `block`.
- Registers map to `register <name> @<offset>`.
- Register-file arrays map to `regfile <name>[<dim>] @<offset> +<stride>`.
- Fields map to `field <name> @<bitOffset>`.
- Field access attributes map to RALF access mnemonics such as `rw`, `ro`,
  `w1c`, `wsrc`, and `wo1`.
- Non-empty reset values map to `hard_reset`.
- `RESET = -` does not emit reset data.
- Optional field HDL paths emit as RALF field instance paths. Reserved fields
  do not emit paths.

Register and block sizes are emitted in bytes and must be byte-aligned.

## Current Limits

The RALF crate has a richer AST than the snapsheet model can currently
populate. Snapsheet output does not yet model:

- multiple physical-interface domains
- memories and virtual registers
- nested subsystems
- enum values
- coverage directives
- constraints
- callbacks and user code

Add those features first to the snapsheet model, then map them into the RALF
AST.

## Useful Gates

```text
cargo test -p irgen-ralf
cargo test -p irgen-cli
```
