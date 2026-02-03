use crate::controlled::{
    ControlledOnChange, ControlledState, ControlledStateOptions, use_controlled_state,
};

pub type ToggleOnChange = ControlledOnChange<bool>;

#[derive(Clone)]
pub struct ToggleState {
    selected: ControlledState<bool>,
    is_read_only: bool,
}

#[derive(Clone, Default)]
pub struct ToggleStateOptions {
    pub is_selected: Option<bool>,
    pub default_selected: Option<bool>,
    pub is_read_only: bool,
    pub on_change: Option<ToggleOnChange>,
}

pub fn use_toggle_state(options: ToggleStateOptions) -> ToggleState {
    ToggleState {
        selected: use_controlled_state(
            false,
            ControlledStateOptions {
                value: options.is_selected,
                default_value: options.default_selected,
                on_change: options.on_change,
            },
        ),
        is_read_only: options.is_read_only,
    }
}

impl ToggleState {
    pub fn is_selected(&self) -> bool {
        *self.selected.value()
    }

    pub fn default_selected(&self) -> bool {
        *self.selected.default_value()
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn is_controlled(&self) -> bool {
        self.selected.is_controlled()
    }

    pub fn sync_controlled(&mut self, is_selected: Option<bool>) {
        self.selected.sync_controlled(is_selected);
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        if self.is_read_only {
            return;
        }

        self.selected.set_value(is_selected);
    }

    pub fn toggle(&mut self) {
        let next = !self.is_selected();
        self.set_selected(next);
    }
}

#[cfg(test)]
mod tests {
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
            on_change: Some(Arc::new(move |v| *called2.lock().unwrap() = Some(v))),
            ..Default::default()
        });

        state.set_selected(true);
        assert_eq!(*called.lock().unwrap(), Some(true));
        assert!(!state.is_selected());

        state.sync_controlled(Some(true));
        assert!(state.is_selected());
    }
}
