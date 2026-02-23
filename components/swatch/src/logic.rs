pub use ui_state_primitives::swatch::{
    SwatchAgentSource, SwatchAriaLabelFallbacks, SwatchBorder, SwatchRounding,
    SwatchSelectionControlInput, SwatchShape, SwatchSize, SwatchState, SwatchStateInput,
    normalize_optional_text, resolve_agent_contract, resolve_agent_source,
    resolve_aria_label_with_fallbacks, resolve_selection_control_state, resolve_state,
    sanitize_color_value,
};

pub fn compose_class_name(base_class_name: Option<String>, state: SwatchState) -> String {
    let mut classes = vec![
        "ui-swatch".to_string(),
        state.size_class.into(),
        state.border_class.into(),
        state.rounding_class.into(),
        state.shape_class.into(),
    ];

    if state.show_mixed_value {
        classes.push("ui-swatch--mixed".to_string());
    }

    if state.show_nothing {
        classes.push("ui-swatch--nothing".to_string());
    }

    if state.disabled {
        classes.push("ui-swatch--disabled".to_string());
    }

    if !state.is_interactive {
        classes.push("ui-swatch--static".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-swatch--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(color: Option<&str>) -> Option<String> {
    color.map(|color| format!("--ui-swatch-color: {color};"))
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
