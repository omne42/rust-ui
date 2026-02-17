pub use ui_state_primitives::badge::{
    BadgeState, BadgeStateInput, BadgeVariant, normalize_optional_text, resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: BadgeState) -> String {
    let mut classes = vec![
        "ui-badge".to_string(),
        state.variant_class.to_string(),
        state.fill_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-badge--custom-class".to_string());
    }

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
        let solid = resolve_state(BadgeStateInput {
            variant: BadgeVariant::Accent,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-badge".to_string()), solid);

        for token in [
            "ui-badge",
            "ui-badge--variant-accent",
            "ui-badge--fill-solid",
            "ui-badge--custom-class",
            "docs-badge",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
