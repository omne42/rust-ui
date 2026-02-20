use crate::iconset::{IconsetGlyph, IconsetState, IconsetStateInput};

pub const DEFAULT_ICONSET_NAMESPACE: &str = "custom-icons";
pub const FALLBACK_GLYPH: &str = "⬚";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn parse_icon_reference(icon: &str) -> (Option<String>, String) {
    let trimmed = icon.trim();

    if let Some((iconset, icon_name)) = trimmed.split_once(':') {
        let iconset = normalize_optional_text(Some(iconset.to_string()));
        let icon_name = normalize_optional_text(Some(icon_name.into())).unwrap_or_default();
        return (iconset, icon_name);
    }

    (None, trimmed.into())
}

pub fn resolve_iconset_namespace(
    iconset_from_prop: Option<String>,
    iconset_from_icon: Option<String>,
) -> (String, &'static str, bool, bool) {
    if let Some(iconset) = iconset_from_prop {
        return (iconset, "prop", true, iconset_from_icon.is_some());
    }

    if let Some(iconset) = iconset_from_icon {
        return (iconset, "icon", false, true);
    }

    (DEFAULT_ICONSET_NAMESPACE.into(), "default", false, false)
}

pub fn glyph_matches(candidate_name: &str, iconset: &str, icon_name: &str) -> bool {
    let Some(candidate_name) = normalize_optional_text(Some(candidate_name.into())) else {
        return false;
    };

    if candidate_name.eq_ignore_ascii_case(icon_name) {
        return true;
    }

    if let Some((candidate_set, candidate_icon_name)) = candidate_name.split_once(':') {
        return candidate_set.eq_ignore_ascii_case(iconset)
            && candidate_icon_name.eq_ignore_ascii_case(icon_name);
    }

    false
}

pub fn resolve_registry_glyph(
    glyphs: Vec<IconsetGlyph>,
    resolved_iconset: &str,
    icon_name: &str,
) -> (String, bool, Option<String>) {
    let registry_match = glyphs
        .into_iter()
        .find(|glyph| glyph_matches(&glyph.name, resolved_iconset, icon_name));

    if let Some(glyph) = registry_match {
        let glyph_content = normalize_optional_text(Some(glyph.glyph))
            .unwrap_or_else(|| FALLBACK_GLYPH.to_string());
        return (
            glyph_content,
            true,
            normalize_optional_text(glyph.aria_label),
        );
    }

    (FALLBACK_GLYPH.to_string(), false, None)
}

pub fn resolve_accessible_label(
    decorative: bool,
    custom_aria_label: Option<String>,
    registry_label: Option<String>,
    icon_name: &str,
) -> String {
    if decorative {
        return String::new();
    }

    normalize_optional_text(custom_aria_label)
        .or(registry_label)
        .unwrap_or_else(|| icon_name.replace(['-', '_'], " "))
}

pub fn resolve_state(input: IconsetStateInput) -> IconsetState {
    IconsetState {
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_registry_match: input.has_registry_match,
        has_registry_label: input.has_registry_label,
        has_custom_iconset_prop: input.has_custom_iconset_prop,
        has_iconset_in_icon_reference: input.has_iconset_in_icon_reference,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_size: input.has_custom_size,
        has_custom_tone: input.has_custom_tone,
        state_attr: if input.disabled {
            "disabled"
        } else if input.decorative {
            "decorative"
        } else if input.has_registry_match {
            "resolved"
        } else {
            "fallback"
        },
        icon_source_attr: if input.has_registry_match {
            "registry"
        } else {
            "fallback"
        },
        iconset_source_attr: if input.has_custom_iconset_prop {
            "prop"
        } else if input.has_iconset_in_icon_reference {
            "icon"
        } else {
            "default"
        },
        label_source_attr: if input.decorative {
            "decorative"
        } else if input.has_custom_aria_label {
            "custom"
        } else if input.has_registry_label {
            "registry"
        } else {
            "fallback"
        },
        class_source_attr: if input.has_custom_class_name {
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

pub fn compose_class_name(base_class_name: Option<String>, state: IconsetState) -> String {
    let mut classes = vec!["ui-iconset".to_string()];

    if state.is_disabled {
        classes.push("ui-iconset--disabled".to_string());
    }

    if state.is_decorative {
        classes.push("ui-iconset--decorative".to_string());
    }

    if state.has_registry_match {
        classes.push("ui-iconset--registry".to_string());
    } else {
        classes.push("ui-iconset--fallback".to_string());
    }

    if state.has_custom_size {
        classes.push("ui-iconset--custom-size".to_string());
    }

    if state.has_custom_tone {
        classes.push("ui-iconset--custom-tone".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-iconset--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/set/logic.rs"]
mod tests;
