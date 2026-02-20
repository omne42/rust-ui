pub use ui_state_primitives::auto_height::{AutoHeightState, AutoHeightStateInput, resolve_state};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: AutoHeightState) -> String {
    let mut classes = vec!["ui-auto-height".to_string()];

    if state.animate_height {
        classes.push("ui-auto-height--animated".to_string());
    }
    if state.is_static {
        classes.push("ui-auto-height--static".to_string());
    }
    if state.has_custom_motion {
        classes.push("ui-auto-height--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-auto-height--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
