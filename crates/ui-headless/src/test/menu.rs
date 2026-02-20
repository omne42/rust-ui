use super::*;

#[test]
fn menu_trigger_open_focus_strategy_for_key_maps_arrow_keys_only() {
    assert_eq!(
        menu_trigger_open_focus_strategy_for_key("ArrowDown"),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        menu_trigger_open_focus_strategy_for_key("ArrowUp"),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(menu_trigger_open_focus_strategy_for_key("Enter"), None);
}

#[test]
fn menu_trigger_open_focus_strategy_respects_disabled_and_open_guards() {
    assert_eq!(
        menu_trigger_open_focus_strategy("ArrowDown", false, false),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        menu_trigger_open_focus_strategy("ArrowDown", true, false),
        None
    );
    assert_eq!(
        menu_trigger_open_focus_strategy("ArrowDown", false, true),
        None
    );
}

#[test]
fn menu_open_focus_strategy_default_index_handles_empty_and_populated_lists() {
    assert_eq!(MenuOpenFocusStrategy::First.default_index(0), 0);
    assert_eq!(MenuOpenFocusStrategy::First.default_index(4), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(0), 0);
    assert_eq!(MenuOpenFocusStrategy::Last.default_index(4), 3);
}
