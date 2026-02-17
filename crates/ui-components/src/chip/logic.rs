pub use ui_state_primitives::chip::{
    ChipSize, ChipState, ChipStateInput, ChipVariant, normalize_optional_text,
    resolve_dismiss_aria_label, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: ChipState) -> String {
    let mut classes = vec![
        "ui-chip".to_string(),
        state.variant_class.to_string(),
        state.size_class.to_string(),
        state.state_class.to_string(),
        state.dismiss_label_source_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ChipStateInput {
                variant: ChipVariant::Accent,
                size: ChipSize::Sm,
                disabled: false,
                has_dismiss_action: false,
                has_custom_dismiss_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-chip",
            "ui-chip--variant-accent",
            "ui-chip--size-sm",
            "ui-chip--static",
            "ui-chip--dismiss-label-default",
            "ui-chip--enabled",
            "ui-chip--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
