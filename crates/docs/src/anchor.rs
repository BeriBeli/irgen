pub(crate) fn anchor_id(parts: &[&str]) -> String {
    parts
        .iter()
        .flat_map(|part| {
            part.chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() {
                        ch.to_ascii_lowercase()
                    } else {
                        '-'
                    }
                })
                .chain(std::iter::once('-'))
        })
        .collect::<String>()
        .trim_matches('-')
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::anchor_id;

    #[test]
    fn builds_stable_ascii_anchor_ids() {
        assert_eq!(
            anchor_id(&["field", "csr", "STATUS_COMMAND", "INT.EN"]),
            "field-csr-status-command-int-en"
        );
    }
}
