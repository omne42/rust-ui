pub use ui_state_primitives::chip::{
    ChipSize, ChipState, ChipStateInput, ChipVariant, normalize_optional_text,
    resolve_dismiss_aria_label, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ChipState) -> String {
    let mut classes = vec![
        "ui-chip".to_string(),
        state.variant_class.into(),
        state.size_class.into(),
        state.state_class.into(),
        state.dismiss_label_source_class.into(),
    ];

    if state.is_enabled {
        classes.push("ui-chip--enabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-chip--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
