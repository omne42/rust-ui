use super::*;
use crate::selection::{OnSingleSelectionChange, SelectedKey};
use std::sync::{Arc, Mutex};

fn keys(values: &[&str]) -> Vec<Key> {
    values.iter().map(|v| (*v).to_string()).collect()
}

#[test]
fn uncontrolled_select_next_and_prev_wraps() {
    let mut state = use_list_state(ListStateOptions {
        items: keys(&["a", "b", "c"]),
        selection: SingleSelectionStateOptions::default(),
    });

    assert_eq!(state.selected_key_str(), None);

    state.select_next();
    assert_eq!(state.selected_key_str(), Some("a"));

    state.select_next();
    assert_eq!(state.selected_key_str(), Some("b"));

    state.select_prev();
    assert_eq!(state.selected_key_str(), Some("a"));

    state.select_prev();
    assert_eq!(state.selected_key_str(), Some("c"));
}

#[test]
fn controlled_selection_does_not_update_internal() {
    let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);
    let on_selection_change: OnSingleSelectionChange = Arc::new(move |v| {
        let mut guard = match called2.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = Some(v);
    });

    let mut state = use_list_state(ListStateOptions {
        items: keys(&["a", "b"]),
        selection: SingleSelectionStateOptions {
            selected_key: Some(SelectedKey::key("a")),
            on_selection_change: Some(on_selection_change),
            ..Default::default()
        },
    });

    state.select_next();
    let called_value = match called.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    assert_eq!(called_value, Some(SelectedKey::key("b")));

    // Still controlled by input until synced.
    assert_eq!(state.selected_key_str(), Some("a"));
}
