use super::*;

#[test]
fn orientation_class_names_are_stable() {
    assert_eq!(
        RadioGroupOrientation::Vertical.class_name(),
        "ui-radio-group--vertical"
    );
    assert_eq!(
        RadioGroupOrientation::Horizontal.class_name(),
        "ui-radio-group--horizontal"
    );
}

#[test]
fn aria_and_data_orientation_values_are_stable() {
    assert_eq!(
        RadioGroupOrientation::Vertical.aria_orientation(),
        "vertical"
    );
    assert_eq!(
        RadioGroupOrientation::Horizontal.aria_orientation(),
        "horizontal"
    );

    assert_eq!(
        RadioGroupOrientation::Vertical.data_orientation(),
        "vertical"
    );
    assert_eq!(
        RadioGroupOrientation::Horizontal.data_orientation(),
        "horizontal"
    );
}

#[test]
fn resolve_state_tracks_empty_disabled_group() {
    let disabled = HashSet::new();
    let state = resolve_state(
        0,
        true,
        &disabled,
        Some(0),
        RadioGroupOrientation::Vertical,
        false,
    );

    assert_eq!(state.item_count, 0);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(state.is_disabled);
    assert!(!state.has_disabled_options);
    assert_eq!(state.disabled_option_count, 0);
    assert_eq!(state.selected_index, None);
    assert!(!state.has_selection);
    assert!(state.selection_empty);
    assert!(!state.is_horizontal);
    assert!(state.is_vertical);
    assert!(!state.has_label);
}

#[test]
fn resolve_state_tracks_selection_and_disabled_options() {
    let disabled = HashSet::from([1_usize, 9_usize]);
    let state = resolve_state(
        3,
        false,
        &disabled,
        Some(2),
        RadioGroupOrientation::Horizontal,
        true,
    );

    assert_eq!(state.item_count, 3);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.is_disabled);
    assert!(state.has_disabled_options);
    assert_eq!(state.disabled_option_count, 1);
    assert_eq!(state.selected_index, Some(2));
    assert!(state.has_selection);
    assert!(!state.selection_empty);
    assert!(state.is_horizontal);
    assert!(!state.is_vertical);
    assert!(state.has_label);
}

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Size  ".to_string())),
        Some("Size".to_string())
    );
}

#[test]
fn resolve_accessible_name_prefers_aria_label() {
    assert_eq!(
        resolve_accessible_name(
            Some("  Plan selector  ".to_string()),
            Some("external-label".to_string()),
            Some("internal-label".to_string())
        ),
        RadioGroupAccessibleName {
            aria_label: Some("Plan selector".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_uses_labelledby_when_available() {
    assert_eq!(
        resolve_accessible_name(
            None,
            Some("external-label".to_string()),
            Some("internal-label".to_string())
        ),
        RadioGroupAccessibleName {
            aria_label: None,
            aria_labelledby: Some("external-label".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_falls_back_to_internal_labelledby() {
    assert_eq!(
        resolve_accessible_name(None, None, Some("group-label".to_string())),
        RadioGroupAccessibleName {
            aria_label: None,
            aria_labelledby: Some("group-label".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_uses_default_label_when_missing() {
    assert_eq!(
        resolve_accessible_name(None, None, None),
        RadioGroupAccessibleName {
            aria_label: Some(DEFAULT_ARIA_LABEL.into()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_checked_axis_prefers_is_checked_and_on_checked_change() {
    let state = resolve_checked_axis(RadioCheckedAxisInput {
        has_is_checked: true,
        has_checked: true,
        has_default_checked: true,
        has_on_checked_change: true,
        has_on_change: true,
    });

    assert_eq!(state.control_mode, RadioCheckedControlMode::Controlled);
    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.checked_source_attr, "is_checked");
    assert_eq!(state.default_checked_source_attr, "provided");
    assert_eq!(state.checked_change_source_attr, "on_checked_change");
}

#[test]
fn resolve_checked_axis_defaults_to_uncontrolled_markers() {
    let state = resolve_checked_axis(RadioCheckedAxisInput {
        has_is_checked: false,
        has_checked: false,
        has_default_checked: false,
        has_on_checked_change: false,
        has_on_change: true,
    });

    assert_eq!(state.control_mode, RadioCheckedControlMode::Uncontrolled);
    assert!(!state.is_controlled);
    assert_eq!(state.control_mode_attr, "uncontrolled");
    assert_eq!(state.checked_source_attr, "default");
    assert_eq!(state.default_checked_source_attr, "default");
    assert_eq!(state.checked_change_source_attr, "on_change");
}
