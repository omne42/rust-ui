use leptos::prelude::*;

pub use ui_state_primitives::dropdown::{
    DropdownOpenFocusStrategy, DropdownState, DropdownStateInput, focus_strategy_for_open_key,
    normalize_aria_label, normalize_disabled_indices, normalize_id_base, normalize_optional_text,
    resolve_state, resolve_trigger_disabled,
};

pub struct DisabledStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub fn normalize_disabled_state(input: DisabledStateInput) -> bool {
    input.is_disabled.unwrap_or(input.disabled)
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
    let open = input.is_open.or(input.open);
    OpenState {
        is_controlled: open.is_some(),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
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
