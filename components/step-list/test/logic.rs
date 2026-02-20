use super::*;

#[test]
fn normalize_selection_axis_keeps_controlled_pair_and_sanitizes_default() {
    let (selected_index, _set_selected_index) = signal(Some(1_usize));
    let callback = Callback::new(|_value: Option<usize>| {});
    let axis = normalize_selection_axis(StepListSelectionAxisInput {
        selected_index: Some(selected_index.into()),
        default_selected_index: Some(9),
        on_selected_index_change: Some(callback),
        item_count: 3,
    });

    assert!(axis.selected_index.is_some());
    assert_eq!(axis.default_selected_index, None);
    assert!(axis.on_selected_index_change.is_some());
}

#[test]
fn compose_class_name_includes_sources() {
    let state = resolve_state(StepListStateInput {
        orientation: crate::StepListOrientation::Horizontal,
        size: crate::StepListSize::M,
        emphasized: true,
        disabled: true,
        item_count: 1,
        selected_index: Some(0),
        completed_count: 0,
        disabled_count: 0,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-step-list".to_string()), state);
    assert!(class_name.contains("ui-step-list"));
    assert!(class_name.contains("ui-step-list--orientation-horizontal"));
    assert!(class_name.contains("ui-step-list--size-m"));
    assert!(class_name.contains("ui-step-list--emphasized"));
    assert!(class_name.contains("ui-step-list--disabled"));
    assert!(class_name.contains("ui-step-list--custom-class"));
    assert!(class_name.contains("docs-step-list"));
}
