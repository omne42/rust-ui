mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, FieldGroupDensity, FieldGroupOrientation};
pub use view::FieldGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldGroupStateInput {
    pub orientation: FieldGroupOrientation,
    pub density: FieldGroupDensity,
    pub disabled: bool,
    pub invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldGroupState {
    pub orientation: FieldGroupOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub density: FieldGroupDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub label_attr: &'static str,
    pub has_description: bool,
    pub description_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub state_attr: &'static str,
}
