pub use ui_state_primitives::aspect_ratio::{
    AspectRatioPreset, AspectRatioRadius, AspectRatioState, AspectRatioStateInput,
    DEFAULT_ARIA_LABEL, normalize_aria_label, normalize_optional_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: AspectRatioState) -> String {
    let mut classes = vec![
        "ui-aspect-ratio".to_string(),
        state.ratio_class.into(),
        state.radius_class.into(),
    ];

    if state.is_bordered {
        classes.push(state.bordered_class.into());
    }

    if state.is_fill {
        classes.push(state.fill_class.into());
    }

    if state.has_custom_class_name {
        classes.push("ui-aspect-ratio--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
