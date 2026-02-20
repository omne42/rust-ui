mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, MenuItemSelectionIndicator};
pub use view::MenuItem;

use ui_headless::MenuItemKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItemStateInput {
    pub kind: MenuItemKind,
    pub is_checked: bool,
    pub disabled: bool,
    pub focused: bool,
    pub has_submenu: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItemState {
    pub role_attr: &'static str,
    pub kind_attr: &'static str,
    pub kind_class: &'static str,
    pub is_checkable: bool,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_focused: bool,
    pub has_submenu: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
