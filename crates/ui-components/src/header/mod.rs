mod logic;
mod render;
pub mod styles;

pub use logic::{DEFAULT_ARIA_LABEL, HeaderTone};
pub use render::Header;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderStateInput {
    pub tone: HeaderTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderState {
    pub tone: HeaderTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_bordered: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
