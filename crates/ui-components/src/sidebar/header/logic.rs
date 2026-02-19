use crate::sidebar_header::{DEFAULT_ARIA_LABEL, SidebarHeaderState, SidebarHeaderStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: SidebarHeaderStateInput) -> SidebarHeaderState {
    SidebarHeaderState {
        disabled: input.disabled,
        enabled: !input.disabled,
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
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
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SidebarHeaderState) -> String {
    let mut classes = vec![
        "ui-sidebar__header".to_string(),
        "ui-sidebar-header".to_string(),
    ];

    if state.disabled {
        classes.push("ui-sidebar-header--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-header--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidebar_header::DEFAULT_ARIA_LABEL;

    #[test]
    fn normalize_aria_label_tracks_default_and_custom_sources() {
        assert_eq!(
            normalize_aria_label(Some("  Workspace header  ".to_string())),
            ("Workspace header".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn resolve_state_reports_disabled_and_source_markers() {
        let state = resolve_state(SidebarHeaderStateInput {
            disabled: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_attr, "disabled");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert!(state.disabled);
        assert!(!state.enabled);
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(SidebarHeaderStateInput {
            disabled: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });
        let class_name = compose_class_name(Some("docs-sidebar-header-custom".to_string()), state);

        for needle in [
            "ui-sidebar__header",
            "ui-sidebar-header",
            "ui-sidebar-header--disabled",
            "ui-sidebar-header--custom-class",
            "docs-sidebar-header-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "missing `{needle}` in sidebar header class contract"
            );
        }
    }
}
