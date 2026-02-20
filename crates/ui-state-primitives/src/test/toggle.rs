use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn uncontrolled_updates_internal_state() {
    let mut state = use_toggle_state(ToggleStateOptions {
        default_selected: Some(false),
        ..Default::default()
    });

    assert!(!state.is_selected());
    state.set_selected(true);
    assert!(state.is_selected());
}

#[test]
fn read_only_does_not_change() {
    let mut state = use_toggle_state(ToggleStateOptions {
        default_selected: Some(false),
        is_read_only: true,
        ..Default::default()
    });

    state.toggle();
    assert!(!state.is_selected());
}

#[test]
fn controlled_calls_on_change_but_does_not_update_internal() {
    let called: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);

    let mut state = use_toggle_state(ToggleStateOptions {
        is_selected: Some(false),
        on_change: Some(Arc::new(move |v| {
            let mut guard = match called2.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(v);
        })),
        ..Default::default()
    });

    state.set_selected(true);
    let called_value = match called.lock() {
        Ok(guard) => *guard,
        Err(poisoned) => *poisoned.into_inner(),
    };
    assert_eq!(called_value, Some(true));
    assert!(!state.is_selected());

    state.sync_controlled(Some(true));
    assert!(state.is_selected());
}
