use std::borrow::Cow;

pub use ui_state_primitives::error_message::{
    DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorMessageElement, ErrorMessageModelInput,
    ErrorMessageState, ErrorMessageTone, resolve_effective_tone, resolve_model,
};
#[cfg(test)]
pub use ui_state_primitives::error_message::{
    ErrorMessageStateFlags, ErrorMessageStateFlagsInput, ErrorMessageStateInput,
    ErrorMessageStatus, normalize_state_flags, resolve_state, resolve_status,
    status_to_primitive_flags,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorMessageState) -> String {
    let mut classes = vec![
        Cow::Borrowed("ui-error-message"),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-error-message--disabled"));
    }

    if state.is_truncated {
        classes.push(Cow::Borrowed("ui-error-message--truncate"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-error-message--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let mut class_name = String::new();
    for token in classes {
        if !class_name.is_empty() {
            class_name.push(' ');
        }
        class_name.push_str(token.as_ref());
    }

    class_name
}

const _: Option<ErrorMessageState> = None;
const _: &str = DEFAULT_MESSAGE;
const _: &str = DEFAULT_ARIA_LABEL;
const _: fn(ErrorMessageTone) -> ErrorMessageTone = resolve_effective_tone;

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
