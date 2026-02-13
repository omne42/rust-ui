mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, HelpTextTone};
pub use motion::HelpTextMotion;
pub use view::HelpText;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextStateInput {
    pub tone: HelpTextTone,
    pub invalid: bool,
    pub disabled: bool,
    pub show_error_icon: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpTextState {
    pub tone: HelpTextTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_invalid: bool,
    pub is_disabled: bool,
    pub show_error_icon: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
