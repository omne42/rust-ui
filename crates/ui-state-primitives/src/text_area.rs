pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Text area";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaStateInput {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub invalid: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_rows: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub rows_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn resolve_label(value: String) -> (String, bool) {
    resolve_label_with_fallback(value, DEFAULT_LABEL)
}

pub fn resolve_label_with_fallback(value: String, fallback_label: &str) -> (String, bool) {
    let trimmed = value.trim();

    if !trimmed.is_empty() {
        return (trimmed.into(), true);
    }

    let fallback_trimmed = fallback_label.trim();
    if !fallback_trimmed.is_empty() {
        return (fallback_trimmed.to_string(), false);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn resolve_state(input: TextAreaStateInput) -> TextAreaState {
    TextAreaState {
        state_attr: if input.disabled {
            "disabled"
        } else if input.invalid {
            "invalid"
        } else if input.read_only {
            "readonly"
        } else {
            "ready"
        },
        value_attr: if input.has_value { "filled" } else { "empty" },
        requirement_attr: if input.required {
            "required"
        } else {
            "optional"
        },
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        description_source_attr: if input.has_custom_description {
            "custom"
        } else {
            "default"
        },
        error_source_attr: if input.has_custom_error {
            "custom"
        } else {
            "default"
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            "custom"
        } else {
            "default"
        },
        rows_source_attr: if input.has_custom_rows {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaValueAxisState {
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn resolve_value_axis_state(input: TextAreaValueAxisInput) -> TextAreaValueAxisState {
    TextAreaValueAxisState {
        is_controlled: input.is_controlled,
        control_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        default_value_source_attr: if input.has_default_value {
            "custom"
        } else {
            "default"
        },
        value_change_source_attr: if input.has_on_value_change {
            "on_value_change"
        } else {
            "none"
        },
        has_value_change_handler: input.has_on_value_change,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaAccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaAccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn resolve_accessibility_state(
    input: TextAreaAccessibilityStateInput,
) -> TextAreaAccessibilityState {
    TextAreaAccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        is_read_only: input.is_read_only.unwrap_or(false),
    }
}

#[cfg(test)]
#[path = "test/text_area.rs"]
mod tests;
