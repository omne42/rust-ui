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
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_CLEAR_ARIA_LABEL.to_string())
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
mod tests {
    use super::*;

    fn base_state() -> InputLogicState {
        InputLogicState {
            is_disabled: false,
            is_read_only: false,
            is_invalid: false,
            is_empty: true,
            is_focused: false,
        }
    }

    #[test]
    fn clear_button_requires_non_empty_and_not_disabled() {
        let mut state = base_state();
        state.is_empty = false;
        assert!(resolve_view_state(None, None, None, false, false, true, state).show_clear);

        state.is_disabled = true;
        assert!(!resolve_view_state(None, None, None, false, false, true, state).show_clear);
    }

    #[test]
    fn error_requires_invalid_and_non_empty_error_text() {
        let mut state = base_state();
        state.is_invalid = true;

        let view = resolve_view_state(None, None, Some(" "), false, false, false, state);
        assert!(!view.show_error);

        let view = resolve_view_state(None, None, Some("Bad"), false, false, false, state);
        assert!(view.show_error);
    }

    #[test]
    fn clear_aria_label_uses_trimmed_value_or_default() {
        assert_eq!(
            resolve_clear_aria_label(Some("  Clear name  ".to_string())),
            "Clear name"
        );
        assert_eq!(
            resolve_clear_aria_label(Some("  ".to_string())),
            DEFAULT_CLEAR_ARIA_LABEL
        );
        assert_eq!(resolve_clear_aria_label(None), DEFAULT_CLEAR_ARIA_LABEL);
    }
}
