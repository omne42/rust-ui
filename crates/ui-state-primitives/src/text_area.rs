pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Text area";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaStateInput {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub invalid: bool,
    pub has_value: bool,
    pub has_custom_label: bool,
    pub has_custom_description: bool,
    pub has_custom_error: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_rows: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextAreaState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
    pub label_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub rows_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn resolve_label(value: String) -> (String, bool) {
    resolve_label_with_fallback(value, DEFAULT_LABEL)
}

pub fn resolve_label_with_fallback(value: String, fallback_label: &str) -> (String, bool) {
    let trimmed = value.trim();

    if !trimmed.is_empty() {
        return (trimmed.to_string(), true);
    }

    let fallback_trimmed = fallback_label.trim();
    if !fallback_trimmed.is_empty() {
        return (fallback_trimmed.to_string(), false);
    }

    (DEFAULT_LABEL.to_string(), false)
}

pub fn resolve_state(input: TextAreaStateInput) -> TextAreaState {
    TextAreaState {
        state_attr: if input.disabled {
            "disabled"
        } else if input.invalid {
            "invalid"
        } else if input.read_only {
            "readonly"
        } else {
            "ready"
        },
        value_attr: if input.has_value { "filled" } else { "empty" },
        requirement_attr: if input.required {
            "required"
        } else {
            "optional"
        },
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        description_source_attr: if input.has_custom_description {
            "custom"
        } else {
            "default"
        },
        error_source_attr: if input.has_custom_error {
            "custom"
        } else {
            "default"
        },
        placeholder_source_attr: if input.has_custom_placeholder {
            "custom"
        } else {
            "default"
        },
        rows_source_attr: if input.has_custom_rows {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-text-area  ".to_string())),
            Some("docs-text-area".to_string())
        );
    }

    #[test]
    fn resolve_label_uses_default_for_blank_values() {
        assert_eq!(
            resolve_label("  ".to_string()),
            (DEFAULT_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_label("  Team notes  ".to_string()),
            ("Team notes".to_string(), true)
        );
    }

    #[test]
    fn resolve_label_with_fallback_prefers_props_then_i18n_then_default() {
        assert_eq!(
            resolve_label_with_fallback("  Summary  ".to_string(), "Localized text area"),
            ("Summary".to_string(), true)
        );
        assert_eq!(
            resolve_label_with_fallback("   ".to_string(), "  Localized text area  "),
            ("Localized text area".to_string(), false)
        );
        assert_eq!(
            resolve_label_with_fallback("   ".to_string(), "   "),
            (DEFAULT_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_rows_markers() {
        let state = resolve_state(TextAreaStateInput {
            disabled: false,
            read_only: true,
            required: true,
            invalid: false,
            has_value: true,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_placeholder: true,
            has_custom_rows: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.state_attr, "readonly");
        assert_eq!(state.value_attr, "filled");
        assert_eq!(state.requirement_attr, "required");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.placeholder_source_attr, "custom");
        assert_eq!(state.rows_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }
}
