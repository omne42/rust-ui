#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonState {
    pub is_disabled: bool,
}

pub fn resolve_state(is_disabled: bool, disabled: bool) -> SearchInputButtonState {
    SearchInputButtonState {
        is_disabled: is_disabled || disabled,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchInputButtonViewState {
    pub placeholder: String,
    pub compact_placeholder: String,
    pub meta_key_label: Option<String>,
    pub key_label: Option<String>,
    pub show_shortcut: bool,
}

pub fn resolve_view_state(
    placeholder: Option<&str>,
    compact_placeholder: Option<&str>,
    meta_key_label: Option<&str>,
    key_label: Option<&str>,
) -> SearchInputButtonViewState {
    let placeholder = placeholder.unwrap_or("Search").trim().to_string();
    let compact_placeholder = compact_placeholder
        .unwrap_or(placeholder.as_str())
        .trim()
        .to_string();

    let meta_key_label = meta_key_label
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());
    let key_label = key_label
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());

    let show_shortcut = meta_key_label.is_some() && key_label.is_some();

    SearchInputButtonViewState {
        placeholder,
        compact_placeholder,
        meta_key_label,
        key_label,
        show_shortcut,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_disabled_is_true_when_either_flag_is_true() {
        assert!(!resolve_state(false, false).is_disabled);
        assert!(resolve_state(true, false).is_disabled);
        assert!(resolve_state(false, true).is_disabled);
    }

    #[test]
    fn view_state_defaults_and_trims() {
        let state = resolve_view_state(Some("  Search docs... "), None, None, None);
        assert_eq!(state.placeholder, "Search docs...");
        assert_eq!(state.compact_placeholder, "Search docs...");
        assert!(!state.show_shortcut);

        let state = resolve_view_state(None, Some("  Go "), Some(" ⌘ "), Some(" K "));
        assert_eq!(state.placeholder, "Search");
        assert_eq!(state.compact_placeholder, "Go");
        assert_eq!(state.meta_key_label.as_deref(), Some("⌘"));
        assert_eq!(state.key_label.as_deref(), Some("K"));
        assert!(state.show_shortcut);
    }

    #[test]
    fn shortcut_requires_both_keys() {
        let state = resolve_view_state(Some("Search"), None, Some("⌘"), None);
        assert!(!state.show_shortcut);

        let state = resolve_view_state(Some("Search"), None, None, Some("K"));
        assert!(!state.show_shortcut);
    }
}
