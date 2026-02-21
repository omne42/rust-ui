use super::*;

#[test]
fn state_attrs_follow_contract() {
    assert_eq!(state_attr(true), "with-description");
    assert_eq!(state_attr(false), "title-only");
    assert_eq!(description_attr(true), "present");
    assert_eq!(description_attr(false), "absent");
    assert_eq!(footer_attr(true), "present");
    assert_eq!(footer_attr(false), "absent");
    assert_eq!(close_button_attr(true), "shown");
    assert_eq!(close_button_attr(false), "hidden");
}

#[test]
fn resolve_open_config_supports_controlled_and_uncontrolled_modes() {
    let controlled = resolve_open_config(DrawerOpenConfigInput {
        has_open: true,
        default_open: Some(false),
        has_on_open_change: true,
    });
    assert_eq!(controlled.mode, DrawerOpenMode::Controlled);
    assert!(!controlled.default_open);
    assert!(controlled.has_default_open);
    assert!(controlled.has_open_change_handler);

    let uncontrolled = resolve_open_config(DrawerOpenConfigInput {
        has_open: false,
        default_open: None,
        has_on_open_change: false,
    });
    assert_eq!(uncontrolled.mode, DrawerOpenMode::Uncontrolled);
    assert_eq!(uncontrolled.default_open, DEFAULT_OPEN);
    assert!(!uncontrolled.has_default_open);
    assert!(!uncontrolled.has_open_change_handler);
}

#[test]
fn can_request_open_change_follows_mode_and_handler() {
    assert!(can_request_open_change(DrawerOpenMode::Uncontrolled, false));
    assert!(can_request_open_change(DrawerOpenMode::Uncontrolled, true));
    assert!(can_request_open_change(DrawerOpenMode::Controlled, true));
    assert!(!can_request_open_change(DrawerOpenMode::Controlled, false));
}

#[test]
fn resolve_close_button_visibility_uses_default_and_custom_values() {
    assert_eq!(
        resolve_close_button_visibility(None),
        DrawerVisibility::Visible
    );
    assert_eq!(
        resolve_close_button_visibility(Some(true)),
        DrawerVisibility::Visible
    );
    assert_eq!(
        resolve_close_button_visibility(Some(false)),
        DrawerVisibility::Hidden
    );
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-drawer ".to_string())),
        Some("docs-drawer".to_string())
    );
}

#[test]
fn normalize_required_text_uses_fallback_for_blank_values() {
    assert_eq!(
        normalize_required_text(" Drawer ".to_string(), DEFAULT_TITLE),
        "Drawer"
    );
    assert_eq!(
        normalize_required_text(" ".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );
}

#[test]
fn normalize_id_base_uses_default_for_blank_values() {
    assert_eq!(
        normalize_id_base(" docs-drawer ".to_string()),
        "docs-drawer"
    );
    assert_eq!(normalize_id_base("  ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Root,
        placement: DrawerPlacement::Left,
        has_description: true,
        has_footer: true,
        show_close_button: false,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "drawer");
    assert_eq!(state.base_class, "ui-drawer");
    assert_eq!(state.placement_attr, "left");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.footer_attr, "present");
    assert_eq!(state.close_button_attr, "hidden");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.footer_source_attr, "custom");
    assert_eq!(state.close_source_attr, "custom");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}
