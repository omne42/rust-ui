mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, ListBoxSectionHeadingTone};
pub use view::ListBoxSection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxSectionStateInput {
    pub heading_tone: ListBoxSectionHeadingTone,
    pub item_count: usize,
    pub disabled: bool,
    pub sticky_heading: bool,
    pub show_divider: bool,
    pub has_title: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxSectionState {
    pub heading_tone: ListBoxSectionHeadingTone,
    pub heading_tone_class: &'static str,
    pub heading_tone_attr: &'static str,
    pub item_count: usize,
    pub has_items: bool,
    pub is_empty: bool,
    pub is_disabled: bool,
    pub has_title: bool,
    pub is_sticky_heading: bool,
    pub has_divider: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub title_source_attr: &'static str,
}
