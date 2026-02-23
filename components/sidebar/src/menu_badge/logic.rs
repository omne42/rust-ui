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

pub fn resolve_muted(is_muted: Option<bool>, muted: bool) -> bool {
    is_muted.unwrap_or(muted)
}

pub fn resolve_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
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
#[path = "../../test/menu_badge/logic.rs"]
mod tests;
