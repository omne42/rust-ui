use super::*;

#[test]
fn sanitize_percent_clamps_and_falls_back_for_invalid_numbers() {
    assert_eq!(sanitize_percent(-1.0), 0.0);
    assert_eq!(sanitize_percent(38.5), 38.5);
    assert_eq!(sanitize_percent(101.0), 100.0);
    assert_eq!(sanitize_percent(f32::NAN), DEFAULT_POSITION_PERCENT);
}

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
    let state = resolve_state(ColorThumbStateInput {
        disabled: false,
        focused: true,
        dragging: true,
        show_loupe: true,
        has_color: true,
        x_percent: 22.0,
        y_percent: 88.0,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "dragging");
    assert!(state.loupe_visible);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.x_bucket_attr, "start");
    assert_eq!(state.y_bucket_attr, "end");

    let class_name = compose_class_name(Some("docs-thumb".to_string()), state);
    assert!(class_name.contains("ui-color-thumb"));
    assert!(class_name.contains("ui-color-thumb--x-start"));
    assert!(class_name.contains("ui-color-thumb--y-end"));
    assert!(class_name.contains("ui-color-thumb--focused"));
    assert!(class_name.contains("ui-color-thumb--dragging"));
    assert!(class_name.contains("ui-color-thumb--custom-class"));
    assert!(class_name.contains("docs-thumb"));
}
