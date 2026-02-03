use leptos::prelude::*;
use ui_headless::{RadioGroupOptions, RovingOrientation, use_radio_group};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

fn poll_effects() {
    any_spawner::Executor::poll_local();
}

#[test]
fn radio_group_arrow_keys_update_selection() {
    init_executor();

    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(Some(0_usize));

    let called: StoredValue<Vec<usize>> = StoredValue::new(Vec::new());
    let on_change = Callback::new(move |index: usize| called.update_value(|v| v.push(index)));

    let aria = use_radio_group(RadioGroupOptions {
        is_disabled: false,
        id_base: "rg".to_string(),
        orientation: RovingOrientation::Horizontal,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_change: Some(on_change),
        is_item_disabled: None,
    });

    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 0);
    assert_eq!(aria.attrs.role, "radiogroup");

    assert!(aria.handlers.on_key_down.run("ArrowRight".to_string()));
    assert_eq!(selected.get_untracked(), Some(1));
    assert_eq!(called.get_value(), vec![1]);
}

#[test]
fn radio_group_skips_disabled_items() {
    init_executor();

    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(Some(0_usize));

    let aria = use_radio_group(RadioGroupOptions {
        is_disabled: false,
        id_base: "rg".to_string(),
        orientation: RovingOrientation::Horizontal,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_change: None,
        is_item_disabled: Some(Callback::new(|index: usize| index == 1)),
    });

    poll_effects();
    assert_eq!(aria.active_index.get_untracked(), 0);

    assert!(aria.handlers.on_key_down.run("ArrowRight".to_string()));
    assert_eq!(selected.get_untracked(), Some(2));
}

#[test]
fn radio_group_click_selects_item() {
    init_executor();

    let (count, _set_count) = signal(2_usize);
    let (selected, set_selected) = signal(None::<usize>);

    let aria = use_radio_group(RadioGroupOptions {
        is_disabled: false,
        id_base: "rg".to_string(),
        orientation: RovingOrientation::Vertical,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_change: None,
        is_item_disabled: None,
    });

    aria.handlers.on_radio_click.run(1);
    assert_eq!(selected.get_untracked(), Some(1));
}
