pub use ui_state_primitives::button::normalize_optional_text;
pub use ui_state_primitives::textarea::{
    TextareaAccessibilityStateInput as PrimitiveAccessibilityStateInput, TextareaSourceAttr,
    TextareaState, TextareaStateInput, TextareaValueAxisInput as PrimitiveValueAxisInput,
    normalize_default_value as primitive_normalize_default_value,
    resolve_accessibility_state as primitive_resolve_accessibility_state,
    resolve_label_with_fallback, resolve_state,
    resolve_value_axis_state as primitive_resolve_value_axis_state,
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
    primitive_normalize_default_value(default_value)
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let markers = primitive_resolve_value_axis_state(PrimitiveValueAxisInput {
        is_controlled: input.has_controlled_value,
        has_default_value: input.default_value.is_some(),
        has_on_value_change: input.has_on_value_change,
    });
    let default_value = normalize_default_value(input.default_value);

    ValueAxisState {
        default_value,
        is_controlled: markers.is_controlled,
        control_mode_attr: if markers.is_controlled {
            ValueControlModeAttr::Controlled
        } else {
            ValueControlModeAttr::Uncontrolled
        },
        default_value_source_attr: markers.default_value_source_attr,
        value_change_source_attr: if markers.has_value_change_handler {
            ValueChangeSourceAttr::OnValueChange
        } else {
            ValueChangeSourceAttr::None
        },
        has_value_change_handler: markers.has_value_change_handler,
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
    let state = primitive_resolve_accessibility_state(PrimitiveAccessibilityStateInput {
        is_disabled: input.is_disabled,
        is_read_only: input.is_read_only,
    });
    AccessibilityState {
        is_disabled: state.is_disabled,
        is_read_only: state.is_read_only,
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
