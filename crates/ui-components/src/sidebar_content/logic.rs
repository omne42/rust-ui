use crate::sidebar_content::{DEFAULT_ARIA_LABEL, SidebarContentState, SidebarContentStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: SidebarContentStateInput) -> SidebarContentState {
    SidebarContentState {
        disabled: input.disabled,
        enabled: !input.disabled,
        padded: input.padded,
        compact: !input.padded,
        scrollable: input.scrollable,
        static_layout: !input.scrollable,
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        padding_attr: if input.padded { "padded" } else { "compact" },
        scroll_attr: if input.scrollable {
            "scrollable"
        } else {
            "static"
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarContentState) -> String {
    let mut classes = vec![
        "ui-sidebar__content".to_string(),
        "ui-sidebar-content".to_string(),
    ];

    if state.disabled {
        classes.push("ui-sidebar-content--disabled".to_string());
    }

    if state.padded {
        classes.push("ui-sidebar-content--padded".to_string());
    }

    if state.scrollable {
        classes.push("ui-sidebar-content--scrollable".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-content--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidebar_content::DEFAULT_ARIA_LABEL;

    #[test]
    fn normalize_aria_label_tracks_default_and_custom_sources() {
        assert_eq!(
            normalize_aria_label(Some("  Sidebar section content  ".to_string())),
            ("Sidebar section content".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_reports_padding_scroll_and_source_markers() {
        let state = resolve_state(SidebarContentStateInput {
            disabled: true,
            padded: false,
            scrollable: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_attr, "disabled");
        assert_eq!(state.padding_attr, "compact");
        assert_eq!(state.scroll_attr, "static");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert!(state.disabled);
        assert!(!state.enabled);
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(SidebarContentStateInput {
            disabled: true,
            padded: true,
            scrollable: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });
        let class_name = compose_class_name(Some("docs-sidebar-content-custom".to_string()), state);

        for needle in [
            "ui-sidebar__content",
            "ui-sidebar-content",
            "ui-sidebar-content--disabled",
            "ui-sidebar-content--padded",
            "ui-sidebar-content--scrollable",
            "ui-sidebar-content--custom-class",
            "docs-sidebar-content-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "missing `{needle}` in sidebar content class contract"
            );
        }
    }
}
