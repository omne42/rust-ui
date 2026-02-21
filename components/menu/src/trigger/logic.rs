use leptos::prelude::*;
use ui_headless::PopoverPlacement;
use ui_state_primitives::menu as menu_state;

pub type MenuOpenFocusStrategy = ui_headless::MenuOpenFocusStrategy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuTriggerDiscreteInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_close_on_action: Option<bool>,
    pub close_on_action: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuTriggerDiscreteProps {
    pub disabled: bool,
    pub action_mode: MenuTriggerActionMode,
}

#[derive(Clone)]
pub struct MenuTriggerOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct MenuTriggerOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuTriggerActionMode {
    #[default]
    CloseOnAction,
    KeepOpenOnAction,
}

impl MenuTriggerActionMode {
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
pub struct MenuTriggerPressResult {
    pub next_open: bool,
    pub open_focus: Option<MenuOpenFocusStrategy>,
}

#[cfg(test)]
pub fn focus_strategy_for_open_key(key: &str) -> Option<MenuOpenFocusStrategy> {
    ui_headless::menu_trigger_open_focus_strategy_for_key(key)
}
#[cfg(test)]
const _: fn(&str) -> Option<MenuOpenFocusStrategy> = focus_strategy_for_open_key;

pub fn resolve_root_state_attr(is_open: bool, trigger_disabled: bool) -> &'static str {
    if is_open {
        "open"
    } else if trigger_disabled {
        "disabled"
    } else {
        "closed"
    }
}

pub fn normalize_discrete_props(input: MenuTriggerDiscreteInput) -> MenuTriggerDiscreteProps {
    MenuTriggerDiscreteProps {
        disabled: input.is_disabled.unwrap_or(input.disabled),
        action_mode: MenuTriggerActionMode::from_bool(
            input.is_close_on_action.unwrap_or(input.close_on_action),
        ),
    }
}

pub fn normalize_open_state(input: MenuTriggerOpenStateInput) -> MenuTriggerOpenState {
    let open = menu_state::normalize_controlled_prop_alias(input.is_open, input.open);
    MenuTriggerOpenState {
        is_controlled: menu_state::is_controlled_prop(&open),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub fn resolve_trigger_press(
    trigger_disabled: bool,
    current_open: bool,
) -> Option<MenuTriggerPressResult> {
    if trigger_disabled {
        return None;
    }

    let next_open = !current_open;
    let open_focus = next_open.then_some(MenuOpenFocusStrategy::First);
    Some(MenuTriggerPressResult {
        next_open,
        open_focus,
    })
}

pub fn resolve_open_focus_strategy(
    key: &str,
    trigger_disabled: bool,
    current_open: bool,
) -> Option<MenuOpenFocusStrategy> {
    ui_headless::menu_trigger_open_focus_strategy(key, trigger_disabled, current_open)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuTriggerIds {
    pub trigger_id: String,
    pub menu_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuTriggerStateInput {
    pub item_count: usize,
    pub trigger_disabled: bool,
    pub action_mode: MenuTriggerActionMode,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub placement: PopoverPlacement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuTriggerState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_trigger_disabled: bool,
    pub is_enabled: bool,
    pub close_on_action: bool,
    pub keep_open_on_action: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_disabled_items: bool,
    pub has_item_kinds: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub placement: PopoverPlacement,
    pub placement_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    menu_state::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    menu_state::normalize_id_base(id_base, "menu-trigger")
}

pub fn resolve_ids(id_base: &str) -> MenuTriggerIds {
    let (trigger_id, menu_id) = menu_state::resolve_id_pair(id_base);
    MenuTriggerIds {
        trigger_id,
        menu_id,
    }
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    menu_state::normalize_disabled_indices(disabled_indices, item_count)
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    menu_state::resolve_trigger_disabled(disabled, item_count)
}

pub fn resolve_trigger_aria_label(value: Option<String>) -> (String, bool) {
    menu_state::resolve_aria_label_with_fallback(value, "Open menu", "Open menu")
}

pub fn resolve_state(input: MenuTriggerStateInput) -> MenuTriggerState {
    let close_on_action = input.action_mode.is_close_on_action();

    MenuTriggerState {
        item_count: input.item_count,
        is_empty: input.item_count == 0,
        has_items: input.item_count > 0,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled: !input.trigger_disabled,
        close_on_action,
        keep_open_on_action: !close_on_action,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_disabled_items: input.has_disabled_items,
        has_item_kinds: input.has_item_kinds,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        placement: input.placement,
        placement_attr: input.placement.as_str(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: MenuTriggerState) -> String {
    let mut classes = vec![
        "ui-menu-trigger".to_string(),
        format!("ui-menu-trigger--placement-{}", state.placement_attr),
    ];

    if state.is_trigger_disabled {
        classes.push("ui-menu-trigger--disabled".to_string());
    }
    if state.has_items {
        classes.push("ui-menu-trigger--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-menu-trigger--empty".to_string());
    }
    if state.keep_open_on_action {
        classes.push("ui-menu-trigger--persistent".to_string());
    }
    if state.is_controlled {
        classes.push("ui-menu-trigger--controlled".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/trigger/logic.rs"]
mod tests;
