use super::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Revenue Chart  ".to_string())),
        Some("Revenue Chart".to_string())
    );

    assert_eq!(normalize_id_base(None), DEFAULT_ID_BASE);
    assert_eq!(
        normalize_id_base(Some(" docs-chart ".to_string())),
        "docs-chart"
    );

    assert_eq!(normalize_aria_label(None), DEFAULT_ARIA_LABEL);
    assert_eq!(
        normalize_aria_label(Some("  Monthly revenue  ".to_string())),
        "Monthly revenue"
    );
}

#[test]
fn normalize_points_sanitizes_identifiers_labels_and_values() {
    let points = normalize_points(vec![
        ChartPoint::new("  q1  ", "  Q1  ", 120.0),
        ChartPoint::new("", "", f64::NAN),
    ]);

    assert_eq!(points[0].id, "q1");
    assert_eq!(points[0].label, "Q1");

    assert_eq!(points[1].id, "point-1");
    assert_eq!(points[1].label, "Point 2");
    assert_eq!(points[1].value, 0.0);
}

#[test]
fn value_domain_handles_empty_and_flat_series() {
    assert_eq!(value_domain(&[]), ChartDomain { min: 0.0, max: 1.0 });

    let flat = vec![
        ChartPoint::new("a", "A", 42.0),
        ChartPoint::new("b", "B", 42.0),
    ];
    let domain = value_domain(&flat);
    assert!(domain.max > domain.min);
}

#[test]
fn resolve_state_and_class_name_surface_markers() {
    let state = resolve_state(ChartStateInput {
        kind: ChartKind::Line,
        point_count: 5,
        active_index: 99,
        disabled: false,
        show_grid: true,
        is_controlled: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.kind_attr, "line");
    assert_eq!(state.active_index, 4);
    assert_eq!(state.state_attr, "ready");
    assert_eq!(state.class_source_attr, "custom");

    let class_name = compose_class_name(Some("docs-chart-custom".to_string()), state);
    for token in [
        "ui-chart",
        "ui-chart--line",
        "ui-chart--grid",
        "ui-chart--controlled",
        "ui-chart--custom-class",
        "docs-chart-custom",
    ] {
        assert!(
            class_name.contains(token),
            "class name should include `{token}`"
        );
    }
}

#[test]
fn geometry_helpers_stay_in_expected_ranges() {
    assert_eq!(point_x(0, 0), 50.0);
    assert_eq!(point_x(0, 1), 50.0);
    assert!((point_x(0, 5) - 8.0).abs() < f64::EPSILON);
    assert!((point_x(4, 5) - 92.0).abs() < 1e-6);

    let domain = ChartDomain {
        min: 10.0,
        max: 20.0,
    };
    assert!((point_y(10.0, domain) - 52.0).abs() < 1e-6);
    assert!((point_y(20.0, domain) - 8.0).abs() < 1e-6);
    assert!((bar_width(1) - 12.0).abs() < 1e-6);
    assert!((bar_width(100) - 4.0).abs() < 1e-6);
}

#[test]
fn polyline_points_and_keyboard_navigation_are_stable() {
    let points = vec![
        ChartPoint::new("a", "A", 10.0),
        ChartPoint::new("b", "B", 20.0),
        ChartPoint::new("c", "C", 15.0),
    ];
    let polyline = polyline_points(
        &points,
        ChartDomain {
            min: 10.0,
            max: 20.0,
        },
    );
    assert!(polyline.contains(','));
    assert_eq!(polyline.split(' ').count(), 3);

    assert_eq!(next_index_for_key("ArrowRight", 0, 3), Some(1));
    assert_eq!(next_index_for_key("ArrowLeft", 0, 3), Some(0));
    assert_eq!(next_index_for_key("End", 0, 3), Some(2));
    assert_eq!(next_index_for_key("Space", 1, 3), None);
    assert_eq!(next_index_for_key("ArrowRight", 0, 0), None);
}
