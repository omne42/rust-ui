use std::sync::Arc;

pub type OverlayOnOpenChange = Arc<dyn Fn(bool) + Send + Sync>;

#[derive(Clone)]
pub struct OverlayTriggerState {
    is_open: bool,
    default_open: bool,
    is_controlled: bool,
    on_open_change: Option<OverlayOnOpenChange>,
}

#[derive(Clone, Default)]
pub struct OverlayTriggerStateOptions {
    pub is_open: Option<bool>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<OverlayOnOpenChange>,
}

pub fn use_overlay_trigger_state(options: OverlayTriggerStateOptions) -> OverlayTriggerState {
    let is_controlled = options.is_open.is_some();
    let initial_open = options.is_open.or(options.default_open).unwrap_or(false);

    OverlayTriggerState {
        is_open: initial_open,
        default_open: options.default_open.unwrap_or(initial_open),
        is_controlled,
        on_open_change: options.on_open_change,
    }
}

impl OverlayTriggerState {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn default_open(&self) -> bool {
        self.default_open
    }

    pub fn is_controlled(&self) -> bool {
        self.is_controlled
    }

    pub fn sync_controlled(&mut self, is_open: Option<bool>) {
        self.is_controlled = is_open.is_some();
        if let Some(is_open) = is_open {
            self.is_open = is_open;
        }
    }

    pub fn open(&mut self) {
        self.set_open(true);
    }

    pub fn close(&mut self) {
        self.set_open(false);
    }

    pub fn toggle(&mut self) {
        let next = !self.is_open;
        self.set_open(next);
    }

    pub fn set_open(&mut self, is_open: bool) {
        if is_open == self.is_open {
            return;
        }

        if let Some(on_open_change) = &self.on_open_change {
            on_open_change(is_open);
        }

        if !self.is_controlled {
            self.is_open = is_open;
        }
    }
}

#[cfg(test)]
mod tests {
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
            on_open_change: Some(Arc::new(move |v| *called2.lock().unwrap() = Some(v))),
            ..Default::default()
        });

        state.open();
        assert_eq!(*called.lock().unwrap(), Some(true));
        assert!(!state.is_open());

        state.sync_controlled(Some(true));
        assert!(state.is_open());
    }
}
