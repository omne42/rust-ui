mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, ListBoxItemSelectionIndicator};
pub use view::ListBoxItem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxItemStateInput {
    pub selected: bool,
    pub focused: bool,
    pub disabled: bool,
    pub show_selection_indicator: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxItemState {
    pub is_selected: bool,
    pub is_focused: bool,
    pub is_disabled: bool,
    pub show_selection_indicator: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub selection_indicator_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
