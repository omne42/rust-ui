use leptos::prelude::*;
use ui_headless::{NumberFieldOptions, use_number_field};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

fn poll_effects() {
    any_spawner::Executor::poll_local();
}

#[test]
fn number_field_allows_intermediate_input_and_commits_on_blur() {
    init_executor();

    let (value, set_value) = signal(0_i64);
    let on_value_change = Callback::new(move |next: i64| set_value.set(next));

    let aria = use_number_field(NumberFieldOptions {
        is_disabled: false,
        value: value.into(),
        on_value_change,
        min: Some(-10),
        max: Some(10),
        step: 1,
    });

    assert_eq!(aria.input_value.get_untracked(), "0");

    aria.handlers.on_focus.run(());
    aria.handlers.on_input.run("-".to_string());
    poll_effects();

    assert_eq!(aria.input_value.get_untracked(), "-");
    assert_eq!(value.get_untracked(), 0);

    aria.handlers.on_input.run("-3".to_string());
    poll_effects();

    assert_eq!(aria.input_value.get_untracked(), "-3");
    assert_eq!(value.get_untracked(), -3);

    aria.handlers.on_input.run("999".to_string());
    poll_effects();

    assert_eq!(aria.input_value.get_untracked(), "999");
    assert_eq!(value.get_untracked(), 10);

    aria.handlers.on_blur.run(());
    poll_effects();

    assert_eq!(aria.input_value.get_untracked(), "10");
}

#[test]
fn number_field_arrow_keys_step_and_request_prevent_default() {
    init_executor();

    let (value, set_value) = signal(0_i64);
    let on_value_change = Callback::new(move |next: i64| set_value.set(next));

    let aria = use_number_field(NumberFieldOptions {
        is_disabled: false,
        value: value.into(),
        on_value_change,
        min: None,
        max: None,
        step: 2,
    });

    let handled = aria.handlers.on_key_down.run("ArrowUp".to_string());
    poll_effects();
    assert!(handled);
    assert_eq!(value.get_untracked(), 2);
    assert_eq!(aria.input_value.get_untracked(), "2");

    let handled = aria.handlers.on_key_down.run("ArrowDown".to_string());
    poll_effects();
    assert!(handled);
    assert_eq!(value.get_untracked(), 0);
    assert_eq!(aria.input_value.get_untracked(), "0");
}

#[test]
fn number_field_syncs_input_value_when_not_editing() {
    init_executor();

    let (value, set_value) = signal(1_i64);
    let on_value_change = Callback::new(move |next: i64| set_value.set(next));

    let aria = use_number_field(NumberFieldOptions {
        is_disabled: false,
        value: value.into(),
        on_value_change,
        min: None,
        max: None,
        step: 1,
    });

    poll_effects();
    assert_eq!(aria.input_value.get_untracked(), "1");

    set_value.set(5);
    poll_effects();
    assert_eq!(aria.input_value.get_untracked(), "5");

    aria.handlers.on_focus.run(());
    aria.handlers.on_input.run("123".to_string());
    poll_effects();
    assert_eq!(aria.input_value.get_untracked(), "123");

    set_value.set(7);
    poll_effects();
    assert_eq!(aria.input_value.get_untracked(), "123");
}
