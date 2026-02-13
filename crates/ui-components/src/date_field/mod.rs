mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, DEFAULT_PLACEHOLDER, DateFieldIds, DateFieldTone,
};
pub use motion::DateFieldMotion;
pub use view::DateField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateFieldStateInput {
    pub tone: DateFieldTone,
    pub disabled: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateFieldState {
    pub tone: DateFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub has_value: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
