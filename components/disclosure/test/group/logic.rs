use super::DisclosureGroupStateInput;
use super::*;

#[test]
fn selection_mode_contracts_are_stable() {
    assert_eq!(
        DisclosureGroupSelectionMode::Single.class_name(),
        "ui-disclosure-group--selection-single"
    );
    assert_eq!(
        DisclosureGroupSelectionMode::Multiple.class_name(),
        "ui-disclosure-group--selection-multiple"
    );
    assert_eq!(DisclosureGroupSelectionMode::Single.as_attr(), "single");
    assert_eq!(DisclosureGroupSelectionMode::Multiple.as_attr(), "multiple");
}

#[test]
fn normalize_aria_label_falls_back_to_default() {
    assert_eq!(
        normalize_aria_label(Some("  Custom Group  ".to_string())),
        ("Custom Group".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("  \n  ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn expanded_axis_state_tracks_control_mode_and_default_source() {
    let controlled = resolve_expanded_axis_state(true, true);
    assert_eq!(controlled.control_mode_attr, "controlled");
    assert_eq!(controlled.default_expanded_source_attr, "prop");
    assert!(controlled.is_controlled);

    let uncontrolled = resolve_expanded_axis_state(false, false);
    assert_eq!(uncontrolled.control_mode_attr, "uncontrolled");
    assert_eq!(
        uncontrolled.default_expanded_source_attr,
        "implicit-default"
    );
    assert!(!uncontrolled.is_controlled);
}

#[test]
fn normalize_expanded_indices_clamps_and_normalizes() {
    let indices = BTreeSet::from([0, 3, 10]);

    let multiple = normalize_expanded_indices(DisclosureGroupSelectionMode::Multiple, &indices, 4);
    assert_eq!(multiple, BTreeSet::from([0, 3]));

    let single = normalize_expanded_indices(DisclosureGroupSelectionMode::Single, &indices, 4);
    assert_eq!(single, BTreeSet::from([0]));
}

#[test]
fn resolve_state_tracks_empty_disabled_and_expanded_flags() {
    let empty_state = resolve_state(DisclosureGroupStateInput {
        selection_mode: DisclosureGroupSelectionMode::Multiple,
        item_count: 0,
        expanded_count: 10,
        disabled: false,
        has_disabled_items: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    assert!(empty_state.is_empty);
    assert_eq!(empty_state.data_state_attr, "empty");
    assert_eq!(empty_state.expanded_count, 0);

    let disabled_state = resolve_state(DisclosureGroupStateInput {
        selection_mode: DisclosureGroupSelectionMode::Multiple,
        item_count: 3,
        expanded_count: 2,
        disabled: true,
        has_disabled_items: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert!(!disabled_state.is_empty);
    assert!(disabled_state.has_expanded_items);
    assert!(disabled_state.has_multiple_expanded);
    assert_eq!(disabled_state.data_state_attr, "disabled");
    assert_eq!(disabled_state.aria_source_attr, "custom");
    assert_eq!(disabled_state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_reflects_state_flags() {
    let state = resolve_state(DisclosureGroupStateInput {
        selection_mode: DisclosureGroupSelectionMode::Multiple,
        item_count: 3,
        expanded_count: 2,
        disabled: true,
        has_disabled_items: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-disclosure-group-custom".to_string()), state);

    assert!(class_name.contains("ui-disclosure-group"));
    assert!(class_name.contains("ui-disclosure-group--selection-multiple"));
    assert!(class_name.contains("ui-disclosure-group--multiple-expanded"));
    assert!(class_name.contains("ui-disclosure-group--disabled"));
    assert!(class_name.contains("ui-disclosure-group--custom-class"));
    assert!(class_name.contains("docs-disclosure-group-custom"));
}
