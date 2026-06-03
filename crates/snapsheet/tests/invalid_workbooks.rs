use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn rejects_invalid_workbook_fixtures() {
    let cases = [
        ("conflicting_registers.xlsx", "register name collides"),
        ("duplicate_fields.xlsx", "field `value` is duplicated"),
        ("overlapping_fields.xlsx", "overlaps field `high`"),
        ("invalid_attribute.xlsx", "invalid attribute: BAD"),
        ("malformed_range.xlsx", "invalid unsigned integer `nope`"),
        ("out_of_range_register.xlsx", "exceeds address block range"),
    ];

    for (filename, expected) in cases {
        let error = irgen_snapsheet::load_excel(&fixture(filename))
            .err()
            .unwrap_or_else(|| panic!("{filename} should be rejected"));
        let message = error.to_string();
        assert!(
            message.contains(expected),
            "{filename} produced `{message}`, expected `{expected}`"
        );
    }
}
