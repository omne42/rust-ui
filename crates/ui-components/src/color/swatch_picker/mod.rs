pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{ColorSwatchPickerItem, DEFAULT_ARIA_LABEL};
pub use motion::ColorSwatchPickerMotion;
pub use view::ColorSwatchPicker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchPickerStateInput {
    pub disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub disabled_item_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchPickerState {
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub is_empty: bool,
    pub has_items: bool,
    pub disabled_item_count: usize,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
