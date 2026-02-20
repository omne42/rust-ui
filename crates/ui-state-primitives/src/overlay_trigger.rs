use crate::controlled::{
    ControlledOnChange, ControlledState, ControlledStateOptions, use_controlled_state,
};

pub type OverlayOnOpenChange = ControlledOnChange<bool>;

#[derive(Clone)]
pub struct OverlayTriggerState {
    open: ControlledState<bool>,
}

#[derive(Clone, Default)]
pub struct OverlayTriggerStateOptions {
    pub is_open: Option<bool>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<OverlayOnOpenChange>,
}

pub fn use_overlay_trigger_state(options: OverlayTriggerStateOptions) -> OverlayTriggerState {
    OverlayTriggerState {
        open: use_controlled_state(
            false,
            ControlledStateOptions {
                value: options.is_open,
                default_value: options.default_open,
                on_change: options.on_open_change,
            },
        ),
    }
}

impl OverlayTriggerState {
    pub fn is_open(&self) -> bool {
        *self.open.value()
    }

    pub fn default_open(&self) -> bool {
        *self.open.default_value()
    }

    pub fn is_controlled(&self) -> bool {
        self.open.is_controlled()
    }

    pub fn sync_controlled(&mut self, is_open: Option<bool>) {
        self.open.sync_controlled(is_open);
    }

    pub fn open(&mut self) {
        self.set_open(true);
    }

    pub fn close(&mut self) {
        self.set_open(false);
    }

    pub fn toggle(&mut self) {
        let next = !self.is_open();
        self.set_open(next);
    }

    pub fn set_open(&mut self, is_open: bool) {
        self.open.set_value(is_open);
    }
}

#[cfg(test)]
#[path = "test/overlay_trigger.rs"]
mod tests;
