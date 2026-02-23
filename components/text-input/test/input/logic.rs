use super::*;
use leptos::prelude::{Signal, signal};

fn base_state() -> InputLogicState {
    InputLogicState {
        is_disabled: false,
        is_read_only: false,
        is_invalid: false,
        is_empty: true,
        is_focused: false,
    }
}

#[test]
fn clear_button_requires_non_empty_and_not_disabled() {
    let mut state = base_state();
    state.is_empty = false;
    assert!(resolve_view_state(None, None, None, false, false, true, state).show_clear);

    state.is_disabled = true;
    assert!(!resolve_view_state(None, None, None, false, false, true, state).show_clear);
}

#[test]
fn label_hidden_does_not_remove_accessible_label() {
    let state = base_state();
    let view = resolve_view_state(Some("Label"), None, None, false, false, false, state);
    assert!(view.show_label);
}

#[test]
fn error_requires_invalid_and_non_empty_error_text() {
    let mut state = base_state();
    state.is_invalid = true;
    let view = resolve_view_state(None, None, Some(" "), false, false, false, state);
    assert!(!view.show_error);
    let view = resolve_view_state(None, None, Some("Bad"), false, false, false, state);
    assert!(view.show_error);
}

#[test]
fn clear_aria_label_uses_trimmed_value_or_default() {
    assert_eq!(
        resolve_clear_aria_label(Some("  Clear name  ".to_string())),
        "Clear name"
    );
    assert_eq!(
        resolve_clear_aria_label(Some("  ".to_string())),
        ui_state_primitives::input::DEFAULT_CLEAR_ARIA_LABEL
    );
    assert_eq!(
        resolve_clear_aria_label(None),
        ui_state_primitives::input::DEFAULT_CLEAR_ARIA_LABEL
    );
}

#[test]
fn default_value_is_normalized_in_logic() {
    assert_eq!(normalize_default_value(None), "");
    assert_eq!(
        normalize_default_value(Some("seed".to_string())),
        "seed".to_string()
    );
}

#[test]
fn accessibility_state_prefers_is_prefixed_inputs() {
    let required_alias: Signal<bool> = signal(false).0.into();
    let required_prefixed: Signal<bool> = signal(true).0.into();
    let invalid_alias: Signal<bool> = signal(false).0.into();
    let invalid_prefixed: Signal<bool> = signal(true).0.into();

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        disabled: false,
        is_read_only: Some(true),
        read_only: false,
        is_required: Some(required_prefixed),
        required: required_alias,
        is_invalid: Some(invalid_prefixed),
        invalid: invalid_alias,
        is_label_hidden: Some(true),
        label_hidden: false,
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
    assert!(state.is_required.get_untracked());
    assert!(state.is_invalid.get_untracked());
    assert!(state.is_label_hidden);
}

#[test]
fn accessibility_state_falls_back_to_alias_inputs() {
    let required: Signal<bool> = signal(true).0.into();
    let invalid: Signal<bool> = signal(false).0.into();

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        disabled: true,
        is_read_only: None,
        read_only: true,
        is_required: None,
        required,
        is_invalid: None,
        invalid,
        is_label_hidden: None,
        label_hidden: true,
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
    assert!(state.is_required.get_untracked());
    assert!(!state.is_invalid.get_untracked());
    assert!(state.is_label_hidden);
}

#[test]
fn input_type_is_normalized_in_logic() {
    let default_type = normalize_input_type(None);
    assert_eq!(default_type.input_type, InputType::Text);
    assert_eq!(default_type.type_source_attr, "default");

    let email_type = normalize_input_type(Some("email"));
    assert_eq!(email_type.input_type, InputType::Email);
    assert_eq!(email_type.type_source_attr, "custom");

    let custom_type = normalize_input_type(Some("datetime-local"));
    assert_eq!(custom_type.input_type, InputType::Custom("datetime-local"));
    assert_eq!(custom_type.type_source_attr, "custom");
}
