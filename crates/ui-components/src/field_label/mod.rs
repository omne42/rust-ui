mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, FieldLabelTone};
pub use view::FieldLabel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabelStateInput {
    pub tone: FieldLabelTone,
    pub required: bool,
    pub disabled: bool,
    pub has_for_id: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabelState {
    pub tone: FieldLabelTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_for_id: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub text_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
