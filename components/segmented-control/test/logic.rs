use super::*;
use leptos::prelude::*;

#[test]
fn orientation_class_names_are_stable() {
    assert_eq!(
        SegmentedControlOrientation::Horizontal.class_name(),
        "ui-segmented-control--horizontal"
    );
    assert_eq!(
        SegmentedControlOrientation::Vertical.class_name(),
        "ui-segmented-control--vertical"
    );
}

#[test]
fn aria_and_data_orientation_values_are_stable() {
    assert_eq!(
        SegmentedControlOrientation::Horizontal.aria_orientation(),
        "horizontal"
    );
    assert_eq!(
        SegmentedControlOrientation::Vertical.aria_orientation(),
        "vertical"
    );
    assert_eq!(
        SegmentedControlOrientation::Horizontal.data_orientation(),
        "horizontal"
    );
    assert_eq!(
        SegmentedControlOrientation::Vertical.data_orientation(),
        "vertical"
    );
}

#[test]
fn orientation_axis_flags_are_stable() {
    assert!(!SegmentedControlOrientation::Horizontal.is_vertical());
    assert!(SegmentedControlOrientation::Vertical.is_vertical());
}

#[test]
fn size_class_names_are_stable() {
    assert_eq!(
        SegmentedControlSize::Default.class_name(),
        "ui-segmented-control--size-default"
    );
    assert_eq!(
        SegmentedControlSize::Sm.class_name(),
        "ui-segmented-control--size-sm"
    );
    assert_eq!(
        SegmentedControlSize::Lg.class_name(),
        "ui-segmented-control--size-lg"
    );
}

#[test]
fn control_mode_attr_is_stable() {
    assert_eq!(
        SegmentedControlControlMode::Controlled.as_attr(),
        "controlled"
    );
    assert_eq!(
        SegmentedControlControlMode::Uncontrolled.as_attr(),
        "uncontrolled"
    );
}

#[test]
fn selection_source_attr_is_closed_set() {
    assert_eq!(
        SegmentedControlSelectionSource::None.as_attr(),
        "external-none"
    );
    assert_eq!(
        SegmentedControlSelectionSource::Selected.as_attr(),
        "external-selected"
    );
    assert_eq!(
        SegmentedControlSelectionSource::OutOfRange.as_attr(),
        "external-out-of-range"
    );
}

#[test]
fn selection_source_resolves_from_raw_and_normalized_selection() {
    assert_eq!(
        SegmentedControlSelectionSource::from_indices(None, None),
        SegmentedControlSelectionSource::None
    );
    assert_eq!(
        SegmentedControlSelectionSource::from_indices(Some(1), Some(1)),
        SegmentedControlSelectionSource::Selected
    );
    assert_eq!(
        SegmentedControlSelectionSource::from_indices(Some(8), None),
        SegmentedControlSelectionSource::OutOfRange
    );
}

#[test]
fn semantic_state_normalization_centralizes_source_markers() {
    let normalized_state = ui_state_primitives::segmented_control::SegmentedControlState {
        item_count: 3,
        is_empty: false,
        has_items: true,
        is_disabled: false,
        has_disabled_options: false,
        disabled_option_count: 0,
        selected_index: Some(1),
        has_selection: true,
        selection_empty: false,
        is_horizontal: true,
        is_vertical: false,
        has_label: true,
    };

    let semantic = normalize_semantic_state(SegmentedControlSemanticStateInput {
        control_mode: SegmentedControlControlMode::Controlled,
        raw_selected_index: Some(5),
        normalized_state,
    });

    assert_eq!(
        semantic.control_mode,
        SegmentedControlControlMode::Controlled
    );
    assert_eq!(
        semantic.selection_source,
        SegmentedControlSelectionSource::Selected
    );
    assert_eq!(semantic.selected_index, Some(1));
    assert!(semantic.has_selection);
}

#[test]
fn selection_origin_attr_is_stable() {
    assert_eq!(
        SegmentedControlSelectionOrigin::Programmatic.as_attr(),
        "programmatic"
    );
    assert_eq!(
        SegmentedControlSelectionOrigin::Keyboard.as_attr(),
        "keyboard"
    );
    assert_eq!(
        SegmentedControlSelectionOrigin::Pointer.as_attr(),
        "pointer"
    );
}

#[test]
fn agent_contract_is_schema_typed_and_stable() {
    let contract = segmented_control_agent_contract();
    assert_eq!(contract.schema_attr, "ui.segmented-control");
    assert_eq!(contract.schema_version_attr, "v1");
    assert_eq!(contract.intent_attr, "single-choice-selection");
    assert_eq!(contract.action_model_attr, "navigate|focus|select");
    assert_eq!(
        contract.state_axis_attr,
        "selection|availability|orientation|label"
    );
    assert_eq!(
        contract.source_axis_attr,
        "control-mode|selection-source|selection-origin|disabled-indices"
    );
}

#[test]
fn selection_axis_default_value_is_normalized_in_logic() {
    let axis = normalize_selection_axis(SegmentedControlSelectionAxisInput {
        selected_index: None,
        on_selected_index_change: None,
        default_selected_index: Some(8),
        item_count: 3,
    });

    assert_eq!(axis.default_selected_index, None);
    assert_eq!(axis.control_mode, SegmentedControlControlMode::Uncontrolled);
}

#[test]
fn selection_axis_resolves_control_mode_from_pair_contract() {
    let (selected_index, on_selected_index_change) = signal(Some(1usize));
    let axis = normalize_selection_axis(SegmentedControlSelectionAxisInput {
        selected_index: Some(selected_index),
        on_selected_index_change: Some(on_selected_index_change),
        default_selected_index: Some(0),
        item_count: 3,
    });

    assert_eq!(axis.control_mode, SegmentedControlControlMode::Controlled);
    assert_eq!(axis.default_selected_index, Some(0));
    assert!(axis.selected_index.is_some());
    assert!(axis.on_selected_index_change.is_some());
}

#[test]
#[should_panic(expected = "must be provided together")]
fn selection_axis_rejects_half_controlled_input() {
    let (selected_index, _on_selected_index_change) = signal(Some(1usize));

    let _ = normalize_selection_axis(SegmentedControlSelectionAxisInput {
        selected_index: Some(selected_index),
        on_selected_index_change: None,
        default_selected_index: Some(0),
        item_count: 3,
    });
}
