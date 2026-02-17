mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, ToggleGroupOrientation, ToggleGroupSelectionMode};
pub use view::ToggleGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleGroupItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl ToggleGroupItem {
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
pub struct ToggleGroupStateInput {
    pub orientation: ToggleGroupOrientation,
    pub selection_mode: ToggleGroupSelectionMode,
    pub disabled: bool,
    pub attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleGroupState {
    pub orientation: ToggleGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub selection_mode: ToggleGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub is_disabled: bool,
    pub is_attached: bool,
    pub item_count: usize,
    pub selected_count: usize,
    pub disabled_item_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
