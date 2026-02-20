pub use crate::button::normalize_optional_text;

pub const DEFAULT_CLEAR_ARIA_LABEL: &str = "Clear";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputLogicState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_invalid: bool,
    pub is_empty: bool,
    pub is_focused: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputViewState {
    pub show_label: bool,
    pub show_description: bool,
    pub show_error: bool,
    pub show_start: bool,
    pub show_end: bool,
    pub show_clear: bool,
    pub has_affix: bool,
    pub is_filled: bool,
    pub is_filled_within: bool,
}

pub fn resolve_clear_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_CLEAR_ARIA_LABEL.into())
}

pub fn resolve_view_state(
    label: Option<&str>,
    description: Option<&str>,
    error: Option<&str>,
    has_start: bool,
    has_end: bool,
    is_clearable: bool,
    state: InputLogicState,
) -> InputViewState {
    let show_label = label.is_some_and(|v| !v.trim().is_empty());
    let show_description = description.is_some_and(|v| !v.trim().is_empty());
    let show_error = state.is_invalid && error.is_some_and(|v| !v.trim().is_empty());
    let show_clear = is_clearable && !state.is_empty && !state.is_read_only && !state.is_disabled;
    let has_affix = has_start || has_end || show_clear;
    let is_filled = !state.is_empty;
    let is_filled_within = is_filled || state.is_focused;

    InputViewState {
        show_label,
        show_description,
        show_error,
        show_start: has_start,
        show_end: has_end,
        show_clear,
        has_affix,
        is_filled,
        is_filled_within,
    }
}

#[cfg(test)]
#[path = "test/input.rs"]
mod tests;
