use crate::sidebar_content::{DEFAULT_ARIA_LABEL, SidebarContentState, SidebarContentStateInput};

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
#[path = "../../test/content/logic.rs"]
mod tests;
