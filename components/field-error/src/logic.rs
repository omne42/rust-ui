use crate::{FieldErrorState, FieldErrorStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "FieldError";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldErrorTone {
    #[default]
    Auto,
    Neutral,
    Negative,
}

impl FieldErrorTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldErrorTone::Auto => "ui-field-error--tone-auto",
            FieldErrorTone::Neutral => "ui-field-error--tone-neutral",
            FieldErrorTone::Negative => "ui-field-error--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldErrorTone::Auto => "auto",
            FieldErrorTone::Neutral => "neutral",
            FieldErrorTone::Negative => "negative",
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

pub fn normalize_message(value: Option<String>, visible: bool) -> (Option<String>, bool) {
    if !visible {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(DEFAULT_MESSAGE.into()), false)
}

pub fn resolve_effective_tone(requested_tone: FieldErrorTone, is_visible: bool) -> FieldErrorTone {
    match requested_tone {
        FieldErrorTone::Neutral => FieldErrorTone::Neutral,
        FieldErrorTone::Negative => FieldErrorTone::Negative,
        FieldErrorTone::Auto if is_visible => FieldErrorTone::Negative,
        FieldErrorTone::Auto => FieldErrorTone::Neutral,
    }
}

pub fn resolve_state(input: FieldErrorStateInput) -> FieldErrorState {
    let is_visible = input.visible && input.has_message;
    let tone = resolve_effective_tone(input.tone, is_visible);
    let show_icon = input.show_icon && is_visible;

    let data_state_attr = if !is_visible {
        "hidden"
    } else if input.disabled {
        "disabled"
    } else {
        "visible"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let message_source_attr = if !input.has_message {
        "none"
    } else if input.has_custom_message {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    FieldErrorState {
        tone,
        tone_class: tone.class_name(),
        tone_attr: tone.as_attr(),
        is_visible,
        is_disabled: input.disabled,
        show_icon,
        has_message: input.has_message,
        data_state_attr,
        aria_source_attr,
        message_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldErrorState) -> String {
    let mut classes = vec!["ui-field-error".to_string(), state.tone_class.into()];

    if state.is_visible {
        classes.push("ui-field-error--visible".to_string());
    }

    if state.is_disabled {
        classes.push("ui-field-error--disabled".to_string());
    }

    if state.show_icon {
        classes.push("ui-field-error--with-icon".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field-error--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
