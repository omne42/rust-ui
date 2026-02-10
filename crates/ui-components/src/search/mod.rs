mod logic;
pub mod styles;
mod view;

pub use view::Search;

pub const DEFAULT_LABEL: &str = "Search";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchStateInput {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub invalid: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_custom_submit_handler: bool,
    pub has_custom_clear_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub submit_handler_source_attr: &'static str,
    pub clear_handler_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}
