use super::{SwitchGroupIds, SwitchGroupState, SwitchGroupStateInput};

pub const DEFAULT_LABEL: &str = "Switches";
pub const DEFAULT_ARIA_LABEL: &str = "SwitchGroup";
pub const DEFAULT_ERROR_MESSAGE: &str = "Invalid selection";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwitchGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl SwitchGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            SwitchGroupOrientation::Vertical => "ui-switch-group--orientation-vertical",
            SwitchGroupOrientation::Horizontal => "ui-switch-group--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwitchGroupOrientation::Vertical => "vertical",
            SwitchGroupOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwitchGroupTone {
    #[default]
    Default,
    Muted,
}

impl SwitchGroupTone {
    pub fn class_name(self) -> &'static str {
        match self {
            SwitchGroupTone::Default => "ui-switch-group--tone-default",
            SwitchGroupTone::Muted => "ui-switch-group--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwitchGroupTone::Default => "default",
            SwitchGroupTone::Muted => "muted",
        }
    }
}

pub fn resolve_ids(id_base: String) -> SwitchGroupIds {
    let normalized = id_base.trim();
    let root_id = if normalized.is_empty() {
        "switch-group".to_string()
    } else {
        normalized.to_string()
    };

    SwitchGroupIds {
        label_id: format!("{root_id}-label"),
        description_id: format!("{root_id}-description"),
        error_id: format!("{root_id}-error"),
        root_id,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_description(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
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

pub fn resolve_state(input: SwitchGroupStateInput) -> SwitchGroupState {
    let shows_error = input.invalid && input.has_error_message;
    let has_messages = input.has_description || shows_error;

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
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

    let message_kind_attr = if shows_error {
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
    } else if input.orientation == SwitchGroupOrientation::Horizontal {
        "horizontal"
    } else if input.tone == SwitchGroupTone::Muted {
        "muted"
    } else {
        "default"
    };

    SwitchGroupState {
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
        shows_error,
        has_messages,
        message_kind_attr,
        data_state_attr,
        label_source_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SwitchGroupState) -> String {
    let mut classes = vec![
        "ui-switch-group".to_string(),
        state.orientation_class.into(),
        state.tone_class.into(),
    ];

    if state.is_required {
        classes.push("ui-switch-group--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-switch-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-switch-group--invalid".to_string());
    }

    if state.has_description {
        classes.push("ui-switch-group--has-description".to_string());
    }

    if state.shows_error {
        classes.push("ui-switch-group--has-error".to_string());
    }

    if state.label_source_attr == "custom" {
        classes.push("ui-switch-group--label-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-switch-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
