use leptos::prelude::*;

use crate::trace::{UiTraceEventKind, use_ui_trace};

#[derive(Clone)]
pub struct ControllableOpenState {
    pub open: Signal<bool>,
    pub request_open_change: Callback<bool>,
}

#[derive(Clone)]
pub struct ControllableState<T>
where
    T: Send + Sync + 'static,
{
    pub value: Signal<T>,
    pub request_change: Callback<T>,
}

pub fn use_controllable_state<T>(
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

pub fn use_controllable_open_state_traced(
    component: &'static str,
    open: Option<Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
) -> ControllableOpenState {
    let state = use_controllable_state(open, default_open, on_open_change);
    let trace = use_ui_trace();

    let request_open_change: Callback<bool> = Callback::new(move |next| {
        if next == state.value.get_untracked() {
            return;
        }
        if let Some(trace) = trace {
            trace.emit(component, UiTraceEventKind::OpenChange { open: next });
        }
        state.request_change.run(next);
    });

    ControllableOpenState {
        open: state.value,
        request_open_change,
    }
}

#[cfg(test)]
#[path = "test/controllable_state.rs"]
mod tests;
