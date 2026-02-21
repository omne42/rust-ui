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

#[test]
fn context_menu_open_focus_strategy_maps_context_shortcuts() {
    assert_eq!(
        context_menu_open_focus_strategy_for_key("ContextMenu", false),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        context_menu_open_focus_strategy_for_key("F10", true),
        Some(MenuOpenFocusStrategy::First)
    );
    assert_eq!(
        context_menu_open_focus_strategy_for_key("ArrowUp", false),
        Some(MenuOpenFocusStrategy::Last)
    );
    assert_eq!(context_menu_open_focus_strategy_for_key("F10", false), None);
    assert_eq!(
        context_menu_open_focus_strategy("ArrowDown", false, true, false),
        None
    );
    assert_eq!(
        context_menu_open_focus_strategy("ArrowDown", false, false, true),
        None
    );
}

#[test]
fn menubar_key_command_covers_navigation_and_open_close() {
    assert_eq!(
        menubar_key_command("ArrowDown", false),
        Some(MenubarKeyCommand::OpenFirst)
    );
    assert_eq!(
        menubar_key_command("ArrowUp", false),
        Some(MenubarKeyCommand::OpenLast)
    );
    assert_eq!(
        menubar_key_command("ArrowRight", false),
        Some(MenubarKeyCommand::MoveNext)
    );
    assert_eq!(
        menubar_key_command("ArrowLeft", false),
        Some(MenubarKeyCommand::MovePrevious)
    );
    assert_eq!(
        menubar_key_command("Escape", false),
        Some(MenubarKeyCommand::Close)
    );
    assert_eq!(menubar_key_command("ArrowDown", true), None);
}

#[test]
fn navigation_menu_key_command_maps_roving_and_activation_keys() {
    assert_eq!(
        navigation_menu_key_command("ArrowRight", false),
        Some(NavigationMenuKeyCommand::MoveNext)
    );
    assert_eq!(
        navigation_menu_key_command("ArrowLeft", false),
        Some(NavigationMenuKeyCommand::MovePrevious)
    );
    assert_eq!(
        navigation_menu_key_command("Home", false),
        Some(NavigationMenuKeyCommand::First)
    );
    assert_eq!(
        navigation_menu_key_command("End", false),
        Some(NavigationMenuKeyCommand::Last)
    );
    assert_eq!(
        navigation_menu_key_command("Enter", false),
        Some(NavigationMenuKeyCommand::Activate)
    );
    assert_eq!(
        navigation_menu_key_command(" ", false),
        Some(NavigationMenuKeyCommand::Activate)
    );
    assert_eq!(navigation_menu_key_command("Home", true), None);
}
