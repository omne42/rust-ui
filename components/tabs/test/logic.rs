use super::*;
use std::collections::HashSet;

#[test]
fn normalize_is_disabled_prefers_is_prefixed_value() {
    assert!(normalize_is_disabled(Some(true), false));
    assert!(!normalize_is_disabled(Some(false), true));
    assert!(normalize_is_disabled(None, true));
    assert!(!normalize_is_disabled(None, false));
}

#[test]
fn normalize_disabled_axis_tracks_source() {
    let from_is_disabled = normalize_disabled_axis(Some(true), false);
    assert!(from_is_disabled.is_disabled);
    assert_eq!(from_is_disabled.source, TabsDisabledSource::IsDisabled);
    assert_eq!(from_is_disabled.source.as_attr(), "is-disabled");

    let from_disabled = normalize_disabled_axis(None, true);
    assert!(from_disabled.is_disabled);
    assert_eq!(from_disabled.source, TabsDisabledSource::Disabled);
    assert_eq!(from_disabled.source.as_attr(), "disabled");
}

#[test]
fn normalize_selection_axis_tracks_control_mode() {
    let (selected, _set_selected) = leptos::prelude::signal(2_usize);
    let controlled = normalize_selection_axis(TabsSelectionAxisInput {
        selected_index: Some(selected),
        default_selected_index: 0,
        on_selection_change: None,
    });
    assert_eq!(controlled.control_mode, TabsControlMode::Controlled);
    assert!(controlled.control_mode.is_controlled());

    let uncontrolled = normalize_selection_axis(TabsSelectionAxisInput {
        selected_index: None,
        default_selected_index: 1,
        on_selection_change: None,
    });
    assert_eq!(uncontrolled.control_mode, TabsControlMode::Uncontrolled);
    assert!(!uncontrolled.control_mode.is_controlled());
}

#[test]
fn resolve_requested_selected_index_uses_controlled_then_default() {
    assert_eq!(resolve_requested_selected_index(Some(3), 1), 3);
    assert_eq!(resolve_requested_selected_index(None, 1), 1);
}

#[test]
fn compose_class_name_keeps_base_and_custom_suffix() {
    assert_eq!(compose_class_name(None), "ui-tabs");
    assert_eq!(
        compose_class_name(Some("  my-tabs ".to_string())),
        "ui-tabs my-tabs"
    );
    assert_eq!(compose_class_name(Some("   ".to_string())), "ui-tabs");
}

#[test]
fn resolve_motion_source_maps_custom_flag() {
    assert_eq!(resolve_motion_source(false), "default");
    assert_eq!(resolve_motion_source(true), "custom");
}

#[test]
fn is_tab_disabled_respects_global_and_index_flags() {
    let disabled_indices = HashSet::from([2_usize]);

    assert!(is_tab_disabled(false, &disabled_indices, 2));
    assert!(!is_tab_disabled(false, &disabled_indices, 1));
    assert!(is_tab_disabled(true, &disabled_indices, 1));
}

#[test]
fn has_disabled_tabs_reports_global_or_index_level_disables() {
    let empty = HashSet::new();
    let indexed = HashSet::from([0_usize]);

    assert!(!has_disabled_tabs(false, &empty));
    assert!(has_disabled_tabs(false, &indexed));
    assert!(has_disabled_tabs(true, &empty));
}

#[test]
fn normalize_selected_with_disabled_skips_blocked_indices() {
    let disabled_indices = HashSet::from([1_usize]);

    let selected = normalize_selected_with_disabled(1, 3, |idx| disabled_indices.contains(&idx));
    assert_eq!(selected, 0);
}

#[test]
fn resolve_selection_request_is_none_for_empty_noop_or_disabled() {
    let disabled_indices = HashSet::from([1_usize]);

    assert_eq!(
        resolve_selection_request(0, 0, 0, |_| false),
        None,
        "empty items should not produce selection changes"
    );
    assert_eq!(
        resolve_selection_request(0, 0, 3, |_| false),
        None,
        "same index should not emit redundant changes"
    );
    assert_eq!(
        resolve_selection_request(1, 0, 3, |idx| disabled_indices.contains(&idx)),
        None,
        "disabled targets should be rejected"
    );
}

#[test]
fn resolve_selection_request_returns_next_for_valid_target() {
    let next = resolve_selection_request(2, 0, 3, |_| false);
    assert_eq!(next, Some(2));
}

#[test]
fn registration_actions_maintain_items_order_with_unregister_compaction() {
    let state = reduce_registration_actions(&[
        TabsRegistrationAction::Register { registration_id: 4 },
        TabsRegistrationAction::Register { registration_id: 2 },
        TabsRegistrationAction::Register { registration_id: 4 },
        TabsRegistrationAction::Unregister { registration_id: 4 },
        TabsRegistrationAction::Register { registration_id: 7 },
    ]);
    assert_eq!(state.items_order, vec![2, 7]);
}

#[test]
fn resolve_registered_items_order_follows_registration_actions() {
    let items_order = resolve_registered_items_order(
        &[
            TabsRegistrationAction::Register { registration_id: 9 },
            TabsRegistrationAction::Register { registration_id: 3 },
            TabsRegistrationAction::Unregister { registration_id: 9 },
            TabsRegistrationAction::Register { registration_id: 5 },
        ],
        &[3, 5, 9],
    );
    assert_eq!(items_order, vec![3, 5, 9]);
}
