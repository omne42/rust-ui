use super::*;
use crate::navigation_menu::{NavigationMenuPartStateInput, NavigationMenuSlot};

#[test]
fn id_base_and_aria_label_have_stable_defaults() {
    assert_eq!(
        normalize_id_base("  primary-nav  ".to_string()),
        "primary-nav"
    );
    assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);

    assert_eq!(resolve_aria_label(None), (DEFAULT_ARIA_LABEL.into(), false));
    assert_eq!(
        resolve_aria_label(Some("  App sections  ".to_string())),
        ("App sections".to_string(), true)
    );
}

#[test]
fn resolve_items_normalizes_ids_labels_and_href() {
    let items = resolve_items(
        "docs-nav",
        vec![
            NavigationMenuItem::new("Docs", "Docs", "/docs"),
            NavigationMenuItem::new("Docs", "", " "),
            NavigationMenuItem::new(" ", "Blog", "/blog"),
        ],
    );

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, "docs");
    assert_eq!(items[1].id, "docs-2");
    assert_eq!(items[2].id, "item-3");
    assert_eq!(items[1].label, "Item 2");
    assert_eq!(items[1].href, "#");
    assert_eq!(items[0].dom_id, "docs-nav-docs");
}

#[test]
fn enabled_index_helpers_skip_disabled_items() {
    let items = vec![
        NavigationMenuItemResolved {
            id: "a".to_string(),
            dom_id: "a".to_string(),
            label: "A".to_string(),
            href: "/a".to_string(),
            disabled: false,
        },
        NavigationMenuItemResolved {
            id: "b".to_string(),
            dom_id: "b".to_string(),
            label: "B".to_string(),
            href: "/b".to_string(),
            disabled: true,
        },
        NavigationMenuItemResolved {
            id: "c".to_string(),
            dom_id: "c".to_string(),
            label: "C".to_string(),
            href: "/c".to_string(),
            disabled: false,
        },
    ];

    assert_eq!(first_enabled_index(&items), Some(0));
    assert_eq!(last_enabled_index(&items), Some(2));
    assert_eq!(next_enabled_index(&items, 0, 1), Some(2));
    assert_eq!(next_enabled_index(&items, 2, -1), Some(0));
}

#[test]
fn selected_id_and_focus_are_sanitized() {
    let items = resolve_items(
        "docs-nav",
        vec![
            NavigationMenuItem::new("home", "Home", "/").disabled(true),
            NavigationMenuItem::new("docs", "Docs", "/docs"),
        ],
    );

    assert_eq!(
        sanitize_selected_id(Some("docs".to_string()), &items),
        Some("docs".to_string())
    );
    assert_eq!(sanitize_selected_id(Some("home".to_string()), &items), None);
    assert_eq!(
        selected_index_for_id(&items, Some("docs".to_string())),
        Some(1)
    );
    assert_eq!(
        selected_index_for_id(&items, Some("home".to_string())),
        None
    );
    assert_eq!(sanitize_focused_index(Some(1), &items), Some(1));
    assert_eq!(sanitize_focused_index(Some(0), &items), None);
    assert_eq!(resolve_initial_focus_index(&items, Some(0)), Some(1));
}

#[test]
fn resolve_state_tracks_source_and_selection_contracts() {
    let state = resolve_state(NavigationMenuPartStateInput {
        slot: NavigationMenuSlot::Root,
        item_count: 3,
        selected_index: Some(1),
        focused_index: Some(1),
        has_disabled_items: true,
        activate_on_focus: false,
        is_controlled: true,
        has_custom_id_base: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_activate_on_focus: true,
        has_custom_selected_id: true,
        has_custom_default_selected_id: true,
        has_custom_on_selected_id_change: true,
        has_custom_motion: true,
    });

    assert_eq!(state.slot_attr, "navigation-menu");
    assert_eq!(state.state_attr, "selected");
    assert_eq!(state.item_attr, "populated");
    assert_eq!(state.selected_attr, "present");
    assert_eq!(state.focus_attr, "present");
    assert_eq!(state.focus_activation_attr, "manual");
    assert_eq!(state.selection_mode_attr, "controlled");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.aria_label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.activate_on_focus_source_attr, "custom");
    assert_eq!(state.selected_id_source_attr, "custom");
    assert_eq!(state.default_selected_id_source_attr, "custom");
    assert_eq!(state.selected_id_change_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(NavigationMenuPartStateInput {
            slot: NavigationMenuSlot::Root,
            item_count: 3,
            selected_index: Some(1),
            focused_index: Some(1),
            has_disabled_items: true,
            activate_on_focus: false,
            is_controlled: true,
            has_custom_id_base: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_activate_on_focus: true,
            has_custom_selected_id: true,
            has_custom_default_selected_id: false,
            has_custom_on_selected_id_change: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-navigation-menu",
        "ui-navigation-menu--has-items",
        "ui-navigation-menu--selected",
        "ui-navigation-menu--focused",
        "ui-navigation-menu--has-disabled-items",
        "ui-navigation-menu--manual-activation",
        "ui-navigation-menu--controlled",
        "ui-navigation-menu--custom-motion",
        "ui-navigation-menu--custom-aria-label",
        "ui-navigation-menu--custom-activate-on-focus",
        "ui-navigation-menu--custom-selected-id",
        "ui-navigation-menu--custom-selected-id-change",
        "ui-navigation-menu--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
