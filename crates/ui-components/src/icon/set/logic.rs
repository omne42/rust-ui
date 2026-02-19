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
mod tests {
    use super::*;

    #[test]
    fn normalize_and_parse_helpers_trim_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  workflow  ".to_string())),
            Some("workflow".to_string())
        );

        assert_eq!(
            parse_icon_reference("workflow:check"),
            (Some("workflow".to_string()), "check".to_string())
        );
        assert_eq!(parse_icon_reference("alert"), (None, "alert".to_string()));
    }

    #[test]
    fn resolve_iconset_namespace_tracks_source_priority() {
        assert_eq!(
            resolve_iconset_namespace(Some("ui".to_string()), Some("workflow".to_string())),
            ("ui".to_string(), "prop", true, true)
        );
        assert_eq!(
            resolve_iconset_namespace(None, Some("workflow".to_string())),
            ("workflow".to_string(), "icon", false, true)
        );
        assert_eq!(
            resolve_iconset_namespace(None, None),
            (DEFAULT_ICONSET_NAMESPACE.into(), "default", false, false)
        );
    }

    #[test]
    fn registry_helpers_resolve_matches_and_fallbacks() {
        assert!(glyph_matches("workflow:check", "workflow", "check"));
        assert!(glyph_matches("check", "workflow", "check"));
        assert!(!glyph_matches("ui:check", "workflow", "check"));

        let (glyph, matched, label) = resolve_registry_glyph(
            vec![IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check")],
            "workflow",
            "check",
        );
        assert_eq!(glyph, "✓");
        assert!(matched);
        assert_eq!(label, Some("Workflow Check".to_string()));

        let (glyph, matched, label) = resolve_registry_glyph(vec![], "workflow", "alert");
        assert_eq!(glyph, FALLBACK_GLYPH);
        assert!(!matched);
        assert_eq!(label, None);
    }

    #[test]
    fn resolve_state_and_class_name_surface_all_markers() {
        let state = resolve_state(IconsetStateInput {
            disabled: false,
            decorative: false,
            has_registry_match: true,
            has_registry_label: true,
            has_custom_iconset_prop: true,
            has_iconset_in_icon_reference: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_size: true,
            has_custom_tone: false,
        });

        assert_eq!(state.state_attr, "resolved");
        assert_eq!(state.icon_source_attr, "registry");
        assert_eq!(state.iconset_source_attr, "prop");
        assert_eq!(state.label_source_attr, "registry");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.tone_source_attr, "default");

        let class_name = compose_class_name(Some("docs-iconset-state".to_string()), state);

        for token in [
            "ui-iconset",
            "ui-iconset--registry",
            "ui-iconset--custom-size",
            "ui-iconset--custom-class",
            "docs-iconset-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn resolve_accessible_label_prioritizes_custom_then_registry_then_fallback() {
        assert_eq!(
            resolve_accessible_label(
                false,
                Some("  Custom Label  ".to_string()),
                Some("Registry".to_string()),
                "workflow-check"
            ),
            "Custom Label"
        );
        assert_eq!(
            resolve_accessible_label(false, None, Some("Registry".to_string()), "workflow-check"),
            "Registry"
        );
        assert_eq!(
            resolve_accessible_label(false, None, None, "workflow-check"),
            "workflow check"
        );
        assert_eq!(
            resolve_accessible_label(
                true,
                Some("Custom".to_string()),
                Some("Registry".to_string()),
                "workflow-check"
            ),
            ""
        );
    }
}
