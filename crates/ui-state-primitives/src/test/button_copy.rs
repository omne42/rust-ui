use super::*;

#[test]
fn resolve_state_reports_copyable_when_enabled_and_text_exists() {
    let state = resolve_state(ButtonCopyStateInput {
        text: "copy me",
        is_disabled: false,
    });

    assert!(state.is_copyable);
    assert!(state.has_text);
    assert!(state.is_enabled);
    assert_eq!(state.state_attr, "copyable");
}

#[test]
fn resolve_state_reports_empty_when_text_is_blank() {
    let state = resolve_state(ButtonCopyStateInput {
        text: "   ",
        is_disabled: false,
    });

    assert!(!state.is_copyable);
    assert!(!state.has_text);
    assert!(state.is_enabled);
    assert_eq!(state.state_attr, "empty");
}

#[test]
fn resolve_state_reports_disabled_when_disabled_even_with_text() {
    let state = resolve_state(ButtonCopyStateInput {
        text: "copy me",
        is_disabled: true,
    });

    assert!(!state.is_copyable);
    assert!(state.has_text);
    assert!(!state.is_enabled);
    assert_eq!(state.state_attr, "disabled");
}

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  copied  ".to_string())),
        Some("copied".to_string())
    );
}
