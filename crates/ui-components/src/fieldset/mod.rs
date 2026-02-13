mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldsetOrientation, FieldsetTone};
pub use motion::FieldsetMotion;
pub use view::Fieldset;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetStateInput {
    pub orientation: FieldsetOrientation,
    pub tone: FieldsetTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_legend: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_actions: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetState {
    pub orientation: FieldsetOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: FieldsetTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_legend: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_actions: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}
