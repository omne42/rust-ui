use crate::sidebar::SidebarSide;
use crate::sidebar_inset::{DEFAULT_ARIA_LABEL, SidebarInsetState, SidebarInsetStateInput};

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

pub fn resolve_state(input: SidebarInsetStateInput) -> SidebarInsetState {
    SidebarInsetState {
        side: input.side,
        side_attr: match input.side {
            SidebarSide::Left => "left",
            SidebarSide::Right => "right",
        },
        padded: input.padded,
        compact: !input.padded,
        recessed: input.recessed,
        plain: !input.recessed,
        disabled: input.disabled,
        enabled: !input.disabled,
        state_attr: if input.disabled {
            "disabled"
        } else if input.recessed {
            "recessed"
        } else {
            "plain"
        },
        padding_attr: if input.padded { "padded" } else { "compact" },
        surface_attr: if input.recessed { "recessed" } else { "plain" },
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

pub fn compose_class_name(class_name: Option<String>, state: SidebarInsetState) -> String {
    let mut classes = vec!["ui-sidebar-inset".to_string()];

    match state.side {
        SidebarSide::Left => classes.push("ui-sidebar-inset--left".to_string()),
        SidebarSide::Right => classes.push("ui-sidebar-inset--right".to_string()),
    }

    if state.padded {
        classes.push("ui-sidebar-inset--padded".to_string());
    }

    if state.recessed {
        classes.push("ui-sidebar-inset--recessed".to_string());
    }

    if state.disabled {
        classes.push("ui-sidebar-inset--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-sidebar-inset--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/inset/logic.rs"]
mod tests;
