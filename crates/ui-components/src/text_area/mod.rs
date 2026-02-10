mod logic;
pub mod styles;
mod view;

pub use view::TextArea;

pub const DEFAULT_LABEL: &str = "Text area";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaStateInput {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub invalid: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_rows: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub rows_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
