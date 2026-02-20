use super::*;

#[test]
fn normalize_items_applies_fallbacks_and_trims() {
    let normalized = normalize_items(vec![SidebarMenuItem {
        id: " ".to_string(),
        label: " ".to_string(),
        href: Some(" /demo ".to_string()),
        badge: Some(" 42 ".to_string()),
        action_label: Some(" ⋯ ".to_string()),
        disabled: false,
        sub_items: vec![SidebarMenuSubItem {
            id: " ".to_string(),
            label: " ".to_string(),
            href: Some(" # ".to_string()),
            disabled: false,
        }],
        default_sub_open: true,
    }]);

    assert_eq!(normalized[0].id, "item-0");
    assert_eq!(normalized[0].label, "Item 1");
    assert_eq!(normalized[0].href.as_deref(), Some("/demo"));
    assert_eq!(normalized[0].badge.as_deref(), Some("42"));
    assert_eq!(normalized[0].action_label.as_deref(), Some("⋯"));
    assert_eq!(normalized[0].sub_items[0].id, "item-0-sub-0");
}

#[test]
fn next_id_for_key_tracks_enabled_items() {
    let items = normalize_items(vec![
        SidebarMenuItem::new("a", "A"),
        SidebarMenuItem {
            id: "b".to_string(),
            label: "B".to_string(),
            href: None,
            badge: None,
            action_label: None,
            disabled: true,
            sub_items: vec![SidebarMenuSubItem::new("b-1", "B1")],
            default_sub_open: false,
        },
    ]);

    assert_eq!(next_id_for_key("Home", &items, None).as_deref(), Some("a"));
    assert_eq!(next_id_for_key("End", &items, None).as_deref(), Some("b-1"));
    assert_eq!(
        next_id_for_key("ArrowDown", &items, Some("a".to_string())).as_deref(),
        Some("b-1"),
    );
}

#[test]
fn compose_class_name_includes_state_flags() {
    let class = compose_class_name(
        Some("custom".to_string()),
        resolve_state(SidebarMenuStateInput {
            item_count: 2,
            disabled: false,
            show_badges: true,
            show_actions: true,
            allow_submenu_collapse: true,
            is_controlled: false,
            has_custom_class_name: true,
            has_shortcut: true,
        }),
    );

    for needle in [
        "ui-sidebar-menu",
        "ui-sidebar-menu--with-badges",
        "ui-sidebar-menu--with-actions",
        "ui-sidebar-menu--collapsible-sub",
        "ui-sidebar-menu--with-shortcut",
        "ui-sidebar-menu--uncontrolled",
        "ui-sidebar-menu--custom-class",
        "custom",
    ] {
        assert!(class.contains(needle), "class should contain `{needle}`");
    }
}
