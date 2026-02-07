mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, DEFAULT_PLACEHOLDER, TimeFieldIds, TimeFieldTone,
};
pub use view::TimeField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldStateInput {
    pub tone: TimeFieldTone,
    pub disabled: bool,
    pub has_value: bool,
    pub minute_step: u8,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeFieldState {
    pub tone: TimeFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub has_value: bool,
    pub is_empty: bool,
    pub minute_step: u8,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
