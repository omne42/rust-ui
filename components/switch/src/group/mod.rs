mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, DEFAULT_LABEL, SwitchGroupOrientation,
    SwitchGroupTone,
};
pub use view::SwitchGroup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchGroupIds {
    pub root_id: String,
    pub label_id: String,
    pub description_id: String,
    pub error_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchGroupStateInput {
    pub orientation: SwitchGroupOrientation,
    pub tone: SwitchGroupTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchGroupState {
    pub orientation: SwitchGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: SwitchGroupTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub shows_error: bool,
    pub has_messages: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
