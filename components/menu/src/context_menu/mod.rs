mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use crate::dropdown_menu::DropdownMenuMotion as ContextMenuMotion;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_CLOSE_ON_ACTION, DEFAULT_DISABLED, DEFAULT_ID_BASE,
    DEFAULT_PLACEMENT,
};
pub use view::ContextMenu;

use ui_headless::PopoverPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextMenuSlot {
    Root,
    Trigger,
}

impl ContextMenuSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            ContextMenuSlot::Root => "context-menu",
            ContextMenuSlot::Trigger => "context-menu-trigger",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            ContextMenuSlot::Root => "ui-context-menu",
            ContextMenuSlot::Trigger => "ui-context-menu__trigger",
        }
    }
}

pub type MenuOpenFocusStrategy = ui_headless::MenuOpenFocusStrategy;

pub fn focus_strategy_for_open_key(key: &str, shift_key: bool) -> Option<MenuOpenFocusStrategy> {
    ui_headless::context_menu_open_focus_strategy_for_key(key, shift_key)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextMenuPartStateInput {
    pub slot: ContextMenuSlot,
    pub is_open: bool,
    pub item_count: usize,
    pub trigger_disabled: bool,
    pub close_on_action: bool,
    pub placement: PopoverPlacement,
    pub is_controlled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_disabled_indices: bool,
    pub has_custom_item_kinds: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextMenuPartState {
    pub slot: ContextMenuSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub disabled_attr: &'static str,
    pub action_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub closed_attr: Option<&'static str>,
    pub is_open: bool,
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_trigger_disabled: bool,
    pub is_enabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_disabled_indices: bool,
    pub has_custom_item_kinds: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub disabled_indices_source_attr: &'static str,
    pub item_kinds_source_attr: &'static str,
    pub close_on_action_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub open_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
