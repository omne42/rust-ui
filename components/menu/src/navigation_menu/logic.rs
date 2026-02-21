use crate::navigation_menu::{
    NavigationMenuItem, NavigationMenuItemResolved, NavigationMenuPartState,
    NavigationMenuPartStateInput, NavigationMenuSlot,
};
use ui_state_primitives::menu as menu_state;

pub const DEFAULT_ID_BASE: &str = "navigation-menu";
pub const DEFAULT_ARIA_LABEL: &str = "Main navigation";
pub const DEFAULT_ACTIVATE_ON_FOCUS: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationSelectionTarget {
    Current,
    Index(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationKeyDecision {
    pub next_focus_index: Option<usize>,
    pub selection_target: Option<NavigationSelectionTarget>,
}

pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool) -> &'static str {
    menu_state::navigation_state_attr(item_count, has_selection, has_focus)
}

pub fn item_attr(item_count: usize) -> &'static str {
    menu_state::item_attr(item_count)
}

pub fn selected_attr(has_selection: bool) -> &'static str {
    menu_state::selected_attr(has_selection)
}

pub fn focus_attr(has_focus: bool) -> &'static str {
    menu_state::focus_attr(has_focus)
}

pub fn focus_activation_attr(activate_on_focus: bool) -> &'static str {
    menu_state::focus_activation_attr(activate_on_focus)
}

pub fn selection_mode_attr(is_controlled: bool) -> &'static str {
    menu_state::open_mode_attr(is_controlled)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    menu_state::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    menu_state::normalize_id_base(id_base, DEFAULT_ID_BASE)
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    menu_state::resolve_aria_label_with_fallback(value, DEFAULT_ARIA_LABEL, DEFAULT_ARIA_LABEL)
}

pub fn resolve_items(
    id_base: &str,
    items: Vec<NavigationMenuItem>,
) -> Vec<NavigationMenuItemResolved> {
    let primitive_items = items
        .into_iter()
        .map(|item| menu_state::NavigationItemInput {
            id: item.id,
            label: item.label,
            href: item.href,
            disabled: item.disabled,
        })
        .collect();

    menu_state::resolve_navigation_items(id_base, primitive_items)
        .into_iter()
        .map(|item| NavigationMenuItemResolved {
            id: item.id,
            dom_id: item.dom_id,
            label: item.label,
            href: item.href,
            disabled: item.disabled,
        })
        .collect()
}

pub fn first_enabled_index(items: &[NavigationMenuItemResolved]) -> Option<usize> {
    let disabled_flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
    menu_state::first_enabled_index(&disabled_flags)
}

pub fn last_enabled_index(items: &[NavigationMenuItemResolved]) -> Option<usize> {
    let disabled_flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
    menu_state::last_enabled_index(&disabled_flags)
}

pub fn next_enabled_index(
    items: &[NavigationMenuItemResolved],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    let disabled_flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
    menu_state::next_enabled_index(&disabled_flags, current_index, step)
}

pub fn selected_index_for_id(
    items: &[NavigationMenuItemResolved],
    selected_id: Option<String>,
) -> Option<usize> {
    let primitive_items: Vec<menu_state::NavigationItemResolved> = items
        .iter()
        .map(|item| menu_state::NavigationItemResolved {
            id: item.id.clone(),
            dom_id: item.dom_id.clone(),
            label: item.label.clone(),
            href: item.href.clone(),
            disabled: item.disabled,
        })
        .collect();
    menu_state::selected_index_for_id(&primitive_items, selected_id)
}

pub fn sanitize_selected_id(
    selected_id: Option<String>,
    items: &[NavigationMenuItemResolved],
) -> Option<String> {
    let primitive_items: Vec<menu_state::NavigationItemResolved> = items
        .iter()
        .map(|item| menu_state::NavigationItemResolved {
            id: item.id.clone(),
            dom_id: item.dom_id.clone(),
            label: item.label.clone(),
            href: item.href.clone(),
            disabled: item.disabled,
        })
        .collect();
    menu_state::sanitize_selected_id(selected_id, &primitive_items)
}

pub fn sanitize_focused_index(
    focused_index: Option<usize>,
    items: &[NavigationMenuItemResolved],
) -> Option<usize> {
    let disabled_flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
    menu_state::sanitize_enabled_index(focused_index, &disabled_flags)
}

pub fn resolve_active_index(
    items: &[NavigationMenuItemResolved],
    selected_index: Option<usize>,
    focused_index: Option<usize>,
) -> usize {
    selected_index
        .or(focused_index)
        .or_else(|| first_enabled_index(items))
        .unwrap_or(0)
}

pub fn resolve_option_id(items: &[NavigationMenuItemResolved], index: usize) -> String {
    items
        .get(index)
        .map(|item| item.dom_id.clone())
        .unwrap_or_default()
}

pub fn resolve_item_tabindex(item_disabled: bool, is_focused: bool) -> &'static str {
    if !item_disabled && is_focused {
        "0"
    } else {
        "-1"
    }
}

pub fn resolve_item_state_attr(
    item_disabled: bool,
    is_selected: bool,
    is_focused: bool,
) -> &'static str {
    if item_disabled {
        "disabled"
    } else if is_selected {
        "selected"
    } else if is_focused {
        "focused"
    } else {
        "idle"
    }
}

pub fn should_ignore_item_interaction(item_disabled: bool) -> bool {
    item_disabled
}

pub fn resolve_selected_id_for_target(
    items: &[NavigationMenuItemResolved],
    current_index: usize,
    target: NavigationSelectionTarget,
) -> Option<String> {
    let index = match target {
        NavigationSelectionTarget::Current => current_index,
        NavigationSelectionTarget::Index(index) => index,
    };
    items.get(index).map(|item| item.id.clone())
}

pub fn resolve_key_decision(
    key: &str,
    item_disabled: bool,
    current_index: usize,
    items: &[NavigationMenuItemResolved],
    activate_on_focus: bool,
) -> Option<NavigationKeyDecision> {
    let command = ui_headless::navigation_menu_key_command(key, item_disabled)?;
    match command {
        ui_headless::NavigationMenuKeyCommand::MoveNext => {
            let next_index = next_enabled_index(items, current_index, 1)?;
            Some(NavigationKeyDecision {
                next_focus_index: Some(next_index),
                selection_target: activate_on_focus
                    .then_some(NavigationSelectionTarget::Index(next_index)),
            })
        }
        ui_headless::NavigationMenuKeyCommand::MovePrevious => {
            let next_index = next_enabled_index(items, current_index, -1)?;
            Some(NavigationKeyDecision {
                next_focus_index: Some(next_index),
                selection_target: activate_on_focus
                    .then_some(NavigationSelectionTarget::Index(next_index)),
            })
        }
        ui_headless::NavigationMenuKeyCommand::First => {
            let next_index = first_enabled_index(items)?;
            Some(NavigationKeyDecision {
                next_focus_index: Some(next_index),
                selection_target: activate_on_focus
                    .then_some(NavigationSelectionTarget::Index(next_index)),
            })
        }
        ui_headless::NavigationMenuKeyCommand::Last => {
            let next_index = last_enabled_index(items)?;
            Some(NavigationKeyDecision {
                next_focus_index: Some(next_index),
                selection_target: activate_on_focus
                    .then_some(NavigationSelectionTarget::Index(next_index)),
            })
        }
        ui_headless::NavigationMenuKeyCommand::Activate => Some(NavigationKeyDecision {
            next_focus_index: None,
            selection_target: Some(NavigationSelectionTarget::Current),
        }),
    }
}

pub fn resolve_initial_focus_index(
    items: &[NavigationMenuItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    let disabled_flags: Vec<bool> = items.iter().map(|item| item.disabled).collect();
    menu_state::resolve_initial_focus_index(selected_index, &disabled_flags)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: NavigationMenuPartStateInput) -> NavigationMenuPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    NavigationMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.item_count, has_selection, has_focus),
        item_attr: item_attr(input.item_count),
        selected_attr: selected_attr(has_selection),
        focus_attr: focus_attr(has_focus),
        focus_activation_attr: focus_activation_attr(input.activate_on_focus),
        selection_mode_attr: selection_mode_attr(input.is_controlled),
        open_attr: has_selection.then_some("true"),
        closed_attr: (!has_selection).then_some("true"),
        item_count: input.item_count,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        is_empty,
        has_items,
        has_selection,
        has_focus,
        has_disabled_items: input.has_disabled_items,
        activate_on_focus: input.activate_on_focus,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_activate_on_focus: input.has_custom_activate_on_focus,
        has_custom_selected_id: input.has_custom_selected_id,
        has_custom_default_selected_id: input.has_custom_default_selected_id,
        has_custom_on_selected_id_change: input.has_custom_on_selected_id_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        activate_on_focus_source_attr: source_attr(input.has_custom_activate_on_focus),
        selected_id_source_attr: source_attr(input.has_custom_selected_id),
        default_selected_id_source_attr: source_attr(input.has_custom_default_selected_id),
        selected_id_change_source_attr: source_attr(input.has_custom_on_selected_id_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: NavigationMenuPartState,
) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, NavigationMenuSlot::Root) {
        if state.is_empty {
            classes.push("ui-navigation-menu--empty".to_string());
        } else {
            classes.push("ui-navigation-menu--has-items".to_string());
        }

        if state.has_selection {
            classes.push("ui-navigation-menu--selected".to_string());
        } else {
            classes.push("ui-navigation-menu--unselected".to_string());
        }

        if state.has_focus {
            classes.push("ui-navigation-menu--focused".to_string());
        }

        if state.has_disabled_items {
            classes.push("ui-navigation-menu--has-disabled-items".to_string());
        }

        if state.activate_on_focus {
            classes.push("ui-navigation-menu--auto-activation".to_string());
        } else {
            classes.push("ui-navigation-menu--manual-activation".to_string());
        }

        if state.is_controlled {
            classes.push("ui-navigation-menu--controlled".to_string());
        } else {
            classes.push("ui-navigation-menu--uncontrolled".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-navigation-menu--custom-motion".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-navigation-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-navigation-menu--custom-aria-label".to_string());
        }

        if state.has_custom_activate_on_focus {
            classes.push("ui-navigation-menu--custom-activate-on-focus".to_string());
        }

        if state.has_custom_selected_id {
            classes.push("ui-navigation-menu--custom-selected-id".to_string());
        }

        if state.has_custom_default_selected_id {
            classes.push("ui-navigation-menu--custom-default-selected-id".to_string());
        }

        if state.has_custom_on_selected_id_change {
            classes.push("ui-navigation-menu--custom-selected-id-change".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-navigation-menu--custom-class".to_string());
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
#[path = "../../test/navigation_menu/logic.rs"]
mod tests;
