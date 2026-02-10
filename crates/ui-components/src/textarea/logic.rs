use crate::textarea::{DEFAULT_LABEL, TextareaState, TextareaStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_label(value: String) -> (String, bool) {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return (DEFAULT_LABEL.to_string(), false);
    }

    (trimmed.to_string(), true)
}

pub fn resolve_state(input: TextareaStateInput) -> TextareaState {
    TextareaState {
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

pub fn compose_class_name(class_name: Option<String>, state: TextareaState) -> String {
    let mut classes = vec![
        "ui-textarea".to_string(),
        format!("ui-textarea--state-{}", state.state_attr),
        format!("ui-textarea--value-{}", state.value_attr),
        format!("ui-textarea--requirement-{}", state.requirement_attr),
    ];

    if state.has_custom_class_name {
        classes.push("ui-textarea--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_label_uses_default_for_blank_values() {
        assert_eq!(
            resolve_label("  ".to_string()),
            (DEFAULT_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_label("  Release summary  ".to_string()),
            ("Release summary".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_rows_markers() {
        let state = resolve_state(TextareaStateInput {
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

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(TextareaStateInput {
            disabled: true,
            read_only: false,
            required: false,
            invalid: false,
            has_value: false,
            has_custom_label: false,
            has_custom_description: false,
            has_custom_error: false,
            has_custom_placeholder: false,
            has_custom_rows: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-textarea".to_string()), state);

        for token in [
            "ui-textarea",
            "ui-textarea--state-disabled",
            "ui-textarea--value-empty",
            "ui-textarea--requirement-optional",
            "ui-textarea--custom-class",
            "docs-textarea",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
