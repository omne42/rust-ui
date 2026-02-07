#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_shortcut: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_compact_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchInputButtonViewState {
    pub placeholder: String,
    pub compact_placeholder: String,
    pub meta_key_label: Option<String>,
    pub key_label: Option<String>,
    pub show_shortcut: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(
    is_disabled: bool,
    disabled: bool,
    has_shortcut: bool,
    has_custom_placeholder: bool,
    has_custom_compact_placeholder: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> SearchInputButtonState {
    let is_disabled = is_disabled || disabled;

    SearchInputButtonState {
        is_disabled,
        is_enabled: !is_disabled,
        has_shortcut,
        has_custom_placeholder,
        has_custom_compact_placeholder,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

pub fn resolve_view_state(
    placeholder: Option<&str>,
    compact_placeholder: Option<&str>,
    meta_key_label: Option<&str>,
    key_label: Option<&str>,
) -> SearchInputButtonViewState {
    let placeholder = placeholder
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Search")
        .to_string();

    let compact_placeholder = compact_placeholder
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(placeholder.as_str())
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

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: SearchInputButtonState,
) -> String {
    let mut classes = vec!["ui-search-input-button".to_string()];

    if state.is_enabled {
        classes.push("ui-search-input-button--enabled".to_string());
    }
    if state.is_disabled {
        classes.push("ui-search-input-button--disabled".to_string());
    }
    if state.has_shortcut {
        classes.push("ui-search-input-button--with-shortcut".to_string());
    }
    if state.has_custom_placeholder {
        classes.push("ui-search-input-button--custom-placeholder".to_string());
    }
    if state.has_custom_compact_placeholder {
        classes.push("ui-search-input-button--custom-compact-placeholder".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Search docs  ".to_string())),
            Some("Search docs".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_state_tracks_enablement_and_metadata_flags() {
        let enabled_state = resolve_state(false, false, true, true, true, true, true);
        assert!(enabled_state.is_enabled);
        assert!(!enabled_state.is_disabled);
        assert!(enabled_state.has_shortcut);
        assert!(enabled_state.has_custom_placeholder);
        assert!(enabled_state.has_custom_compact_placeholder);
        assert!(enabled_state.has_custom_aria_label);
        assert!(enabled_state.has_custom_class_name);

        let disabled_state = resolve_state(true, false, false, false, false, false, false);
        assert!(!disabled_state.is_enabled);
        assert!(disabled_state.is_disabled);
        assert!(!disabled_state.has_shortcut);
    }

    #[test]
    fn view_state_defaults_and_trims_with_blank_fallbacks() {
        let state = resolve_view_state(Some("  Search docs... "), None, None, None);
        assert_eq!(state.placeholder, "Search docs...");
        assert_eq!(state.compact_placeholder, "Search docs...");
        assert!(!state.show_shortcut);

        let state = resolve_view_state(Some("   "), Some("  Go "), Some(" ⌘ "), Some(" K "));
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

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(false, false, true, true, true, false, true),
        );

        for token in [
            "ui-search-input-button",
            "ui-search-input-button--enabled",
            "ui-search-input-button--with-shortcut",
            "ui-search-input-button--custom-placeholder",
            "ui-search-input-button--custom-compact-placeholder",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
