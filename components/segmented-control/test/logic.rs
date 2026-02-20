use super::*;

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
