# IP-XACT 2014 Compliance

## Status

The active compliance target is IEEE 1685-2014.

`crates/ipxact` is not yet a fully audited general-purpose IP-XACT library, but
the dedicated IEEE 1685-2014 surface now covers the current register-oriented
CLI path plus all eight root documents listed by the vendored 2014
`index.xsd`:

- component
- catalog
- bus definition
- abstraction definition
- abstractor
- design
- design configuration
- generator chain

IEEE 1685-2009 and IEEE 1685-2022 are P2 multi-version compliance work. They
are not blockers for the current 2014 milestone.

## HDL Path Compatibility

The generated IP-XACT 2014 and 2022 outputs carry HDL backdoor paths through standard
`accessHandles`, not a vendor extension:

```xml
<ipxact:accessHandles>
  <ipxact:accessHandle>
    <ipxact:slices>
      <ipxact:slice>
        <ipxact:pathSegments>
          <ipxact:pathSegment>
            <ipxact:pathSegmentName>...</ipxact:pathSegmentName>
          </ipxact:pathSegment>
        </ipxact:pathSegments>
      </ipxact:slice>
    </ipxact:slices>
  </ipxact:accessHandle>
</ipxact:accessHandles>
```

IP-XACT 2022 uses the same access-handle structure, with the path segment value
serialized as text content of `ipxact:pathSegment` per the 2022 schema.

The narrower IP-XACT 2009 emitter does not carry HDL backdoor paths because
that version does not provide the same standard register-model access-handle
structure.

Generated IP-XACT does not emit Synopsys `snps:*` vendor extensions. Legacy
snapsheet `SETTING` values are parsed for workbook compatibility, but they are
not serialized as vendor metadata.

## Evidence

- `crates/model` emits IP-XACT through `ip_xact::v2014`.
- The 2014 integration test suite validates representative documents against
  the vendored official Accellera IEEE 1685-2014 XSD.
- The component path covers register-oriented memory maps, address spaces,
  bus interfaces, indirect interfaces, channels, remap states, model ports and
  views, file sets, CPUs, component generators, whitebox elements, reset
  policies, assertions, parameters, choices, vendor extensions, and selected
  key/keyref constraints.
- Read-modify-write coverage exists for the main 2014 roots and key
  QName-preserving extension paths.
- `component_reset_type_ref_rejects_missing_reset_type_against_official_2014_xsd`
  proves the official XSD gate rejects a missing field reset-policy keyref.
- `bus_interface_abstraction_type_rejects_duplicate_view_refs_against_official_2014_xsd`
  proves the official XSD gate rejects duplicate abstraction `viewRef` values.

## P0

Closed for the current IEEE 1685-2014 component milestone.

Closure evidence:

- All component top-level optional structures have representative Rust surface
  coverage or have been accounted for.
- Included component schemas for bus interfaces, memory maps, model/file
  metadata, common structures, signal drivers, ports, sub-instances, and
  constraints have representative official-XSD coverage.
- Selected negative XSD gates cover key/keyref failures that the serializer
  should not silently accept.

Remaining work is P1 unless a later schema audit finds a concrete
component-surface omission. Known vendored-schema inconsistencies must be
documented, not modeled as non-schema Rust fields.

## P1

- Replace remaining placeholder or stringly typed nested structures with typed
  schema models where they are still on active 2014 paths.
- Continue narrowing expression wrapper types where the schema disallows
  extension attributes. `generatorExe` has already been narrowed; similar
  URI/string attachment points still need audit.
- Add more workbook fixtures as new parser-level failure modes are discovered.

## P2

- Expand namespace-aware serializers and official XSD validation for IEEE
  1685-2009 and IEEE 1685-2022 before claiming multi-version compliance.
- Review generated 2022 serde names; the 2022 access restriction model has a
  known non-schema `lower_bound` rename.
- Grow the official 2009 SPIRIT and 2022 fixture coverage beyond the minimal
  schema smoke tests before treating those versions as compliant.

## Verification

Latest full verification used:

```text
cargo fmt --all
cargo test -p ip-xact --test v2014_test --offline -- --nocapture
cargo test --workspace --offline --lib --tests
cargo clippy --workspace --all-targets --all-features --offline -- -D warnings
git diff --check
```

`xmllint` is required for tests that perform official XSD validation. Tests
skip those XSD checks when `xmllint` is unavailable.

## Schema Fixture Note

Official schema fixtures live under
`crates/ipxact/tests/fixtures/schemas/{1685-2009,1685-2014,1685-2022}`. The
2014 fixture is sufficient for the current CLI component validation path and
has been refreshed with the official `configurable.xsd` include. The 2009 and
2022 fixtures are present for smoke validation and future multi-version
compliance work.

## References

- Accellera IEEE 1685-2009 schemas:
  <https://www.accellera.org/XMLSchema/SPIRIT/1685-2009/>
- Accellera IEEE 1685-2014 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2014/>
- Accellera IEEE 1685-2022 schemas:
  <https://www.accellera.org/XMLSchema/IPXACT/1685-2022/>
- Accellera IP-XACT downloads:
  <https://www.accellera.org/downloads/standards/ip-xact>
