mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_DESCRIPTION, DEFAULT_TITLE, EmptyStateAlign, EmptyStateTone,
};
pub use view::EmptyState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateStateInput {
    pub tone: EmptyStateTone,
    pub align: EmptyStateAlign,
    pub compact: bool,
    pub bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateState {
    pub tone: EmptyStateTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub align: EmptyStateAlign,
    pub align_class: &'static str,
    pub align_attr: &'static str,
    pub is_compact: bool,
    pub is_bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub data_state_attr: &'static str,
    pub title_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
