use super::*;

#[test]
fn normalize_shortcut_key_defaults_and_trims() {
    assert_eq!(
        normalize_shortcut_key(None, true),
        Some(DEFAULT_SHORTCUT_KEY.into())
    );
    assert_eq!(
        normalize_shortcut_key(Some("  K  ".to_string()), true),
        Some("k".to_string())
    );
    assert_eq!(
        normalize_shortcut_key(Some("".to_string()), true),
        Some("b".to_string())
    );
}

#[test]
fn resolve_default_priority_prefers_new_api_flags() {
    assert!(resolve_disabled(Some(true), false));
    assert!(!resolve_disabled(None, false));
    assert!(resolve_trigger_visibility(Some(true), false));
    assert!(!resolve_trigger_visibility(None, false));
    assert!(resolve_shortcut_enabled(Some(true), false));
    assert!(!resolve_shortcut_enabled(None, false));
    assert_eq!(normalize_trigger_label(None), "Toggle sidebar");
}

#[test]
fn resolve_state_tracks_sidebar_flags_and_attrs() {
    let state = resolve_state(SidebarStateInput {
        side: SidebarSide::Right,
        variant: SidebarVariant::Inset,
        collapsible: SidebarCollapsible::Icon,
        open: false,
        disabled: false,
        is_controlled: true,
        show_trigger: false,
        has_shortcut_key: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.side_attr, "right");
    assert_eq!(state.variant_attr, "inset");
    assert_eq!(state.collapsible_attr, "icon");
    assert_eq!(state.state_attr, "closed");
    assert!(state.closed);
    assert!(state.enabled);
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
    assert_eq!(state.control_attr, "manual");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_and_custom_classes() {
    let class_name = compose_class_name(
        Some("demo".to_string()),
        resolve_state(SidebarStateInput {
            side: SidebarSide::Left,
            variant: SidebarVariant::Sidebar,
            collapsible: SidebarCollapsible::Offcanvas,
            open: true,
            disabled: true,
            is_controlled: false,
            show_trigger: true,
            has_shortcut_key: true,
            has_custom_class_name: true,
        }),
    );

    for needle in [
        "ui-sidebar",
        "ui-sidebar--left",
        "ui-sidebar--variant-sidebar",
        "ui-sidebar--offcanvas",
        "ui-sidebar--open",
        "ui-sidebar--disabled",
        "ui-sidebar--with-trigger",
        "ui-sidebar--with-shortcut",
        "ui-sidebar--uncontrolled",
        "ui-sidebar--custom-class",
        "demo",
    ] {
        assert!(
            class_name.contains(needle),
            "composed class should contain `{needle}`",
        );
    }
}
