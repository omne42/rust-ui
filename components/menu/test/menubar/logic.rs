use super::*;
use crate::menubar::{MenuOpenFocusStrategy, MenubarPartStateInput, MenubarSlot};

#[test]
fn normalize_id_base_falls_back_when_blank() {
    assert_eq!(
        normalize_id_base("  menubar-root  ".to_string()),
        "menubar-root"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_menus_normalizes_ids_items_and_disabled_indices() {
    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new(
                " File ",
                " File ",
                vec![
                    "New".to_string(),
                    "  ".to_string(),
                    "Open".to_string(),
                    "Save".to_string(),
                ],
            )
            .disabled_indices(vec![2, 2, 6]),
            MenubarMenu::new("File", "Edit", vec!["Undo".to_string()]),
        ],
    );

    assert_eq!(menus.len(), 2);
    assert_eq!(menus[0].id, "file");
    assert_eq!(menus[1].id, "file-2");
    assert_eq!(menus[0].label, "File");
    assert_eq!(menus[0].items.len(), 3);
    assert_eq!(menus[0].disabled_indices, vec![2]);
    assert_eq!(menus[0].trigger_id, "docs-menubar-file-trigger");
    assert_eq!(menus[0].menu_id, "docs-menubar-file-menu");
}

#[test]
fn disabled_or_empty_menu_disables_trigger() {
    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new("view", "View", Vec::new()),
            MenubarMenu::new("help", "Help", vec!["About".to_string()]).disabled(true),
        ],
    );

    assert!(menus[0].is_trigger_disabled);
    assert!(menus[1].is_trigger_disabled);
}

#[test]
fn next_enabled_menu_skips_disabled_and_wraps() {
    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new("file", "File", vec!["New".to_string()]),
            MenubarMenu::new("edit", "Edit", vec!["Undo".to_string()]).disabled(true),
            MenubarMenu::new("view", "View", vec!["Zoom".to_string()]),
        ],
    );

    assert_eq!(next_enabled_menu_index(&menus, 0, 1), Some(2));
    assert_eq!(next_enabled_menu_index(&menus, 2, 1), Some(0));
    assert_eq!(next_enabled_menu_index(&menus, 0, -1), Some(2));
}

#[test]
fn sanitize_open_index_rejects_invalid_or_disabled_menu() {
    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new("file", "File", vec!["New".to_string()]),
            MenubarMenu::new("help", "Help", vec!["About".to_string()]).disabled(true),
        ],
    );

    assert_eq!(sanitize_open_index_for_menus(Some(0), &menus), Some(0));
    assert_eq!(sanitize_open_index_for_menus(Some(1), &menus), None);
    assert_eq!(sanitize_open_index_for_menus(Some(8), &menus), None);
}

#[test]
fn focus_strategy_maps_arrow_open_keys() {
    assert_eq!(
        crate::menubar::focus_strategy_for_open_key("ArrowDown"),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        crate::menubar::focus_strategy_for_open_key("ArrowUp"),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(crate::menubar::focus_strategy_for_open_key("Enter"), None);
}

#[test]
fn focus_strategy_default_index() {
    assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
}

#[test]
fn resolve_state_tracks_source_and_open_contracts() {
    let state = resolve_state(MenubarPartStateInput {
        slot: MenubarSlot::Root,
        menu_count: 3,
        open_index: Some(1),
        has_disabled_menus: true,
        close_on_action: false,
        is_controlled: true,
        placement: PopoverPlacement::TopEnd,
        has_custom_id_base: true,
        has_custom_class_name: true,
        has_custom_close_on_action: true,
        has_custom_placement: true,
        has_custom_open_index: true,
        has_custom_default_open_index: true,
        has_custom_on_open_index_change: true,
        has_custom_motion: true,
    });

    assert_eq!(state.slot_attr, "menubar");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.menu_attr, "populated");
    assert_eq!(state.action_attr, "keep-open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.placement_attr, "top-end");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.close_on_action_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.open_index_source_attr, "custom");
    assert_eq!(state.default_open_index_source_attr, "custom");
    assert_eq!(state.open_index_change_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(MenubarPartStateInput {
            slot: MenubarSlot::Root,
            menu_count: 2,
            open_index: Some(1),
            has_disabled_menus: true,
            close_on_action: false,
            is_controlled: true,
            placement: PopoverPlacement::BottomStart,
            has_custom_id_base: true,
            has_custom_class_name: true,
            has_custom_close_on_action: true,
            has_custom_placement: true,
            has_custom_open_index: true,
            has_custom_default_open_index: true,
            has_custom_on_open_index_change: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-menubar",
        "ui-menubar--placement-bottom-start",
        "ui-menubar--has-menus",
        "ui-menubar--open",
        "ui-menubar--has-disabled-menus",
        "ui-menubar--persistent",
        "ui-menubar--controlled",
        "ui-menubar--custom-motion",
        "ui-menubar--custom-id",
        "ui-menubar--custom-close-on-action",
        "ui-menubar--custom-placement",
        "ui-menubar--custom-open-index",
        "ui-menubar--custom-default-open-index",
        "ui-menubar--custom-open-index-change",
        "ui-menubar--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn menubar_runtime_decisions_are_centralized() {
    assert_eq!(resolve_menu_state_attr(true, false), "open");
    assert_eq!(resolve_menu_state_attr(false, true), "disabled");
    assert_eq!(resolve_menu_state_attr(false, false), "closed");
    assert_eq!(resolve_aria_expanded(true), "true");
    assert_eq!(resolve_aria_expanded(false), "false");

    assert_eq!(
        resolve_next_open_index_on_trigger_press(false, Some(2), 2),
        Some(None)
    );
    assert_eq!(
        resolve_next_open_index_on_trigger_press(false, None, 1),
        Some(Some(1))
    );
    assert_eq!(
        resolve_next_open_index_on_trigger_press(true, None, 1),
        None
    );

    assert_eq!(
        resolve_next_open_index_on_pointer_enter(false, Some(0), 1),
        Some(Some(1))
    );
    assert_eq!(
        resolve_next_open_index_on_pointer_enter(false, None, 1),
        None
    );

    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new("file", "File", vec!["New".to_string()]),
            MenubarMenu::new("edit", "Edit", vec!["Undo".to_string()]),
        ],
    );
    assert_eq!(
        resolve_key_decision("ArrowDown", false, 0, &menus),
        Some(MenubarKeyDecision::OpenCurrent {
            focus: MenuOpenFocusStrategy::First
        })
    );
}

#[test]
fn normalize_close_on_action_resolves_alias_priority() {
    assert_eq!(
        normalize_close_on_action(MenubarActionModeInput {
            is_close_on_action: Some(false),
            close_on_action: true,
        }),
        MenubarActionMode::KeepOpenOnAction
    );
    assert_eq!(
        normalize_close_on_action(MenubarActionModeInput {
            is_close_on_action: None,
            close_on_action: true,
        }),
        MenubarActionMode::CloseOnAction
    );
}

#[test]
fn normalize_default_open_index_sanitizes_in_one_step() {
    let menus = resolve_menus(
        "docs-menubar",
        vec![
            MenubarMenu::new("file", "File", vec!["New".to_string()]),
            MenubarMenu::new("help", "Help", vec!["About".to_string()]).disabled(true),
        ],
    );

    assert_eq!(
        normalize_default_open_index(Some(0), menus.len(), &menus),
        Some(0)
    );
    assert_eq!(
        normalize_default_open_index(Some(1), menus.len(), &menus),
        None
    );
}
