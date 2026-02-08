mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, FieldErrorTone};
pub use view::FieldError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldErrorStateInput {
    pub tone: FieldErrorTone,
    pub visible: bool,
    pub disabled: bool,
    pub show_icon: bool,
    pub has_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldErrorState {
    pub tone: FieldErrorTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_visible: bool,
    pub is_disabled: bool,
    pub show_icon: bool,
    pub has_message: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub message_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
