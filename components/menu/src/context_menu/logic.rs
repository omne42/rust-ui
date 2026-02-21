use crate::context_menu::{
    ContextMenuIds, ContextMenuPartState, ContextMenuPartStateInput, ContextMenuSlot,
};
use leptos::prelude::*;
use ui_headless::PopoverPlacement;
use ui_state_primitives::menu as menu_state;

pub const DEFAULT_ID_BASE: &str = "context-menu";
pub const DEFAULT_ARIA_LABEL: &str = "Open context menu";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextMenuDiscreteInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_close_on_action: Option<bool>,
    pub close_on_action: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextMenuDiscreteProps {
    pub disabled_state: ContextMenuDisabledState,
    pub action_mode: ContextMenuActionMode,
}

#[derive(Clone)]
pub struct ContextMenuOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct ContextMenuOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextMenuDisabledState {
    #[default]
    Enabled,
    Disabled,
}

impl ContextMenuDisabledState {
    pub fn from_bool(disabled: bool) -> Self {
        if disabled {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextMenuActionMode {
    #[default]
    CloseOnAction,
    KeepOpenOnAction,
}

impl ContextMenuActionMode {
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

pub fn state_attr(is_open: bool, trigger_disabled: bool) -> &'static str {
    menu_state::context_state_attr(is_open, trigger_disabled)
}

pub fn item_attr(item_count: usize) -> &'static str {
    menu_state::item_attr(item_count)
}

pub fn disabled_attr(trigger_disabled: bool) -> &'static str {
    menu_state::disabled_attr(trigger_disabled)
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    menu_state::action_attr(close_on_action)
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    menu_state::open_mode_attr(is_controlled)
}

pub fn normalize_discrete_props(input: ContextMenuDiscreteInput) -> ContextMenuDiscreteProps {
    ContextMenuDiscreteProps {
        disabled_state: ContextMenuDisabledState::from_bool(
            input.is_disabled.unwrap_or(input.disabled),
        ),
        action_mode: ContextMenuActionMode::from_bool(
            input.is_close_on_action.unwrap_or(input.close_on_action),
        ),
    }
}

pub fn resolve_open_focus_strategy(
    key: &str,
    shift_key: bool,
    trigger_disabled: bool,
    current_open: bool,
) -> Option<ui_headless::MenuOpenFocusStrategy> {
    ui_headless::context_menu_open_focus_strategy(key, shift_key, trigger_disabled, current_open)
}

pub fn should_open_from_context_menu(trigger_disabled: bool) -> bool {
    !trigger_disabled
}

pub fn resolve_ui_action(is_open: bool) -> &'static str {
    if is_open { "open" } else { "idle" }
}

pub fn resolve_ui_output_status(is_open: bool) -> &'static str {
    if is_open { "draft" } else { "submittable" }
}

pub fn resolve_aria_expanded(is_open: bool) -> &'static str {
    if is_open { "true" } else { "false" }
}

pub fn normalize_open_state(input: ContextMenuOpenStateInput) -> ContextMenuOpenState {
    let open = menu_state::normalize_controlled_prop_alias(input.is_open, input.open);
    ContextMenuOpenState {
        is_controlled: menu_state::is_controlled_prop(&open),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    menu_state::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    menu_state::normalize_id_base(id_base, DEFAULT_ID_BASE)
}

pub fn resolve_ids(id_base: &str) -> ContextMenuIds {
    let (trigger_id, menu_id) = menu_state::resolve_id_pair(id_base);
    ContextMenuIds {
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
    menu_state::resolve_aria_label_with_fallback(value, DEFAULT_ARIA_LABEL, DEFAULT_ARIA_LABEL)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ContextMenuPartStateInput) -> ContextMenuPartState {
    let is_empty = input.item_count == 0;
    let has_items = input.item_count > 0;
    let is_enabled = !input.trigger_disabled;
    let keep_open_on_action = !input.close_on_action;
    let is_uncontrolled = !input.is_controlled;

    ContextMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open, input.trigger_disabled),
        item_attr: item_attr(input.item_count),
        disabled_attr: disabled_attr(input.trigger_disabled),
        action_attr: action_attr(input.close_on_action),
        open_mode_attr: open_mode_attr(input.is_controlled),
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        open_attr: input.is_open.then_some("true"),
        closed_attr: (!input.is_open).then_some("true"),
        is_open: input.is_open,
        item_count: input.item_count,
        is_empty,
        has_items,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled,
        close_on_action: input.close_on_action,
        keep_open_on_action,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_disabled_indices: input.has_custom_disabled_indices,
        has_custom_item_kinds: input.has_custom_item_kinds,
        has_custom_close_on_action: input.has_custom_close_on_action,
        has_custom_placement: input.has_custom_placement,
        has_custom_open: input.has_custom_open,
        has_custom_default_open: input.has_custom_default_open,
        has_custom_on_open_change: input.has_custom_on_open_change,
        has_custom_motion: input.has_custom_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        disabled_indices_source_attr: source_attr(input.has_custom_disabled_indices),
        item_kinds_source_attr: source_attr(input.has_custom_item_kinds),
        close_on_action_source_attr: source_attr(input.has_custom_close_on_action),
        placement_source_attr: source_attr(input.has_custom_placement),
        open_source_attr: source_attr(input.has_custom_open),
        default_open_source_attr: source_attr(input.has_custom_default_open),
        open_change_source_attr: source_attr(input.has_custom_on_open_change),
        motion_source_attr: source_attr(input.has_custom_motion),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ContextMenuPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if matches!(state.slot, ContextMenuSlot::Root) {
        classes.push(format!(
            "ui-context-menu--placement-{}",
            state.placement_attr
        ));

        if state.is_open {
            classes.push("ui-context-menu--open".to_string());
        } else {
            classes.push("ui-context-menu--closed".to_string());
        }

        if state.is_trigger_disabled {
            classes.push("ui-context-menu--disabled".to_string());
        } else {
            classes.push("ui-context-menu--enabled".to_string());
        }

        if state.has_items {
            classes.push("ui-context-menu--has-items".to_string());
        } else {
            classes.push("ui-context-menu--empty".to_string());
        }

        if state.keep_open_on_action {
            classes.push("ui-context-menu--persistent".to_string());
        } else {
            classes.push("ui-context-menu--close-on-action".to_string());
        }

        if state.is_controlled {
            classes.push("ui-context-menu--controlled".to_string());
        } else {
            classes.push("ui-context-menu--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-context-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-context-menu--custom-aria-label".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-context-menu--custom-disabled".to_string());
        }

        if state.has_custom_disabled_indices {
            classes.push("ui-context-menu--custom-disabled-indices".to_string());
        }

        if state.has_custom_item_kinds {
            classes.push("ui-context-menu--custom-item-kinds".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-context-menu--custom-close-on-action".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-context-menu--custom-placement".to_string());
        }

        if state.has_custom_open {
            classes.push("ui-context-menu--custom-open".to_string());
        }

        if state.has_custom_default_open {
            classes.push("ui-context-menu--custom-default-open".to_string());
        }

        if state.has_custom_on_open_change {
            classes.push("ui-context-menu--custom-open-change".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-context-menu--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-context-menu--custom-class".to_string());
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
#[path = "../../test/context_menu/logic.rs"]
mod tests;
