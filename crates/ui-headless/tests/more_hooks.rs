use leptos::prelude::*;
use ui_headless::{
    FocusWithinOptions, ListBoxOptions, MenuItemKind, MenuItemOptions, MenuOptions, PressOptions,
    use_focus_within, use_listbox, use_menu, use_menu_item, use_press,
};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

fn poll_effects() {
    any_spawner::Executor::poll_local();
}

#[test]
fn focus_within_tracks_focus_in_and_out() {
    init_executor();

    let focus = use_focus_within(FocusWithinOptions { is_disabled: false });
    assert!(!focus.is_focus_within.get_untracked());

    focus.handlers.on_focus_in.run(());
    assert!(focus.is_focus_within.get_untracked());

    focus.handlers.on_focus_out.run(());
    assert!(!focus.is_focus_within.get_untracked());
}

#[test]
fn focus_within_disabled_ignores_focus_in() {
    init_executor();

    let focus = use_focus_within(FocusWithinOptions { is_disabled: true });
    focus.handlers.on_focus_in.run(());
    assert!(!focus.is_focus_within.get_untracked());
}

#[test]
fn listbox_syncs_active_index_to_selected_index() {
    init_executor();

    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(Some(2_usize));

    let aria = use_listbox(ListBoxOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "sync".to_string(),
        default_index: 0,
        sync_active_index_to_selected: true,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_action: None,
        is_item_disabled: None,
        item_text: None,
    });

    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 2);

    set_selected.set(Some(0));
    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 0);
}

#[test]
fn listbox_can_disable_sync_to_support_focus_strategy() {
    init_executor();

    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(Some(2_usize));

    let aria = use_listbox(ListBoxOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "nosync".to_string(),
        default_index: 0,
        sync_active_index_to_selected: false,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_action: None,
        is_item_disabled: None,
        item_text: None,
    });

    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 0);
}

#[test]
fn menu_item_roles_and_aria_checked_match_kind() {
    init_executor();

    let (count, _set_count) = signal(1_usize);
    let menu = use_menu(MenuOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "demo".to_string(),
        item_count: count,
        default_index: 0,
        on_action: None,
        is_item_disabled: None,
        item_text: None,
    });

    let action = use_menu_item(
        &menu,
        MenuItemOptions {
            index: 0,
            kind: MenuItemKind::Action,
            is_disabled: false,
        },
    );
    assert_eq!(action.attrs.role, "menuitem");
    assert_eq!(action.attrs.aria_checked.get_untracked(), None);

    let (checked, set_checked) = signal(false);
    let radio = use_menu_item(
        &menu,
        MenuItemOptions {
            index: 0,
            kind: MenuItemKind::Radio {
                is_checked: checked.into(),
            },
            is_disabled: false,
        },
    );
    assert_eq!(radio.attrs.role, "menuitemradio");
    assert_eq!(radio.attrs.aria_checked.get_untracked(), Some("false"));
    set_checked.set(true);
    assert_eq!(radio.attrs.aria_checked.get_untracked(), Some("true"));
}

#[test]
fn menu_item_disabled_short_circuits_handlers_and_sets_aria_disabled() {
    init_executor();

    let (count, _set_count) = signal(2_usize);
    let activated: StoredValue<Vec<usize>> = StoredValue::new(Vec::new());
    let on_action = Callback::new(move |index: usize| activated.update_value(|v| v.push(index)));

    let menu = use_menu(MenuOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "demo".to_string(),
        item_count: count,
        default_index: 0,
        on_action: Some(on_action),
        is_item_disabled: None,
        item_text: None,
    });

    poll_effects();
    assert_eq!(menu.active_index.get_untracked(), 0);

    let disabled = use_menu_item(
        &menu,
        MenuItemOptions {
            index: 1,
            kind: MenuItemKind::Action,
            is_disabled: true,
        },
    );
    assert_eq!(disabled.attrs.aria_disabled, Some("true"));

    disabled.handlers.on_pointer_move.run(());
    assert_eq!(menu.active_index.get_untracked(), 0);
    disabled.handlers.on_click.run(());
    assert_eq!(activated.get_value(), Vec::<usize>::new());
}

#[test]
fn menu_default_index_skips_disabled_items() {
    init_executor();

    let (count, _set_count) = signal(4_usize);
    let menu = use_menu(MenuOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "demo".to_string(),
        item_count: count,
        default_index: 3,
        on_action: None,
        is_item_disabled: Some(Callback::new(|index: usize| matches!(index, 0 | 3))),
        item_text: None,
    });

    poll_effects();
    assert_eq!(menu.active_index.get_untracked(), 2);
}

#[test]
fn press_pointer_cancel_and_blur_clear_pressed_state() {
    init_executor();

    let calls = StoredValue::new(0_usize);
    let on_press = Callback::new(move |_| calls.update_value(|n| *n += 1));

    let press = use_press(PressOptions {
        on_press: Some(on_press),
        ..Default::default()
    });

    press.handlers.on_pointer_down.run(());
    assert!(press.is_pressed.get_untracked());

    press.handlers.on_pointer_cancel.run(());
    assert!(!press.is_pressed.get_untracked());

    // Cancelled pointer press should not invoke on_press on pointer up.
    press.handlers.on_pointer_up.run(());
    assert_eq!(calls.get_value(), 0);

    press.handlers.on_pointer_down.run(());
    assert!(press.is_pressed.get_untracked());
    press.handlers.on_blur.run(());
    assert!(!press.is_pressed.get_untracked());
}
