use crate::color::handle::{ColorHandleState, ColorHandleStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Color handle";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn sanitize_color(value: Option<String>) -> Option<String> {
    crate::color::swatch::sanitize_color_value(normalize_optional_text(value))
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: ColorHandleStateInput) -> ColorHandleState {
    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.dragging {
        "dragging"
    } else if input.focused {
        "focused"
    } else if input.has_color {
        "color"
    } else {
        "idle"
    };

    ColorHandleState {
        is_disabled: input.disabled,
        is_focused: input.focused,
        is_dragging: input.dragging,
        loupe_visible: !input.disabled && input.show_loupe && input.dragging,
        has_color: input.has_color,
        data_state_attr,
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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorHandleState) -> String {
    let mut classes = vec!["ui-color-handle".to_string()];

    if state.is_disabled {
        classes.push("ui-color-handle--disabled".to_string());
    }

    if state.is_focused {
        classes.push("ui-color-handle--focused".to_string());
    }

    if state.is_dragging {
        classes.push("ui-color-handle--dragging".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-handle--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
