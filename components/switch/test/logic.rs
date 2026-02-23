use super::*;
use leptos::prelude::{Callable, GetUntracked, Update, signal};

#[test]
fn normalize_checked_axis_uses_controlled_mode_with_explicit_value() {
    let state = normalize_checked_axis(CheckedAxisInput {
        checked: Some(Signal::derive(|| true)),
        set_checked: None,
        default_checked: Some(false),
        on_checked_change: Some(Callback::new(|_| {})),
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode, SwitchCheckedControlMode::Controlled);
    assert_eq!(state.control_mode.data_attr(), "controlled");
    assert_eq!(state.checked_source_attr, "checked");
    assert_eq!(state.default_checked_source_attr, "provided");
    assert_eq!(state.checked_change_source_attr, "on_checked_change");
}

#[test]
fn normalize_checked_axis_defaults_to_uncontrolled_and_default_false() {
    let state = normalize_checked_axis(CheckedAxisInput {
        checked: None,
        set_checked: None,
        default_checked: None,
        on_checked_change: None,
    });

    assert!(!state.is_controlled);
    assert_eq!(state.control_mode, SwitchCheckedControlMode::Uncontrolled);
    assert_eq!(state.control_mode.data_attr(), "uncontrolled");
    assert_eq!(state.default_checked, DEFAULT_CHECKED);
    assert_eq!(state.checked_source_attr, "default");
    assert_eq!(state.default_checked_source_attr, "default");
    assert_eq!(state.checked_change_source_attr, "none");
}

#[test]
fn normalize_checked_axis_uses_provided_uncontrolled_default() {
    let state = normalize_checked_axis(CheckedAxisInput {
        checked: None,
        set_checked: None,
        default_checked: Some(true),
        on_checked_change: None,
    });

    assert!(!state.is_controlled);
    assert!(state.default_checked);
    assert_eq!(state.default_checked_source_attr, "provided");
}

#[test]
fn normalize_checked_axis_combines_legacy_and_new_handlers() {
    let (legacy_value, set_legacy_value) = signal(false);
    let (callback_count, set_callback_count) = signal(0_u8);
    let on_checked_change = Callback::new(move |_| {
        set_callback_count.update(|count| *count += 1);
    });

    let state = normalize_checked_axis(CheckedAxisInput {
        checked: Some(Signal::derive(|| true)),
        set_checked: Some(set_legacy_value),
        default_checked: Some(true),
        on_checked_change: Some(on_checked_change),
    });

    assert_eq!(
        state.checked_change_source_attr,
        "on_checked_change+set_checked"
    );

    let merged = state
        .on_checked_change
        .expect("merged callback should exist when both handlers are provided");
    merged.run(true);

    assert!(legacy_value.get_untracked());
    assert_eq!(callback_count.get_untracked(), 1);
}

#[test]
fn next_checked_toggles_boolean_value() {
    assert!(!next_checked(true));
    assert!(next_checked(false));
}
