mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_LABEL};
pub use view::ColorPicker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerStateInput {
    pub disabled: bool,
    pub open: bool,
    pub has_selection: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub is_open_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorPickerState {
    pub is_disabled: bool,
    pub is_open: bool,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub data_state_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
