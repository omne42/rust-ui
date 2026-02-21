pub const DEFAULT_LABEL: &str = "Options";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_invalid: bool,
    pub is_valid: bool,
    pub is_required: bool,
    pub is_optional: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub shows_error: bool,
    pub has_messages: bool,
}

pub fn resolve_checkbox_group_state(
    is_disabled: bool,
    is_invalid: bool,
    is_required: bool,
    has_description: bool,
    has_error: bool,
) -> CheckboxGroupState {
    let shows_error = has_error && is_invalid;
    let has_messages = has_description || shows_error;

    CheckboxGroupState {
        is_disabled,
        is_enabled: !is_disabled,
        is_invalid,
        is_valid: !is_invalid,
        is_required,
        is_optional: !is_required,
        has_description,
        has_error,
        shows_error,
        has_messages,
    }
}

pub fn normalize_checkbox_group_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        DEFAULT_LABEL.to_string()
    } else {
        trimmed.into()
    }
}

pub fn normalize_checkbox_group_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

#[cfg(test)]
#[path = "test/checkbox_group.rs"]
mod tests;
