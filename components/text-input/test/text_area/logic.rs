use super::*;

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
    let (preferred_required, _set_preferred_required) = signal(true);
    let (preferred_invalid, _set_preferred_invalid) = signal(true);

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        is_read_only: Some(true),
        is_required: Some(preferred_required.into()),
        is_invalid: Some(preferred_invalid.into()),
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
    assert!(state.is_required.get_untracked());
    assert!(state.is_invalid.get_untracked());
}

#[test]
fn normalize_accessibility_state_uses_defaults_when_values_are_absent() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        is_read_only: None,
        is_required: None,
        is_invalid: None,
    });

    assert!(!state.is_disabled);
    assert!(!state.is_read_only);
    assert!(!state.is_required.get_untracked());
    assert!(!state.is_invalid.get_untracked());
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
fn normalize_on_value_change_handler_prefers_on_value_change() {
    let (from_on_value_change, set_from_on_value_change) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_from_on_value_change.set(next));

    let handler =
        normalize_on_value_change_handler(Some(on_value_change)).expect("handler should exist");

    handler.run("new-value".to_string());

    assert_eq!(
        from_on_value_change.get_untracked(),
        "new-value",
        "on_value_change should be forwarded as-is"
    );
}

#[test]
fn normalize_on_value_change_handler_allows_absence() {
    assert!(normalize_on_value_change_handler(None).is_none());
}

#[test]
fn normalize_value_axis_centralizes_default_priority_and_sources() {
    let (value, _set_value) = signal("controlled".to_string());
    let state = normalize_value_axis(ValueAxisInput {
        value: Some(value.into()),
        default_value: Some("default".to_string()),
        on_value_change: None,
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.default_value, "default");
    assert_eq!(state.default_value_source_attr, "custom");
    assert_eq!(state.value_change_source_attr, "none");
    assert!(!state.has_value_change_handler);
}

#[test]
fn normalize_value_axis_tracks_on_value_change_source() {
    let (on_value_change_value, set_on_value_change_value) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_on_value_change_value.set(next));
    let state = normalize_value_axis(ValueAxisInput {
        value: None,
        default_value: None,
        on_value_change: Some(on_value_change),
    });

    let handler = state
        .on_value_change
        .expect("value axis should keep normalized callback");
    handler.run("prioritized".to_string());

    assert_eq!(state.control_mode_attr, "uncontrolled");
    assert_eq!(state.default_value_source_attr, "default");
    assert_eq!(state.value_change_source_attr, "on_value_change");
    assert!(state.has_value_change_handler);
    assert_eq!(on_value_change_value.get_untracked(), "prioritized");
}

#[test]
fn resolve_props_uses_fallback_and_normalizes_optional_inputs() {
    let resolved = resolve_props(ResolvedTextAreaPropsInput {
        label: "  ".to_string(),
        fallback_label: "Localized Text area".to_string(),
        description: Some("  desc  ".to_string()),
        error: Some("  ".to_string()),
        placeholder: Some("  hint  ".to_string()),
        rows: Some(0),
        class_name: Some("  docs-text-area  ".to_string()),
    });

    assert_eq!(resolved.label, "Localized Text area");
    assert!(!resolved.has_custom_label);
    assert_eq!(resolved.description.as_deref(), Some("desc"));
    assert!(resolved.has_custom_description);
    assert_eq!(resolved.error, None);
    assert!(!resolved.has_custom_error);
    assert_eq!(resolved.placeholder.as_deref(), Some("hint"));
    assert!(resolved.has_custom_placeholder);
    assert_eq!(resolved.rows, None);
    assert!(!resolved.has_custom_rows);
    assert_eq!(resolved.class_name.as_deref(), Some("docs-text-area"));
    assert!(resolved.has_custom_class_name);
}

#[test]
fn resolve_label_with_fallback_uses_default_for_blank_values() {
    assert_eq!(
        resolve_label_with_fallback(
            "  ".to_string(),
            ui_state_primitives::text_area::DEFAULT_LABEL
        ),
        (ui_state_primitives::text_area::DEFAULT_LABEL.into(), false)
    );
    assert_eq!(
        resolve_label_with_fallback(
            "  Team notes  ".to_string(),
            ui_state_primitives::text_area::DEFAULT_LABEL,
        ),
        ("Team notes".to_string(), true)
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
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(TextAreaStateInput {
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

    let class_name = compose_class_name(Some("docs-text-area".to_string()), state);

    for token in [
        "ui-text-area",
        "ui-text-area--state-disabled",
        "ui-text-area--value-empty",
        "ui-text-area--requirement-optional",
        "ui-text-area--custom-class",
        "docs-text-area",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn default_label_is_sourced_from_state_primitives() {
    assert_eq!(ui_state_primitives::text_area::DEFAULT_LABEL, "Text area");
}
