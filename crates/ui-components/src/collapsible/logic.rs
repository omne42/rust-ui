pub use ui_state_primitives::collapsible::{
    CollapsibleState, DEFAULT_ID_BASE, DEFAULT_TITLE, normalize_id_base, normalize_optional_text,
    resolve_aria_label, resolve_state, resolve_title,
};

const _: &str = DEFAULT_ID_BASE;
const _: &str = DEFAULT_TITLE;

pub fn compose_class_name(class_name: Option<String>, state: CollapsibleState) -> String {
    let mut classes = vec![
        "ui-collapsible".to_string(),
        format!("ui-collapsible--state-{}", state.state_attr),
        format!("ui-collapsible--mode-{}", state.open_mode_attr),
    ];

    if state.has_custom_motion {
        classes.push("ui-collapsible--custom-motion".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-collapsible--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::collapsible::CollapsibleStateInput;

    #[test]
    fn compose_class_name_includes_state_mode_and_custom_markers() {
        let state = resolve_state(CollapsibleStateInput {
            is_open: false,
            is_disabled: true,
            is_controlled: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        let class_name = compose_class_name(Some("docs-collapsible".to_string()), state);

        for token in [
            "ui-collapsible",
            "ui-collapsible--state-disabled",
            "ui-collapsible--mode-uncontrolled",
            "ui-collapsible--custom-motion",
            "ui-collapsible--custom-class",
            "docs-collapsible",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
