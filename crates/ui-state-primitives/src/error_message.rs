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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageStateInput {
    pub tone: ErrorMessageTone,
    pub disabled: bool,
    pub truncate: bool,
    pub has_custom_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageState {
    pub tone: ErrorMessageTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub data_state_attr: &'static str,
    pub message_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_message(value: Option<String>) -> (String, bool) {
    if let Some(message) = normalize_optional_text(value) {
        return (message, true);
    }

    (DEFAULT_MESSAGE.into(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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

#[cfg(test)]
#[path = "test/error_message.rs"]
mod tests;
