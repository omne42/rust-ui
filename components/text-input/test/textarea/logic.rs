use super::*;

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        is_read_only: Some(true),
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
}

#[test]
fn normalize_accessibility_state_uses_defaults_when_values_are_absent() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        is_read_only: None,
    });

    assert!(!state.is_disabled);
    assert!(!state.is_read_only);
}

#[test]
fn normalize_default_value_uses_empty_string_when_absent() {
    assert_eq!(normalize_default_value(None), String::new());
    assert_eq!(
        normalize_default_value(Some("prefilled".to_string())),
        "prefilled".to_string()
    );
}

#[test]
fn normalize_value_axis_centralizes_default_priority_and_sources() {
    let state = normalize_value_axis(ValueAxisInput {
        has_controlled_value: true,
        default_value: Some("default".to_string()),
        has_on_value_change: false,
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, ValueControlModeAttr::Controlled);
    assert_eq!(state.default_value, "default");
    assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.value_change_source_attr, ValueChangeSourceAttr::None);
    assert!(!state.has_value_change_handler);
}

#[test]
fn normalize_value_axis_tracks_on_value_change_source() {
    let state = normalize_value_axis(ValueAxisInput {
        has_controlled_value: false,
        default_value: None,
        has_on_value_change: true,
    });

    assert_eq!(state.control_mode_attr, ValueControlModeAttr::Uncontrolled);
    assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Default);
    assert_eq!(
        state.value_change_source_attr,
        ValueChangeSourceAttr::OnValueChange
    );
    assert!(state.has_value_change_handler);
}

#[test]
fn normalize_value_axis_uses_closed_enumerated_source_markers() {
    for has_controlled_value in [false, true] {
        for has_default_value in [false, true] {
            for has_on_value_change in [false, true] {
                let default_value = has_default_value.then(|| "default-value".to_string());
                let state = normalize_value_axis(ValueAxisInput {
                    has_controlled_value,
                    default_value,
                    has_on_value_change,
                });

                assert!(
                    matches!(
                        state.control_mode_attr,
                        ValueControlModeAttr::Controlled | ValueControlModeAttr::Uncontrolled
                    ),
                    "unexpected `data-value-control-mode` value: {}",
                    state.control_mode_attr.as_str()
                );
                assert!(
                    matches!(
                        state.default_value_source_attr,
                        TextareaSourceAttr::Custom | TextareaSourceAttr::Default
                    ),
                    "unexpected `data-default-value-source` value: {}",
                    state.default_value_source_attr.as_str()
                );
                assert!(
                    matches!(
                        state.value_change_source_attr,
                        ValueChangeSourceAttr::OnValueChange | ValueChangeSourceAttr::None
                    ),
                    "unexpected `data-value-change-source` value: {}",
                    state.value_change_source_attr.as_str()
                );
            }
        }
    }
}

#[test]
fn resolve_label_with_fallback_uses_default_for_blank_values() {
    assert_eq!(
        resolve_label_with_fallback(
            "  ".to_string(),
            ui_state_primitives::textarea::DEFAULT_LABEL
        ),
        (ui_state_primitives::textarea::DEFAULT_LABEL.into(), false)
    );
    assert_eq!(
        resolve_label_with_fallback(
            "  Release summary  ".to_string(),
            ui_state_primitives::textarea::DEFAULT_LABEL,
        ),
        ("Release summary".to_string(), true)
    );
}

#[test]
fn resolve_state_tracks_sources_and_rows_markers() {
    let state = resolve_state(TextareaStateInput {
        disabled: false,
        read_only: true,
        required: true,
        invalid: false,
        has_value: true,
        has_custom_label: true,
        has_custom_description: true,
        has_custom_error: false,
        has_custom_placeholder: true,
        has_custom_rows: true,
        has_custom_class_name: false,
    });

    assert_eq!(
        state.state_attr,
        ui_state_primitives::textarea::TextareaVisualStateAttr::Readonly
    );
    assert_eq!(
        state.value_attr,
        ui_state_primitives::textarea::TextareaValueAttr::Filled
    );
    assert_eq!(
        state.requirement_attr,
        ui_state_primitives::textarea::TextareaRequirementAttr::Required
    );
    assert_eq!(state.label_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.description_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.error_source_attr, TextareaSourceAttr::Default);
    assert_eq!(state.placeholder_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.rows_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.class_source_attr, TextareaSourceAttr::Default);
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(TextareaStateInput {
        disabled: true,
        read_only: false,
        required: false,
        invalid: false,
        has_value: false,
        has_custom_label: false,
        has_custom_description: false,
        has_custom_error: false,
        has_custom_placeholder: false,
        has_custom_rows: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-textarea".to_string()), state);

    for token in [
        "ui-textarea",
        "ui-textarea--state-disabled",
        "ui-textarea--value-empty",
        "ui-textarea--requirement-optional",
        "ui-textarea--custom-class",
        "docs-textarea",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
