pub use ui_state_primitives::color_loupe::{
    ColorLoupeState, ColorLoupeStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR,
    DEFAULT_POSITION_PERCENT, normalize_aria_label, normalize_optional_text, resolve_state,
    sanitize_color,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ColorLoupeState) -> String {
    let mut classes = vec![
        "ui-color-loupe".to_string(),
        state.x_bucket_class.into(),
        state.y_bucket_class.into(),
    ];

    if state.is_open {
        classes.push("ui-color-loupe--open".to_string());
    }

    if state.is_disabled {
        classes.push("ui-color-loupe--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-color-loupe--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
