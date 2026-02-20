use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn uncontrolled_updates_internal_state() {
    let mut state = use_overlay_trigger_state(OverlayTriggerStateOptions {
        default_open: Some(false),
        ..Default::default()
    });

    assert!(!state.is_open());
    state.open();
    assert!(state.is_open());
    state.close();
    assert!(!state.is_open());
}

#[test]
fn controlled_calls_on_change_but_does_not_update_internal() {
    let called: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);

    let mut state = use_overlay_trigger_state(OverlayTriggerStateOptions {
        is_open: Some(false),
        on_open_change: Some(Arc::new(move |v| {
            let mut guard = match called2.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(v);
        })),
        ..Default::default()
    });

    state.open();
    let called_value = match called.lock() {
        Ok(guard) => *guard,
        Err(poisoned) => *poisoned.into_inner(),
    };
    assert_eq!(called_value, Some(true));
    assert!(!state.is_open());

    state.sync_controlled(Some(true));
    assert!(state.is_open());
}
