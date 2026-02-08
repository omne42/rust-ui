mod logic;
mod render;
pub mod styles;

pub use logic::{DEFAULT_ARIA_LABEL, FooterTone};
pub use render::Footer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterStateInput {
    pub tone: FooterTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterState {
    pub tone: FooterTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_bordered: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
