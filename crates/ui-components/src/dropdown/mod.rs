mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::DropdownMotion;
pub use view::Dropdown;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownStateInput {
    pub item_count: usize,
    pub disabled: bool,
    pub close_on_action: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub is_controlled: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
