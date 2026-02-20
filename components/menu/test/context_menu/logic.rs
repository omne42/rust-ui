use super::*;
use crate::context_menu::{ContextMenuPartStateInput, ContextMenuSlot, MenuOpenFocusStrategy};

#[test]
fn menu_id_derives_from_base() {
    let ids = resolve_ids("demo");
    assert_eq!(ids.trigger_id, "demo-trigger");
    assert_eq!(ids.menu_id, "demo-menu");
}

#[test]
fn normalize_id_base_falls_back_when_blank() {
    assert_eq!(
        normalize_id_base("  demo-context-menu  ".to_string()),
        "demo-context-menu"
    );
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn disabled_indices_are_deduped_and_clamped_to_item_count() {
    assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
    assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
}

#[test]
fn aria_label_defaults_and_trims() {
    assert_eq!(
        resolve_trigger_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_trigger_aria_label(Some("  Context actions  ".to_string())),
        ("Context actions".to_string(), true)
    );
}

#[test]
fn focus_strategy_for_open_key_maps_context_shortcuts() {
    assert_eq!(
        crate::context_menu::focus_strategy_for_open_key("ContextMenu", false),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        crate::context_menu::focus_strategy_for_open_key("F10", true),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        crate::context_menu::focus_strategy_for_open_key("ArrowUp", false),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(
        crate::context_menu::focus_strategy_for_open_key("F10", false),
        None
    );
}

#[test]
fn focus_strategy_default_index() {
    assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
}

#[test]
fn trigger_disabled_when_component_or_items_disabled() {
    assert!(resolve_trigger_disabled(true, 3));
    assert!(resolve_trigger_disabled(false, 0));
    assert!(!resolve_trigger_disabled(false, 2));
}

#[test]
fn resolve_state_tracks_source_and_open_contracts() {
    let state = resolve_state(ContextMenuPartStateInput {
        slot: ContextMenuSlot::Root,
        is_open: true,
        item_count: 3,
        trigger_disabled: false,
        close_on_action: false,
        placement: PopoverPlacement::TopEnd,
        is_controlled: true,
        has_custom_id_base: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_disabled: true,
        has_custom_disabled_indices: true,
        has_custom_item_kinds: true,
        has_custom_close_on_action: true,
        has_custom_placement: true,
        has_custom_open: true,
        has_custom_default_open: true,
        has_custom_on_open_change: true,
        has_custom_motion: true,
    });

    assert_eq!(state.slot_attr, "context-menu");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.item_attr, "populated");
    assert_eq!(state.disabled_attr, "false");
    assert_eq!(state.action_attr, "keep-open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.placement_attr, "top-end");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.aria_label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.disabled_source_attr, "custom");
    assert_eq!(state.disabled_indices_source_attr, "custom");
    assert_eq!(state.item_kinds_source_attr, "custom");
    assert_eq!(state.close_on_action_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.open_source_attr, "custom");
    assert_eq!(state.default_open_source_attr, "custom");
    assert_eq!(state.open_change_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(ContextMenuPartStateInput {
            slot: ContextMenuSlot::Root,
            is_open: false,
            item_count: 0,
            trigger_disabled: true,
            close_on_action: false,
            placement: PopoverPlacement::BottomStart,
            is_controlled: true,
            has_custom_id_base: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
            has_custom_disabled: true,
            has_custom_disabled_indices: true,
            has_custom_item_kinds: true,
            has_custom_close_on_action: true,
            has_custom_placement: true,
            has_custom_open: true,
            has_custom_default_open: true,
            has_custom_on_open_change: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-context-menu",
        "ui-context-menu--placement-bottom-start",
        "ui-context-menu--closed",
        "ui-context-menu--disabled",
        "ui-context-menu--empty",
        "ui-context-menu--persistent",
        "ui-context-menu--controlled",
        "ui-context-menu--custom-id",
        "ui-context-menu--custom-aria-label",
        "ui-context-menu--custom-disabled",
        "ui-context-menu--custom-disabled-indices",
        "ui-context-menu--custom-item-kinds",
        "ui-context-menu--custom-close-on-action",
        "ui-context-menu--custom-placement",
        "ui-context-menu--custom-open",
        "ui-context-menu--custom-default-open",
        "ui-context-menu--custom-open-change",
        "ui-context-menu--custom-motion",
        "ui-context-menu--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
