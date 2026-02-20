use super::*;

#[test]
fn resolve_state_shows_clear_button_only_for_editable_non_empty_values() {
    assert_eq!(
        resolve_state(SearchFieldStateInput {
            is_disabled: false,
            is_read_only: false,
            has_value: false,
        }),
        SearchFieldState {
            show_clear_button: false,
        }
    );

    assert_eq!(
        resolve_state(SearchFieldStateInput {
            is_disabled: false,
            is_read_only: false,
            has_value: true,
        }),
        SearchFieldState {
            show_clear_button: true,
        }
    );

    assert_eq!(
        resolve_state(SearchFieldStateInput {
            is_disabled: true,
            is_read_only: false,
            has_value: true,
        }),
        SearchFieldState {
            show_clear_button: false,
        }
    );

    assert_eq!(
        resolve_state(SearchFieldStateInput {
            is_disabled: false,
            is_read_only: true,
            has_value: true,
        }),
        SearchFieldState {
            show_clear_button: false,
        }
    );
}

#[test]
fn resolve_value_axis_tracks_control_and_source_markers() {
    let state = resolve_value_axis_state(SearchFieldValueAxisInput {
        is_controlled: true,
        has_default_value: true,
        has_on_value_change: true,
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.default_value_source_attr, "custom");
    assert_eq!(state.value_change_source_attr, "on_value_change");
    assert!(state.has_value_change_handler);
}

#[test]
fn resolve_value_axis_without_change_callback_marks_none_source() {
    let state = resolve_value_axis_state(SearchFieldValueAxisInput {
        is_controlled: false,
        has_default_value: false,
        has_on_value_change: false,
    });

    assert!(!state.is_controlled);
    assert_eq!(state.control_mode_attr, "uncontrolled");
    assert_eq!(state.default_value_source_attr, "default");
    assert_eq!(state.value_change_source_attr, "none");
    assert!(!state.has_value_change_handler);
}

#[test]
fn resolve_value_axis_handles_read_only_callbacks() {
    let state = resolve_value_axis_state(SearchFieldValueAxisInput {
        is_controlled: false,
        has_default_value: false,
        has_on_value_change: false,
    });

    assert_eq!(state.value_change_source_attr, "none");
    assert!(!state.has_value_change_handler);
}

#[test]
fn resolve_semantic_state_yields_closed_attr_sets() {
    let ready = resolve_semantic_state(SearchFieldSemanticStateInput {
        is_disabled: false,
        is_invalid: false,
        is_read_only: false,
        is_required: false,
        has_value: false,
    });
    assert_eq!(ready.state_attr, "ready");
    assert_eq!(ready.value_attr, "empty");
    assert_eq!(ready.requirement_attr, "optional");

    let invalid = resolve_semantic_state(SearchFieldSemanticStateInput {
        is_disabled: false,
        is_invalid: true,
        is_read_only: true,
        is_required: true,
        has_value: true,
    });
    assert_eq!(invalid.state_attr, "invalid");
    assert_eq!(invalid.value_attr, "filled");
    assert_eq!(invalid.requirement_attr, "required");

    let disabled = resolve_semantic_state(SearchFieldSemanticStateInput {
        is_disabled: true,
        is_invalid: true,
        is_read_only: true,
        is_required: true,
        has_value: true,
    });
    assert_eq!(disabled.state_attr, "disabled");
}
