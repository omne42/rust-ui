pub const DEFAULT_ARIA_LABEL: &str = "Field";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl FieldOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "ui-field--orientation-vertical",
            FieldOrientation::Horizontal => "ui-field--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "vertical",
            FieldOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldTone {
    #[default]
    Default,
    Muted,
}

impl FieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldTone::Default => "ui-field--tone-default",
            FieldTone::Muted => "ui-field--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldTone::Default => "default",
            FieldTone::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldStateInput {
    pub orientation: FieldOrientation,
    pub tone: FieldTone,
    pub required: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldState {
    pub orientation: FieldOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: FieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub has_label: bool,
    pub has_description: bool,
    pub has_error_message: bool,
    pub message_kind_attr: &'static str,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

pub fn resolve_state(input: FieldStateInput) -> FieldState {
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

    let message_kind_attr = if input.has_error_message {
        "error"
    } else if input.has_description {
        "description"
    } else {
        "none"
    };

    let data_state_attr = if input.invalid && input.disabled {
        "invalid-disabled"
    } else if input.invalid {
        "invalid"
    } else if input.disabled {
        "disabled"
    } else if input.required {
        "required"
    } else if input.orientation == FieldOrientation::Horizontal {
        "horizontal"
    } else if input.tone == FieldTone::Muted {
        "muted"
    } else {
        "default"
    };

    FieldState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.required,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        has_label: input.has_label,
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

pub fn compose_class_name(base_class_name: Option<String>, state: FieldState) -> String {
    let mut classes = vec![
        "ui-field".to_string(),
        state.orientation_class.into(),
        state.tone_class.into(),
    ];

    if state.is_required {
        classes.push("ui-field--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-field--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-field--invalid".to_string());
    }

    if state.has_label {
        classes.push("ui-field--has-label".to_string());
    }

    if state.has_description {
        classes.push("ui-field--has-description".to_string());
    }

    if state.has_error_message {
        classes.push("ui-field--has-error".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/field.rs"]
mod tests;
