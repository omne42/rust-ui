use super::*;
use std::collections::BTreeSet;

#[test]
fn normalize_items_applies_fallbacks_and_trimmed_text() {
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
fn open_sub_id_helpers_filter_and_toggle_by_valid_submenu_roots() {
    let items = normalize_items(vec![
        SidebarMenuItem {
            id: "alpha".to_string(),
            label: "Alpha".to_string(),
            href: None,
            badge: None,
            action_label: None,
            disabled: false,
            sub_items: vec![SidebarMenuSubItem::new("alpha-sub", "Alpha sub")],
            default_sub_open: true,
        },
        SidebarMenuItem::new("beta", "Beta"),
    ]);

    assert_eq!(
        default_open_sub_id_set(&items),
        BTreeSet::from(["alpha".to_string()])
    );

    let normalized = normalize_open_sub_id_set(
        &BTreeSet::from(["unknown".to_string(), "alpha".to_string()]),
        &items,
    );
    assert_eq!(normalized, BTreeSet::from(["alpha".to_string()]));

    let toggled_open = toggle_open_sub_id(&normalized, "alpha", &items);
    assert!(toggled_open.is_empty());

    let toggled_invalid = toggle_open_sub_id(&normalized, "beta", &items);
    assert_eq!(toggled_invalid, normalized);
}

#[test]
fn next_id_for_key_navigates_enabled_linear_ids() {
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
fn active_index_helpers_default_to_zero_for_missing_active_id() {
    let items = normalize_items(vec![
        SidebarMenuItem::new("a", "A"),
        SidebarMenuItem::new("b", "B"),
    ]);

    assert_eq!(active_index_for_current(&items, Some("b")), 1);
    assert_eq!(active_index_for_current(&items, Some("missing")), 0);
    assert_eq!(resolve_active_index(&linear_enabled_ids(&items), None), 0);
}
