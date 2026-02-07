#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyViewState {
    pub is_copyable: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_text: bool,
    pub has_custom_label: bool,
    pub has_custom_copied_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_view_state(
    text: &str,
    disabled: bool,
    has_custom_label: bool,
    has_custom_copied_label: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ButtonCopyViewState {
    let has_text = !text.trim().is_empty();
    let is_copyable = !disabled && has_text;

    ButtonCopyViewState {
        is_copyable,
        is_disabled: disabled,
        is_enabled: !disabled,
        has_text,
        has_custom_label,
        has_custom_copied_label,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ButtonCopyViewState) -> String {
    let mut classes = vec!["ui-button-copy".to_string()];

    if state.is_copyable {
        classes.push("ui-button-copy--copyable".to_string());
    }
    if state.is_disabled {
        classes.push("ui-button-copy--disabled".to_string());
    }
    if !state.has_text {
        classes.push("ui-button-copy--empty".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-button-copy--custom-label".to_string());
    }
    if state.has_custom_copied_label {
        classes.push("ui-button-copy--custom-copied-label".to_string());
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
            normalize_optional_text(Some("  Copy now  ".to_string())),
            Some("Copy now".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn empty_text_is_not_copyable() {
        assert!(!resolve_view_state("", false, false, false, false, false).is_copyable);
        assert!(!resolve_view_state("   ", false, false, false, false, false).is_copyable);
    }

    #[test]
    fn disabled_is_not_copyable_even_when_text_present() {
        assert!(!resolve_view_state("hello", true, false, false, false, false).is_copyable);
    }

    #[test]
    fn enabled_with_text_is_copyable() {
        assert!(resolve_view_state("hello", false, false, false, false, false).is_copyable);
    }

    #[test]
    fn resolve_view_state_tracks_metadata_flags() {
        let state = resolve_view_state("hello", false, true, true, true, true);
        assert!(state.is_copyable);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.has_text);
        assert!(state.has_custom_label);
        assert!(state.has_custom_copied_label);
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_view_state("hello", false, true, true, false, true),
        );

        for token in [
            "ui-button-copy",
            "ui-button-copy--copyable",
            "ui-button-copy--custom-label",
            "ui-button-copy--custom-copied-label",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
