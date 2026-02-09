use crate::sidebar_menu_action::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, SidebarMenuActionState, SidebarMenuActionStateInput,
};

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

pub fn normalize_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_LABEL.to_string())
}

pub fn resolve_state(input: SidebarMenuActionStateInput) -> SidebarMenuActionState {
    SidebarMenuActionState {
        hover_only: input.hover_only,
        always_visible: !input.hover_only,
        disabled: input.disabled,
        enabled: !input.disabled,
        state_attr: if input.disabled {
            "disabled"
        } else if input.hover_only {
            "hover-only"
        } else {
            "visible"
        },
        visibility_attr: if input.hover_only { "hover" } else { "always" },
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarMenuActionState) -> String {
    let mut classes = vec!["ui-sidebar-menu-action".to_string()];

    if state.hover_only {
        classes.push("ui-sidebar-menu-action--hover-only".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar-menu-action--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-menu-action--custom-class".to_string());
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
    fn normalize_defaults_and_trimmed_values_are_stable() {
        assert_eq!(
            normalize_aria_label(Some("  Item options  ".to_string())),
            ("Item options".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(normalize_label(None), DEFAULT_LABEL.to_string());
        assert_eq!(
            normalize_label(Some("  Open actions  ".to_string())),
            "Open actions".to_string()
        );
    }

    #[test]
    fn resolve_state_tracks_visibility_and_source_markers() {
        let hover = resolve_state(SidebarMenuActionStateInput {
            hover_only: true,
            disabled: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });
        assert_eq!(hover.state_attr, "hover-only");
        assert_eq!(hover.visibility_attr, "hover");
        assert_eq!(hover.class_source_attr, "custom");

        let disabled = resolve_state(SidebarMenuActionStateInput {
            hover_only: false,
            disabled: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });
        assert_eq!(disabled.state_attr, "disabled");
        assert_eq!(disabled.visibility_attr, "always");
        assert_eq!(disabled.aria_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(SidebarMenuActionStateInput {
            hover_only: true,
            disabled: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });
        let class_name =
            compose_class_name(Some("docs-sidebar-menu-action-custom".to_string()), state);

        for token in [
            "ui-sidebar-menu-action",
            "ui-sidebar-menu-action--hover-only",
            "ui-sidebar-menu-action--disabled",
            "ui-sidebar-menu-action--custom-class",
            "docs-sidebar-menu-action-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
