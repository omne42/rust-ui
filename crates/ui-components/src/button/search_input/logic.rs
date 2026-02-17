use super::super::{ButtonType, logic as button_logic};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonStateInput {
    pub is_disabled: bool,
    pub has_shortcut: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_compact_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchInputButtonState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub state_attr: &'static str,
    pub state_class: &'static str,
    pub has_shortcut: bool,
    pub shortcut_attr: &'static str,
    pub shortcut_class: &'static str,
    pub has_custom_placeholder: bool,
    pub placeholder_source_attr: &'static str,
    pub placeholder_source_class: &'static str,
    pub has_custom_compact_placeholder: bool,
    pub compact_placeholder_source_attr: &'static str,
    pub compact_placeholder_source_class: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_label_source_attr: &'static str,
    pub aria_label_source_class: &'static str,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchInputButtonAriaLabelResolution {
    pub aria_label: String,
    pub has_custom_aria_label: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    button_logic::normalize_optional_text(value)
}

pub fn resolve_button_type(button_type: Option<ButtonType>) -> ButtonType {
    button_type.unwrap_or_default()
}

pub fn resolve_effective_aria_label(
    aria_label: Option<String>,
    placeholder: &str,
) -> SearchInputButtonAriaLabelResolution {
    match normalize_optional_text(aria_label) {
        Some(aria_label) => SearchInputButtonAriaLabelResolution {
            aria_label,
            has_custom_aria_label: true,
        },
        None => SearchInputButtonAriaLabelResolution {
            aria_label: placeholder.to_string(),
            has_custom_aria_label: false,
        },
    }
}

pub fn resolve_state(input: SearchInputButtonStateInput) -> SearchInputButtonState {
    let is_disabled = input.is_disabled;

    let (state_attr, state_class) = if is_disabled {
        ("disabled", "ui-search-input-button--disabled")
    } else {
        ("enabled", "ui-search-input-button--enabled")
    };

    let (shortcut_attr, shortcut_class) = if input.has_shortcut {
        ("visible", "ui-search-input-button--with-shortcut")
    } else {
        ("hidden", "ui-search-input-button--without-shortcut")
    };

    let (placeholder_source_attr, placeholder_source_class) = if input.has_custom_placeholder {
        ("custom", "ui-search-input-button--custom-placeholder")
    } else {
        ("default", "ui-search-input-button--default-placeholder")
    };

    let (compact_placeholder_source_attr, compact_placeholder_source_class) =
        if input.has_custom_compact_placeholder {
            (
                "custom",
                "ui-search-input-button--custom-compact-placeholder",
            )
        } else {
            (
                "default",
                "ui-search-input-button--default-compact-placeholder",
            )
        };

    let (aria_label_source_attr, aria_label_source_class) = if input.has_custom_aria_label {
        ("custom", "ui-search-input-button--custom-aria-label")
    } else {
        (
            "placeholder",
            "ui-search-input-button--placeholder-aria-label",
        )
    };

    SearchInputButtonState {
        is_disabled,
        is_enabled: !is_disabled,
        state_attr,
        state_class,
        has_shortcut: input.has_shortcut,
        shortcut_attr,
        shortcut_class,
        has_custom_placeholder: input.has_custom_placeholder,
        placeholder_source_attr,
        placeholder_source_class,
        has_custom_compact_placeholder: input.has_custom_compact_placeholder,
        compact_placeholder_source_attr,
        compact_placeholder_source_class,
        has_custom_aria_label: input.has_custom_aria_label,
        aria_label_source_attr,
        aria_label_source_class,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_view_state(
    placeholder: Option<&str>,
    compact_placeholder: Option<&str>,
    meta_key_label: Option<&str>,
    key_label: Option<&str>,
    fallback_placeholder: &str,
) -> SearchInputButtonViewState {
    let placeholder = placeholder
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback_placeholder)
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
    let mut classes = vec![
        "ui-search-input-button".to_string(),
        state.state_class.to_string(),
        state.shortcut_class.to_string(),
        state.placeholder_source_class.to_string(),
        state.compact_placeholder_source_class.to_string(),
        state.aria_label_source_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-search-input-button--custom-class".to_string());
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
    fn resolve_button_type_defaults_to_button() {
        assert_eq!(resolve_button_type(None), ButtonType::Button);
        assert_eq!(
            resolve_button_type(Some(ButtonType::Button)),
            ButtonType::Button
        );
        assert_eq!(
            resolve_button_type(Some(ButtonType::Submit)),
            ButtonType::Submit
        );
        assert_eq!(
            resolve_button_type(Some(ButtonType::Reset)),
            ButtonType::Reset
        );
    }

    #[test]
    fn resolve_effective_aria_label_prefers_explicit_and_falls_back_to_placeholder() {
        let custom = resolve_effective_aria_label(Some("  Open search  ".to_string()), "Search");
        assert_eq!(custom.aria_label, "Open search");
        assert!(custom.has_custom_aria_label);

        let fallback = resolve_effective_aria_label(Some("   ".to_string()), "Search docs");
        assert_eq!(fallback.aria_label, "Search docs");
        assert!(!fallback.has_custom_aria_label);
    }

    #[test]
    fn resolve_state_tracks_enablement_and_metadata_flags() {
        let enabled_state = resolve_state(SearchInputButtonStateInput {
            is_disabled: false,
            has_shortcut: true,
            has_custom_placeholder: true,
            has_custom_compact_placeholder: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });
        assert!(enabled_state.is_enabled);
        assert!(!enabled_state.is_disabled);
        assert_eq!(enabled_state.state_attr, "enabled");
        assert_eq!(enabled_state.shortcut_attr, "visible");
        assert_eq!(enabled_state.placeholder_source_attr, "custom");
        assert_eq!(enabled_state.compact_placeholder_source_attr, "custom");
        assert_eq!(enabled_state.aria_label_source_attr, "custom");
        assert!(enabled_state.has_custom_class_name);

        let disabled_state = resolve_state(SearchInputButtonStateInput {
            is_disabled: true,
            has_shortcut: false,
            has_custom_placeholder: false,
            has_custom_compact_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });
        assert!(!disabled_state.is_enabled);
        assert!(disabled_state.is_disabled);
        assert_eq!(disabled_state.state_attr, "disabled");
        assert_eq!(disabled_state.shortcut_attr, "hidden");
        assert_eq!(disabled_state.placeholder_source_attr, "default");
        assert_eq!(disabled_state.compact_placeholder_source_attr, "default");
        assert_eq!(disabled_state.aria_label_source_attr, "placeholder");
    }

    #[test]
    fn view_state_defaults_and_trims_with_blank_fallbacks() {
        let state = resolve_view_state(Some("  Search docs... "), None, None, None, "Search");
        assert_eq!(state.placeholder, "Search docs...");
        assert_eq!(state.compact_placeholder, "Search docs...");
        assert!(!state.show_shortcut);

        let state = resolve_view_state(
            Some("   "),
            Some("  Go "),
            Some(" ⌘ "),
            Some(" K "),
            "Search",
        );
        assert_eq!(state.placeholder, "Search");
        assert_eq!(state.compact_placeholder, "Go");
        assert_eq!(state.meta_key_label.as_deref(), Some("⌘"));
        assert_eq!(state.key_label.as_deref(), Some("K"));
        assert!(state.show_shortcut);
    }

    #[test]
    fn shortcut_requires_both_keys() {
        let state = resolve_view_state(Some("Search"), None, Some("⌘"), None, "Search");
        assert!(!state.show_shortcut);

        let state = resolve_view_state(Some("Search"), None, None, Some("K"), "Search");
        assert!(!state.show_shortcut);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SearchInputButtonStateInput {
                is_disabled: false,
                has_shortcut: true,
                has_custom_placeholder: true,
                has_custom_compact_placeholder: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-search-input-button",
            "ui-search-input-button--enabled",
            "ui-search-input-button--with-shortcut",
            "ui-search-input-button--custom-placeholder",
            "ui-search-input-button--custom-compact-placeholder",
            "ui-search-input-button--placeholder-aria-label",
            "ui-search-input-button--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
