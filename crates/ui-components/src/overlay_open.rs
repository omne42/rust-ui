use leptos::prelude::*;

#[derive(Clone)]
pub(crate) struct ControllableOpenState {
    pub open: Signal<bool>,
    pub request_open_change: Callback<bool>,
}

pub(crate) fn use_controllable_open_state(
    open: Option<Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
) -> ControllableOpenState {
    let (uncontrolled_open, set_uncontrolled_open) = signal(default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let open = open.unwrap_or(uncontrolled_open.into());

    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_| {}));
    let request_open_change: Callback<bool> = Callback::new(move |next_open: bool| {
        if next_open == open.get_untracked() {
            return;
        }
        on_open_change.run(next_open);
        if !is_controlled {
            set_uncontrolled_open.set(next_open);
        }
    });

    ControllableOpenState {
        open,
        request_open_change,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
