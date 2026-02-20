use crate::sidebar_footer::{DEFAULT_ARIA_LABEL, SidebarFooterState, SidebarFooterStateInput};

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

pub fn resolve_state(input: SidebarFooterStateInput) -> SidebarFooterState {
    SidebarFooterState {
        disabled: input.disabled,
        enabled: !input.disabled,
        bordered: input.bordered,
        unbordered: !input.bordered,
        state_attr: if input.disabled {
            "disabled"
        } else {
            "enabled"
        },
        border_attr: if input.bordered { "bordered" } else { "plain" },
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarFooterState) -> String {
    let mut classes = vec![
        "ui-sidebar__footer".to_string(),
        "ui-sidebar-footer".to_string(),
    ];

    if state.disabled {
        classes.push("ui-sidebar-footer--disabled".to_string());
    }

    if state.bordered {
        classes.push("ui-sidebar-footer--bordered".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-footer--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/footer/logic.rs"]
mod tests;
