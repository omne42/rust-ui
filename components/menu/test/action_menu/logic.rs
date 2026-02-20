use super::*;
use crate::action_menu::{ActionMenuPartStateInput, ActionMenuSlot};

#[test]
fn ids_include_menu_suffix() {
    let ids = resolve_ids("demo");
    assert_eq!(ids.trigger_id, "demo-trigger");
    assert_eq!(ids.menu_id, "demo-menu");
}

#[test]
fn normalize_id_base_falls_back_when_blank() {
    assert_eq!(normalize_id_base("  demo-menu  ".to_string()), "demo-menu");
    assert_eq!(normalize_id_base("   ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn aria_label_defaults_and_trims() {
    assert_eq!(
        resolve_trigger_aria_label(None, DEFAULT_TRIGGER_ARIA_LABEL),
        (DEFAULT_TRIGGER_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_trigger_aria_label(Some("  More  ".to_string()), DEFAULT_TRIGGER_ARIA_LABEL),
        ("More".to_string(), true)
    );
    assert_eq!(
        resolve_trigger_aria_label(None, "  Workspace actions "),
        ("Workspace actions".to_string(), false)
    );
}

#[test]
fn disabled_indices_are_deduped_and_clamped_to_item_count() {
    assert_eq!(normalize_disabled_indices(vec![2, 1, 1, 9], 3), vec![1, 2]);
    assert_eq!(normalize_disabled_indices(vec![4], 0), Vec::<usize>::new());
}

#[test]
fn discrete_props_defaults_and_alias_priority_are_centralized() {
    assert_eq!(
        normalize_discrete_props(None, None, None, None, None, None),
        ActionMenuDiscreteProps {
            disabled_state: ActionMenuDisabledState::Enabled,
            action_mode: ActionMenuActionMode::CloseOnAction,
            has_custom_disabled: false,
            has_custom_close_on_action: false,
        }
    );

    assert_eq!(
        normalize_discrete_props(None, None, Some(true), None, None, Some(false)),
        ActionMenuDiscreteProps {
            disabled_state: ActionMenuDisabledState::Disabled,
            action_mode: ActionMenuActionMode::KeepOpenOnAction,
            has_custom_disabled: true,
            has_custom_close_on_action: true,
        }
    );

    assert_eq!(
        normalize_discrete_props(
            Some(ActionMenuDisabledState::Enabled),
            Some(true),
            Some(true),
            Some(ActionMenuActionMode::CloseOnAction),
            Some(false),
            Some(false),
        ),
        ActionMenuDiscreteProps {
            disabled_state: ActionMenuDisabledState::Enabled,
            action_mode: ActionMenuActionMode::CloseOnAction,
            has_custom_disabled: false,
            has_custom_close_on_action: false,
        }
    );
}

#[test]
fn item_specs_become_single_typed_source_of_item_semantics() {
    let output = normalize_menu_items(ActionMenuItemsInput {
        item_specs: vec![
            ActionMenuItemSpec::action("Profile"),
            ActionMenuItemSpec::action("Settings").with_disabled(true),
        ],
        items: vec!["legacy".to_string()],
        item_kinds: vec![MenuItemKind::Action],
        disabled_indices: vec![0],
    });

    assert!(output.has_item_specs);
    assert_eq!(
        output.items,
        vec!["Profile".to_string(), "Settings".to_string()]
    );
    assert_eq!(output.item_count, 2);
    assert_eq!(
        output.item_kinds,
        vec![MenuItemKind::Action, MenuItemKind::Action]
    );
    assert_eq!(output.disabled_indices, vec![1]);
}

#[test]
fn legacy_parallel_arrays_still_bridge_when_item_specs_absent() {
    let output = normalize_menu_items(ActionMenuItemsInput {
        item_specs: Vec::new(),
        items: vec!["A".to_string(), "B".to_string()],
        item_kinds: vec![MenuItemKind::Action, MenuItemKind::Action],
        disabled_indices: vec![1, 8, 1],
    });

    assert!(!output.has_item_specs);
    assert_eq!(output.items, vec!["A".to_string(), "B".to_string()]);
    assert_eq!(output.item_count, 2);
    assert_eq!(
        output.item_kinds,
        vec![MenuItemKind::Action, MenuItemKind::Action]
    );
    assert_eq!(output.disabled_indices, vec![1]);
}

#[test]
fn normalize_props_centralizes_state_derivation() {
    let normalized = normalize_props(ActionMenuNormalizeInput {
        id_base: "  ".to_string(),
        item_count: 3,
        disabled_indices: vec![2, 2, 9],
        item_kinds_len: 1,
        class_name: Some("  docs-menu  ".to_string()),
        aria_label: Some("  Workspace actions  ".to_string()),
        fallback_aria_label: "More actions".to_string(),
        disabled_state: None,
        is_disabled: None,
        disabled: Some(true),
        action_mode: None,
        is_close_on_action: Some(false),
        close_on_action: Some(true),
        placement: PopoverPlacement::BottomEnd,
        has_custom_open: true,
        has_custom_default_open: false,
        has_custom_on_open_change: true,
        has_custom_motion: true,
    });

    assert_eq!(normalized.id_base, DEFAULT_ID_BASE);
    assert!(normalized.has_custom_disabled);
    assert!(normalized.has_custom_close_on_action);
    assert!(normalized.has_custom_disabled_indices);
    assert!(normalized.has_custom_item_kinds);
    assert!(normalized.has_custom_class_name);
    assert!(normalized.has_custom_aria_label);
    assert!(normalized.has_custom_placement);
    assert!(normalized.trigger_disabled);
    assert!(normalized.is_controlled);
    assert_eq!(normalized.disabled_state, ActionMenuDisabledState::Disabled);
    assert_eq!(
        normalized.action_mode,
        ActionMenuActionMode::KeepOpenOnAction
    );
    assert_eq!(normalized.disabled_indices, vec![2]);
    assert_eq!(normalized.aria_label, "Workspace actions".to_string());
    assert_eq!(normalized.class_name, Some("docs-menu".to_string()));
}

#[test]
fn trigger_press_state_machine_is_centralized() {
    assert_eq!(resolve_trigger_press(true, false), None);
    assert_eq!(
        resolve_trigger_press(false, false),
        Some(ActionMenuTriggerPressResult {
            next_open: true,
            open_focus: Some(MenuOpenFocusStrategy::First),
        })
    );
    assert_eq!(
        resolve_trigger_press(false, true),
        Some(ActionMenuTriggerPressResult {
            next_open: false,
            open_focus: None,
        })
    );
}

#[test]
fn action_open_change_is_centralized() {
    assert_eq!(
        resolve_action_open_change(ActionMenuActionMode::CloseOnAction),
        Some(false)
    );
    assert_eq!(
        resolve_action_open_change(ActionMenuActionMode::KeepOpenOnAction),
        None
    );
}

#[test]
fn state_helpers_remain_stable() {
    assert_eq!(state_attr(true, false, 3), "open");
    assert_eq!(state_attr(false, true, 3), "disabled");
    assert_eq!(state_attr(false, false, 0), "empty");
    assert_eq!(state_attr(false, false, 3), "closed");

    assert_eq!(item_attr(0), "empty");
    assert_eq!(item_attr(2), "populated");
    assert_eq!(action_attr(true), "close");
    assert_eq!(action_attr(false), "keep-open");
    assert_eq!(open_mode_attr(true), "controlled");
    assert_eq!(open_mode_attr(false), "uncontrolled");
}

#[test]
fn resolve_state_sets_source_markers_and_flags() {
    let state = resolve_state(ActionMenuPartStateInput {
        slot: ActionMenuSlot::Root,
        is_open: true,
        item_count: 3,
        trigger_disabled: false,
        close_on_action: false,
        has_disabled_items: true,
        has_item_kinds: true,
        is_controlled: true,
        placement: PopoverPlacement::BottomStart,
        has_custom_id_base: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_custom_disabled: true,
        has_custom_disabled_indices: true,
        has_custom_item_kinds: true,
        has_custom_close_on_action: true,
        has_custom_placement: true,
        has_custom_open: true,
        has_custom_default_open: false,
        has_custom_on_open_change: true,
        has_custom_motion: true,
    });

    assert_eq!(state.state_attr, "open");
    assert_eq!(state.action_attr, "keep-open");
    assert_eq!(state.open_mode_attr, "controlled");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.default_open_source_attr, "default");
    assert_eq!(state.motion_source_attr, "custom");
}

#[test]
fn compose_class_name_contains_stable_tokens() {
    let state = resolve_state(ActionMenuPartStateInput {
        slot: ActionMenuSlot::Root,
        is_open: false,
        item_count: 3,
        trigger_disabled: false,
        close_on_action: false,
        has_disabled_items: true,
        has_item_kinds: true,
        is_controlled: true,
        placement: PopoverPlacement::BottomEnd,
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
    let class_name = compose_class_name(Some("docs-action-menu".to_string()), state);

    for token in [
        "ui-action-menu",
        "ui-action-menu--placement-bottom-end",
        "ui-action-menu--closed",
        "ui-action-menu--has-items",
        "ui-action-menu--persistent",
        "ui-action-menu--controlled",
        "ui-action-menu--custom-id",
        "ui-action-menu--custom-aria-label",
        "ui-action-menu--custom-disabled",
        "ui-action-menu--custom-disabled-indices",
        "ui-action-menu--custom-item-kinds",
        "ui-action-menu--custom-close-on-action",
        "ui-action-menu--custom-placement",
        "ui-action-menu--custom-open",
        "ui-action-menu--custom-default-open",
        "ui-action-menu--custom-open-change",
        "ui-action-menu--custom-motion",
        "docs-action-menu",
    ] {
        assert!(
            class_name.contains(token),
            "composed class list should include `{token}`"
        );
    }
}
