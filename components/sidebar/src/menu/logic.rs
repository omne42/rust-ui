use crate::sidebar_menu::{DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};
use std::collections::BTreeSet;
use ui_state_primitives::sidebar_menu as primitives;

pub use primitives::{SidebarMenuItem, SidebarMenuSubItem};

const DEFAULT_ITEM_ACTION_LABEL: &str = "item action";
const DEFAULT_SUBMENU_TOGGLE_LABEL: &str = "Toggle submenu";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuState {
    pub item_count: usize,
    pub has_items: bool,
    pub is_empty: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub show_badges: bool,
    pub show_actions: bool,
    pub allow_submenu_collapse: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
    pub state_attr: &'static str,
    pub control_attr: &'static str,
    pub class_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarMenuStateInput {
    pub item_count: usize,
    pub disabled: bool,
    pub show_badges: bool,
    pub show_actions: bool,
    pub allow_submenu_collapse: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
    pub has_shortcut: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    primitives::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    primitives::normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.into())
}

pub fn normalize_id_base(value: Option<String>) -> String {
    primitives::normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn resolve_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn resolve_show_badges(is_badges_visible: Option<bool>, show_badges: bool) -> bool {
    is_badges_visible.unwrap_or(show_badges)
}

pub fn resolve_show_actions(is_actions_visible: Option<bool>, show_actions: bool) -> bool {
    is_actions_visible.unwrap_or(show_actions)
}

pub fn resolve_allow_submenu_collapse(
    is_submenu_collapse_allowed: Option<bool>,
    allow_submenu_collapse: bool,
) -> bool {
    is_submenu_collapse_allowed.unwrap_or(allow_submenu_collapse)
}

pub fn resolve_keyboard_shortcut_enabled(
    is_keyboard_shortcut_enabled: Option<bool>,
    enable_keyboard_shortcut: bool,
) -> bool {
    is_keyboard_shortcut_enabled.unwrap_or(enable_keyboard_shortcut)
}

pub fn normalize_keyboard_shortcut_key(
    keyboard_shortcut_key: Option<String>,
    enable_keyboard_shortcut: bool,
) -> Option<String> {
    normalize_optional_text(keyboard_shortcut_key)
        .map(|key| key.to_ascii_lowercase())
        .filter(|_| enable_keyboard_shortcut)
}

pub fn normalize_item_action_label(action_label: Option<String>) -> String {
    normalize_optional_text(action_label).unwrap_or_else(|| DEFAULT_ITEM_ACTION_LABEL.to_string())
}

pub fn normalize_submenu_toggle_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_SUBMENU_TOGGLE_LABEL.to_string())
}

pub fn selection_state_attr(active_id: Option<String>) -> &'static str {
    if active_id.is_some() {
        "selected"
    } else {
        "none"
    }
}

pub fn normalize_items(items: Vec<SidebarMenuItem>) -> Vec<SidebarMenuItem> {
    primitives::normalize_items(items)
}

pub fn default_open_sub_ids(items: &[SidebarMenuItem]) -> Vec<String> {
    primitives::default_open_sub_ids(items)
}

pub fn default_open_sub_id_set(items: &[SidebarMenuItem]) -> BTreeSet<String> {
    let derived: BTreeSet<String> = default_open_sub_ids(items).into_iter().collect();
    debug_assert_eq!(derived, primitives::default_open_sub_id_set(items));
    derived
}

pub fn default_active_id(items: &[SidebarMenuItem], requested: Option<String>) -> Option<String> {
    primitives::default_active_id(items, requested)
}

pub fn contains_id(items: &[SidebarMenuItem], id: &str) -> bool {
    primitives::contains_id(items, id)
}

pub fn first_enabled_id(items: &[SidebarMenuItem]) -> Option<String> {
    primitives::first_enabled_id(items)
}

pub fn next_enabled_id(
    items: &[SidebarMenuItem],
    current: Option<String>,
    step: i32,
) -> Option<String> {
    primitives::next_enabled_id(items, current, step)
}

pub fn linear_enabled_ids(items: &[SidebarMenuItem]) -> Vec<String> {
    primitives::linear_enabled_ids(items)
}

pub fn active_index_for_current(items: &[SidebarMenuItem], current: Option<&str>) -> usize {
    primitives::active_index_for_current(items, current)
}

pub fn toggle_open_sub_ids(
    open_sub_ids: &BTreeSet<String>,
    id: &str,
    items: &[SidebarMenuItem],
) -> BTreeSet<String> {
    primitives::toggle_open_sub_id(open_sub_ids, id, items)
}

pub fn resolve_state(input: SidebarMenuStateInput) -> SidebarMenuState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    SidebarMenuState {
        item_count: input.item_count,
        has_items,
        is_empty,
        disabled: input.disabled,
        enabled,
        show_badges: input.show_badges,
        show_actions: input.show_actions,
        allow_submenu_collapse: input.allow_submenu_collapse,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_class_name: input.has_custom_class_name,
        has_shortcut: input.has_shortcut,
        state_attr: if input.disabled && is_empty {
            "disabled-empty"
        } else if input.disabled {
            "disabled"
        } else if is_empty {
            "empty"
        } else {
            "ready"
        },
        control_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SidebarMenuState) -> String {
    let mut classes = vec!["ui-sidebar-menu".to_string()];

    if state.disabled {
        classes.push("ui-sidebar-menu--disabled".to_string());
    }

    if state.is_empty {
        classes.push("ui-sidebar-menu--empty".to_string());
    }

    if state.show_badges {
        classes.push("ui-sidebar-menu--with-badges".to_string());
    }

    if state.show_actions {
        classes.push("ui-sidebar-menu--with-actions".to_string());
    }

    if state.allow_submenu_collapse {
        classes.push("ui-sidebar-menu--collapsible-sub".to_string());
    }

    if state.has_shortcut {
        classes.push("ui-sidebar-menu--with-shortcut".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar-menu--controlled".to_string());
    } else {
        classes.push("ui-sidebar-menu--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-menu--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/menu/logic.rs"]
mod tests;
