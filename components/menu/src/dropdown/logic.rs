use leptos::prelude::*;
use ui_state_primitives::menu as menu_state;

#[cfg(test)]
pub use ui_state_primitives::dropdown::focus_strategy_for_open_key;
pub use ui_state_primitives::dropdown::{
    DropdownOpenFocusStrategy, DropdownState, DropdownStateInput, normalize_aria_label,
    normalize_disabled_indices, normalize_id_base, normalize_optional_text, resolve_state,
    resolve_trigger_disabled,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownPressResult {
    pub next_open: bool,
    pub open_focus: Option<DropdownOpenFocusStrategy>,
}

pub struct DisabledStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_disabled_state(input: DisabledStateInput) -> bool {
    input.is_disabled.unwrap_or(input.disabled)
}

pub struct ActionModeInput {
    pub is_close_on_action: Option<bool>,
    pub close_on_action: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DropdownActionMode {
    #[default]
    CloseOnAction,
    KeepOpenOnAction,
}

impl DropdownActionMode {
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

pub fn normalize_close_on_action(input: ActionModeInput) -> DropdownActionMode {
    DropdownActionMode::from_bool(input.is_close_on_action.unwrap_or(input.close_on_action))
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct OpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = menu_state::normalize_controlled_prop_alias(input.is_open, input.open);
    OpenState {
        is_controlled: menu_state::is_controlled_prop(&open),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub fn resolve_root_state_attr(is_open: bool, state: DropdownState) -> &'static str {
    if is_open {
        "open"
    } else {
        state.data_state_attr
    }
}

pub fn resolve_trigger_press(
    trigger_disabled: bool,
    current_open: bool,
) -> Option<DropdownPressResult> {
    if trigger_disabled {
        return None;
    }

    let next_open = !current_open;
    let open_focus = next_open.then_some(DropdownOpenFocusStrategy::First);
    Some(DropdownPressResult {
        next_open,
        open_focus,
    })
}

pub fn resolve_open_focus_strategy(
    key: &str,
    trigger_disabled: bool,
    current_open: bool,
) -> Option<DropdownOpenFocusStrategy> {
    ui_headless::menu_trigger_open_focus_strategy(key, trigger_disabled, current_open).map(
        |strategy| match strategy {
            ui_headless::MenuOpenFocusStrategy::First => DropdownOpenFocusStrategy::First,
            ui_headless::MenuOpenFocusStrategy::Last => DropdownOpenFocusStrategy::Last,
        },
    )
}

pub fn compose_class_name(base_class_name: Option<String>, state: DropdownState) -> String {
    let mut classes = vec!["ui-dropdown".to_string()];

    if state.is_disabled {
        classes.push("ui-dropdown--disabled".to_string());
    }
    if state.has_items {
        classes.push("ui-dropdown--has-items".to_string());
    }
    if state.is_empty {
        classes.push("ui-dropdown--empty".to_string());
    }
    if state.keep_open_on_action {
        classes.push("ui-dropdown--persistent".to_string());
    }
    if state.is_controlled {
        classes.push("ui-dropdown--controlled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-dropdown--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/dropdown/logic.rs"]
mod tests;
