use crate::sidebar_menu_badge::{
    DEFAULT_ARIA_LABEL, SidebarMenuBadgeState, SidebarMenuBadgeStateInput,
};

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

pub fn resolve_state(input: SidebarMenuBadgeStateInput) -> SidebarMenuBadgeState {
    SidebarMenuBadgeState {
        muted: input.muted,
        emphasized: !input.muted,
        disabled: input.disabled,
        enabled: !input.disabled,
        state_attr: if input.disabled {
            "disabled"
        } else if input.muted {
            "muted"
        } else {
            "emphasized"
        },
        tone_attr: if input.muted { "muted" } else { "emphasized" },
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarMenuBadgeState) -> String {
    let mut classes = vec!["ui-sidebar-menu-badge".to_string()];

    if state.muted {
        classes.push("ui-sidebar-menu-badge--muted".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar-menu-badge--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-menu-badge--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_aria_label_tracks_default_and_custom_sources() {
        assert_eq!(
            normalize_aria_label(Some("  Open reviews  ".to_string())),
            ("Open reviews".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_tone_disabled_and_source_markers() {
        let muted = resolve_state(SidebarMenuBadgeStateInput {
            muted: true,
            disabled: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(muted.state_attr, "muted");
        assert_eq!(muted.tone_attr, "muted");
        assert!(muted.enabled);
        assert_eq!(muted.aria_source_attr, "default");
        assert_eq!(muted.class_source_attr, "custom");

        let disabled = resolve_state(SidebarMenuBadgeStateInput {
            muted: false,
            disabled: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(disabled.state_attr, "disabled");
        assert_eq!(disabled.tone_attr, "emphasized");
        assert!(disabled.disabled);
        assert_eq!(disabled.aria_source_attr, "custom");
        assert_eq!(disabled.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(SidebarMenuBadgeStateInput {
            muted: true,
            disabled: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });
        let class_name =
            compose_class_name(Some("docs-sidebar-menu-badge-custom".to_string()), state);

        for token in [
            "ui-sidebar-menu-badge",
            "ui-sidebar-menu-badge--muted",
            "ui-sidebar-menu-badge--disabled",
            "ui-sidebar-menu-badge--custom-class",
            "docs-sidebar-menu-badge-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
