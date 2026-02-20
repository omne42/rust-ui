use crate::{TooltipPartState, TooltipPartStateInput, TooltipSlot};
use leptos::prelude::*;
use ui_headless::TooltipTriggerMode;
use ui_state_primitives::tooltip as tooltip_state;

pub const DEFAULT_DELAY_MS: u64 = tooltip_state::DEFAULT_DELAY_MS;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = tooltip_state::DEFAULT_CLOSE_DELAY_MS;
pub const DEFAULT_SHOULD_CLOSE_ON_PRESS: bool = tooltip_state::DEFAULT_SHOULD_CLOSE_ON_PRESS;

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
    }
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
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = input.is_open.or(input.open);
    let has_custom_open = open.is_some();
    let has_custom_default_open = input.default_open.is_some();
    let has_custom_on_open_change = input.on_open_change.is_some();
    let is_controlled = has_custom_open;

    OpenState {
        is_controlled,
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
        has_custom_open,
        has_custom_default_open,
        has_custom_on_open_change,
        open_mode_attr: if is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        open_source_attr: if has_custom_open { "custom" } else { "default" },
        default_open_source_attr: if has_custom_default_open {
            "provided"
        } else {
            "implicit"
        },
        open_change_source_attr: if has_custom_on_open_change {
            "provided"
        } else {
            "none"
        },
    }
}

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    tooltip_state::state_attr_for_open(is_open)
}

pub fn trigger_attr(trigger: TooltipTriggerMode) -> &'static str {
    let trigger = match trigger {
        TooltipTriggerMode::Hover => tooltip_state::TooltipTriggerMode::Hover,
        TooltipTriggerMode::Focus => tooltip_state::TooltipTriggerMode::Focus,
    };

    tooltip_state::trigger_attr(trigger)
}

pub fn press_behavior_attr(should_close_on_press: bool) -> &'static str {
    tooltip_state::press_behavior_attr(should_close_on_press)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    tooltip_state::normalize_optional_text(value)
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: String) -> (String, bool) {
    tooltip_state::resolve_id(custom_id, fallback_id)
}

pub fn has_custom_delays(delay_ms: u64, close_delay_ms: u64) -> bool {
    tooltip_state::has_custom_delays(delay_ms, close_delay_ms)
}

pub fn resolve_state(input: TooltipPartStateInput) -> TooltipPartState {
    let mut state = tooltip_state::resolve_state(input);
    if matches!(state.slot, TooltipSlot::Root) {
        state.state_attr = state_attr_for_open(state.is_open);
    }
    state
}

pub fn compose_class_name(base_class_name: Option<String>, state: TooltipPartState) -> String {
    tooltip_state::compose_class_name(base_class_name, state)
}

pub fn compose_panel_vars(top_px: f64, left_px: f64) -> String {
    tooltip_state::compose_panel_vars(top_px, left_px)
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
