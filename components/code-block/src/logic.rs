pub use ui_state_primitives::code_block::{
    CodeBlockStateInput, CodeBlockViewState, normalize_optional_text, resolve_state,
    resolve_view_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: CodeBlockViewState) -> String {
    let mut classes = vec![
        "ui-code-block".to_string(),
        state.state_class.into(),
        state.header_class.into(),
        state.motion_source_class.into(),
    ];

    if state.copyable {
        classes.push("ui-code-block--copyable".to_string());
    }
    if state.has_label {
        classes.push("ui-code-block--with-label".to_string());
    }
    if state.has_language {
        classes.push("ui-code-block--with-language".to_string());
    }
    if state.is_empty {
        classes.push("ui-code-block--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-code-block--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
