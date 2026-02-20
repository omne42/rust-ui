use super::*;

#[test]
fn focus_strategy_for_open_key_maps_arrow_keys_only() {
    assert_eq!(
        focus_strategy_for_open_key("ArrowDown"),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        focus_strategy_for_open_key("ArrowUp"),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(focus_strategy_for_open_key("Enter"), None);
    assert_eq!(focus_strategy_for_open_key(" "), None);
}

#[test]
fn menu_open_focus_strategy_default_index_handles_empty_and_populated_lists() {
    assert_eq!(MenuOpenFocusStrategy::First.default_index(0), 0);
    assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
}

#[test]
fn discrete_axes_map_to_bool_consistently() {
    assert!(ActionMenuDisabledState::from_bool(true).is_disabled());
    assert!(!ActionMenuDisabledState::from_bool(false).is_disabled());
    assert!(ActionMenuActionMode::from_bool(true).is_close_on_action());
    assert!(!ActionMenuActionMode::from_bool(false).is_close_on_action());
}

#[test]
fn item_spec_builders_keep_item_metadata_in_one_structure() {
    let spec = ActionMenuItemSpec::action("Profile")
        .with_kind(MenuItemKind::Action)
        .with_disabled(true);

    assert_eq!(spec.label, "Profile");
    assert_eq!(spec.kind, MenuItemKind::Action);
    assert!(spec.is_disabled);
}
