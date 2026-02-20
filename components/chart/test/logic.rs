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
