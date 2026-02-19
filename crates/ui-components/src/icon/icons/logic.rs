use crate::icons::{IconsSet, IconsState, IconsStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn parse_set_from_name(name: &str) -> Option<IconsSet> {
    let (prefix, _) = name.trim().split_once(':')?;

    match prefix {
        "workflow" => Some(IconsSet::Workflow),
        "ui" => Some(IconsSet::Ui),
        _ => None,
    }
}

pub fn resolve_set(name: &str, set: IconsSet) -> (IconsSet, bool) {
    let parsed = parse_set_from_name(name);
    (parsed.unwrap_or(set), parsed.is_some())
}

pub fn normalize_name(name: String, set: IconsSet) -> String {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return format!("{}:help", set.as_attr());
    }

    if trimmed.contains(':') {
        return trimmed.into();
    }

    format!("{}:{trimmed}", set.as_attr())
}

pub fn resolve_state(input: IconsStateInput) -> IconsState {
    IconsState {
        set: input.set,
        scale: input.scale,
        is_disabled: input.disabled,
        is_decorative: input.decorative,
        has_set_prefix_in_name: input.has_set_prefix_in_name,
        has_custom_set_prop: input.has_custom_set_prop,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_glyphs: input.has_custom_glyphs,
        has_custom_tone: input.has_custom_tone,
        set_attr: input.set.as_attr(),
        scale_attr: input.scale.as_attr(),
        state_attr: if input.disabled {
            "disabled"
        } else if input.decorative {
            "decorative"
        } else {
            "ready"
        },
        set_source_attr: if input.has_set_prefix_in_name {
            "name"
        } else if input.has_custom_set_prop {
            "prop"
        } else {
            "default"
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
        tone_source_attr: if input.has_custom_tone {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: IconsState) -> String {
    let mut classes = vec![
        "ui-icons".to_string(),
        format!("ui-icons--set-{}", state.set_attr),
    ];

    classes.push(format!("ui-icons--scale-{}", state.scale_attr));

    if state.is_disabled {
        classes.push("ui-icons--disabled".to_string());
    }

    if state.is_decorative {
        classes.push("ui-icons--decorative".to_string());
    }

    if state.has_custom_glyphs {
        classes.push("ui-icons--custom-glyphs".to_string());
    }

    if state.has_custom_tone {
        classes.push("ui-icons--custom-tone".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-icons--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons::IconsScale;

    #[test]
    fn parse_and_resolve_set_follow_name_prefix_priority() {
        assert_eq!(
            parse_set_from_name("workflow:check"),
            Some(IconsSet::Workflow)
        );
        assert_eq!(parse_set_from_name("ui:check"), Some(IconsSet::Ui));
        assert_eq!(parse_set_from_name("check"), None);

        assert_eq!(
            resolve_set("workflow:check", IconsSet::Ui),
            (IconsSet::Workflow, true)
        );
        assert_eq!(
            resolve_set("check", IconsSet::Workflow),
            (IconsSet::Workflow, false)
        );
    }

    #[test]
    fn normalize_name_preserves_prefix_and_applies_defaults() {
        assert_eq!(
            normalize_name("check".to_string(), IconsSet::Ui),
            "ui:check"
        );
        assert_eq!(
            normalize_name("workflow:check".to_string(), IconsSet::Ui),
            "workflow:check"
        );
        assert_eq!(
            normalize_name("".to_string(), IconsSet::Workflow),
            "workflow:help"
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_state_markers() {
        let state = resolve_state(IconsStateInput {
            set: IconsSet::Workflow,
            scale: IconsScale::Large,
            disabled: false,
            decorative: false,
            has_set_prefix_in_name: true,
            has_custom_set_prop: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_glyphs: true,
            has_custom_tone: false,
        });

        assert_eq!(state.set_attr, "workflow");
        assert_eq!(state.scale_attr, "large");
        assert_eq!(state.state_attr, "ready");
        assert_eq!(state.set_source_attr, "name");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert_eq!(state.glyph_source_attr, "custom");
        assert_eq!(state.tone_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-icons-state".to_string()),
            resolve_state(IconsStateInput {
                set: IconsSet::Ui,
                scale: IconsScale::Medium,
                disabled: true,
                decorative: true,
                has_set_prefix_in_name: false,
                has_custom_set_prop: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_glyphs: true,
                has_custom_tone: true,
            }),
        );

        for token in [
            "ui-icons",
            "ui-icons--set-ui",
            "ui-icons--scale-medium",
            "ui-icons--disabled",
            "ui-icons--decorative",
            "ui-icons--custom-glyphs",
            "ui-icons--custom-tone",
            "ui-icons--custom-class",
            "docs-icons-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
