use super::*;

#[test]
fn normalizes_digits_only() {
    assert_eq!(normalize_otp_value("a1b2c3", 6), "123");
}

#[test]
fn insert_overwrites_and_extends() {
    let (next, focus) = apply_otp_input("12", 1, "9", 6);
    assert_eq!(next, "19");
    assert_eq!(focus, Some(2));

    let (next, focus) = apply_otp_input("12", 9, "9", 6);
    assert_eq!(next, "129");
    assert_eq!(focus, Some(3));
}

#[test]
fn paste_fills_forward() {
    let (next, focus) = apply_otp_input("12", 1, "3456", 6);
    assert_eq!(next, "13456");
    assert_eq!(focus, Some(5));
}

#[test]
fn empty_input_deletes_at_index() {
    let (next, focus) = apply_otp_input("1234", 1, "", 6);
    assert_eq!(next, "134");
    assert_eq!(focus, None);
}

#[test]
fn backspace_deletes_current_or_last() {
    let (next, focus) = apply_otp_backspace("1234", 1, 6);
    assert_eq!(next, "134");
    assert_eq!(focus, 1);

    let (next, focus) = apply_otp_backspace("1234", 10, 6);
    assert_eq!(next, "123");
    assert_eq!(focus, 3);
}

#[test]
fn default_value_is_normalized_in_logic() {
    assert_eq!(normalize_default_value(None), "");
    assert_eq!(
        normalize_default_value(Some("123456".to_string())),
        "123456".to_string()
    );
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
