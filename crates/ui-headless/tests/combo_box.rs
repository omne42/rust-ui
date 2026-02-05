use leptos::prelude::*;
use ui_headless::{ComboBoxOptions, use_combo_box};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

fn poll_effects() {
    any_spawner::Executor::poll_local();
}

#[test]
fn combobox_opens_on_arrow_down_and_commits_selection() {
    init_executor();

    let (open, set_open) = signal(false);
    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(None::<usize>);

    let last_action: StoredValue<Option<usize>> = StoredValue::new(None);
    let on_action = Callback::new(move |index: usize| last_action.set_value(Some(index)));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: Some(on_action),
        is_item_disabled: None,
    });

    assert!(!open.get_untracked());
    assert_eq!(aria.input.aria_expanded.get_untracked(), Some("false"));
    assert_eq!(aria.input.aria_activedescendant.get_untracked(), None);

    // ArrowDown opens and focuses the first option.
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert!(open.get_untracked());
    assert_eq!(aria.input.aria_expanded.get_untracked(), Some("true"));
    assert_eq!(
        aria.input.aria_activedescendant.get_untracked(),
        Some("demo-option-0".to_string())
    );

    // Move active index and commit via Enter.
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert!(aria.handlers.on_input_key_down.run("Enter".to_string()));

    assert_eq!(last_action.get_value(), Some(1));
    assert!(!open.get_untracked());

    // Sync active index from selected index.
    set_selected.set(Some(2));
    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 2);
}

#[test]
fn combobox_opens_on_arrow_up_and_focuses_last_enabled_option() {
    init_executor();

    let (open, set_open) = signal(false);
    let (count, _set_count) = signal(3_usize);
    let (selected, _set_selected) = signal(Some(0_usize));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: None,
        is_item_disabled: Some(Callback::new(|index: usize| index == 2)),
    });

    poll_effects();
    assert!(!open.get_untracked());

    assert!(aria.handlers.on_input_key_down.run("ArrowUp".to_string()));
    assert!(open.get_untracked());

    // Option 2 is disabled, so ArrowUp should focus option 1.
    assert_eq!(
        aria.input.aria_activedescendant.get_untracked(),
        Some("demo-option-1".to_string())
    );
    assert_eq!(aria.active_index.get_untracked(), 1);
}

#[test]
fn combobox_home_does_not_interfere_with_text_editing_when_closed() {
    init_executor();

    let (open, set_open) = signal(false);
    let (count, _set_count) = signal(3_usize);
    let (selected, _set_selected) = signal(None::<usize>);

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: None,
        is_item_disabled: None,
    });

    assert!(!aria.handlers.on_input_key_down.run("Home".to_string()));
    assert!(!open.get_untracked());
}

#[test]
fn combobox_tab_commits_active_option_and_allows_focus_to_move() {
    init_executor();

    let (open, set_open) = signal(false);
    let (count, _set_count) = signal(3_usize);
    let (selected, _set_selected) = signal(None::<usize>);

    let last_action: StoredValue<Option<usize>> = StoredValue::new(None);
    let on_action = Callback::new(move |index: usize| last_action.set_value(Some(index)));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: Some(on_action),
        is_item_disabled: None,
    });

    // Open, then move active to index 2.
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert_eq!(aria.active_index.get_untracked(), 2);

    // Tab should commit but not request preventDefault.
    assert!(!aria.handlers.on_input_key_down.run("Tab".to_string()));
    assert_eq!(last_action.get_value(), Some(2));
    assert!(!open.get_untracked());
}

#[test]
fn combobox_option_click_calls_on_action_and_closes() {
    init_executor();

    let (open, set_open) = signal(true);
    let (count, _set_count) = signal(3_usize);
    let (selected, _set_selected) = signal(None::<usize>);

    let called: StoredValue<Vec<usize>> = StoredValue::new(Vec::new());
    let on_action = Callback::new(move |index: usize| called.update_value(|v| v.push(index)));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: Some(on_action),
        is_item_disabled: None,
    });

    aria.handlers.on_option_click.run(2);
    assert_eq!(called.get_value(), vec![2]);
    assert!(!open.get_untracked());
}

#[test]
fn disabled_combobox_ignores_input_events() {
    init_executor();

    let (open, set_open) = signal(false);
    let (count, _set_count) = signal(3_usize);
    let (selected, _set_selected) = signal(None::<usize>);

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: true,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: Callback::new(move |next: bool| set_open.set(next)),
        item_count: count,
        selected_index: selected.into(),
        on_action: None,
        is_item_disabled: None,
    });

    assert!(!aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert!(!open.get_untracked());
}

#[test]
fn controlled_combobox_requests_open_change_without_mutating_open_signal() {
    init_executor();

    let (open, _set_open) = signal(false);
    let (count, _set_count) = signal(1_usize);
    let (selected, _set_selected) = signal(None::<usize>);

    let requested: StoredValue<Vec<bool>> = StoredValue::new(Vec::new());
    let on_open_change = Callback::new(move |next: bool| requested.update_value(|v| v.push(next)));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "demo".to_string(),
        is_open: open.into(),
        set_open: on_open_change,
        item_count: count,
        selected_index: selected.into(),
        on_action: None,
        is_item_disabled: None,
    });

    assert!(!open.get_untracked());
    assert!(aria.handlers.on_input_key_down.run("ArrowDown".to_string()));
    assert_eq!(requested.get_value(), vec![true]);
    assert!(!open.get_untracked());
}
