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
#[path = "test/toggle.rs"]
mod tests;
