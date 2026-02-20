use super::*;

fn init_executor() {
    drop(any_spawner::Executor::init_futures_executor());
}

#[test]
fn default_index_skips_disabled_items() {
    init_executor();
    let (count, _set_count) = signal(3_usize);
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: count,
        is_item_disabled: Some(Callback::new(|index: usize| index == 0)),
    });

    any_spawner::Executor::poll_local();
    assert_eq!(roving.active_index.get_untracked(), 1);
}

#[test]
fn default_index_last_skips_to_previous_enabled_item() {
    init_executor();
    let (count, _set_count) = signal(4_usize);
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 3,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: count,
        is_item_disabled: Some(Callback::new(|index: usize| matches!(index, 0 | 3))),
    });

    any_spawner::Executor::poll_local();
    assert_eq!(roving.active_index.get_untracked(), 2);
}

#[test]
fn arrow_navigation_skips_disabled_items_and_loops() {
    init_executor();
    let (count, _set_count) = signal(3_usize);
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: count,
        is_item_disabled: Some(Callback::new(|index: usize| index == 1)),
    });

    any_spawner::Executor::poll_local();
    assert_eq!(roving.active_index.get_untracked(), 0);

    roving.handlers.on_key_down.run("ArrowDown".to_string());
    assert_eq!(roving.active_index.get_untracked(), 2);

    roving.handlers.on_key_down.run("ArrowDown".to_string());
    assert_eq!(roving.active_index.get_untracked(), 0);

    roving.handlers.on_key_down.run("ArrowUp".to_string());
    assert_eq!(roving.active_index.get_untracked(), 2);
}

#[test]
fn focus_ignores_disabled_items() {
    init_executor();
    let (count, _set_count) = signal(3_usize);
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: count,
        is_item_disabled: Some(Callback::new(|index: usize| index == 2)),
    });

    any_spawner::Executor::poll_local();
    roving.handlers.on_item_focus.run(2);
    assert_eq!(roving.active_index.get_untracked(), 0);
}

#[test]
fn home_and_end_select_first_and_last_enabled() {
    init_executor();
    let (count, _set_count) = signal(4_usize);
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: count,
        is_item_disabled: Some(Callback::new(|index: usize| matches!(index, 0 | 3))),
    });

    any_spawner::Executor::poll_local();
    roving.handlers.on_key_down.run("Home".to_string());
    assert_eq!(roving.active_index.get_untracked(), 1);

    roving.handlers.on_key_down.run("End".to_string());
    assert_eq!(roving.active_index.get_untracked(), 2);
}
