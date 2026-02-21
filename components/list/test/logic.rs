use super::*;

#[test]
fn resolve_accessible_name_prefers_explicit_aria_label() {
    assert_eq!(
        resolve_accessible_name(
            Some("  Fruit options  ".to_string()),
            Some("trigger-id".to_string())
        ),
        ListAccessibleName {
            aria_label: Some("Fruit options".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_uses_labelledby_when_label_missing() {
    assert_eq!(
        resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
        ListAccessibleName {
            aria_label: None,
            aria_labelledby: Some("trigger-id".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_defaults_when_none_provided() {
    assert_eq!(
        resolve_accessible_name(None, None),
        ListAccessibleName {
            aria_label: Some("Listbox".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_ignores_blank_inputs() {
    assert_eq!(
        resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
        ListAccessibleName {
            aria_label: Some("Listbox".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_state_tracks_item_and_selection_flags() {
    let state = resolve_state(4, Some(2), true);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_selection);
    assert!(state.has_disabled_options);
}

#[test]
fn resolve_state_treats_out_of_range_selection_as_empty_selection() {
    let state = resolve_state(2, Some(9), false);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.has_selection);
    assert!(!state.has_disabled_options);
}

#[test]
fn resolve_state_handles_empty_listbox() {
    let state = resolve_state(0, None, false);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_selection);
    assert!(!state.has_disabled_options);
}

#[test]
fn normalize_list_class_name_is_single_source_for_default_and_custom_values() {
    assert_eq!(normalize_list_class_name(None), "ui-listbox");
    assert_eq!(
        normalize_list_class_name(Some("  docs-list  ".to_string())),
        "ui-listbox docs-list"
    );
    assert_eq!(
        normalize_list_class_name(Some("   ".to_string())),
        "ui-listbox"
    );
}

#[test]
fn normalize_options_axis_collects_disabled_indices_and_root_marker() {
    let axis = normalize_options_axis(ListOptionsAxisInput {
        is_disabled: false,
        disabled_indices: vec![1, 3, 3],
    });
    assert!(axis.disabled_indices.contains(&1));
    assert!(axis.disabled_indices.contains(&3));
    assert!(axis.has_disabled_options);

    let root_disabled = normalize_options_axis(ListOptionsAxisInput {
        is_disabled: true,
        disabled_indices: vec![],
    });
    assert!(root_disabled.has_disabled_options);
}

#[test]
fn resolve_option_state_derives_selected_focused_and_disabled_bits() {
    let state = resolve_option_state(ListOptionStateInput {
        index: 2,
        active_index: 2,
        selected_index: Some(2),
        is_disabled_root: false,
        is_disabled_item: true,
    });
    assert!(state.is_selected);
    assert!(state.is_focused);
    assert!(state.is_disabled);
}

#[test]
fn is_disabled_index_checks_membership_without_view_side_rules() {
    let mut disabled = std::collections::HashSet::new();
    disabled.insert(4);
    assert!(is_disabled_index(&disabled, 4));
    assert!(!is_disabled_index(&disabled, 1));
}

#[test]
fn resolve_selection_source_state_covers_controlled_and_uncontrolled_matrix() {
    let controlled = resolve_selection_source_state(ListSelectionSourceStateInput {
        is_controlled: true,
        has_default_selected_index: false,
        has_on_selected_index_change: true,
    });
    assert_eq!(controlled.selection_mode_attr, "controlled");
    assert_eq!(controlled.selection_value_source_attr, "external");
    assert_eq!(controlled.default_selection_source_attr, "none");
    assert_eq!(controlled.selection_change_source_attr, "provided");

    let uncontrolled = resolve_selection_source_state(ListSelectionSourceStateInput {
        is_controlled: false,
        has_default_selected_index: true,
        has_on_selected_index_change: false,
    });
    assert_eq!(uncontrolled.selection_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled.selection_value_source_attr, "internal");
    assert_eq!(uncontrolled.default_selection_source_attr, "provided");
    assert_eq!(uncontrolled.selection_change_source_attr, "none");
}

#[test]
fn list_interaction_source_attr_is_closed_set_for_none_keyboard_and_pointer() {
    assert_eq!(ListInteractionSource::None.as_attr(), "none");
    assert_eq!(ListInteractionSource::Keyboard.as_attr(), "keyboard");
    assert_eq!(ListInteractionSource::Pointer.as_attr(), "pointer");
}
