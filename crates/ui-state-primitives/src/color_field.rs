pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Color";
pub const DEFAULT_PLACEHOLDER: &str = "#RRGGBB";
pub const DEFAULT_ARIA_LABEL: &str = "Color value";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldStateInput {
    pub disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub has_custom_label: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorFieldState {
    pub is_disabled: bool,
    pub has_value: bool,
    pub has_valid_value: bool,
    pub has_preview: bool,
    pub data_state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn normalize_placeholder(value: Option<String>) -> (String, bool) {
    if let Some(placeholder) = normalize_optional_text(value) {
        return (placeholder, true);
    }

    (DEFAULT_PLACEHOLDER.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    if !label.trim().is_empty() {
        return (format!("{label} value"), false);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn normalize_color_value(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn sanitize_preview_color(value: Option<String>) -> Option<String> {
    crate::swatch::sanitize_color_value(normalize_color_value(value))
}

pub fn resolve_state(input: ColorFieldStateInput) -> ColorFieldState {
    let data_state_attr = if input.disabled {
        "disabled"
    } else if !input.has_value {
        "empty"
    } else if input.has_valid_value {
        "valid"
    } else {
        "invalid"
    };

    ColorFieldState {
        is_disabled: input.disabled,
        has_value: input.has_value,
        has_valid_value: input.has_valid_value,
        has_preview: input.has_preview,
        data_state_attr,
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            "custom"
        } else {
            "default"
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

pub fn compose_class_name(base_class_name: Option<String>, state: ColorFieldState) -> String {
    let mut classes = vec!["ui-color-field".to_string()];

    if state.is_disabled {
        classes.push("ui-color-field--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-field--custom-class".to_string());
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
    fn normalize_contracts_use_defaults_and_trim_custom_values() {
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.to_string(), false));
        assert_eq!(
            normalize_label(Some("  Fill color  ".to_string())),
            ("Fill color".to_string(), true)
        );

        assert_eq!(
            normalize_placeholder(None),
            (DEFAULT_PLACEHOLDER.to_string(), false)
        );
        assert_eq!(
            normalize_placeholder(Some("  #ABCDEF  ".to_string())),
            ("#ABCDEF".to_string(), true)
        );

        assert_eq!(
            normalize_aria_label(None, "Fill color"),
            ("Fill color value".to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(Some("  Theme color  ".to_string()), "Fill color"),
            ("Theme color".to_string(), true)
        );
    }

    #[test]
    fn preview_color_sanitization_rejects_unsafe_values() {
        assert_eq!(
            sanitize_preview_color(Some("#09f".to_string())),
            Some("#09f".to_string())
        );
        assert_eq!(
            sanitize_preview_color(Some("rgba(12, 34, 56, 0.5)".to_string())),
            Some("rgba(12, 34, 56, 0.5)".to_string())
        );
        assert_eq!(
            sanitize_preview_color(Some("javascript:alert(1)".to_string())),
            None
        );
    }

    #[test]
    fn resolve_state_and_class_name_track_state_and_sources() {
        let valid = resolve_state(ColorFieldStateInput {
            disabled: false,
            has_value: true,
            has_valid_value: true,
            has_preview: true,
            has_custom_label: true,
            has_custom_placeholder: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(valid.data_state_attr, "valid");
        assert_eq!(valid.label_source_attr, "custom");
        assert_eq!(valid.placeholder_source_attr, "custom");
        assert_eq!(valid.aria_source_attr, "default");
        assert_eq!(valid.class_source_attr, "custom");

        let class_name = compose_class_name(Some("docs-color-field".to_string()), valid);
        assert!(class_name.contains("ui-color-field"));
        assert!(class_name.contains("ui-color-field--custom-class"));
        assert!(class_name.contains("docs-color-field"));

        let invalid = resolve_state(ColorFieldStateInput {
            disabled: false,
            has_value: true,
            has_valid_value: false,
            has_preview: false,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });
        assert_eq!(invalid.data_state_attr, "invalid");

        let empty = resolve_state(ColorFieldStateInput {
            disabled: false,
            has_value: false,
            has_valid_value: false,
            has_preview: false,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });
        assert_eq!(empty.data_state_attr, "empty");

        let disabled = resolve_state(ColorFieldStateInput {
            disabled: true,
            has_value: true,
            has_valid_value: true,
            has_preview: true,
            has_custom_label: false,
            has_custom_placeholder: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });
        assert_eq!(disabled.data_state_attr, "disabled");
    }
}
