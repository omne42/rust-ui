pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyStateInput<'a> {
    pub text: &'a str,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyState {
    pub is_copyable: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_text: bool,
    pub state_attr: &'static str,
}

pub fn resolve_state(input: ButtonCopyStateInput<'_>) -> ButtonCopyState {
    let has_text = !input.text.trim().is_empty();
    let is_copyable = !input.is_disabled && has_text;
    let state_attr = if is_copyable {
        "copyable"
    } else if input.is_disabled {
        "disabled"
    } else {
        "empty"
    };

    ButtonCopyState {
        is_copyable,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_text,
        state_attr,
    }
}

#[cfg(test)]
#[path = "test/button_copy.rs"]
mod tests;
