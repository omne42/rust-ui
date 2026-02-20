use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn single_uncontrolled_updates_internal_state() {
    let mut state = use_single_selection_state(SingleSelectionStateOptions {
        default_selected_key: Some(SelectedKey::none()),
        ..Default::default()
    });

    assert!(matches!(state.selected_key(), SelectedKey::None));
    state.set_selected_key(SelectedKey::key("a"));
    assert_eq!(state.selected_key_str(), Some("a"));
    assert!(state.is_selected("a"));
}

#[test]
fn single_controlled_calls_on_change_but_does_not_update_internal() {
    let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);

    let mut state = use_single_selection_state(SingleSelectionStateOptions {
        selected_key: Some(SelectedKey::key("a")),
        on_selection_change: Some(Arc::new(move |v| {
            let mut guard = match called2.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(v);
        })),
        ..Default::default()
    });

    state.set_selected_key(SelectedKey::key("b"));
    let called_value = match called.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    assert_eq!(called_value, Some(SelectedKey::key("b")));
    assert_eq!(state.selected_key_str(), Some("a"));

    state.sync_controlled(Some(SelectedKey::key("b")));
    assert_eq!(state.selected_key_str(), Some("b"));
}

#[test]
fn single_controlled_none_is_representable() {
    let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);

    let mut state = use_single_selection_state(SingleSelectionStateOptions {
        selected_key: Some(SelectedKey::none()),
        on_selection_change: Some(Arc::new(move |v| {
            let mut guard = match called2.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(v);
        })),
        ..Default::default()
    });

    assert_eq!(state.selected_key_str(), None);
    state.set_selected_key(SelectedKey::key("a"));
    let called_value = match called.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    assert_eq!(called_value, Some(SelectedKey::key("a")));
    assert_eq!(state.selected_key_str(), None);
}

#[test]
fn multiple_uncontrolled_updates_internal_state() {
    let mut state = use_multiple_selection_state(MultipleSelectionStateOptions::default());

    assert!(!state.is_selected("a"));
    state.insert("a");
    assert!(state.is_selected("a"));
    state.remove("a");
    assert!(!state.is_selected("a"));
}

#[test]
fn multiple_controlled_calls_on_change_but_does_not_update_internal() {
    let called: Arc<Mutex<Option<BTreeSet<Key>>>> = Arc::new(Mutex::new(None));
    let called2 = Arc::clone(&called);

    let mut initial = BTreeSet::new();
    initial.insert("a".to_string());

    let mut state = use_multiple_selection_state(MultipleSelectionStateOptions {
        selected_keys: Some(initial.clone()),
        on_selection_change: Some(Arc::new(move |v| {
            let mut guard = match called2.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(v);
        })),
        ..Default::default()
    });

    state.insert("b");
    let called_value = match called.lock() {
        Ok(guard) => guard.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    };
    let called_value = match called_value {
        Some(value) => value,
        None => panic!("selection change callback was not called"),
    };
    assert!(called_value.contains("a"));
    assert!(called_value.contains("b"));

    // Internal state remains the controlled input until synced.
    assert!(state.is_selected("a"));
    assert!(!state.is_selected("b"));

    let mut next = BTreeSet::new();
    next.insert("b".to_string());
    state.sync_controlled(Some(next));
    assert!(!state.is_selected("a"));
    assert!(state.is_selected("b"));
}
