use crate::{CheckboxFieldState, CheckboxFieldStateInput};

pub const DEFAULT_LABEL: &str = "Checkbox option";
pub const DEFAULT_ARIA_LABEL: &str = "Checkbox field";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxFieldTone {
    #[default]
    Default,
    Quiet,
}

impl CheckboxFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            CheckboxFieldTone::Default => "ui-checkbox-field--tone-default",
            CheckboxFieldTone::Quiet => "ui-checkbox-field--tone-quiet",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CheckboxFieldTone::Default => "default",
            CheckboxFieldTone::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxFieldIndicatorPlacement {
    #[default]
    Start,
    End,
}

impl CheckboxFieldIndicatorPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            CheckboxFieldIndicatorPlacement::Start => "ui-checkbox-field--indicator-start",
            CheckboxFieldIndicatorPlacement::End => "ui-checkbox-field--indicator-end",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CheckboxFieldIndicatorPlacement::Start => "start",
            CheckboxFieldIndicatorPlacement::End => "end",
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
        "ui-checkbox-field".to_string()
    }
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_LABEL.into(), false)
    }
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else if !fallback.trim().is_empty() {
        (fallback.trim().into(), false)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn resolve_state(input: CheckboxFieldStateInput) -> CheckboxFieldState {
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

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let state_attr = if input.invalid && input.checked {
        "checked-invalid"
    } else if input.invalid {
        "invalid"
    } else if input.disabled {
        "disabled"
    } else if input.checked {
        "checked"
    } else {
        "unchecked"
    };

    CheckboxFieldState {
        is_checked: input.checked,
        is_unchecked: !input.checked,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        indicator_placement: input.indicator_placement,
        indicator_placement_class: input.indicator_placement.class_name(),
        indicator_placement_attr: input.indicator_placement.as_attr(),
        has_description: input.has_description,
        description_attr: if input.has_description {
            "present"
        } else {
            "absent"
        },
        has_custom_label: input.has_custom_label,
        label_source_attr,
        has_custom_aria_label: input.has_custom_aria_label,
        aria_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
        state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CheckboxFieldState) -> String {
    let mut classes = vec![
        "ui-checkbox-field".to_string(),
        state.tone_class.into(),
        state.indicator_placement_class.into(),
    ];

    if state.is_checked {
        classes.push("ui-checkbox-field--checked".to_string());
    } else {
        classes.push("ui-checkbox-field--unchecked".to_string());
    }

    if state.is_invalid {
        classes.push("ui-checkbox-field--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-checkbox-field--disabled".to_string());
    }

    if state.has_description {
        classes.push("ui-checkbox-field--with-description".to_string());
    } else {
        classes.push("ui-checkbox-field--no-description".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-checkbox-field--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
