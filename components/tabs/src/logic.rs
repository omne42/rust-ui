use leptos::prelude::{Callback, ReadSignal, Signal};
use std::collections::HashSet;

pub use ui_state_primitives::tabs::{
    TabsKeyboardActivation, normalize_index_skipping_disabled, resolve_tabs_state,
};

pub const BASE_CLASS_NAME: &str = "ui-tabs";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsControlMode {
    Controlled,
    Uncontrolled,
}

impl TabsControlMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            TabsControlMode::Controlled => "controlled",
            TabsControlMode::Uncontrolled => "uncontrolled",
        }
    }

    pub const fn is_controlled(self) -> bool {
        matches!(self, TabsControlMode::Controlled)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsDisabledSource {
    IsDisabled,
    Disabled,
}

impl TabsDisabledSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            TabsDisabledSource::IsDisabled => "is-disabled",
            TabsDisabledSource::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabsDisabledAxis {
    pub is_disabled: bool,
    pub source: TabsDisabledSource,
}

pub fn normalize_disabled_axis(is_disabled: Option<bool>, disabled: bool) -> TabsDisabledAxis {
    let source = if is_disabled.is_some() {
        TabsDisabledSource::IsDisabled
    } else {
        TabsDisabledSource::Disabled
    };
    let resolved_is_disabled = normalize_is_disabled(is_disabled, disabled);

    TabsDisabledAxis {
        is_disabled: resolved_is_disabled,
        source,
    }
}

#[derive(Clone)]
pub struct TabsSelectionAxisInput {
    pub selected_index: Option<ReadSignal<usize>>,
    pub default_selected_index: usize,
    pub on_selection_change: Option<Callback<usize>>,
}

#[derive(Clone)]
pub struct TabsSelectionAxis {
    pub selected_index: Option<Signal<usize>>,
    pub default_selected_index: usize,
    pub on_selection_change: Option<Callback<usize>>,
    pub control_mode: TabsControlMode,
}

pub fn normalize_selection_axis(input: TabsSelectionAxisInput) -> TabsSelectionAxis {
    let control_mode = if input.selected_index.is_some() {
        TabsControlMode::Controlled
    } else {
        TabsControlMode::Uncontrolled
    };

    TabsSelectionAxis {
        selected_index: input.selected_index.map(Into::into),
        default_selected_index: input.default_selected_index,
        on_selection_change: input.on_selection_change,
        control_mode,
    }
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn is_tab_disabled(disabled: bool, disabled_indices: &HashSet<usize>, index: usize) -> bool {
    disabled || disabled_indices.contains(&index)
}

pub fn has_disabled_tabs(disabled: bool, disabled_indices: &HashSet<usize>) -> bool {
    disabled || !disabled_indices.is_empty()
}

pub fn normalize_selected_with_disabled(
    selected_index: usize,
    item_count: usize,
    is_disabled: impl Fn(usize) -> bool,
) -> usize {
    normalize_index_skipping_disabled(selected_index, item_count, is_disabled)
}

pub fn resolve_selection_request(
    requested_index: usize,
    current_selected_index: usize,
    item_count: usize,
    is_disabled: impl Fn(usize) -> bool,
) -> Option<usize> {
    if item_count == 0 {
        return None;
    }

    let next = normalize_selected_with_disabled(requested_index, item_count, &is_disabled);
    if is_disabled(next) || next == current_selected_index {
        None
    } else {
        Some(next)
    }
}

pub fn resolve_requested_selected_index(
    controlled_selected_index: Option<usize>,
    default_selected_index: usize,
) -> usize {
    controlled_selected_index.unwrap_or(default_selected_index)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsRegistrationAction {
    Register { registration_id: usize },
    Unregister { registration_id: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TabsRegistrationState {
    pub items_order: Vec<usize>,
}

pub fn apply_registration_action(
    state: &mut TabsRegistrationState,
    action: TabsRegistrationAction,
) {
    match action {
        TabsRegistrationAction::Register { registration_id } => {
            if !state.items_order.contains(&registration_id) {
                state.items_order.push(registration_id);
            }
        }
        TabsRegistrationAction::Unregister { registration_id } => {
            state.items_order.retain(|id| *id != registration_id);
        }
    }
}

pub fn reduce_registration_actions(actions: &[TabsRegistrationAction]) -> TabsRegistrationState {
    let mut state = TabsRegistrationState::default();
    for action in actions {
        apply_registration_action(&mut state, *action);
    }
    state
}

pub fn resolve_registered_items_order(
    actions: &[TabsRegistrationAction],
    active_registration_ids: &[usize],
) -> Vec<usize> {
    let items_order = reduce_registration_actions(actions).items_order;
    let mut resolved = Vec::with_capacity(active_registration_ids.len());

    for registration_id in items_order {
        if active_registration_ids.contains(&registration_id)
            && !resolved.contains(&registration_id)
        {
            resolved.push(registration_id);
        }
    }

    for registration_id in active_registration_ids {
        if !resolved.contains(registration_id) {
            resolved.push(*registration_id);
        }
    }

    resolved
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(class_name: Option<String>) -> String {
    if let Some(class_name) = normalize_optional_text(class_name) {
        format!("{BASE_CLASS_NAME} {class_name}")
    } else {
        BASE_CLASS_NAME.to_string()
    }
}

pub fn resolve_motion_source(has_custom_motion: bool) -> &'static str {
    if has_custom_motion {
        "custom"
    } else {
        "default"
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
