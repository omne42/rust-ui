pub use ui_state_primitives::button::normalize_optional_text;
pub use ui_state_primitives::textarea::{
    TextareaSourceAttr, TextareaState, TextareaStateInput, resolve_label_with_fallback,
    resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueControlModeAttr {
    Controlled,
    Uncontrolled,
}

impl ValueControlModeAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueChangeSourceAttr {
    OnValueChange,
    None,
}

impl ValueChangeSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

pub struct ValueAxisInput {
    pub has_controlled_value: bool,
    pub default_value: Option<String>,
    pub has_on_value_change: bool,
}

pub struct ValueAxisState {
    pub default_value: String,
    pub is_controlled: bool,
    pub control_mode_attr: ValueControlModeAttr,
    pub default_value_source_attr: TextareaSourceAttr,
    pub value_change_source_attr: ValueChangeSourceAttr,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let is_controlled = input.has_controlled_value;
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.has_on_value_change;
    let default_value = normalize_default_value(input.default_value);

    let control_mode_attr = if is_controlled {
        ValueControlModeAttr::Controlled
    } else {
        ValueControlModeAttr::Uncontrolled
    };
    let default_value_source_attr = if has_default_value {
        TextareaSourceAttr::Custom
    } else {
        TextareaSourceAttr::Default
    };
    let value_change_source_attr = if has_on_value_change {
        ValueChangeSourceAttr::OnValueChange
    } else {
        ValueChangeSourceAttr::None
    };
    let has_value_change_handler = has_on_value_change;

    ValueAxisState {
        default_value,
        is_controlled,
        control_mode_attr,
        default_value_source_attr,
        value_change_source_attr,
        has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        is_read_only: input.is_read_only.unwrap_or(false),
    }
}

pub fn compose_class_name(class_name: Option<String>, state: TextareaState) -> String {
    let mut classes = vec![
        "ui-textarea".to_string(),
        format!("ui-textarea--state-{}", state.state_attr.as_str()),
        format!("ui-textarea--value-{}", state.value_attr.as_str()),
        format!(
            "ui-textarea--requirement-{}",
            state.requirement_attr.as_str()
        ),
    ];

    if state.has_custom_class_name {
        classes.push("ui-textarea--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/textarea/logic.rs"]
mod tests;
