use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn enter_does_not_trigger_when_activation_keys_disallow_enter() {
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let press = use_press(PressOptions {
        on_press: Some(Callback::new(move |_| {
            called2.fetch_add(1, Ordering::SeqCst);
        })),
        activation_keys: PressActivationKeys::SPACE,
        ..Default::default()
    });

    press.handlers.on_key_down.run("Enter".to_string());
    press.handlers.on_key_up.run("Enter".to_string());

    assert_eq!(called.load(Ordering::SeqCst), 0);
}

#[test]
fn space_does_not_trigger_when_activation_keys_disallow_space() {
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let press = use_press(PressOptions {
        on_press: Some(Callback::new(move |_| {
            called2.fetch_add(1, Ordering::SeqCst);
        })),
        activation_keys: PressActivationKeys::ENTER,
        ..Default::default()
    });

    press.handlers.on_key_down.run(" ".to_string());
    press.handlers.on_key_up.run(" ".to_string());

    assert_eq!(called.load(Ordering::SeqCst), 0);
}
