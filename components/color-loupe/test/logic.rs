use super::*;

#[test]
fn sanitize_color_rejects_unsafe_values() {
    assert_eq!(
        sanitize_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn resolve_state_and_class_name_track_flags_and_sources() {
    let state = resolve_state(ColorLoupeStateInput {
        open: true,
        disabled: false,
        has_color: true,
        x_percent: 22.0,
        y_percent: 88.0,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(state.is_open);
    assert_eq!(state.data_state_attr, "open");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.x_bucket_attr, "start");
    assert_eq!(state.y_bucket_attr, "end");

    let class_name = compose_class_name(Some("docs-color-loupe".to_string()), state);
    assert!(class_name.contains("ui-color-loupe"));
    assert!(class_name.contains("ui-color-loupe--open"));
    assert!(class_name.contains("ui-color-loupe--custom-class"));
    assert!(class_name.contains("docs-color-loupe"));
}

#[test]
fn normalize_aria_label_uses_default_or_custom_values() {
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Accent loupe  ".to_string())),
        ("Accent loupe".to_string(), true)
    );
}
