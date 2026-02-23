use leptos::prelude::*;
use ui_headless::{InputOtpOptions, input_otp_slot_selection_range, use_input_otp};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

#[test]
fn input_otp_filters_digits_and_limits_length() {
    init_executor();

    let (value, set_value) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_value.set(next));

    let completed: StoredValue<Vec<String>> = StoredValue::new(Vec::new());
    let on_complete = Callback::new(move |code: String| {
        completed.update_value(|items| items.push(code));
    });

    let otp = use_input_otp(InputOtpOptions {
        is_disabled: false,
        length: 6,
        value: value.into(),
        on_value_change,
        on_complete: Some(on_complete),
    });

    otp.handlers.on_input.run("a1b2c3d4e5f6g7".to_string());

    assert_eq!(value.get_untracked(), "123456");
    assert_eq!(otp.input_value.get_untracked(), "123456");
    assert_eq!(otp.caret_index.get_untracked(), 6);
    assert_eq!(completed.get_value().as_slice(), &["123456".to_string()]);
}

#[test]
fn input_otp_on_complete_fires_once_per_fill_cycle() {
    init_executor();

    let (value, set_value) = signal(String::new());
    let on_value_change = Callback::new(move |next: String| set_value.set(next));

    let completed: StoredValue<Vec<String>> = StoredValue::new(Vec::new());
    let on_complete = Callback::new(move |code: String| {
        completed.update_value(|items| items.push(code));
    });

    let otp = use_input_otp(InputOtpOptions {
        is_disabled: false,
        length: 4,
        value: value.into(),
        on_value_change,
        on_complete: Some(on_complete),
    });

    otp.handlers.on_input.run("1234".to_string());
    otp.handlers.on_input.run("1234".to_string());

    assert_eq!(completed.get_value().len(), 1);

    otp.handlers.on_input.run("123".to_string());
    otp.handlers.on_input.run("1234".to_string());

    assert_eq!(
        completed.get_value().as_slice(),
        &["1234".to_string(), "1234".to_string()]
    );
}

#[test]
fn input_otp_caret_change_clamps_to_length() {
    init_executor();

    let (value, set_value) = signal("12".to_string());
    let on_value_change = Callback::new(move |next: String| set_value.set(next));

    let otp = use_input_otp(InputOtpOptions {
        is_disabled: false,
        length: 6,
        value: value.into(),
        on_value_change,
        on_complete: None,
    });

    otp.handlers.on_focus.run(());
    assert_eq!(otp.caret_index.get_untracked(), 2);
    assert_eq!(otp.active_slot.get_untracked(), 2);

    otp.handlers.on_caret_change.run(10);
    assert_eq!(otp.caret_index.get_untracked(), 6);
    assert_eq!(otp.active_slot.get_untracked(), 5);
}

#[test]
fn input_otp_ignores_handlers_when_disabled() {
    init_executor();

    let (value, set_value) = signal("12".to_string());
    let on_value_change = Callback::new(move |next: String| set_value.set(next));

    let otp = use_input_otp(InputOtpOptions {
        is_disabled: true,
        length: 6,
        value: value.into(),
        on_value_change,
        on_complete: None,
    });

    otp.handlers.on_focus.run(());
    otp.handlers.on_input.run("999".to_string());
    otp.handlers.on_caret_change.run(3);

    assert_eq!(value.get_untracked(), "12");
    assert!(!otp.is_focused.get_untracked());
    assert_eq!(otp.caret_index.get_untracked(), 0);
}

#[test]
fn input_otp_slot_selection_range_clamps_within_value_length() {
    assert_eq!(input_otp_slot_selection_range(0, 0), (0, 0));
    assert_eq!(input_otp_slot_selection_range(0, 4), (0, 1));
    assert_eq!(input_otp_slot_selection_range(2, 4), (2, 3));
    assert_eq!(input_otp_slot_selection_range(4, 4), (4, 4));
    assert_eq!(input_otp_slot_selection_range(9, 4), (4, 4));
}
