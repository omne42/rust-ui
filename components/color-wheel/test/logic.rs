use super::*;

#[test]
fn normalize_helpers_use_defaults_and_trim_custom_values() {
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert_eq!(
        normalize_label(Some("  Accent hue  ".to_string())),
        ("Accent hue".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None, "Hue"),
        ("Hue wheel".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Brand wheel  ".to_string()), "Hue"),
        ("Brand wheel".to_string(), true)
    );
}

#[test]
fn sanitize_helpers_wrap_and_snap_values() {
    assert_eq!(sanitize_step(0.0), DEFAULT_STEP);
    assert_eq!(sanitize_step(120.0), 90.0);

    assert_eq!(normalize_angle(-1.0), 359.0);
    assert_eq!(normalize_angle(361.0), 1.0);

    assert_eq!(sanitize_value(370.0, 1.0), 10.0);
    assert_eq!(sanitize_value(-15.0, 1.0), 345.0);
    assert_eq!(sanitize_value(14.0, 5.0), 15.0);

    assert_eq!(parse_value(" 42.5 "), Some(42.5));
    assert_eq!(parse_value(""), None);
    assert_eq!(page_step(1.0), 15.0);
    assert_eq!(move_value_by_delta(355.0, 10.0, 1.0), 5.0);
}

#[test]
fn pointer_conversion_and_percent_are_stable() {
    let top = pointer_to_hue_angle(50.0, 0.0, 0.0, 0.0, 100.0, 100.0);
    let right = pointer_to_hue_angle(100.0, 50.0, 0.0, 0.0, 100.0, 100.0);
    let bottom = pointer_to_hue_angle(50.0, 100.0, 0.0, 0.0, 100.0, 100.0);
    let left = pointer_to_hue_angle(0.0, 50.0, 0.0, 0.0, 100.0, 100.0);

    assert_eq!(top.round(), 0.0);
    assert_eq!(right.round(), 90.0);
    assert_eq!(bottom.round(), 180.0);
    assert_eq!(left.round(), 270.0);

    assert_eq!(resolve_percent(180.0), 50.0);
    assert_eq!(format_value_text(123.6), "124°");
}

#[test]
fn resolve_state_and_class_name_track_markers() {
    let state = resolve_state(ColorWheelStateInput {
        disabled: false,
        value: 95.0,
        step: 1.0,
        show_value_label: true,
        has_custom_motion: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "active");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");

    let class = compose_class_name(Some("docs-wheel".to_string()), state);
    assert!(class.contains("ui-color-wheel"));
    assert!(class.contains("ui-color-wheel--motion-custom"));
    assert!(class.contains("ui-color-wheel--custom-class"));
    assert!(class.contains("docs-wheel"));
}
