use super::*;

#[test]
fn normalize_contracts_use_defaults_or_trimmed_values() {
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert_eq!(
        normalize_label(Some("  Brand color editor  ".to_string())),
        ("Brand color editor".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None, "Brand color editor"),
        ("Brand color editor".to_string(), false)
    );
    assert_eq!(
        normalize_aria_label(
            Some("  Accent color composer  ".to_string()),
            "Brand color editor"
        ),
        ("Accent color composer".to_string(), true)
    );
}

#[test]
fn sanitize_helpers_and_hsb_conversions_are_stable() {
    assert_eq!(sanitize_hue(-10.0), 350.0);
    assert_eq!(sanitize_hue(f64::NAN), DEFAULT_HUE);
    assert_eq!(sanitize_alpha(144.0), 100.0);
    assert_eq!(sanitize_alpha(f64::NAN), DEFAULT_ALPHA);
    assert_eq!(sanitize_area((1.2, -0.2)), (1.0, 0.0));

    assert_eq!(hsb_to_rgb(0.0, 100.0, 100.0), (255, 0, 0));
    assert_eq!(hsb_to_rgb(120.0, 100.0, 100.0), (0, 255, 0));
    assert_eq!(hsb_to_rgb(240.0, 100.0, 100.0), (0, 0, 255));

    let (hue, saturation, lightness) = hsb_to_hsl(220.0, 75.0, 80.0);
    assert_eq!(hue.round() as i64, 220);
    assert!(saturation > 60.0);
    assert!(lightness > 40.0);
}

#[test]
fn compose_color_and_channel_preview_follow_format_contract() {
    assert_eq!(
        compose_color_from_hsb(220.0, 80.0, 90.0, 100.0, false),
        "#2e6be6"
    );
    assert_eq!(
        compose_color_from_hsb(220.0, 80.0, 90.0, 40.0, false),
        "rgba(46, 107, 230, 0.400)"
    );

    let hex_rows = format_channel_preview(ColorEditorFormat::Hex, 220.0, 80.0, 90.0, 40.0, false);
    assert_eq!(hex_rows.first().map(|row| row.0.as_str()), Some("HEX"));
    assert_eq!(hex_rows.last().map(|row| row.0.as_str()), Some("A"));

    let rgb_rows = format_channel_preview(ColorEditorFormat::Rgb, 220.0, 80.0, 90.0, 40.0, true);
    assert_eq!(rgb_rows.len(), 3);
    assert_eq!(rgb_rows[0].0, "R");
    assert_eq!(rgb_rows[1].0, "G");
    assert_eq!(rgb_rows[2].0, "B");
}

#[test]
fn resolve_state_and_class_name_track_sources_and_format() {
    let state = resolve_state(ColorEditorStateInput {
        disabled: false,
        hide_alpha_channel: true,
        format: ColorEditorFormat::Hsl,
        has_selection: true,
        has_custom_motion: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "ready");
    assert_eq!(state.format_attr, "hsl");
    assert_eq!(state.alpha_visibility_attr, "hidden");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-color-editor".to_string()), state);
    assert!(class_name.contains("ui-color-editor"));
    assert!(class_name.contains("ui-color-editor--format-hsl"));
    assert!(class_name.contains("ui-color-editor--alpha-hidden"));
    assert!(class_name.contains("ui-color-editor--custom-class"));
    assert!(class_name.contains("docs-color-editor"));
}
