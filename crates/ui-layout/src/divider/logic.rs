pub use ui_state_primitives::divider::{
    DividerOrientation, DividerState, DividerStateInput, normalize_optional_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: DividerState) -> String {
    let mut classes = vec!["ui-divider".to_string(), state.orientation_class.into()];

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
