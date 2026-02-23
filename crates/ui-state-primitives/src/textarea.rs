pub const DEFAULT_LABEL: &str = "Textarea";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaVisualStateAttr {
    Disabled,
    Invalid,
    Readonly,
    Ready,
}

impl TextareaVisualStateAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Invalid => "invalid",
            Self::Readonly => "readonly",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaValueAttr {
    Filled,
    Empty,
}

impl TextareaValueAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaRequirementAttr {
    Required,
    Optional,
}

impl TextareaRequirementAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaSourceAttr {
    Custom,
    Default,
}

impl TextareaSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaValueControlModeAttr {
    Controlled,
    Uncontrolled,
}

impl TextareaValueControlModeAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextareaValueChangeSourceAttr {
    OnValueChange,
    None,
}

impl TextareaValueChangeSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaStateInput {
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
pub struct TextareaState {
    pub state_attr: TextareaVisualStateAttr,
    pub value_attr: TextareaValueAttr,
    pub requirement_attr: TextareaRequirementAttr,
    pub label_source_attr: TextareaSourceAttr,
    pub description_source_attr: TextareaSourceAttr,
    pub error_source_attr: TextareaSourceAttr,
    pub placeholder_source_attr: TextareaSourceAttr,
    pub rows_source_attr: TextareaSourceAttr,
    pub class_source_attr: TextareaSourceAttr,
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

pub fn resolve_state(input: TextareaStateInput) -> TextareaState {
    TextareaState {
        state_attr: if input.disabled {
            TextareaVisualStateAttr::Disabled
        } else if input.invalid {
            TextareaVisualStateAttr::Invalid
        } else if input.read_only {
            TextareaVisualStateAttr::Readonly
        } else {
            TextareaVisualStateAttr::Ready
        },
        value_attr: if input.has_value {
            TextareaValueAttr::Filled
        } else {
            TextareaValueAttr::Empty
        },
        requirement_attr: if input.required {
            TextareaRequirementAttr::Required
        } else {
            TextareaRequirementAttr::Optional
        },
        label_source_attr: if input.has_custom_label {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        description_source_attr: if input.has_custom_description {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        error_source_attr: if input.has_custom_error {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        rows_source_attr: if input.has_custom_rows {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        class_source_attr: if input.has_custom_class_name {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaValueAxisInput {
    pub is_controlled: bool,
    pub has_default_value: bool,
    pub has_on_value_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaValueAxisState {
    pub is_controlled: bool,
    pub control_mode_attr: TextareaValueControlModeAttr,
    pub default_value_source_attr: TextareaSourceAttr,
    pub value_change_source_attr: TextareaValueChangeSourceAttr,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn resolve_value_axis_state(input: TextareaValueAxisInput) -> TextareaValueAxisState {
    TextareaValueAxisState {
        is_controlled: input.is_controlled,
        control_mode_attr: if input.is_controlled {
            TextareaValueControlModeAttr::Controlled
        } else {
            TextareaValueControlModeAttr::Uncontrolled
        },
        default_value_source_attr: if input.has_default_value {
            TextareaSourceAttr::Custom
        } else {
            TextareaSourceAttr::Default
        },
        value_change_source_attr: if input.has_on_value_change {
            TextareaValueChangeSourceAttr::OnValueChange
        } else {
            TextareaValueChangeSourceAttr::None
        },
        has_value_change_handler: input.has_on_value_change,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaAccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextareaAccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn resolve_accessibility_state(
    input: TextareaAccessibilityStateInput,
) -> TextareaAccessibilityState {
    TextareaAccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        is_read_only: input.is_read_only.unwrap_or(false),
    }
}

#[cfg(test)]
#[path = "test/textarea.rs"]
mod tests;
