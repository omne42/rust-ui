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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(DividerStateInput {
                orientation: DividerOrientation::Horizontal,
                has_custom_class_name: true,
            }),
        );

        for token in ["ui-divider", "ui-divider--horizontal", "custom"] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
