use crate::combobox::{ComboboxState, ComboboxStateInput, DEFAULT_LABEL};

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

pub fn resolve_state(input: ComboboxStateInput) -> ComboboxState {
    let has_disabled_options = input.disabled_option_count > 0;

    ComboboxState {
        state_attr: if input.disabled {
            "disabled"
        } else if input.invalid {
            "invalid"
        } else {
            "ready"
        },
        selection_attr: match input.selected_index {
            Some(index) if index < input.item_count => "selected",
            Some(_) => "out-of-range",
            None => "empty",
        },
        options_attr: if input.item_count == 0 {
            "empty"
        } else if has_disabled_options {
            "has-disabled"
        } else {
            "enabled"
        },
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
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_disabled_options,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: ComboboxState) -> String {
    let mut classes = vec![
        "ui-combobox".to_string(),
        format!("ui-combobox--state-{}", state.state_attr),
        format!("ui-combobox--selection-{}", state.selection_attr),
        format!("ui-combobox--options-{}", state.options_attr),
        format!("ui-combobox--requirement-{}", state.requirement_attr),
    ];

    if state.has_custom_motion {
        classes.push("ui-combobox--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-combobox--custom-class".to_string());
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
            resolve_label("  Runtime language  ".to_string()),
            ("Runtime language".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_selection_sources_and_motion() {
        let state = resolve_state(ComboboxStateInput {
            item_count: 4,
            disabled_option_count: 1,
            selected_index: Some(2),
            required: true,
            invalid: false,
            disabled: false,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_placeholder: true,
            has_custom_class_name: false,
            has_custom_motion: true,
        });

        assert_eq!(state.state_attr, "ready");
        assert_eq!(state.selection_attr, "selected");
        assert_eq!(state.options_attr, "has-disabled");
        assert_eq!(state.requirement_attr, "required");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.placeholder_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert!(state.has_disabled_options);
    }

    #[test]
    fn resolve_state_marks_out_of_range_selection() {
        let state = resolve_state(ComboboxStateInput {
            item_count: 3,
            disabled_option_count: 0,
            selected_index: Some(9),
            required: false,
            invalid: true,
            disabled: false,
            has_custom_label: false,
            has_custom_description: false,
            has_custom_error: true,
            has_custom_placeholder: false,
            has_custom_class_name: true,
            has_custom_motion: false,
        });

        assert_eq!(state.state_attr, "invalid");
        assert_eq!(state.selection_attr, "out-of-range");
        assert_eq!(state.options_attr, "enabled");
        assert_eq!(state.error_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(ComboboxStateInput {
            item_count: 0,
            disabled_option_count: 0,
            selected_index: None,
            required: false,
            invalid: false,
            disabled: true,
            has_custom_label: false,
            has_custom_description: false,
            has_custom_error: false,
            has_custom_placeholder: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        let class_name = compose_class_name(Some("docs-combobox".to_string()), state);

        for token in [
            "ui-combobox",
            "ui-combobox--state-disabled",
            "ui-combobox--selection-empty",
            "ui-combobox--options-empty",
            "ui-combobox--requirement-optional",
            "ui-combobox--custom-motion",
            "ui-combobox--custom-class",
            "docs-combobox",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
