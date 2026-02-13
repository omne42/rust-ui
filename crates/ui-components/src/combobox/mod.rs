mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::ComboboxMotion;
pub use view::Combobox;

pub const DEFAULT_LABEL: &str = "Combobox";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboboxStateInput {
    pub item_count: usize,
    pub disabled_option_count: usize,
    pub selected_index: Option<usize>,
    pub required: bool,
    pub invalid: bool,
    pub disabled: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboboxState {
    pub state_attr: &'static str,
    pub selection_attr: &'static str,
    pub options_attr: &'static str,
    pub requirement_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_disabled_options: bool,
}
