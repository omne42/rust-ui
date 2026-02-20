use crate::sidebar_menu_action::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL, SidebarMenuActionState, SidebarMenuActionStateInput,
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

pub fn normalize_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_LABEL.into())
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
#[path = "../../test/menu_action/logic.rs"]
mod tests;
