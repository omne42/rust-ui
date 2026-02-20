pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Text field";
pub const DEFAULT_INPUT_TYPE: &str = "text";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldStateInput<'a> {
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub value: &'a str,
    pub is_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
}

pub fn resolve_label(label: String) -> (String, &'static str) {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        (DEFAULT_LABEL.into(), "default")
    } else {
        (trimmed.into(), "custom")
    }
}

pub fn resolve_input_type(input_type: Option<&'static str>) -> (&'static str, &'static str) {
    match input_type.map(str::trim).filter(|value| !value.is_empty()) {
        Some(DEFAULT_INPUT_TYPE) => (DEFAULT_INPUT_TYPE, "default"),
        Some(value) => (value, "custom"),
        None => (DEFAULT_INPUT_TYPE, "default"),
    }
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: TextFieldStateInput<'_>) -> TextFieldState {
    TextFieldState {
        state_attr: resolve_state_attr(input.is_disabled, input.is_invalid, input.is_read_only),
        value_attr: resolve_value_attr(input.value),
        requirement_attr: resolve_requirement_attr(input.is_required),
    }
}

pub fn resolve_state_attr(is_disabled: bool, is_invalid: bool, is_read_only: bool) -> &'static str {
    if is_disabled {
        "disabled"
    } else if is_invalid {
        "invalid"
    } else if is_read_only {
        "readonly"
    } else {
        "ready"
    }
}

pub fn resolve_value_attr(value: &str) -> &'static str {
    if value.trim().is_empty() {
        "empty"
    } else {
        "filled"
    }
}

pub fn resolve_requirement_attr(is_required: bool) -> &'static str {
    if is_required { "required" } else { "optional" }
}

#[cfg(test)]
#[path = "test/text_field.rs"]
mod tests;
