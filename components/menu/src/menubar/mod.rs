mod logic;
pub mod styles;
mod view;

pub use crate::dropdown_menu::DropdownMenuMotion as MenubarMotion;
pub use logic::{DEFAULT_CLOSE_ON_ACTION, DEFAULT_ID_BASE, DEFAULT_PLACEMENT};
pub use view::Menubar;

use std::sync::Arc;

use crate::MenuItemKind;
use ui_headless::PopoverPlacement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenu {
    pub id: String,
    pub label: String,
    pub items: Vec<String>,
    pub disabled_indices: Vec<usize>,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled: bool,
}

impl MenubarMenu {
    pub fn new(id: impl Into<String>, label: impl Into<String>, items: Vec<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            items,
            disabled_indices: Vec::new(),
            item_kinds: Vec::new(),
            disabled: false,
        }
    }

    pub fn disabled_indices(mut self, disabled_indices: Vec<usize>) -> Self {
        self.disabled_indices = disabled_indices;
        self
    }

    pub fn item_kinds(mut self, item_kinds: Vec<MenuItemKind>) -> Self {
        self.item_kinds = item_kinds;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenubarSlot {
    Root,
    Menu,
    Trigger,
}

impl MenubarSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            MenubarSlot::Root => "menubar",
            MenubarSlot::Menu => "menubar-menu",
            MenubarSlot::Trigger => "menubar-trigger",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            MenubarSlot::Root => "ui-menubar",
            MenubarSlot::Menu => "ui-menubar__menu",
            MenubarSlot::Trigger => "ui-menubar__trigger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuOpenFocusStrategy {
    #[default]
    First,
    Last,
}

impl MenuOpenFocusStrategy {
    pub fn default_index(self, item_count: usize) -> usize {
        match self {
            Self::First => 0,
            Self::Last => item_count.saturating_sub(1),
        }
    }
}

pub fn focus_strategy_for_open_key(key: &str) -> Option<MenuOpenFocusStrategy> {
    match key {
        "ArrowDown" => Some(MenuOpenFocusStrategy::First),
        "ArrowUp" => Some(MenuOpenFocusStrategy::Last),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenubarMenuResolved {
    pub id: String,
    pub label: String,
    pub items: Arc<[String]>,
    pub disabled_indices: Vec<usize>,
    pub item_kinds: Vec<MenuItemKind>,
    pub is_trigger_disabled: bool,
    pub has_items: bool,
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenubarPartStateInput {
    pub slot: MenubarSlot,
    pub menu_count: usize,
    pub open_index: Option<usize>,
    pub has_disabled_menus: bool,
    pub close_on_action: bool,
    pub is_controlled: bool,
    pub placement: PopoverPlacement,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open_index: bool,
    pub has_custom_default_open_index: bool,
    pub has_custom_on_open_index_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenubarPartState {
    pub slot: MenubarSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub menu_attr: &'static str,
    pub action_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
    pub open_attr: Option<&'static str>,
    pub closed_attr: Option<&'static str>,
    pub menu_count: usize,
    pub open_index: Option<usize>,
    pub has_open_menu: bool,
    pub is_empty: bool,
    pub has_menus: bool,
    pub has_disabled_menus: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_class_name: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub has_custom_open_index: bool,
    pub has_custom_default_open_index: bool,
    pub has_custom_on_open_index_change: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub close_on_action_source_attr: &'static str,
    pub placement_source_attr: &'static str,
    pub open_index_source_attr: &'static str,
    pub default_open_index_source_attr: &'static str,
    pub open_index_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
