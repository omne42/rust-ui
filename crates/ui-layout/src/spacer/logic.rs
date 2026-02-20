pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: SpacerState) -> String {
    let mut classes = vec![
        "ui-spacer".to_string(),
        state.axis_class.into(),
        state.size_class.into(),
    ];

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
