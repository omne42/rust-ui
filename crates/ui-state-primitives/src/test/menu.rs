use super::*;

#[test]
fn text_and_id_helpers_trim_and_fallback() {
    assert_eq!(
        normalize_optional_text(Some("  demo  ".to_string())),
        Some("demo".to_string())
    );
    assert_eq!(normalize_optional_text(Some(" \n ".to_string())), None);
    assert_eq!(
        normalize_id_base("  menu-id  ".to_string(), "menu"),
        "menu-id"
    );
    assert_eq!(normalize_id_base(" ".to_string(), "menu"), "menu");
    assert_eq!(
        resolve_id_pair("docs"),
        ("docs-trigger".to_string(), "docs-menu".to_string())
    );
}

#[test]
fn controlled_prop_alias_uses_is_prefix_priority() {
    assert_eq!(
        normalize_controlled_prop_alias(Some("is-value".to_string()), Some("legacy".to_string())),
        Some("is-value".to_string())
    );
    assert_eq!(
        normalize_controlled_prop_alias::<String>(None, Some("legacy".to_string())),
        Some("legacy".to_string())
    );
    assert_eq!(normalize_controlled_prop_alias::<String>(None, None), None);
}

#[test]
fn controlled_prop_flag_reflects_presence() {
    let controlled = Some(1usize);
    let uncontrolled = None::<usize>;

    assert!(is_controlled_prop(&controlled));
    assert!(!is_controlled_prop(&uncontrolled));
}

#[test]
fn aria_label_and_accessible_name_are_normalized() {
    assert_eq!(
        resolve_aria_label_with_fallback(Some("  Open actions ".to_string()), "Fallback", "Menu"),
        ("Open actions".to_string(), true)
    );
    assert_eq!(
        resolve_aria_label_with_fallback(None, "  ", "Menu"),
        ("Menu".to_string(), false)
    );

    assert_eq!(
        resolve_menu_accessible_name(
            Some("  Menu  ".to_string()),
            Some("menu-id".to_string()),
            "Menu"
        ),
        MenuAccessibleName {
            aria_label: Some("Menu".to_string()),
            aria_labelledby: None,
        }
    );
    assert_eq!(
        resolve_menu_accessible_name(None, Some("  menu-title ".to_string()), "Menu"),
        MenuAccessibleName {
            aria_label: None,
            aria_labelledby: Some("menu-title".to_string()),
        }
    );
}

#[test]
fn index_helpers_skip_disabled_entries() {
    let disabled = vec![false, true, false, false];
    assert_eq!(normalize_index(Some(2), 4), Some(2));
    assert_eq!(normalize_index(Some(4), 4), None);
    assert_eq!(normalize_disabled_indices(vec![3, 1, 1, 9], 4), vec![1, 3]);
    assert_eq!(first_enabled_index(&disabled), Some(0));
    assert_eq!(last_enabled_index(&disabled), Some(3));
    assert_eq!(sanitize_enabled_index(Some(1), &disabled), None);
    assert_eq!(sanitize_enabled_index(Some(2), &disabled), Some(2));
    assert_eq!(next_enabled_index(&disabled, 0, 1), Some(2));
    assert_eq!(next_enabled_index(&disabled, 3, -1), Some(2));
    assert_eq!(resolve_initial_focus_index(Some(1), &disabled), Some(0));
}

#[test]
fn navigation_items_are_sanitized_and_deduped() {
    let items = resolve_navigation_items(
        "docs-nav",
        vec![
            NavigationItemInput {
                id: "Docs".to_string(),
                label: "Docs".to_string(),
                href: "/docs".to_string(),
                disabled: false,
            },
            NavigationItemInput {
                id: "Docs".to_string(),
                label: " ".to_string(),
                href: " ".to_string(),
                disabled: true,
            },
            NavigationItemInput {
                id: " ".to_string(),
                label: "Blog".to_string(),
                href: "/blog".to_string(),
                disabled: false,
            },
        ],
    );

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, "docs");
    assert_eq!(items[1].id, "docs-2");
    assert_eq!(items[2].id, "item-3");
    assert_eq!(items[1].label, "Item 2");
    assert_eq!(items[1].href, "#");
    assert_eq!(items[0].dom_id, "docs-nav-docs");

    assert_eq!(
        sanitize_selected_id(Some("docs".to_string()), &items),
        Some("docs".to_string())
    );
    assert_eq!(
        selected_index_for_id(&items, Some("docs-2".to_string())),
        None
    );
}

#[test]
fn menubar_items_and_open_index_are_sanitized() {
    let menus = resolve_menubar_menus(
        "main",
        vec![
            MenubarMenuInput {
                id: "File".to_string(),
                label: "File".to_string(),
                items: vec!["New".to_string(), " ".to_string(), "Save".to_string()],
                disabled_indices: vec![1, 1, 9],
                disabled: false,
            },
            MenubarMenuInput {
                id: " ".to_string(),
                label: " ".to_string(),
                items: vec![],
                disabled_indices: vec![],
                disabled: false,
            },
        ],
    );

    assert_eq!(menus.len(), 2);
    assert_eq!(menus[0].id, "file");
    assert_eq!(menus[0].items, vec!["New".to_string(), "Save".to_string()]);
    assert_eq!(menus[0].disabled_indices, vec![1]);
    assert_eq!(menus[1].id, "menu-2");
    assert!(menus[1].is_trigger_disabled);
    assert_eq!(
        sanitize_open_index_for_trigger_disabled(
            Some(0),
            &menus
                .iter()
                .map(|menu| menu.is_trigger_disabled)
                .collect::<Vec<_>>()
        ),
        Some(0)
    );
    assert_eq!(
        sanitize_open_index_for_trigger_disabled(
            Some(1),
            &menus
                .iter()
                .map(|menu| menu.is_trigger_disabled)
                .collect::<Vec<_>>()
        ),
        None
    );
}
