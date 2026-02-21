use crate::MenuItemKind;
use crate::action_menu::{
    ActionMenuActionMode, ActionMenuDisabledState, ActionMenuIds, ActionMenuItemSpec,
    ActionMenuPartState, ActionMenuPartStateInput, ActionMenuSlot, MenuOpenFocusStrategy,
};
use leptos::prelude::*;
use ui_headless::PopoverPlacement;
use ui_state_primitives::action_menu as action_menu_state;
use ui_state_primitives::menu as menu_state;

pub const DEFAULT_ID_BASE: &str = action_menu_state::DEFAULT_ID_BASE;
pub const DEFAULT_TRIGGER_ARIA_LABEL: &str = action_menu_state::DEFAULT_TRIGGER_ARIA_LABEL;
pub const DEFAULT_DISABLED: bool = action_menu_state::DEFAULT_DISABLED;
pub const DEFAULT_CLOSE_ON_ACTION: bool = action_menu_state::DEFAULT_CLOSE_ON_ACTION;
pub const DEFAULT_PLACEMENT: PopoverPlacement = PopoverPlacement::BottomStart;

#[derive(Clone)]
pub struct ActionMenuOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct ActionMenuOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    #[cfg(test)]
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionMenuDiscreteProps {
    pub disabled_state: ActionMenuDisabledState,
    pub action_mode: ActionMenuActionMode,
    pub has_custom_disabled: bool,
    pub has_custom_close_on_action: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuItemsInput {
    pub item_specs: Vec<ActionMenuItemSpec>,
    pub items: Vec<String>,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuItemsOutput {
    pub has_item_specs: bool,
    pub items: Vec<String>,
    pub item_count: usize,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuNormalizeInput {
    pub id_base: String,
    pub item_count: usize,
    pub disabled_indices: Vec<usize>,
    pub item_kinds_len: usize,
    pub class_name: Option<String>,
    pub aria_label: Option<String>,
    pub fallback_aria_label: String,
    pub disabled_state: Option<ActionMenuDisabledState>,
    pub is_disabled: Option<bool>,
    pub disabled: Option<bool>,
    pub action_mode: Option<ActionMenuActionMode>,
    pub is_close_on_action: Option<bool>,
    pub close_on_action: Option<bool>,
    pub placement: PopoverPlacement,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMenuNormalizedProps {
    pub id_base: String,
    pub has_custom_id_base: bool,
    pub disabled_indices: Vec<usize>,
    pub has_disabled_items: bool,
    pub has_custom_disabled_indices: bool,
    pub has_item_kinds: bool,
    pub has_custom_item_kinds: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub aria_label: String,
    pub has_custom_aria_label: bool,
    pub disabled_state: ActionMenuDisabledState,
    pub action_mode: ActionMenuActionMode,
    pub has_custom_disabled: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_placement: bool,
    pub trigger_disabled: bool,
    pub is_controlled: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionMenuTriggerPressResult {
    pub next_open: bool,
    pub open_focus: Option<MenuOpenFocusStrategy>,
}

pub fn normalize_menu_items(input: ActionMenuItemsInput) -> ActionMenuItemsOutput {
    if !input.item_specs.is_empty() {
        let mut items = Vec::with_capacity(input.item_specs.len());
        let mut item_kinds = Vec::with_capacity(input.item_specs.len());
        let mut disabled_indices = Vec::new();

        for (index, spec) in input.item_specs.into_iter().enumerate() {
            items.push(spec.label);
            item_kinds.push(spec.kind);
            if spec.is_disabled {
                disabled_indices.push(index);
            }
        }

        let item_count = items.len();
        let disabled_indices = normalize_disabled_indices(disabled_indices, item_count);

        return ActionMenuItemsOutput {
            has_item_specs: true,
            items,
            item_count,
            item_kinds,
            disabled_indices,
        };
    }

    let item_count = input.items.len();
    let disabled_indices = normalize_disabled_indices(input.disabled_indices, item_count);

    ActionMenuItemsOutput {
        has_item_specs: false,
        items: input.items,
        item_count,
        item_kinds: input.item_kinds,
        disabled_indices,
    }
}

pub fn normalize_open_state(input: ActionMenuOpenStateInput) -> ActionMenuOpenState {
    let open = menu_state::normalize_controlled_prop_alias(input.is_open, input.open);
    let has_custom_open = menu_state::is_controlled_prop(&open);
    let has_custom_default_open = input.default_open.is_some();
    let has_custom_on_open_change = input.on_open_change.is_some();

    ActionMenuOpenState {
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
        has_custom_open,
        has_custom_default_open,
        has_custom_on_open_change,
        #[cfg(test)]
        is_controlled: has_custom_open,
    }
}

pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize) -> &'static str {
    action_menu_state::state_attr(is_open, trigger_disabled, item_count)
}

pub fn item_attr(item_count: usize) -> &'static str {
    action_menu_state::item_attr(item_count)
}

pub fn action_attr(close_on_action: bool) -> &'static str {
    action_menu_state::action_attr(close_on_action)
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    action_menu_state::open_mode_attr(is_controlled)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    action_menu_state::normalize_optional_text(value)
}

pub fn normalize_id_base(id_base: String) -> String {
    action_menu_state::normalize_id_base(id_base)
}

pub fn resolve_ids(id_base: &str) -> ActionMenuIds {
    let (trigger_id, menu_id) = action_menu_state::resolve_id_pair(id_base);

    ActionMenuIds {
        trigger_id,
        menu_id,
    }
}

pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize) -> Vec<usize> {
    action_menu_state::normalize_disabled_indices(disabled_indices, item_count)
}

pub fn resolve_trigger_disabled(disabled: bool, item_count: usize) -> bool {
    action_menu_state::resolve_trigger_disabled(disabled, item_count)
}

pub fn resolve_trigger_aria_label(
    value: Option<String>,
    fallback_aria_label: &str,
) -> (String, bool) {
    action_menu_state::resolve_trigger_aria_label_with_fallback(value, fallback_aria_label)
}

pub fn normalize_discrete_props(
    disabled_state: Option<ActionMenuDisabledState>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    action_mode: Option<ActionMenuActionMode>,
    is_close_on_action: Option<bool>,
    close_on_action: Option<bool>,
) -> ActionMenuDiscreteProps {
    let boolean_props = action_menu_state::normalize_boolean_props(
        is_disabled,
        disabled,
        is_close_on_action,
        close_on_action,
    );
    let disabled_state = disabled_state
        .unwrap_or_else(|| ActionMenuDisabledState::from_bool(boolean_props.is_disabled));
    let action_mode = action_mode
        .unwrap_or_else(|| ActionMenuActionMode::from_bool(boolean_props.is_close_on_action));

    ActionMenuDiscreteProps {
        disabled_state,
        action_mode,
        has_custom_disabled: disabled_state != ActionMenuDisabledState::default(),
        has_custom_close_on_action: action_mode != ActionMenuActionMode::default(),
    }
}

pub fn normalize_props(input: ActionMenuNormalizeInput) -> ActionMenuNormalizedProps {
    let id_base = normalize_id_base(input.id_base);
    let has_custom_id_base = id_base != DEFAULT_ID_BASE;

    let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);
    let has_disabled_items = !disabled_indices.is_empty();
    let has_custom_disabled_indices = has_disabled_items;

    let has_item_kinds = input.item_kinds_len > 0;
    let has_custom_item_kinds = has_item_kinds;

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let (aria_label, has_custom_aria_label) =
        resolve_trigger_aria_label(input.aria_label, input.fallback_aria_label.as_str());
    let discrete_props = normalize_discrete_props(
        input.disabled_state,
        input.is_disabled,
        input.disabled,
        input.action_mode,
        input.is_close_on_action,
        input.close_on_action,
    );
    let has_custom_placement = input.placement != DEFAULT_PLACEMENT;
    let trigger_disabled = resolve_trigger_disabled(
        discrete_props.disabled_state.is_disabled(),
        input.item_count,
    );
    let is_controlled = input.has_custom_open;

    ActionMenuNormalizedProps {
        id_base,
        has_custom_id_base,
        disabled_indices,
        has_disabled_items,
        has_custom_disabled_indices,
        has_item_kinds,
        has_custom_item_kinds,
        class_name,
        has_custom_class_name,
        aria_label,
        has_custom_aria_label,
        disabled_state: discrete_props.disabled_state,
        action_mode: discrete_props.action_mode,
        has_custom_disabled: discrete_props.has_custom_disabled,
        has_custom_close_on_action: discrete_props.has_custom_close_on_action,
        has_custom_placement,
        trigger_disabled,
        is_controlled,
        has_custom_open: input.has_custom_open,
        has_custom_default_open: input.has_custom_default_open,
        has_custom_on_open_change: input.has_custom_on_open_change,
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn resolve_trigger_press(
    trigger_disabled: bool,
    current_open: bool,
) -> Option<ActionMenuTriggerPressResult> {
    if trigger_disabled {
        return None;
    }

    let next_open = !current_open;
    let open_focus = next_open.then_some(MenuOpenFocusStrategy::First);

    Some(ActionMenuTriggerPressResult {
        next_open,
        open_focus,
    })
}

pub fn resolve_action_open_change(action_mode: ActionMenuActionMode) -> Option<bool> {
    action_mode.is_close_on_action().then_some(false)
}

pub fn resolve_open_focus_strategy(
    key: &str,
    trigger_disabled: bool,
    current_open: bool,
) -> Option<MenuOpenFocusStrategy> {
    ui_headless::menu_trigger_open_focus_strategy(key, trigger_disabled, current_open)
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ActionMenuPartStateInput) -> ActionMenuPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    ActionMenuPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open, input.trigger_disabled, input.item_count),
        item_attr: item_attr(input.item_count),
        action_attr: action_attr(input.close_on_action),
        open_mode_attr: open_mode_attr(input.is_controlled),
        placement: input.placement,
        placement_attr: input.placement.as_str(),
        open_attr: input.is_open.then_some("true"),
        closed_attr: (!input.is_open).then_some("true"),
        item_count: input.item_count,
        is_empty,
        has_items,
        is_open: input.is_open,
        is_trigger_disabled: input.trigger_disabled,
        is_enabled: !input.trigger_disabled,
        close_on_action: input.close_on_action,
        keep_open_on_action: !input.close_on_action,
        has_disabled_items: input.has_disabled_items,
        has_item_kinds: input.has_item_kinds,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
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

pub fn compose_class_name(base_class_name: Option<String>, state: ActionMenuPartState) -> String {
    let mut classes = vec![
        state.base_class.into(),
        format!("ui-action-menu--placement-{}", state.placement_attr),
    ];

    if matches!(state.slot, ActionMenuSlot::Root) {
        if state.is_open {
            classes.push("ui-action-menu--open".to_string());
        } else {
            classes.push("ui-action-menu--closed".to_string());
        }

        if state.is_trigger_disabled {
            classes.push("ui-action-menu--disabled".to_string());
        } else {
            classes.push("ui-action-menu--enabled".to_string());
        }

        if state.is_empty {
            classes.push("ui-action-menu--empty".to_string());
        } else {
            classes.push("ui-action-menu--has-items".to_string());
        }

        if state.keep_open_on_action {
            classes.push("ui-action-menu--persistent".to_string());
        } else {
            classes.push("ui-action-menu--close-on-action".to_string());
        }

        if state.is_controlled {
            classes.push("ui-action-menu--controlled".to_string());
        } else {
            classes.push("ui-action-menu--uncontrolled".to_string());
        }

        if state.has_custom_id_base {
            classes.push("ui-action-menu--custom-id".to_string());
        }

        if state.has_custom_aria_label {
            classes.push("ui-action-menu--custom-aria-label".to_string());
        }

        if state.has_custom_disabled {
            classes.push("ui-action-menu--custom-disabled".to_string());
        }

        if state.has_custom_disabled_indices {
            classes.push("ui-action-menu--custom-disabled-indices".to_string());
        }

        if state.has_custom_item_kinds {
            classes.push("ui-action-menu--custom-item-kinds".to_string());
        }

        if state.has_custom_close_on_action {
            classes.push("ui-action-menu--custom-close-on-action".to_string());
        }

        if state.has_custom_placement {
            classes.push("ui-action-menu--custom-placement".to_string());
        }

        if state.has_custom_open {
            classes.push("ui-action-menu--custom-open".to_string());
        }

        if state.has_custom_default_open {
            classes.push("ui-action-menu--custom-default-open".to_string());
        }

        if state.has_custom_on_open_change {
            classes.push("ui-action-menu--custom-open-change".to_string());
        }

        if state.has_custom_motion {
            classes.push("ui-action-menu--custom-motion".to_string());
        }

        if state.has_custom_class_name {
            classes.push("ui-action-menu--custom-class".to_string());
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
#[path = "../../test/action_menu/logic.rs"]
mod tests;
