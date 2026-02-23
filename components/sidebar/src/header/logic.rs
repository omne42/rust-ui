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

pub fn resolve_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
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
#[path = "../../test/header/logic.rs"]
mod tests;
