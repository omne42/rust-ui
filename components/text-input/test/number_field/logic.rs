use super::*;

#[test]
fn clamps_min_and_max() {
    assert_eq!(clamp_i64(5, Some(10), None), 10);
    assert_eq!(clamp_i64(5, None, Some(3)), 3);
    assert_eq!(clamp_i64(5, Some(0), Some(10)), 5);
}

#[test]
fn steps_with_limits() {
    assert_eq!(step_i64(0, 1, 1, Some(0), Some(2)), 1);
    assert_eq!(step_i64(2, 1, 1, Some(0), Some(2)), 2);
    assert_eq!(step_i64(2, -1, 1, Some(0), Some(2)), 1);
}

#[test]
fn parses_trimmed_numbers() {
    assert_eq!(parse_i64(" 42 "), Some(42));
    assert_eq!(parse_i64(""), None);
    assert_eq!(parse_i64("nope"), None);
}

#[test]
fn default_value_is_normalized_in_logic() {
    assert_eq!(normalize_default_value(None), 0);
    assert_eq!(normalize_default_value(Some(7)), 7);
}

#[test]
fn accessibility_state_prefers_is_prefixed_inputs() {
    let required_alias: Signal<bool> = leptos::prelude::signal(false).0.into();
    let required_prefixed: Signal<bool> = leptos::prelude::signal(true).0.into();
    let invalid_alias: Signal<bool> = leptos::prelude::signal(false).0.into();
    let invalid_prefixed: Signal<bool> = leptos::prelude::signal(true).0.into();

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        disabled: false,
        is_required: Some(required_prefixed),
        required: required_alias,
        is_invalid: Some(invalid_prefixed),
        invalid: invalid_alias,
    });

    assert!(state.is_disabled);
    assert!(state.is_required.get_untracked());
    assert!(state.is_invalid.get_untracked());
}

#[test]
fn accessibility_state_falls_back_to_alias_inputs() {
    let required: Signal<bool> = leptos::prelude::signal(true).0.into();
    let invalid: Signal<bool> = leptos::prelude::signal(false).0.into();

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        disabled: true,
        is_required: None,
        required,
        is_invalid: None,
        invalid,
    });

    assert!(state.is_disabled);
    assert!(state.is_required.get_untracked());
    assert!(!state.is_invalid.get_untracked());
}
