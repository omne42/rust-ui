use super::*;

#[test]
fn reexports_state_primitives_chart_contract() {
    let point = ChartPoint::new("q1", "Q1", 12.0);
    let points = normalize_points(vec![point]);
    let domain = value_domain(&points);
    let state = resolve_state(ChartStateInput {
        kind: ChartKind::Bar,
        point_count: points.len(),
        active_index: 0,
        disabled: false,
        show_grid: true,
        is_controlled: false,
        has_custom_class_name: false,
    });

    assert_eq!(DEFAULT_ARIA_LABEL, "Chart");
    assert_eq!(DEFAULT_ID_BASE, "ui-chart");
    assert_eq!(state.kind_attr, "bar");
    assert_eq!(state.state_attr, "ready");
    assert!(domain.max >= domain.min);
    assert_eq!(clamp_active_index(99, 5), 4);
}

#[test]
fn normalizes_input_and_derives_state_through_logic_boundaries() {
    let normalized = normalize_input_boundary(ChartInputBoundary {
        id_base: Some(" chart ".to_string()),
        class_name: Some(" demo-chart ".to_string()),
        aria_label: Some(" Revenue ".to_string()),
        points: vec![
            ChartPoint::new("a", "A", 1.0),
            ChartPoint::new("b", "B", 2.0),
        ],
        default_active_index: Some(99),
    });

    assert_eq!(normalized.id_base, "chart");
    assert_eq!(normalized.class_name.as_deref(), Some("demo-chart"));
    assert_eq!(normalized.aria_label, "Revenue");
    assert_eq!(normalized.point_count, 2);
    assert_eq!(normalized.default_active_index, 1);

    let state = derive_state_from_boundary(ChartStateBoundary {
        kind: ChartKind::Line,
        point_count: normalized.point_count,
        active_index: normalized.default_active_index,
        is_disabled: false,
        is_show_grid: true,
        is_controlled: false,
        has_custom_class_name: normalized.class_name.is_some(),
    });

    assert_eq!(state.kind_attr, "line");
    assert_eq!(state.state_attr, "ready");
    assert!(state.is_uncontrolled);
    assert_eq!(
        normalize_interaction_index(5, normalized.point_count, false),
        Some(1)
    );
    assert_eq!(
        normalize_interaction_index(0, normalized.point_count, true),
        None
    );
}
