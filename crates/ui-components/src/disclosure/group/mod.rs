mod logic;
pub mod styles;
mod view;

pub use crate::accordion::AccordionMotion as DisclosureGroupMotion;
pub use logic::{DEFAULT_ARIA_LABEL, DisclosureGroupSelectionMode};
pub use view::DisclosureGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureGroupStateInput {
    pub selection_mode: DisclosureGroupSelectionMode,
    pub item_count: usize,
    pub expanded_count: usize,
    pub disabled: bool,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisclosureGroupState {
    pub selection_mode: DisclosureGroupSelectionMode,
    pub selection_mode_class: &'static str,
    pub selection_mode_attr: &'static str,
    pub item_count: usize,
    pub expanded_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_expanded_items: bool,
    pub has_multiple_expanded: bool,
    pub is_disabled: bool,
    pub has_disabled_items: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
