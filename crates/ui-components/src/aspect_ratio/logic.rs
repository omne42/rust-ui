pub use ui_state_primitives::aspect_ratio::{
    AspectRatioPreset, AspectRatioRadius, AspectRatioState, AspectRatioStateInput,
    DEFAULT_ARIA_LABEL, normalize_aria_label, normalize_optional_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: AspectRatioState) -> String {
    let mut classes = vec![
        "ui-aspect-ratio".to_string(),
        state.ratio_class.to_string(),
        state.radius_class.to_string(),
    ];

    if state.is_bordered {
        classes.push(state.bordered_class.to_string());
    }

    if state.is_fill {
        classes.push(state.fill_class.to_string());
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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_merges_custom_class_and_flags() {
        let state = resolve_state(AspectRatioStateInput {
            ratio: AspectRatioPreset::Video,
            radius: AspectRatioRadius::Sm,
            bordered: true,
            fill: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class = compose_class_name(Some("docs-aspect".to_string()), state);

        for class_name in [
            "ui-aspect-ratio",
            "ui-aspect-ratio--ratio-video",
            "ui-aspect-ratio--radius-sm",
            "ui-aspect-ratio--bordered",
            "ui-aspect-ratio--custom-class",
            "docs-aspect",
        ] {
            assert!(
                class.contains(class_name),
                "class list should include `{class_name}`; got: {class}"
            );
        }
    }
}
