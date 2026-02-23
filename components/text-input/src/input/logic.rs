use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputLabelPlacement {
    #[default]
    Outside,
    Inside,
}

impl InputLabelPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            InputLabelPlacement::Outside => "ui-input--label-outside",
            InputLabelPlacement::Inside => "ui-input--label-inside",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn class_name(self) -> &'static str {
        match self {
            InputSize::Sm => "ui-input--size-sm",
            InputSize::Md => "ui-input--size-md",
            InputSize::Lg => "ui-input--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputVariant {
    #[default]
    Bordered,
    Flat,
    Underlined,
}

impl InputVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            InputVariant::Bordered => "ui-input--variant-bordered",
            InputVariant::Flat => "ui-input--variant-flat",
            InputVariant::Underlined => "ui-input--variant-underlined",
        }
    }
}

pub use ui_state_primitives::input::{
    InputLogicState, resolve_clear_aria_label, resolve_view_state,
};

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_read_only: Option<bool>,
    pub read_only: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Signal<bool>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Signal<bool>,
    pub is_label_hidden: Option<bool>,
    pub label_hidden: bool,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub is_label_hidden: bool,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        is_read_only: input.is_read_only.unwrap_or(input.read_only),
        is_required: input.is_required.unwrap_or(input.required),
        is_invalid: input.is_invalid.unwrap_or(input.invalid),
        is_label_hidden: input.is_label_hidden.unwrap_or(input.label_hidden),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Search,
    Tel,
    Url,
    Number,
    Custom(&'static str),
}

impl InputType {
    pub fn as_html_attr(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Number => "number",
            Self::Custom(value) => value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputTypeState {
    pub input_type: InputType,
    pub type_source_attr: &'static str,
}

pub fn normalize_input_type(input_type: Option<&'static str>) -> InputTypeState {
    let normalized = input_type.map(str::trim).filter(|value| !value.is_empty());
    let input_type = match normalized {
        None | Some("text") => InputType::Text,
        Some("email") => InputType::Email,
        Some("password") => InputType::Password,
        Some("search") => InputType::Search,
        Some("tel") => InputType::Tel,
        Some("url") => InputType::Url,
        Some("number") => InputType::Number,
        Some(value) => InputType::Custom(value),
    };

    let type_source_attr = if normalized.is_some_and(|value| value != "text") {
        "custom"
    } else {
        "default"
    };

    InputTypeState {
        input_type,
        type_source_attr,
    }
}

#[cfg(test)]
#[path = "../../test/input/logic.rs"]
mod tests;
