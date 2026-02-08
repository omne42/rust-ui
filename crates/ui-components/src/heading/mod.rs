mod logic;
mod render;
pub mod styles;

pub use logic::{DEFAULT_ARIA_LABEL, HeadingLevel, HeadingTone};
pub use render::Heading;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadingStateInput {
    pub level: HeadingLevel,
    pub tone: HeadingTone,
    pub truncate: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadingState {
    pub level: HeadingLevel,
    pub level_class: &'static str,
    pub level_attr: &'static str,
    pub tone: HeadingTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_truncated: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
