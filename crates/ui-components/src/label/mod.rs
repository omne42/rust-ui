mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, LabelEmphasis};
pub use view::Label;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelStateInput {
    pub emphasis: LabelEmphasis,
    pub required: bool,
    pub disabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelState {
    pub emphasis: LabelEmphasis,
    pub emphasis_class: &'static str,
    pub emphasis_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
