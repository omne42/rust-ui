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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_markers() {
        let class_name = compose_class_name(
            Some("docs-error-message".to_string()),
            resolve_state(ErrorMessageStateInput {
                tone: ErrorMessageTone::Negative,
                disabled: true,
                truncate: true,
                has_custom_message: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-error-message",
            "ui-error-message--tone-negative",
            "ui-error-message--disabled",
            "ui-error-message--truncate",
            "ui-error-message--custom-class",
            "docs-error-message",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
