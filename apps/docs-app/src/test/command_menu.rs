use super::*;

#[test]
fn truncate_chars_appends_ellipsis_when_truncated() {
    assert_eq!(truncate_chars("abc", 10), "abc");
    assert_eq!(truncate_chars("abcdefghij", 10), "abcdefghij");
    assert_eq!(truncate_chars("abcdefghijk", 10), "abcdefghij…");
}

#[test]
fn find_ascii_case_insensitive_matches_substrings() {
    assert_eq!(
        find_ascii_case_insensitive("Hello Button", "button"),
        Some(6)
    );
    assert_eq!(
        find_ascii_case_insensitive("Hello Button", "BUTTON"),
        Some(6)
    );
    assert_eq!(find_ascii_case_insensitive("Hello Button", "nope"), None);
}

#[test]
fn create_snippet_returns_context_around_match() {
    let content = "Alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu";
    let snippet = match create_snippet(content, "GAMMA") {
        Some(value) => value,
        None => panic!("expected snippet"),
    };
    assert!(snippet.to_lowercase().contains("gamma"));
}

#[test]
fn create_snippet_uses_ellipsis_for_long_content() {
    let content = format!("{} match {}", "a".repeat(160), "b".repeat(160));
    let snippet = match create_snippet(&content, "match") {
        Some(value) => value,
        None => panic!("expected snippet"),
    };
    assert!(snippet.starts_with('…'));
    assert!(snippet.ends_with('…'));
    assert!(snippet.contains("match"));
}
