# IP-XACT Generation

## Status

`irgen` emits register-oriented IP-XACT component XML from the shared
`irgen_model` register model. IP-XACT 2014 is the default CLI output version,
and the current CLI supports these schema versions:

| CLI version | Schema family | Rust module | Notes |
| --- | --- | --- | --- |
| `1.4` | SPIRIT 1.4 | `v1_4` | Uses the `spirit:` namespace. The schema has no `registerFile`, so register-file arrays are flattened into ordinary registers. |
| `1.5` | SPIRIT 1.5 | `v1_5` | Uses the `spirit:` namespace and emits register-file arrays. |
| `2009` | IEEE 1685-2009 | `v2009` | Uses the `spirit:` namespace and emits register-file arrays. |
| `2014` | IEEE 1685-2014 | `v2014` | Default output. Uses the `ipxact:` namespace and emits HDL paths through standard `accessHandles`. |
| `2022` | IEEE 1685-2022 | `v2022` | Uses the `ipxact:` namespace, 2022 array syntax, and standard `accessHandles`. |

The emitters cover the register-oriented component subset produced by
snapsheets: memory maps, address blocks, registers, register-file arrays where
the target schema supports them, fields, resets, and field access metadata.
They are not complete models for every IP-XACT root document or schema feature.

Generated IP-XACT does not emit Synopsys `snps:*` vendor extensions. Legacy
snapsheet `SETTING` values are parsed for workbook compatibility, but they are
not serialized as vendor metadata.

## Crate Layout

- `crates/model`: schema-independent register model.
- `crates/ipxact`: conversion from `irgen_model` to versioned IP-XACT XML.
- `crates/ipxact-codegen`: `xsd-parser` based generator for the versioned Rust
  schema modules.
- `crates/ipxact/schema`: third-party schema files used for generation and XSD
  validation.

The dependency direction is intentionally one-way: `ipxact` depends on
`irgen_model`, and `irgen_model` does not depend on IP-XACT schemas or XML
serialization.

## Version Behavior

SPIRIT 1.4, SPIRIT 1.5, and IEEE 1685-2009 outputs do not carry HDL backdoor
paths because those schemas do not provide the same standard register-model
`accessHandles` structure used by 2014 and 2022. RALF and SystemRDL outputs
still preserve field HDL paths independently of IP-XACT version selection.

`--format all` writes one IP-XACT file for each supported version:

```text
<component>-ipxact-1.4.xml
<component>-ipxact-1.5.xml
<component>-ipxact-2009.xml
<component>-ipxact-2014.xml
<component>-ipxact-2022.xml
```

## Validation

IP-XACT XSD validation is opt-in from the CLI:

```sh
cargo run -p irgen-cli -- example.xlsx --snapsheet-spec snapsheet.toml --ipxact-version 2022 --validate crates/ipxact/schema/1685-2022/index.xsd
```

Useful local gates:

```text
cargo fmt --all
cargo test -p ip-xact --test ipxact_xml
cargo test -p irgen-cli
cargo check --workspace
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

`xmllint` is required for tests or CLI runs that perform official XSD
validation. Tests skip XSD checks when `xmllint` is unavailable.

## Schema Sources

Official schema files live under
`crates/ipxact/schema/{1.4,1.5,1685-2009,1685-2014,1685-2022}`.

- Accellera SPIRIT 1.4 schemas:
  <https://www.accellera.org/XMLSchema/SPIRIT/1.4/>
- Accellera SPIRIT 1.5 schemas:
  <https://www.accellera.org/XMLSchema/SPIRIT/1.5/>
- Accellera IEEE 1685-2009 schemas:
  <https://www.accellera.org/XMLSchema/SPIRIT/1685-2009/>
- Accellera IEEE 1685-2014 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2014/>
- Accellera IEEE 1685-2022 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2022/>
