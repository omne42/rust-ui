use std::sync::Arc;

#[cfg(test)]
use crate::menubar::MenubarMenuIds;
use crate::menubar::{
    MenuOpenFocusStrategy, MenubarMenu, MenubarMenuResolved, MenubarPartState,
    MenubarPartStateInput, MenubarSlot,
};
use ui_headless::PopoverPlacement;
use ui_state_primitives::menu as menu_state;

pub const DEFAULT_ID_BASE: &str = "menubar";
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenubarActionModeInput {
    pub is_close_on_action: Option<bool>,
    pub close_on_action: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenubarActionMode {
    #[default]
    CloseOnAction,
    KeepOpenOnAction,
}

impl MenubarActionMode {
    pub fn from_bool(close_on_action: bool) -> Self {
        if close_on_action {
            Self::CloseOnAction
        } else {
            Self::KeepOpenOnAction
        }
    }

    pub fn is_close_on_action(self) -> bool {
        matches!(self, Self::CloseOnAction)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenubarKeyDecision {
    OpenCurrent {
        focus: MenuOpenFocusStrategy,
    },
    MoveTo {
        index: usize,
        focus: MenuOpenFocusStrategy,
        focus_trigger: bool,
    },
    Close,
}

pub fn state_attr(menu_count: usize, has_open_menu: bool) -> &'static str {
    menu_state::menubar_state_attr(menu_count, has_open_menu)
}

pub fn menu_attr(menu_count: usize) -> &'static str {
    menu_state::menubar_menu_attr(menu_count)
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    menu_state::action_attr(close_on_action)
}

pub fn normalize_close_on_action(input: MenubarActionModeInput) -> MenubarActionMode {
    MenubarActionMode::from_bool(input.is_close_on_action.unwrap_or(input.close_on_action))
}

pub fn resolve_menu_state_attr(is_open: bool, is_trigger_disabled: bool) -> &'static str {
    if is_open {
        "open"
    } else if is_trigger_disabled {
        "disabled"
    } else {
        "closed"
    }
}

pub fn resolve_aria_expanded(is_open: bool) -> &'static str {
    if is_open { "true" } else { "false" }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    menu_state::open_mode_attr(is_controlled)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    menu_state::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    menu_state::normalize_id_base(id_base, DEFAULT_ID_BASE)
}

#[cfg(test)]
pub fn resolve_menu_ids(id_base: &str, menu_id: &str) -> MenubarMenuIds {
    let (trigger_id, menu_id) = menu_state::resolve_menubar_menu_ids(id_base, menu_id);
    MenubarMenuIds {
        trigger_id,
        menu_id,
    }
}

pub fn normalize_open_index(open_index: Option<usize>, menu_count: usize) -> Option<usize> {
    menu_state::normalize_index(open_index, menu_count)
}

#[cfg(test)]
pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    menu_state::normalize_disabled_indices(disabled_indices, item_count)
}
#[cfg(test)]
const _: fn(&str, &str) -> MenubarMenuIds = resolve_menu_ids;
#[cfg(test)]
const _: fn(Vec<usize>, usize) -> Vec<usize> = normalize_disabled_indices;

pub fn resolve_menus(id_base: &str, menus: Vec<MenubarMenu>) -> Vec<MenubarMenuResolved> {
    let mut item_kinds_by_index = Vec::with_capacity(menus.len());
    let mut primitive_inputs = Vec::with_capacity(menus.len());

    for menu in menus {
        item_kinds_by_index.push(menu.item_kinds);
        primitive_inputs.push(menu_state::MenubarMenuInput {
            id: menu.id,
            label: menu.label,
            items: menu.items,
            disabled_indices: menu.disabled_indices,
            disabled: menu.disabled,
        });
    }

    menu_state::resolve_menubar_menus(id_base, primitive_inputs)
        .into_iter()
        .enumerate()
        .map(|(index, resolved)| {
            let item_kinds = item_kinds_by_index
                .get(index)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .take(resolved.items.len())
                .collect();

            MenubarMenuResolved {
                id: resolved.id,
                label: resolved.label,
                items: Arc::<[String]>::from(resolved.items),
                disabled_indices: resolved.disabled_indices,
                item_kinds,
                is_trigger_disabled: resolved.is_trigger_disabled,
                has_items: resolved.has_items,
                trigger_id: resolved.trigger_id,
                menu_id: resolved.menu_id,
            }
        })
        .collect()
}

pub fn sanitize_open_index_for_menus(
    open_index: Option<usize>,
    menus: &[MenubarMenuResolved],
) -> Option<usize> {
    let trigger_disabled: Vec<bool> = menus.iter().map(|menu| menu.is_trigger_disabled).collect();
    menu_state::sanitize_open_index_for_trigger_disabled(open_index, &trigger_disabled)
}

pub fn normalize_default_open_index(
    default_open_index: Option<usize>,
    menu_count: usize,
    menus: &[MenubarMenuResolved],
) -> Option<usize> {
    sanitize_open_index_for_menus(normalize_open_index(default_open_index, menu_count), menus)
}

pub fn next_enabled_menu_index(
    menus: &[MenubarMenuResolved],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    let trigger_disabled: Vec<bool> = menus.iter().map(|menu| menu.is_trigger_disabled).collect();
    menu_state::next_enabled_index(&trigger_disabled, current_index, step)
}

pub fn resolve_next_open_index_on_trigger_press(
    is_trigger_disabled: bool,
    active_open: Option<usize>,
    index: usize,
) -> Option<Option<usize>> {
    if is_trigger_disabled {
        return None;
    }

    Some(if active_open == Some(index) {
        None
    } else {
        Some(index)
    })
}

pub fn resolve_next_open_index_on_pointer_enter(
    is_trigger_disabled: bool,
    active_open: Option<usize>,
    index: usize,
) -> Option<Option<usize>> {
    if is_trigger_disabled {
        return None;
    }

    if active_open.is_some() && active_open != Some(index) {
        return Some(Some(index));
    }

    None
}

pub fn resolve_key_decision(
    key: &str,
    is_trigger_disabled: bool,
    current_index: usize,
    menus: &[MenubarMenuResolved],
) -> Option<MenubarKeyDecision> {
    let command = ui_headless::menubar_key_command(key, is_trigger_disabled)?;
    match command {
        ui_headless::MenubarKeyCommand::OpenFirst => Some(MenubarKeyDecision::OpenCurrent {
            focus: MenuOpenFocusStrategy::First,
        }),
        ui_headless::MenubarKeyCommand::OpenLast => Some(MenubarKeyDecision::OpenCurrent {
            focus: MenuOpenFocusStrategy::Last,
        }),
        ui_headless::MenubarKeyCommand::MoveNext => {
            next_enabled_menu_index(menus, current_index, 1).map(|index| {
                MenubarKeyDecision::MoveTo {
                    index,
                    focus: MenuOpenFocusStrategy::First,
                    focus_trigger: true,
                }
            })
        }
        ui_headless::MenubarKeyCommand::MovePrevious => {
            next_enabled_menu_index(menus, current_index, -1).map(|index| {
                MenubarKeyDecision::MoveTo {
                    index,
                    focus: MenuOpenFocusStrategy::First,
                    focus_trigger: true,
                }
            })
        }
        ui_headless::MenubarKeyCommand::Close => Some(MenubarKeyDecision::Close),
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: MenubarPartStateInput) -> MenubarPartState {
    let has_menus = input.menu_count > 0;
    let has_open_menu = input.open_index.is_some();
    let is_empty = !has_menus;
    let keep_open_on_action = !input.close_on_action;

    MenubarPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.menu_count, has_open_menu),
        menu_attr: menu_attr(input.menu_count),
        action_attr: action_attr(input.close_on_action),
        open_mode_attr: open_mode_attr(input.is_controlled),
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        open_attr: has_open_menu.then_some("true"),
        closed_attr: (!has_open_menu).then_some("true"),
        menu_count: input.menu_count,
        open_index: input.open_index,
        has_open_menu,
        is_empty,
        has_menus,
        has_disabled_menus: input.has_disabled_menus,
        close_on_action: input.close_on_action,
        keep_open_on_action,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_close_on_action: input.has_custom_close_on_action,
        has_custom_placement: input.has_custom_placement,
        has_custom_open_index: input.has_custom_open_index,
        has_custom_default_open_index: input.has_custom_default_open_index,
        has_custom_on_open_index_change: input.has_custom_on_open_index_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        class_source_attr: source_attr(input.has_custom_class_name),
        close_on_action_source_attr: source_attr(input.has_custom_close_on_action),
        placement_source_attr: source_attr(input.has_custom_placement),
        open_index_source_attr: source_attr(input.has_custom_open_index),
        default_open_index_source_attr: source_attr(input.has_custom_default_open_index),
        open_index_change_source_attr: source_attr(input.has_custom_on_open_index_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: MenubarPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, MenubarSlot::Root) {
        classes.push(format!("ui-menubar--placement-{}", state.placement_attr));

        if state.is_empty {
            classes.push("ui-menubar--empty".to_string());
        } else {
            classes.push("ui-menubar--has-menus".to_string());
        }

        if state.has_open_menu {
            classes.push("ui-menubar--open".to_string());
        } else {
            classes.push("ui-menubar--closed".to_string());
        }

        if state.has_disabled_menus {
            classes.push("ui-menubar--has-disabled-menus".to_string());
        }

        if state.keep_open_on_action {
            classes.push("ui-menubar--persistent".to_string());
        } else {
            classes.push("ui-menubar--close-on-action".to_string());
        }

        if state.is_controlled {
            classes.push("ui-menubar--controlled".to_string());
        } else {
            classes.push("ui-menubar--uncontrolled".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-menubar--custom-motion".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-menubar--custom-id".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-menubar--custom-close-on-action".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-menubar--custom-placement".to_string());
        }

        if state.has_custom_open_index {
            classes.push("ui-menubar--custom-open-index".to_string());
        }

        if state.has_custom_default_open_index {
            classes.push("ui-menubar--custom-default-open-index".to_string());
        }

        if state.has_custom_on_open_index_change {
            classes.push("ui-menubar--custom-open-index-change".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-menubar--custom-class".to_string());
            if let Some(base_class_name) = normalize_optional_text(base_class_name) {
                classes.push(base_class_name);
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/menubar/logic.rs"]
mod tests;
