mod logic;
mod render;
pub mod styles;

pub use logic::{ContentTone, DEFAULT_ARIA_LABEL};
pub use render::Content;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContentStateInput {
    pub tone: ContentTone,
    pub padded: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContentState {
    pub tone: ContentTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_padded: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
