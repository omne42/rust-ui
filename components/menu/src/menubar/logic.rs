use std::collections::BTreeSet;
use std::sync::Arc;

use crate::menubar::{
    MenubarMenu, MenubarMenuIds, MenubarMenuResolved, MenubarPartState, MenubarPartStateInput,
    MenubarSlot,
};
use ui_headless::PopoverPlacement;

pub const DEFAULT_ID_BASE: &str = "menubar";
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

pub fn state_attr(menu_count: usize, has_open_menu: bool) -> &'static str {
    if menu_count == 0 {
        "empty"
    } else if has_open_menu {
        "open"
    } else {
        "closed"
    }
}

pub fn menu_attr(menu_count: usize) -> &'static str {
    if menu_count == 0 {
        "empty"
    } else {
        "populated"
    }
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    if close_on_action {
        "close"
    } else {
        "keep-open"
    }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
            continue;
        }

        if (ch == '-' || ch == '_' || ch == ' ') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        return fallback.into();
    }

    out
}

pub fn resolve_menu_ids(id_base: &str, menu_id: &str) -> MenubarMenuIds {
    MenubarMenuIds {
        trigger_id: format!("{id_base}-{menu_id}-trigger"),
        menu_id: format!("{id_base}-{menu_id}-menu"),
    }
}

pub fn normalize_open_index(open_index: Option<usize>, menu_count: usize) -> Option<usize> {
    open_index.filter(|index| *index < menu_count)
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    let mut unique = BTreeSet::new();
    for index in disabled_indices {
        if index < item_count {
            unique.insert(index);
        }
    }

    unique.into_iter().collect()
}

pub fn resolve_menus(id_base: &str, menus: Vec<MenubarMenu>) -> Vec<MenubarMenuResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::new();

    for (index, menu) in menus.into_iter().enumerate() {
        let fallback_id = format!("menu-{}", index + 1);
        let raw_id = normalize_optional_text(Some(menu.id)).unwrap_or_else(|| fallback_id.clone());
        let base_id = sanitize_token(&raw_id, &fallback_id);

        let mut unique_id = base_id.clone();
        let mut suffix = 2;
        while seen_ids.contains(&unique_id) {
            unique_id = format!("{base_id}-{suffix}");
            suffix += 1;
        }
        seen_ids.insert(unique_id.clone());

        let label = normalize_optional_text(Some(menu.label))
            .unwrap_or_else(|| format!("Menu {}", index + 1));

        let items: Vec<String> = menu
            .items
            .into_iter()
            .filter_map(|item| normalize_optional_text(Some(item)))
            .collect();

        let item_count = items.len();
        let disabled_indices = normalize_disabled_indices(menu.disabled_indices, item_count);
        let item_kinds = menu.item_kinds.into_iter().take(item_count).collect();

        let is_trigger_disabled = menu.disabled || item_count == 0;
        let ids = resolve_menu_ids(id_base, &unique_id);

        resolved.push(MenubarMenuResolved {
            id: unique_id,
            label,
            items: Arc::<[String]>::from(items),
            disabled_indices,
            item_kinds,
            is_trigger_disabled,
            has_items: item_count > 0,
            trigger_id: ids.trigger_id,
            menu_id: ids.menu_id,
        });
    }

    resolved
}

pub fn sanitize_open_index_for_menus(
    open_index: Option<usize>,
    menus: &[MenubarMenuResolved],
) -> Option<usize> {
    let index = normalize_open_index(open_index, menus.len())?;
    (!menus[index].is_trigger_disabled).then_some(index)
}

pub fn next_enabled_menu_index(
    menus: &[MenubarMenuResolved],
    current_index: usize,
    step: isize,
) -> Option<usize> {
    if menus.is_empty() || step == 0 {
        return None;
    }

    let len = menus.len() as isize;
    let mut cursor = current_index as isize;

    for _ in 0..menus.len().saturating_sub(1) {
        cursor = (cursor + step).rem_euclid(len);
        let index = cursor as usize;
        if !menus[index].is_trigger_disabled {
            return Some(index);
        }
    }

    None
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
