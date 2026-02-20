use super::*;

#[test]
fn normalize_label_and_aria_use_defaults_or_trimmed_values() {
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert_eq!(
        normalize_label(Some("  Fill color  ".to_string())),
        ("Fill color".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None, "Fill color"),
        ("Fill color picker".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Accent chooser  ".to_string()), "Fill color"),
        ("Accent chooser".to_string(), true)
    );
}

#[test]
fn selected_color_sanitization_and_ids_are_stable() {
    assert_eq!(
        sanitize_selected_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_selected_color(Some("javascript:alert(1)".to_string())),
        None
    );

    let ids = resolve_ids("docs-color-picker");
    assert_eq!(ids.root_id, "docs-color-picker");
    assert_eq!(ids.trigger_id, "docs-color-picker-trigger");
    assert_eq!(ids.label_id, "docs-color-picker-label");
    assert_eq!(ids.panel_id, "docs-color-picker-panel");
    assert_eq!(ids.content_id, "docs-color-picker-content");
}

#[test]
fn resolve_state_and_class_name_track_markers() {
    let state = resolve_state(ColorPickerStateInput {
        disabled: false,
        open: true,
        has_selection: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        is_open_controlled: true,
    });

    assert_eq!(state.data_state_attr, "open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");

    let class = compose_class_name(Some("docs-custom".to_string()), state);
    assert!(class.contains("ui-color-picker"));
    assert!(class.contains("ui-color-picker--open"));
    assert!(class.contains("ui-color-picker--custom-class"));
    assert!(class.contains("docs-custom"));
}
