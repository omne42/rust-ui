use std::sync::Arc;

pub type ToggleOnChange = Arc<dyn Fn(bool) + Send + Sync>;

#[derive(Clone)]
pub struct ToggleState {
    is_selected: bool,
    default_selected: bool,
    is_read_only: bool,
    is_controlled: bool,
    on_change: Option<ToggleOnChange>,
}

#[derive(Clone, Default)]
pub struct ToggleStateOptions {
    pub is_selected: Option<bool>,
    pub default_selected: Option<bool>,
    pub is_read_only: bool,
    pub on_change: Option<ToggleOnChange>,
}

pub fn use_toggle_state(options: ToggleStateOptions) -> ToggleState {
    let is_controlled = options.is_selected.is_some();
    let initial_selected = options
        .is_selected
        .or(options.default_selected)
        .unwrap_or(false);

    ToggleState {
        is_selected: initial_selected,
        default_selected: options.default_selected.unwrap_or(initial_selected),
        is_read_only: options.is_read_only,
        is_controlled,
        on_change: options.on_change,
    }
}

impl ToggleState {
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn default_selected(&self) -> bool {
        self.default_selected
    }

    pub fn is_read_only(&self) -> bool {
        self.is_read_only
    }

    pub fn is_controlled(&self) -> bool {
        self.is_controlled
    }

    pub fn sync_controlled(&mut self, is_selected: Option<bool>) {
        self.is_controlled = is_selected.is_some();
        if let Some(is_selected) = is_selected {
            self.is_selected = is_selected;
        }
    }

    pub fn set_selected(&mut self, is_selected: bool) {
        if self.is_read_only || is_selected == self.is_selected {
            return;
        }

        if let Some(on_change) = &self.on_change {
            on_change(is_selected);
        }

        if !self.is_controlled {
            self.is_selected = is_selected;
        }
    }

    pub fn toggle(&mut self) {
        let next = !self.is_selected;
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
