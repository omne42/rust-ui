use crate::sidebar_trigger::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, SidebarTriggerState, SidebarTriggerStateInput,
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

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.into(), false)
}

pub fn normalize_default_open(value: Option<bool>) -> bool {
    value.unwrap_or(true)
}

pub fn resolve_state(input: SidebarTriggerStateInput) -> SidebarTriggerState {
    SidebarTriggerState {
        open: input.open,
        closed: !input.open,
        disabled: input.disabled,
        enabled: !input.disabled,
        is_controlled: input.is_controlled,
        is_uncontrolled: !input.is_controlled,
        state_attr: if input.disabled {
            if input.open {
                "disabled-open"
            } else {
                "disabled-closed"
            }
        } else if input.open {
            "open"
        } else {
            "closed"
        },
        control_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_label {
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarTriggerState) -> String {
    let mut classes = vec![
        "ui-sidebar__trigger".to_string(),
        "ui-sidebar-trigger".to_string(),
    ];

    if state.open {
        classes.push("ui-sidebar-trigger--open".to_string());
    } else {
        classes.push("ui-sidebar-trigger--closed".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar-trigger--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar-trigger--controlled".to_string());
    } else {
        classes.push("ui-sidebar-trigger--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-trigger--custom-class".to_string());
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
    fn normalize_helpers_track_defaults_and_custom_sources() {
        assert_eq!(
            normalize_aria_label(Some("  Toggle workspace  ".to_string())),
            ("Toggle workspace".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );

        assert_eq!(
            normalize_label(Some("  Collapse  ".to_string())),
            ("Collapse".to_string(), true)
        );
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));
        assert!(normalize_default_open(None));
    }

    #[test]
    fn resolve_state_reports_open_control_and_source_markers() {
        let state = resolve_state(SidebarTriggerStateInput {
            open: false,
            disabled: false,
            is_controlled: true,
            has_custom_aria_label: false,
            has_custom_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_attr, "closed");
        assert_eq!(state.control_attr, "controlled");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_control_and_custom_markers() {
        let state = resolve_state(SidebarTriggerStateInput {
            open: true,
            disabled: true,
            is_controlled: false,
            has_custom_aria_label: true,
            has_custom_label: true,
            has_custom_class_name: true,
        });
        let class_name = compose_class_name(Some("docs-sidebar-trigger-custom".to_string()), state);

        for needle in [
            "ui-sidebar__trigger",
            "ui-sidebar-trigger",
            "ui-sidebar-trigger--open",
            "ui-sidebar-trigger--disabled",
            "ui-sidebar-trigger--uncontrolled",
            "ui-sidebar-trigger--custom-class",
            "docs-sidebar-trigger-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "missing `{needle}` in sidebar trigger class contract"
            );
        }
    }
}
