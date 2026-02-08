mod logic;
mod render;
pub mod styles;

pub use logic::{DEFAULT_ARIA_LABEL, KeyboardTone};
pub use render::Keyboard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardStateInput {
    pub tone: KeyboardTone,
    pub compact: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardState {
    pub tone: KeyboardTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_compact: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
