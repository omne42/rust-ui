pub use ui_state_primitives::error_message::{
    DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorMessageElement, ErrorMessageState,
    ErrorMessageStateInput, ErrorMessageTone, normalize_aria_label, normalize_message,
    normalize_optional_text, resolve_effective_tone, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorMessageState) -> String {
    let mut classes = vec!["ui-error-message".to_string(), state.tone_class.into()];

    if state.is_disabled {
        classes.push("ui-error-message--disabled".to_string());
    }

    if state.is_truncated {
        classes.push("ui-error-message--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-error-message--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

const _: Option<ErrorMessageState> = None;
const _: &str = DEFAULT_MESSAGE;
const _: &str = DEFAULT_ARIA_LABEL;
const _: fn(ErrorMessageTone) -> ErrorMessageTone = resolve_effective_tone;

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
