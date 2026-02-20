use super::*;

#[test]
fn resolves_ids() {
    let ids = resolve_ids("demo");
    assert_eq!(ids.trigger_id, "demo-trigger");
    assert_eq!(ids.menu_id, "demo-menu");
}

#[test]
fn normalize_id_base_falls_back_when_blank() {
    assert_eq!(
        normalize_id_base("  demo-trigger  ".to_string()),
        "demo-trigger"
    );
    assert_eq!(normalize_id_base("   ".to_string()), "menu-trigger");
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
        ("Open menu".to_string(), false)
    );
    assert_eq!(
        resolve_trigger_aria_label(Some("  Trigger  ".to_string())),
        ("Trigger".to_string(), true)
    );
}

#[test]
fn focus_strategy_for_open_key_maps_arrow_keys() {
    assert_eq!(
        focus_strategy_for_open_key("ArrowDown"),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        focus_strategy_for_open_key("ArrowUp"),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(focus_strategy_for_open_key("Enter"), None);
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
fn resolve_state_tracks_trigger_items_and_strategy_flags() {
    let state = resolve_state(MenuTriggerStateInput {
        item_count: 3,
        trigger_disabled: false,
        close_on_action: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
        has_disabled_items: true,
        has_item_kinds: true,
        is_controlled: true,
        placement: PopoverPlacement::TopEnd,
    });

    assert_eq!(state.item_count, 3);
    assert!(state.has_items);
    assert!(!state.is_empty);
    assert!(!state.is_trigger_disabled);
    assert!(state.is_enabled);
    assert!(!state.close_on_action);
    assert!(state.keep_open_on_action);
    assert!(state.has_custom_aria_label);
    assert!(state.has_custom_class_name);
    assert!(state.has_disabled_items);
    assert!(state.has_item_kinds);
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
    assert_eq!(state.placement_attr, "top-end");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(MenuTriggerStateInput {
            item_count: 0,
            trigger_disabled: true,
            close_on_action: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_disabled_items: false,
            has_item_kinds: false,
            is_controlled: true,
            placement: PopoverPlacement::BottomStart,
        }),
    );

    for token in [
        "ui-menu-trigger",
        "ui-menu-trigger--placement-bottom-start",
        "ui-menu-trigger--disabled",
        "ui-menu-trigger--empty",
        "ui-menu-trigger--persistent",
        "ui-menu-trigger--controlled",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
