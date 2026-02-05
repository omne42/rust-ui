use leptos::prelude::*;

#[derive(Clone)]
pub(crate) struct ControllableOpenState {
    pub open: Signal<bool>,
    pub request_open_change: Callback<bool>,
}

#[derive(Clone)]
pub(crate) struct ControllableState<T>
where
    T: Send + Sync + 'static,
{
    pub value: Signal<T>,
    pub request_change: Callback<T>,
}

pub(crate) fn use_controllable_state<T>(
    value: Option<Signal<T>>,
    default_value: Option<T>,
    on_change: Option<Callback<T>>,
) -> ControllableState<T>
where
    T: Clone + Default + PartialEq + Send + Sync + 'static,
{
    let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.unwrap_or_default());
    let is_controlled = value.is_some();
    let value = value.unwrap_or(uncontrolled_value.into());

    let on_change = on_change.unwrap_or_else(|| Callback::new(|_| {}));
    let request_change: Callback<T> = Callback::new(move |next: T| {
        if next == value.get_untracked() {
            return;
        }
        on_change.run(next.clone());
        if !is_controlled {
            set_uncontrolled_value.set(next);
        }
    });

    ControllableState {
        value,
        request_change,
    }
}

pub(crate) fn use_controllable_open_state(
    open: Option<Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
) -> ControllableOpenState {
    let state = use_controllable_state(open, default_open, on_open_change);

    ControllableOpenState {
        open: state.value,
        request_open_change: state.request_change,
    }
}

#[cfg(test)]
mod tests {
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

        let state = use_controllable_open_state(
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

        let state = use_controllable_open_state(
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

        let state = use_controllable_open_state(
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
}
