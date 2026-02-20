pub use ui_state_primitives::input_group::{
    InputGroupState, InputGroupStateInput, normalize_aria_label, normalize_optional_text,
    resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: InputGroupState) -> String {
    let mut classes = vec![
        "ui-input-group".to_string(),
        state.phase_class.into(),
        state.attachment_class.into(),
        state.label_source_class.into(),
    ];

    if state.is_invalid {
        classes.push("ui-input-group--invalid".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-input-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../../test/input/group/logic.rs"]
mod tests;
