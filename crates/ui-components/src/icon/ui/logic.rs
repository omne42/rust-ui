use crate::icons_ui::{IconsUiState, IconsUiStateInput, IconsetGlyph};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_icon_reference(icon: String) -> (String, &'static str, bool, bool) {
    let icon = icon.trim();

    if icon.is_empty() {
        return ("ui:help".to_string(), "default", false, true);
    }

    if icon.contains(':') {
        return (icon.into(), "explicit", true, false);
    }

    (format!("ui:{icon}"), "prefixed", false, false)
}

pub fn default_ui_glyphs() -> Vec<IconsetGlyph> {
    vec![
        IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help"),
        IconsetGlyph::new("ui:check", "✓").with_aria_label("UI Check"),
        IconsetGlyph::new("ui:close", "✕").with_aria_label("UI Close"),
        IconsetGlyph::new("ui:chevron-right", "›").with_aria_label("UI Chevron Right"),
        IconsetGlyph::new("ui:chevron-left", "‹").with_aria_label("UI Chevron Left"),
    ]
}

pub fn resolve_state(input: IconsUiStateInput) -> IconsUiState {
    IconsUiState {
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_explicit_icon_reference: input.has_explicit_icon_reference,
        used_default_icon_reference: input.used_default_icon_reference,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_glyphs: input.has_custom_glyphs,
        has_custom_size: input.has_custom_size,
        has_custom_tone: input.has_custom_tone,
        state_attr: if input.disabled {
            "disabled"
        } else if input.decorative {
            "decorative"
        } else {
            "ready"
        },
        icon_reference_source_attr: if input.used_default_icon_reference {
            "default"
        } else if input.has_explicit_icon_reference {
            "explicit"
        } else {
            "prefixed"
        },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        glyph_source_attr: if input.has_custom_glyphs {
            "custom"
        } else {
            "default"
        },
        size_source_attr: if input.has_custom_size {
            "custom"
        } else {
            "default"
        },
        tone_source_attr: if input.has_custom_tone {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconsUiState) -> String {
    let mut classes = vec!["ui-icons-ui".to_string()];

    if state.is_disabled {
        classes.push("ui-icons-ui--disabled".to_string());
    }

    if state.is_decorative {
        classes.push("ui-icons-ui--decorative".to_string());
    }

    if state.has_custom_glyphs {
        classes.push("ui-icons-ui--custom-glyphs".to_string());
    }

    if state.has_custom_size {
        classes.push("ui-icons-ui--custom-size".to_string());
    }

    if state.has_custom_tone {
        classes.push("ui-icons-ui--custom-tone".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-icons-ui--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_icon_reference_tracks_default_explicit_prefixed_paths() {
        assert_eq!(
            normalize_icon_reference("ui:check".to_string()),
            ("ui:check".to_string(), "explicit", true, false)
        );
        assert_eq!(
            normalize_icon_reference("check".to_string()),
            ("ui:check".to_string(), "prefixed", false, false)
        );
        assert_eq!(
            normalize_icon_reference("".to_string()),
            ("ui:help".to_string(), "default", false, true)
        );
    }

    #[test]
    fn default_ui_glyphs_includes_help_and_common_contract_entries() {
        let glyphs = default_ui_glyphs();
        assert!(glyphs.iter().any(|glyph| glyph.name == "ui:help"));
        assert!(glyphs.iter().any(|glyph| glyph.name == "ui:check"));
    }

    #[test]
    fn resolve_state_tracks_sources_and_markers() {
        let state = resolve_state(IconsUiStateInput {
            disabled: false,
            decorative: false,
            has_explicit_icon_reference: true,
            used_default_icon_reference: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_glyphs: true,
            has_custom_size: true,
            has_custom_tone: false,
        });

        assert_eq!(state.state_attr, "ready");
        assert_eq!(state.icon_reference_source_attr, "explicit");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.glyph_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.tone_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-icons-ui-state".to_string()),
            resolve_state(IconsUiStateInput {
                disabled: true,
                decorative: true,
                has_explicit_icon_reference: false,
                used_default_icon_reference: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_glyphs: true,
                has_custom_size: true,
                has_custom_tone: true,
            }),
        );

        for token in [
            "ui-icons-ui",
            "ui-icons-ui--disabled",
            "ui-icons-ui--decorative",
            "ui-icons-ui--custom-glyphs",
            "ui-icons-ui--custom-size",
            "ui-icons-ui--custom-tone",
            "ui-icons-ui--custom-class",
            "docs-icons-ui-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
