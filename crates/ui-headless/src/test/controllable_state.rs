use super::*;
use std::collections::BTreeSet;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn uncontrolled_open_updates_state_and_calls_on_change() {
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let state = use_controllable_open_state_traced(
        "tests",
        None,
        Some(false),
        Some(Callback::new(move |next: bool| {
            assert!(next);
            called2.fetch_add(1, Ordering::SeqCst);
        })),
    );

    assert!(!state.open.get_untracked());
    state.request_open_change.run(true);
    assert!(state.open.get_untracked());
    assert_eq!(called.load(Ordering::SeqCst), 1);
}

#[test]
fn request_open_change_ignores_noop_updates() {
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let state = use_controllable_open_state_traced(
        "tests",
        None,
        Some(false),
        Some(Callback::new(move |_| {
            called2.fetch_add(1, Ordering::SeqCst);
        })),
    );

    state.request_open_change.run(false);
    assert_eq!(called.load(Ordering::SeqCst), 0);
}

#[test]
fn controlled_open_does_not_update_internal_state() {
    let (controlled, set_controlled) = signal(false);
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let state = use_controllable_open_state_traced(
        "tests",
        Some(controlled.into()),
        Some(false),
        Some(Callback::new(move |next: bool| {
            assert!(next);
            called2.fetch_add(1, Ordering::SeqCst);
        })),
    );

    state.request_open_change.run(true);
    assert!(!state.open.get_untracked());
    assert_eq!(called.load(Ordering::SeqCst), 1);

    set_controlled.set(true);
    assert!(state.open.get_untracked());
}

#[test]
fn controlled_open_ignores_default_open_value() {
    let (controlled, _set_controlled) = signal(false);

    let state = use_controllable_open_state_traced(
        "tests",
        Some(controlled.into()),
        Some(true),
        Some(Callback::new(|_: bool| {})),
    );

    assert!(
        !state.open.get_untracked(),
        "controlled value must stay the single source of truth, default_open must not override it"
    );
}

#[test]
fn controlled_state_without_on_change_is_read_only() {
    let (controlled, _set_controlled) = signal(false);

    let state = use_controllable_state(Some(controlled.into()), Some(true), None);
    state.request_change.run(true);

    assert!(
        !state.value.get_untracked(),
        "controlled state without callback must not mutate internal value implicitly"
    );
}

#[test]
fn uncontrolled_state_supports_non_copy_values() {
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let mut initial = BTreeSet::new();
    initial.insert(1);

    let state = use_controllable_state(
        None,
        Some(initial.clone()),
        Some(Callback::new(move |next: BTreeSet<usize>| {
            assert!(next.contains(&2));
            called2.fetch_add(1, Ordering::SeqCst);
        })),
    );

    assert_eq!(state.value.get_untracked(), initial);

    let mut next = BTreeSet::new();
    next.insert(2);
    state.request_change.run(next.clone());
    assert_eq!(state.value.get_untracked(), next);
    assert_eq!(called.load(Ordering::SeqCst), 1);
}
