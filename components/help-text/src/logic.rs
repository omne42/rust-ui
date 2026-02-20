use crate::{HelpTextState, HelpTextStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "HelpText";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HelpTextTone {
    #[default]
    Auto,
    Neutral,
    Negative,
}

impl HelpTextTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HelpTextTone::Auto => "ui-help-text--tone-auto",
            HelpTextTone::Neutral => "ui-help-text--tone-neutral",
            HelpTextTone::Negative => "ui-help-text--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HelpTextTone::Auto => "auto",
            HelpTextTone::Neutral => "neutral",
            HelpTextTone::Negative => "negative",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_error_message(value: Option<String>, invalid: bool) -> (Option<String>, bool) {
    if !invalid {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_ERROR_MESSAGE.into()), false)
}

pub fn resolve_effective_tone(
    requested_tone: HelpTextTone,
    invalid: bool,
    has_error_message: bool,
) -> HelpTextTone {
    match requested_tone {
        HelpTextTone::Neutral => HelpTextTone::Neutral,
        HelpTextTone::Negative => HelpTextTone::Negative,
        HelpTextTone::Auto if invalid && has_error_message => HelpTextTone::Negative,
        HelpTextTone::Auto => HelpTextTone::Neutral,
    }
}

pub fn resolve_state(input: HelpTextStateInput) -> HelpTextState {
    let message_kind_attr = if input.has_error_message && input.invalid {
        "error"
    } else if input.has_description {
        "description"
    } else {
        "none"
    };

    let tone = resolve_effective_tone(input.tone, input.invalid, input.has_error_message);

    let show_error_icon = input.show_error_icon && message_kind_attr == "error";

    let data_state_attr = if message_kind_attr == "error" && input.disabled {
        "error-disabled"
    } else if message_kind_attr == "error" {
        "error"
    } else if input.disabled {
        "disabled"
    } else if message_kind_attr == "description" {
        "description"
    } else {
        "empty"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let error_source_attr = if !input.has_error_message {
        "none"
    } else if input.has_custom_error_message {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    HelpTextState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_invalid: input.invalid,
        is_disabled: input.disabled,
        show_error_icon,
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        message_kind_attr,
        data_state_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HelpTextState) -> String {
    let mut classes = vec!["ui-help-text".to_string(), state.tone_class.into()];

    if state.is_invalid {
        classes.push("ui-help-text--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-help-text--disabled".to_string());
    }

    if state.show_error_icon {
        classes.push("ui-help-text--with-icon".to_string());
    }

    if state.has_error_message {
        classes.push("ui-help-text--has-error".to_string());
    }

    if state.has_description {
        classes.push("ui-help-text--has-description".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-help-text--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
