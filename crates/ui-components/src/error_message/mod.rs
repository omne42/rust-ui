mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorMessageElement, ErrorMessageTone};
pub use view::ErrorMessage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageStateInput {
    pub tone: ErrorMessageTone,
    pub disabled: bool,
    pub truncate: bool,
    pub has_custom_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageState {
    pub tone: ErrorMessageTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub data_state_attr: &'static str,
    pub message_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
