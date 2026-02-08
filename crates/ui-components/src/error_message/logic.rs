use crate::error_message::{ErrorMessageState, ErrorMessageStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "ErrorMessage";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ErrorMessageTone {
    #[default]
    Auto,
    Neutral,
    Negative,
}

impl ErrorMessageTone {
    pub fn class_name(self) -> &'static str {
        match self {
            ErrorMessageTone::Auto => "ui-error-message--tone-auto",
            ErrorMessageTone::Neutral => "ui-error-message--tone-neutral",
            ErrorMessageTone::Negative => "ui-error-message--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorMessageTone::Auto => "auto",
            ErrorMessageTone::Neutral => "neutral",
            ErrorMessageTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ErrorMessageElement {
    Span,
    #[default]
    Paragraph,
    Div,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_message(value: Option<String>) -> (String, bool) {
    if let Some(message) = normalize_optional_text(value) {
        return (message, true);
    }

    (DEFAULT_MESSAGE.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_effective_tone(requested_tone: ErrorMessageTone) -> ErrorMessageTone {
    match requested_tone {
        ErrorMessageTone::Auto => ErrorMessageTone::Negative,
        ErrorMessageTone::Neutral => ErrorMessageTone::Neutral,
        ErrorMessageTone::Negative => ErrorMessageTone::Negative,
    }
}

pub fn resolve_state(input: ErrorMessageStateInput) -> ErrorMessageState {
    let tone = resolve_effective_tone(input.tone);

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.truncate {
        "truncate"
    } else {
        "default"
    };

    let message_source_attr = if input.has_custom_message {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ErrorMessageState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_disabled: input.disabled,
        is_truncated: input.truncate,
        data_state_attr,
        message_source_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorMessageState) -> String {
    let mut classes = vec!["ui-error-message".to_string(), state.tone_class.to_string()];

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_message::ErrorMessageStateInput;

    #[test]
    fn tone_and_element_contracts_are_stable() {
        assert_eq!(
            ErrorMessageTone::Auto.class_name(),
            "ui-error-message--tone-auto"
        );
        assert_eq!(
            ErrorMessageTone::Neutral.class_name(),
            "ui-error-message--tone-neutral"
        );
        assert_eq!(
            ErrorMessageTone::Negative.class_name(),
            "ui-error-message--tone-negative"
        );

        assert_eq!(ErrorMessageTone::Auto.as_attr(), "auto");
        assert_eq!(ErrorMessageTone::Neutral.as_attr(), "neutral");
        assert_eq!(ErrorMessageTone::Negative.as_attr(), "negative");

        assert_eq!(
            ErrorMessageElement::default(),
            ErrorMessageElement::Paragraph
        );
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  bad input  ".to_string())),
            Some("bad input".to_string())
        );

        let (message, custom_message) = normalize_message(Some("  Required  ".to_string()));
        assert_eq!(message, "Required");
        assert!(custom_message);

        let (message, custom_message) = normalize_message(None);
        assert_eq!(message, DEFAULT_MESSAGE);
        assert!(!custom_message);

        let (label, custom_label) = normalize_aria_label(Some("  Email error  ".to_string()));
        assert_eq!(label, "Email error");
        assert!(custom_label);

        let (label, custom_label) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom_label);
    }

    #[test]
    fn resolve_state_tracks_sources_and_priority() {
        let state = resolve_state(ErrorMessageStateInput {
            tone: ErrorMessageTone::Auto,
            disabled: false,
            truncate: true,
            has_custom_message: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "negative");
        assert_eq!(state.data_state_attr, "truncate");
        assert_eq!(state.message_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

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
