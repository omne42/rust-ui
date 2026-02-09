use crate::sidebar::SidebarSide;
use crate::sidebar_rail::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, SidebarRailState, SidebarRailStateInput,
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

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn normalize_default_open(value: Option<bool>) -> bool {
    value.unwrap_or(true)
}

pub fn resolve_state(input: SidebarRailStateInput) -> SidebarRailState {
    SidebarRailState {
        open: input.open,
        closed: !input.open,
        side: input.side,
        side_attr: match input.side {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        },
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarRailState) -> String {
    let mut classes = vec![
        "ui-sidebar__rail".to_string(),
        "ui-sidebar-rail".to_string(),
    ];

    if state.open {
        classes.push("ui-sidebar-rail--open".to_string());
    } else {
        classes.push("ui-sidebar-rail--closed".to_string());
    }

    match state.side {
        SidebarSide::Left => classes.push("ui-sidebar-rail--left".to_string()),
        SidebarSide::Right => classes.push("ui-sidebar-rail--right".to_string()),
    }

    if state.disabled {
        classes.push("ui-sidebar-rail--disabled".to_string());
    }

    if state.is_controlled {
        classes.push("ui-sidebar-rail--controlled".to_string());
    } else {
        classes.push("ui-sidebar-rail--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-rail--custom-class".to_string());
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
            normalize_aria_label(Some("  Toggle inspector rail  ".to_string())),
            ("Toggle inspector rail".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );

        assert_eq!(
            normalize_label(Some("  collapse  ".to_string())),
            ("collapse".to_string(), true)
        );
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.to_string(), false));
        assert!(normalize_default_open(None));
    }

    #[test]
    fn resolve_state_reports_side_control_and_source_markers() {
        let state = resolve_state(SidebarRailStateInput {
            open: false,
            side: SidebarSide::Right,
            disabled: false,
            is_controlled: true,
            has_custom_aria_label: false,
            has_custom_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.state_attr, "closed");
        assert_eq!(state.side_attr, "right");
        assert_eq!(state.control_attr, "controlled");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_side_control_and_custom_markers() {
        let state = resolve_state(SidebarRailStateInput {
            open: true,
            side: SidebarSide::Left,
            disabled: true,
            is_controlled: false,
            has_custom_aria_label: true,
            has_custom_label: true,
            has_custom_class_name: true,
        });
        let class_name = compose_class_name(Some("docs-sidebar-rail-custom".to_string()), state);

        for needle in [
            "ui-sidebar__rail",
            "ui-sidebar-rail",
            "ui-sidebar-rail--open",
            "ui-sidebar-rail--left",
            "ui-sidebar-rail--disabled",
            "ui-sidebar-rail--uncontrolled",
            "ui-sidebar-rail--custom-class",
            "docs-sidebar-rail-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "missing `{needle}` in sidebar rail class contract"
            );
        }
    }
}
