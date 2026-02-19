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
mod tests {
    use super::*;
    use ui_state_primitives::spacer::{SpacerAxis, SpacerSize};

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-spacer  ".to_string())),
            Some("docs-spacer".to_string())
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SpacerStateInput {
                axis: SpacerAxis::Vertical,
                size: SpacerSize::Md,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-spacer",
            "ui-spacer--axis-vertical",
            "ui-spacer--size-md",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
