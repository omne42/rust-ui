use super::*;

#[test]
fn resolve_label_uses_default_for_blank_values() {
    assert_eq!(
        resolve_label("  ".to_string()),
        (DEFAULT_LABEL.into(), false)
    );
    assert_eq!(
        resolve_label("  Release summary  ".to_string()),
        ("Release summary".to_string(), true)
    );
}

#[test]
fn resolve_label_with_fallback_prefers_props_then_i18n_then_default() {
    assert_eq!(
        resolve_label_with_fallback("  Summary  ".to_string(), "Localized Textarea"),
        ("Summary".to_string(), true)
    );
    assert_eq!(
        resolve_label_with_fallback("   ".to_string(), "  Localized Textarea  "),
        ("Localized Textarea".to_string(), false)
    );
    assert_eq!(
        resolve_label_with_fallback("   ".to_string(), "   "),
        (DEFAULT_LABEL.into(), false)
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

    assert_eq!(state.state_attr, TextareaVisualStateAttr::Readonly);
    assert_eq!(state.value_attr, TextareaValueAttr::Filled);
    assert_eq!(state.requirement_attr, TextareaRequirementAttr::Required);
    assert_eq!(state.label_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.description_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.error_source_attr, TextareaSourceAttr::Default);
    assert_eq!(state.placeholder_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.rows_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(state.class_source_attr, TextareaSourceAttr::Default);
}

#[test]
fn resolve_state_uses_closed_enumerated_marker_values() {
    for disabled in [false, true] {
        for read_only in [false, true] {
            for required in [false, true] {
                for invalid in [false, true] {
                    for has_value in [false, true] {
                        for has_custom_label in [false, true] {
                            for has_custom_description in [false, true] {
                                for has_custom_error in [false, true] {
                                    for has_custom_placeholder in [false, true] {
                                        for has_custom_rows in [false, true] {
                                            for has_custom_class_name in [false, true] {
                                                let state = resolve_state(TextareaStateInput {
                                                    disabled,
                                                    read_only,
                                                    required,
                                                    invalid,
                                                    has_value,
                                                    has_custom_label,
                                                    has_custom_description,
                                                    has_custom_error,
                                                    has_custom_placeholder,
                                                    has_custom_rows,
                                                    has_custom_class_name,
                                                });

                                                assert!(
                                                    matches!(
                                                        state.state_attr,
                                                        TextareaVisualStateAttr::Disabled
                                                            | TextareaVisualStateAttr::Invalid
                                                            | TextareaVisualStateAttr::Readonly
                                                            | TextareaVisualStateAttr::Ready
                                                    ),
                                                    "unexpected `data-state` value: {}",
                                                    state.state_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.value_attr,
                                                        TextareaValueAttr::Filled
                                                            | TextareaValueAttr::Empty
                                                    ),
                                                    "unexpected `data-value` value: {}",
                                                    state.value_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.requirement_attr,
                                                        TextareaRequirementAttr::Required
                                                            | TextareaRequirementAttr::Optional
                                                    ),
                                                    "unexpected `data-requirement` value: {}",
                                                    state.requirement_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.label_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-label-source` value: {}",
                                                    state.label_source_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.description_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-description-source` value: {}",
                                                    state.description_source_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.error_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-error-source` value: {}",
                                                    state.error_source_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.placeholder_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-placeholder-source` value: {}",
                                                    state.placeholder_source_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.rows_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-rows-source` value: {}",
                                                    state.rows_source_attr.as_str()
                                                );
                                                assert!(
                                                    matches!(
                                                        state.class_source_attr,
                                                        TextareaSourceAttr::Custom
                                                            | TextareaSourceAttr::Default
                                                    ),
                                                    "unexpected `data-class-source` value: {}",
                                                    state.class_source_attr.as_str()
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn resolve_value_axis_tracks_control_and_source_markers() {
    let state = resolve_value_axis_state(TextareaValueAxisInput {
        is_controlled: true,
        has_default_value: true,
        has_on_value_change: false,
    });

    assert!(state.is_controlled);
    assert_eq!(
        state.control_mode_attr,
        TextareaValueControlModeAttr::Controlled
    );
    assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Custom);
    assert_eq!(
        state.value_change_source_attr,
        TextareaValueChangeSourceAttr::None
    );
    assert!(!state.has_value_change_handler);
}

#[test]
fn resolve_accessibility_state_uses_boolean_defaults() {
    let state = resolve_accessibility_state(TextareaAccessibilityStateInput {
        is_disabled: None,
        is_read_only: Some(true),
    });

    assert!(!state.is_disabled);
    assert!(state.is_read_only);
}
