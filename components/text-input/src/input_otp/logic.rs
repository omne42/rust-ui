use leptos::prelude::*;

pub use ui_state_primitives::input_otp::{
    apply_otp_backspace, apply_otp_input, normalize_otp_value,
};

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Signal<bool>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Signal<bool>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        is_required: input.is_required.unwrap_or(input.required),
        is_invalid: input.is_invalid.unwrap_or(input.invalid),
    }
}

#[cfg(test)]
#[path = "../../test/input_otp/logic.rs"]
mod tests;
