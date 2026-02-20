use crate::sidebar::SidebarSide;
use crate::sidebar_rail::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, SidebarRailState, SidebarRailStateInput,
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
#[path = "../../test/rail/logic.rs"]
mod tests;
