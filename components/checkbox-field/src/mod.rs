mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    CheckboxFieldIndicatorPlacement, CheckboxFieldTone, DEFAULT_ARIA_LABEL, DEFAULT_LABEL,
};
pub use motion::CheckboxFieldMotion;
pub use view::CheckboxField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldStateInput {
    pub checked: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub tone: CheckboxFieldTone,
    pub indicator_placement: CheckboxFieldIndicatorPlacement,
    pub has_description: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldState {
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub tone: CheckboxFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub indicator_placement: CheckboxFieldIndicatorPlacement,
    pub indicator_placement_class: &'static str,
    pub indicator_placement_attr: &'static str,
    pub has_description: bool,
    pub description_attr: &'static str,
    pub has_custom_label: bool,
    pub label_source_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub state_attr: &'static str,
}
