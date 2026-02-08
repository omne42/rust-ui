mod logic;
pub mod styles;
mod view;

pub use logic::{ActionGroupSelectionMode, ActionGroupTone, DEFAULT_ARIA_LABEL};
pub use view::ActionGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl ActionGroupItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionGroupStateInput {
    pub tone: ActionGroupTone,
    pub selection_mode: ActionGroupSelectionMode,
    pub disabled: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionGroupState {
    pub tone: ActionGroupTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub selection_mode: ActionGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
