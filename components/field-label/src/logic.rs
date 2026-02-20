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
#[path = "../test/logic.rs"]
mod tests;
