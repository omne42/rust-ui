pub use ui_state_primitives::labeled_value::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL_TEXT, DEFAULT_VALUE_TEXT, LabeledValueOrientation,
    LabeledValueState, LabeledValueStateInput, LabeledValueTone, normalize_aria_label,
    normalize_label_text, normalize_optional_text, normalize_value_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: LabeledValueState) -> String {
    let mut classes = vec![
        "ui-labeled-value".to_string(),
        state.orientation_class.into(),
        state.tone_class.into(),
    ];

    if state.has_description {
        classes.push("ui-labeled-value--with-description".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-labeled-value--label-custom".to_string());
    }
    if state.has_custom_value {
        classes.push("ui-labeled-value--value-custom".to_string());
    }
    if state.has_custom_aria_label {
        classes.push("ui-labeled-value--aria-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-labeled-value--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
