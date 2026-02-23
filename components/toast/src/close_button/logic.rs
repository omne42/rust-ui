use crate::close_button::{CloseButtonState, CloseButtonStateInput};
use ui_state_primitives::close_button as close_button_state;

pub use close_button_state::{CloseButtonSize, CloseButtonVariant, DEFAULT_ARIA_LABEL};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    close_button_state::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    close_button_state::normalize_aria_label(value, default)
}

pub fn resolve_state(input: CloseButtonStateInput) -> CloseButtonState {
    close_button_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: CloseButtonState) -> String {
    let mut classes = vec![
        "ui-close-button".to_string(),
        state.variant_class.into(),
        state.size_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-close-button--disabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-close-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-close-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-close-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/close_button/logic.rs"]
mod tests;
