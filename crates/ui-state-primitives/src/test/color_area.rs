use super::*;

#[test]
fn normalize_and_sanitize_helpers_work() {
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
    assert_eq!(
        normalize_label(Some("  Saturation/Lightness  ".to_string())),
        ("Saturation/Lightness".to_string(), true)
    );

    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Fill area  ".to_string())),
        ("Fill area".to_string(), true)
    );
    assert_eq!(
        normalize_x_axis_label(None),
        (DEFAULT_X_AXIS_LABEL.into(), false)
    );
    assert_eq!(
        normalize_y_axis_label(None),
        (DEFAULT_Y_AXIS_LABEL.into(), false)
    );

    assert_eq!(sanitize_step(0.0), 0.01);
    assert_eq!(sanitize_step(9.0), 1.0);
    assert_eq!(sanitize_grid_size(1), 3);
    assert_eq!(sanitize_grid_size(99), 31);
    assert_eq!(clamp_value((1.2, -0.2)), (1.0, 0.0));

    assert_eq!(
        sanitize_preview_color(Some("#09f".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_preview_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn cell_mapping_and_axis_parse_are_stable() {
    let (x, y) = value_from_cell(5, 5, 11);
    assert!((x - 0.5).abs() < 0.0001);
    assert!((y - 0.5).abs() < 0.0001);

    assert_eq!(parse_axis_percent("75"), Some(0.75));
    assert_eq!(parse_axis_percent("-5"), Some(0.0));
    assert_eq!(parse_axis_percent("foo"), None);

    let moved = move_value_by_delta((0.5, 0.5), 1.0, -1.0, 0.1);
    assert!((moved.0 - 0.6).abs() < 0.0001);
    assert!((moved.1 - 0.4).abs() < 0.0001);
}

#[test]
fn resolve_state_tracks_sources_and_markers() {
    let state = resolve_state(ColorAreaStateInput {
        disabled: false,
        step: 0.1,
        value: (0.35, 0.8),
        grid_size: 11,
        has_preview_color: true,
        has_custom_label: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
        has_custom_x_axis_label: false,
        has_custom_y_axis_label: true,
    });

    assert_eq!(state.data_state_attr, ColorAreaDataStateAttr::Active);
    assert_eq!(state.value_x_percent, 35);
    assert_eq!(state.value_y_percent, 80);
    assert_eq!(state.selected_col, 4);
    assert_eq!(state.selected_row, 2);
    assert_eq!(state.label_source_attr, ColorAreaSourceAttr::Custom);
    assert_eq!(state.aria_source_attr, ColorAreaSourceAttr::Default);
    assert_eq!(state.class_source_attr, ColorAreaSourceAttr::Custom);
    assert_eq!(state.x_axis_source_attr, ColorAreaSourceAttr::Default);
    assert_eq!(state.y_axis_source_attr, ColorAreaSourceAttr::Custom);
}
