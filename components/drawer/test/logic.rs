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

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-drawer".to_string()),
        resolve_state(DrawerPartStateInput {
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
        }),
    );

    for token in [
        "ui-drawer",
        "ui-drawer--placement-left",
        "ui-drawer--with-description",
        "ui-drawer--with-footer",
        "ui-drawer--close-hidden",
        "ui-drawer--custom-placement",
        "ui-drawer--custom-id",
        "ui-drawer--custom-title",
        "ui-drawer--custom-description",
        "ui-drawer--custom-footer",
        "ui-drawer--custom-close",
        "ui-drawer--custom-motion",
        "ui-drawer--custom-exit",
        "ui-drawer--custom-class",
        "docs-drawer",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_open_state_supports_controlled_mode() {
    let (open_raw, _set_open_raw) = leptos::prelude::signal(true);
    let on_open_change = Callback::new(|_: bool| {});
    let open = Signal::derive(move || open_raw.get());

    let state = normalize_open_state(DrawerOpenStateInput {
        is_open: Some(open),
        default_open: Some(false),
        on_open_change: Some(on_open_change),
    });

    assert!(state.open.is_some());
    assert!(!state.default_open);
    assert_eq!(state.mode, DrawerOpenMode::Controlled);
    assert!(state.has_default_open);
    assert!(state.has_open_change_handler);
}

#[test]
fn normalize_open_state_supports_uncontrolled_mode_with_default() {
    let state = normalize_open_state(DrawerOpenStateInput {
        is_open: None,
        default_open: Some(true),
        on_open_change: None,
    });

    assert!(state.open.is_none());
    assert!(state.default_open);
    assert_eq!(state.mode, DrawerOpenMode::Uncontrolled);
    assert!(state.has_default_open);
    assert!(!state.has_open_change_handler);
}

#[test]
fn can_request_open_change_follows_control_contract() {
    assert!(can_request_open_change(DrawerOpenMode::Uncontrolled, false));
    assert!(can_request_open_change(DrawerOpenMode::Uncontrolled, true));
    assert!(can_request_open_change(DrawerOpenMode::Controlled, true));
    assert!(!can_request_open_change(DrawerOpenMode::Controlled, false));
}

#[test]
fn open_state_source_markers_are_closed_sets() {
    assert_eq!(open_state_attr(true), "open");
    assert_eq!(open_state_attr(false), "closed");
    assert_eq!(open_mode_attr(DrawerOpenMode::Controlled), "controlled");
    assert_eq!(open_mode_attr(DrawerOpenMode::Uncontrolled), "uncontrolled");

    assert_eq!(
        resolve_open_value_source(DrawerOpenMode::Controlled, true).as_attr(),
        "external"
    );
    assert_eq!(
        resolve_open_value_source(DrawerOpenMode::Uncontrolled, true).as_attr(),
        "default"
    );
    assert_eq!(
        resolve_open_value_source(DrawerOpenMode::Uncontrolled, false).as_attr(),
        "primitive-default"
    );

    assert_eq!(
        DrawerOpenActionSource::Programmatic.as_attr(),
        "programmatic"
    );
    assert_eq!(DrawerOpenActionSource::Interaction.as_attr(), "interaction");
}

#[test]
fn drawer_visibility_models_discrete_close_button_states() {
    assert!(DrawerVisibility::Visible.is_visible());
    assert!(!DrawerVisibility::Hidden.is_visible());
}

#[test]
fn normalize_view_config_uses_defaults_for_missing_values() {
    let config = normalize_view_config(DrawerViewConfigInput {
        placement: None,
        is_close_button_visible: None,
        close_label: None,
        on_exit_complete: None,
    });

    assert_eq!(config.placement, DEFAULT_PLACEMENT);
    assert_eq!(config.close_button_visibility, DrawerVisibility::Visible);
    assert_eq!(config.close_label, DEFAULT_CLOSE_LABEL);
    assert!(!config.has_on_exit_complete);
}

#[test]
fn normalize_view_config_respects_custom_values_and_trims_blank_label() {
    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let called_clone = std::sync::Arc::clone(&called);
    let on_exit_complete = Callback::new(move |_| {
        called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    let custom = normalize_view_config(DrawerViewConfigInput {
        placement: Some(DrawerPlacement::Left),
        is_close_button_visible: Some(false),
        close_label: Some("Dismiss"),
        on_exit_complete: Some(on_exit_complete),
    });

    assert_eq!(custom.placement, DrawerPlacement::Left);
    assert_eq!(custom.close_button_visibility, DrawerVisibility::Hidden);
    assert_eq!(custom.close_label, "Dismiss");
    assert!(custom.has_on_exit_complete);
    custom.on_exit_complete.run(());
    assert!(called.load(std::sync::atomic::Ordering::SeqCst));

    let fallback_label = normalize_view_config(DrawerViewConfigInput {
        placement: None,
        is_close_button_visible: None,
        close_label: Some("   "),
        on_exit_complete: None,
    });
    assert_eq!(fallback_label.close_label, DEFAULT_CLOSE_LABEL);
}

#[test]
fn resolve_part_states_centralizes_slot_state_derivation() {
    let states = resolve_part_states(DrawerPartStatesInput {
        placement: DrawerPlacement::Left,
        has_description: true,
        has_footer: true,
        close_button_visibility: DrawerVisibility::Hidden,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(states.root.slot, DrawerSlot::Root);
    assert_eq!(states.header.slot, DrawerSlot::Header);
    assert_eq!(states.title.slot, DrawerSlot::Title);
    assert_eq!(states.description.slot, DrawerSlot::Description);
    assert_eq!(states.body.slot, DrawerSlot::Body);
    assert_eq!(states.footer.slot, DrawerSlot::Footer);
    assert_eq!(states.close.slot, DrawerSlot::Close);
    assert!(states.root.has_custom_class_name);
    assert!(!states.header.has_custom_class_name);
}

#[test]
fn resolve_part_classes_uses_root_base_class_only() {
    let states = resolve_part_states(DrawerPartStatesInput {
        placement: DrawerPlacement::Right,
        has_description: false,
        has_footer: false,
        close_button_visibility: DrawerVisibility::Visible,
        has_custom_id_base: false,
        has_custom_title: false,
        has_custom_description: false,
        has_custom_class_name: true,
        has_custom_motion: false,
        has_on_exit_complete: false,
    });
    let classes = resolve_part_classes(Some("docs-drawer".to_string()), states);

    assert!(classes.root.contains("docs-drawer"));
    assert!(!classes.header.contains("docs-drawer"));
    assert!(!classes.title.contains("docs-drawer"));
}
