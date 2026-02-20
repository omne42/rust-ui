use super::*;

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
