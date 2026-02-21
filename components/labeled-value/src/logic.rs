use std::borrow::Cow;

pub use ui_state_primitives::labeled_value::{
    DEFAULT_ARIA_LABEL, DEFAULT_LABEL_TEXT, DEFAULT_VALUE_TEXT, LabeledValueOrientation,
    LabeledValueState, LabeledValueStateInput, LabeledValueTone, normalize_aria_label,
    normalize_label_text, normalize_optional_text, normalize_value_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: LabeledValueState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-labeled-value"),
        Cow::Borrowed(state.orientation_class),
        Cow::Borrowed(state.tone_class),
    ];

    if state.has_description {
        classes.push(Cow::Borrowed("ui-labeled-value--with-description"));
    }
    if state.has_custom_label {
        classes.push(Cow::Borrowed("ui-labeled-value--label-custom"));
    }
    if state.has_custom_value {
        classes.push(Cow::Borrowed("ui-labeled-value--value-custom"));
    }
    if state.has_custom_aria_label {
        classes.push(Cow::Borrowed("ui-labeled-value--aria-custom"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-labeled-value--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let total_len =
        classes.iter().map(|class| class.len()).sum::<usize>() + classes.len().saturating_sub(1);
    let mut class_name = String::with_capacity(total_len);

    for (index, class) in classes.iter().enumerate() {
        if index > 0 {
            class_name.push(' ');
        }
        class_name.push_str(class);
    }

    class_name
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
