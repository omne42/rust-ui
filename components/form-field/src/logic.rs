use super::{FormFieldState, FormFieldStateInput};

pub const DEFAULT_LABEL: &str = "Form field";
pub const DEFAULT_ARIA_LABEL: &str = "Form field control";
pub const DEFAULT_ERROR_MESSAGE: &str = "Selection is required";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldTone {
    #[default]
    Default,
    Quiet,
}

impl FormFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldTone::Default => "ui-form-field--tone-default",
            FormFieldTone::Quiet => "ui-form-field--tone-quiet",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldTone::Default => "default",
            FormFieldTone::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldIndicatorVariant {
    #[default]
    Switch,
    Checkbox,
}

impl FormFieldIndicatorVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldIndicatorVariant::Switch => "ui-form-field--indicator-switch",
            FormFieldIndicatorVariant::Checkbox => "ui-form-field--indicator-checkbox",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldIndicatorVariant::Switch => "switch",
            FormFieldIndicatorVariant::Checkbox => "checkbox",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormFieldIndicatorPlacement {
    Start,
    #[default]
    End,
}

impl FormFieldIndicatorPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            FormFieldIndicatorPlacement::Start => "ui-form-field--placement-start",
            FormFieldIndicatorPlacement::End => "ui-form-field--placement-end",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FormFieldIndicatorPlacement::Start => "start",
            FormFieldIndicatorPlacement::End => "end",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    if let Some(id_base) = normalize_optional_text(value) {
        id_base
    } else {
        "ui-form-field".to_string()
    }
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_LABEL.into(), false)
    }
}

pub fn normalize_aria_label(value: Option<String>, fallback_label: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else if !fallback_label.trim().is_empty() {
        (fallback_label.trim().into(), false)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
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

pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState {
    let shows_error = input.invalid && input.has_error_message;

    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else if input.has_custom_label {
        "label"
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

    let state_attr = if input.invalid && input.disabled {
        "invalid-disabled"
    } else if input.invalid && input.selected {
        "selected-invalid"
    } else if input.invalid {
        "invalid"
    } else if input.disabled && input.selected {
        "selected-disabled"
    } else if input.disabled {
        "disabled"
    } else if input.selected {
        "selected"
    } else {
        "unselected"
    };

    FormFieldState {
        is_selected: input.selected,
        is_unselected: !input.selected,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        indicator_variant: input.indicator_variant,
        indicator_variant_class: input.indicator_variant.class_name(),
        indicator_variant_attr: input.indicator_variant.as_attr(),
        indicator_placement: input.indicator_placement,
        indicator_placement_class: input.indicator_placement.class_name(),
        indicator_placement_attr: input.indicator_placement.as_attr(),
        has_description: input.has_description,
        has_error_message: input.has_error_message,
        shows_error,
        message_kind_attr,
        state_attr,
        label_source_attr,
        aria_source_attr,
        error_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FormFieldState) -> String {
    let mut classes = vec![
        "ui-form-field".to_string(),
        state.tone_class.into(),
        state.indicator_variant_class.into(),
        state.indicator_placement_class.into(),
    ];

    if state.is_selected {
        classes.push("ui-form-field--selected".to_string());
    } else {
        classes.push("ui-form-field--unselected".to_string());
    }

    if state.is_invalid {
        classes.push("ui-form-field--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-form-field--disabled".to_string());
    }

    if state.has_description {
        classes.push("ui-form-field--with-description".to_string());
    }

    if state.has_error_message {
        classes.push("ui-form-field--with-error".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-form-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
