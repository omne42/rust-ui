pub use ui_state_primitives::input_group::{
    InputGroupState, InputGroupStateInput, normalize_aria_label, normalize_optional_text,
    resolve_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: InputGroupState) -> String {
    let mut classes = vec![
        "ui-input-group".to_string(),
        state.phase_class.into(),
        state.attachment_class.into(),
        state.label_source_class.into(),
    ];

    if state.is_invalid {
        classes.push("ui-input-group--invalid".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-input-group--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-input-group ".to_string())),
            Some("docs-input-group".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_trimmed_label_or_fallback() {
        let (label, explicit) = normalize_aria_label(Some("  Query controls  ".to_string()));
        assert_eq!(label, "Query controls");
        assert!(explicit);

        let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
        assert_eq!(label, ui_state_primitives::input_group::DEFAULT_ARIA_LABEL);
        assert!(!explicit);

        let (label, explicit) = normalize_aria_label(None);
        assert_eq!(label, ui_state_primitives::input_group::DEFAULT_ARIA_LABEL);
        assert!(!explicit);
    }

    #[test]
    fn resolve_state_tracks_phase_attachment_and_source_markers() {
        let state = resolve_state(InputGroupStateInput {
            disabled: true,
            invalid: true,
            attached: false,
            has_start_content: true,
            has_end_content: false,
            has_custom_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.phase_class, "ui-input-group--state-disabled");
        assert_eq!(state.phase_attr, "disabled");
        assert!(state.is_disabled);
        assert!(!state.is_enabled);

        assert_eq!(state.attachment_class, "ui-input-group--detached");
        assert_eq!(state.attachment_attr, "detached");
        assert!(state.is_detached);
        assert!(!state.is_attached);

        assert!(state.is_invalid);
        assert!(state.has_start_content);
        assert!(!state.has_end_content);
        assert_eq!(state.label_source_class, "ui-input-group--label-custom");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(InputGroupStateInput {
            disabled: false,
            invalid: true,
            attached: true,
            has_start_content: true,
            has_end_content: true,
            has_custom_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-input-group".to_string()), state);

        for token in [
            "ui-input-group",
            "ui-input-group--state-enabled",
            "ui-input-group--attached",
            "ui-input-group--label-default",
            "ui-input-group--invalid",
            "ui-input-group--custom-class",
            "docs-input-group",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
