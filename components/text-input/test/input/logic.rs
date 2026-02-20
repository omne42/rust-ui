use super::*;

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
