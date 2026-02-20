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
#[path = "../../test/ui/logic.rs"]
mod tests;
