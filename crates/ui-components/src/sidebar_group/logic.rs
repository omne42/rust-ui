use crate::sidebar_group::{DEFAULT_ACTION_LABEL, DEFAULT_ARIA_LABEL, DEFAULT_LABEL};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarGroupStateInput {
    pub open: bool,
    pub collapsible: bool,
    pub disabled: bool,
    pub show_label: bool,
    pub show_action: bool,
    pub has_label: bool,
    pub has_action: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarGroupState {
    pub open: bool,
    pub closed: bool,
    pub collapsible: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub show_label: bool,
    pub show_action: bool,
    pub has_label: bool,
    pub has_action: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub state_attr: &'static str,
    pub collapse_attr: &'static str,
    pub control_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string())
}

pub fn normalize_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_LABEL.to_string())
}

pub fn normalize_action_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ACTION_LABEL.to_string())
}

pub fn normalize_default_open(value: Option<bool>) -> bool {
    value.unwrap_or(true)
}

pub fn resolve_state(input: SidebarGroupStateInput) -> SidebarGroupState {
    let enabled = !input.disabled;
    let closed = !input.open;
    let is_uncontrolled = !input.is_controlled;

    SidebarGroupState {
        open: input.open,
        closed,
        collapsible: input.collapsible,
        disabled: input.disabled,
        enabled,
        show_label: input.show_label,
        show_action: input.show_action,
        has_label: input.has_label,
        has_action: input.has_action,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_class_name: input.has_custom_class_name,
        state_attr: if input.disabled && closed {
            "disabled-closed"
        } else if input.disabled {
            "disabled"
        } else if closed {
            "closed"
        } else {
            "open"
        },
        collapse_attr: if input.collapsible {
            "collapsible"
        } else {
            "static"
        },
        control_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(class_name: Option<String>, state: SidebarGroupState) -> String {
    let mut classes = vec!["ui-sidebar-group".to_string()];

    if state.collapsible {
        classes.push("ui-sidebar-group--collapsible".to_string());
    }

    if state.open {
        classes.push("ui-sidebar-group--open".to_string());
    } else {
        classes.push("ui-sidebar-group--closed".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar-group--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar-group--controlled".to_string());
    } else {
        classes.push("ui-sidebar-group--uncontrolled".to_string());
    }

    if !state.show_label {
        classes.push("ui-sidebar-group--label-hidden".to_string());
    }

    if !state.show_action {
        classes.push("ui-sidebar-group--action-hidden".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-group--custom-class".to_string());
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
    fn normalize_helpers_apply_defaults() {
        assert_eq!(normalize_aria_label(None), DEFAULT_ARIA_LABEL);
        assert_eq!(normalize_label(None), DEFAULT_LABEL);
        assert_eq!(normalize_action_label(None), DEFAULT_ACTION_LABEL);
        assert!(normalize_default_open(None));
    }

    #[test]
    fn resolve_state_tracks_flags_and_attrs() {
        let state = resolve_state(SidebarGroupStateInput {
            open: false,
            collapsible: true,
            disabled: false,
            show_label: true,
            show_action: false,
            has_label: true,
            has_action: false,
            is_controlled: true,
            has_custom_class_name: true,
        });

        assert!(state.closed);
        assert_eq!(state.state_attr, "closed");
        assert_eq!(state.collapse_attr, "collapsible");
        assert_eq!(state.control_attr, "controlled");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_markers() {
        let class_name = compose_class_name(
            Some("demo".to_string()),
            resolve_state(SidebarGroupStateInput {
                open: true,
                collapsible: true,
                disabled: true,
                show_label: false,
                show_action: true,
                has_label: true,
                has_action: true,
                is_controlled: false,
                has_custom_class_name: true,
            }),
        );

        for needle in [
            "ui-sidebar-group",
            "ui-sidebar-group--collapsible",
            "ui-sidebar-group--open",
            "ui-sidebar-group--disabled",
            "ui-sidebar-group--uncontrolled",
            "ui-sidebar-group--label-hidden",
            "ui-sidebar-group--custom-class",
            "demo",
        ] {
            assert!(
                class_name.contains(needle),
                "missing `{needle}` in class_name"
            );
        }
    }
}
