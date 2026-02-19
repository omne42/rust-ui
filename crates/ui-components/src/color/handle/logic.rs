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
mod tests {
    use super::*;

    #[test]
    fn sanitize_color_rejects_unsafe_values() {
        assert_eq!(
            sanitize_color(Some(" #09f ".to_string())),
            Some("#09f".to_string())
        );
        assert_eq!(
            sanitize_color(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn normalize_aria_label_uses_default_or_custom_values() {
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Accent handle  ".to_string())),
            ("Accent handle".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_and_class_name_track_sources_and_flags() {
        let state = resolve_state(ColorHandleStateInput {
            disabled: false,
            focused: true,
            dragging: true,
            show_loupe: true,
            has_color: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state_attr, "dragging");
        assert!(state.loupe_visible);
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");

        let class_name = compose_class_name(Some("docs-color-handle".to_string()), state);
        assert!(class_name.contains("ui-color-handle"));
        assert!(class_name.contains("ui-color-handle--focused"));
        assert!(class_name.contains("ui-color-handle--dragging"));
        assert!(class_name.contains("ui-color-handle--custom-class"));
        assert!(class_name.contains("docs-color-handle"));
    }
}
