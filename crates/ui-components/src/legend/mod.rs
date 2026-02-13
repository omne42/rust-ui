mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, LegendTone};
pub use motion::LegendMotion;
pub use view::Legend;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegendStateInput {
    pub tone: LegendTone,
    pub required: bool,
    pub disabled: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegendState {
    pub tone: LegendTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
    pub text_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
