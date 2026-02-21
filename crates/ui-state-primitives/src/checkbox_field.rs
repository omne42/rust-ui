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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxFieldStatus {
    #[default]
    Unchecked,
    Checked,
    Disabled,
    Invalid,
    CheckedInvalid,
}

impl CheckboxFieldStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            CheckboxFieldStatus::Unchecked => "unchecked",
            CheckboxFieldStatus::Checked => "checked",
            CheckboxFieldStatus::Disabled => "disabled",
            CheckboxFieldStatus::Invalid => "invalid",
            CheckboxFieldStatus::CheckedInvalid => "checked-invalid",
        }
    }
}

pub const DEFAULT_LABEL: &str = "Checkbox option";
pub const DEFAULT_ARIA_LABEL: &str = "Checkbox field";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldStateInput {
    pub checked: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub tone: CheckboxFieldTone,
    pub indicator_placement: CheckboxFieldIndicatorPlacement,
    pub has_description: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldState {
    pub status: CheckboxFieldStatus,
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub tone: CheckboxFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub indicator_placement: CheckboxFieldIndicatorPlacement,
    pub indicator_placement_class: &'static str,
    pub indicator_placement_attr: &'static str,
    pub has_description: bool,
    pub description_attr: &'static str,
    pub has_custom_label: bool,
    pub label_source_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
    pub state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| "ui-checkbox-field".to_string())
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

pub const fn resolve_status(checked: bool, disabled: bool, invalid: bool) -> CheckboxFieldStatus {
    if invalid && checked {
        CheckboxFieldStatus::CheckedInvalid
    } else if invalid {
        CheckboxFieldStatus::Invalid
    } else if disabled {
        CheckboxFieldStatus::Disabled
    } else if checked {
        CheckboxFieldStatus::Checked
    } else {
        CheckboxFieldStatus::Unchecked
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

    let status = resolve_status(input.checked, input.disabled, input.invalid);

    CheckboxFieldState {
        status,
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
        state_attr: status.as_attr(),
    }
}

#[cfg(test)]
#[path = "test/checkbox_field.rs"]
mod tests;
