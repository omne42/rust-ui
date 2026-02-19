pub use ui_state_primitives::field_label::{
    DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, FieldLabelState,
    FieldLabelStateInput, FieldLabelTone, normalize_aria_label, normalize_optional_text,
    normalize_required_indicator, normalize_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: FieldLabelState) -> String {
    let mut classes = vec!["ui-field-label".to_string(), state.tone_class.into()];

    if state.is_required {
        classes.push("ui-field-label--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-field-label--disabled".to_string());
    }

    if state.has_for_id {
        classes.push("ui-field-label--for".to_string());
    }

    if state.has_custom_text {
        classes.push("ui-field-label--text-custom".to_string());
    }

    if state.has_custom_indicator {
        classes.push("ui-field-label--indicator-custom".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-field-label--aria-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field-label--custom-class".to_string());
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
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(FieldLabelStateInput {
            tone: FieldLabelTone::Muted,
            required: true,
            disabled: true,
            has_for_id: true,
            has_custom_text: true,
            has_custom_indicator: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-field-label-custom".to_string()), state);

        for token in [
            "ui-field-label",
            "ui-field-label--tone-muted",
            "ui-field-label--required",
            "ui-field-label--disabled",
            "ui-field-label--for",
            "ui-field-label--text-custom",
            "ui-field-label--indicator-custom",
            "ui-field-label--aria-custom",
            "ui-field-label--custom-class",
            "docs-field-label-custom",
        ] {
            assert!(class_name.contains(token), "class should contain `{token}`");
        }
    }
}
