use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-text-area  ".to_string())),
        Some("docs-text-area".to_string())
    );
}

#[test]
fn resolve_label_uses_default_for_blank_values() {
    assert_eq!(
        resolve_label("  ".to_string()),
        (DEFAULT_LABEL.into(), false)
    );
    assert_eq!(
        resolve_label("  Team notes  ".to_string()),
        ("Team notes".to_string(), true)
    );
}

#[test]
fn resolve_label_with_fallback_prefers_props_then_i18n_then_default() {
    assert_eq!(
        resolve_label_with_fallback("  Summary  ".to_string(), "Localized text area"),
        ("Summary".to_string(), true)
    );
    assert_eq!(
        resolve_label_with_fallback("   ".to_string(), "  Localized text area  "),
        ("Localized text area".to_string(), false)
    );
    assert_eq!(
        resolve_label_with_fallback("   ".to_string(), "   "),
        (DEFAULT_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_sources_and_rows_markers() {
    let state = resolve_state(TextAreaStateInput {
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

    assert_eq!(state.state_attr, "readonly");
    assert_eq!(state.value_attr, "filled");
    assert_eq!(state.requirement_attr, "required");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.error_source_attr, "default");
    assert_eq!(state.placeholder_source_attr, "custom");
    assert_eq!(state.rows_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn resolve_value_axis_tracks_control_and_source_markers() {
    let state = resolve_value_axis_state(TextAreaValueAxisInput {
        is_controlled: true,
        has_default_value: true,
        has_on_value_change: false,
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.default_value_source_attr, "custom");
    assert_eq!(state.value_change_source_attr, "none");
    assert!(!state.has_value_change_handler);
}

#[test]
fn resolve_accessibility_state_uses_boolean_defaults() {
    let state = resolve_accessibility_state(TextAreaAccessibilityStateInput {
        is_disabled: None,
        is_read_only: Some(true),
    });

    assert!(!state.is_disabled);
    assert!(state.is_read_only);
}
