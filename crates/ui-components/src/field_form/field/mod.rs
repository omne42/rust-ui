#[cfg(feature = "component-field_group")]
pub mod group;
pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldOrientation, FieldTone};
pub use motion::FieldMotion;
pub use view::Field;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldStateInput {
    pub orientation: FieldOrientation,
    pub tone: FieldTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldState {
    pub orientation: FieldOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: FieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
