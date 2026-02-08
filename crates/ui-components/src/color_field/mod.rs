mod logic;
pub mod styles;
mod view;

pub use view::ColorField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldStateInput {
    pub disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldState {
    pub is_disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
