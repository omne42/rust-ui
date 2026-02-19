pub use ui_state_primitives::status_light::{
    StatusLightRole, StatusLightState, StatusLightStateInput, StatusLightVariant,
    normalize_optional_text, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightRootInput {
    pub variant: Option<StatusLightVariant>,
    pub role: Option<StatusLightRole>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightRootState {
    pub state: StatusLightState,
    pub class_name: String,
}

pub fn compose_class_name(base_class_name: Option<String>, state: StatusLightState) -> String {
    let mut classes = vec![
        "ui-status-light".to_string(),
        state.variant_class.into(),
        state.state_class.into(),
        state.role_source_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-status-light--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn normalize_root_state(input: StatusLightRootInput) -> StatusLightRootState {
    let variant = input.variant.unwrap_or_default();
    let class_name = normalize_optional_text(input.class_name);
    let state = resolve_state(StatusLightStateInput {
        variant,
        role: input.role,
        has_custom_class_name: class_name.is_some(),
    });

    StatusLightRootState {
        state,
        class_name: compose_class_name(class_name, state),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_and_attrs_are_stable() {
        assert_eq!(
            StatusLightVariant::Default.class_name(),
            "ui-status-light--variant-default"
        );
        assert_eq!(
            StatusLightVariant::Accent.class_name(),
            "ui-status-light--variant-accent"
        );
        assert_eq!(
            StatusLightVariant::Danger.class_name(),
            "ui-status-light--variant-danger"
        );

        assert_eq!(StatusLightVariant::Default.as_attr(), "default");
        assert_eq!(StatusLightVariant::Accent.as_attr(), "accent");
        assert_eq!(StatusLightVariant::Danger.as_attr(), "danger");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-status  ".to_string())),
            Some("docs-status".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_variant_role_and_sources() {
        let live = resolve_state(StatusLightStateInput {
            variant: StatusLightVariant::Danger,
            role: Some(StatusLightRole::Status),
            has_custom_class_name: true,
        });

        assert_eq!(live.variant, StatusLightVariant::Danger);
        assert_eq!(live.variant_class, "ui-status-light--variant-danger");
        assert_eq!(live.variant_attr, "danger");
        assert_eq!(live.role, Some(StatusLightRole::Status));
        assert_eq!(live.role_attr, Some("status"));
        assert_eq!(live.state_class, "ui-status-light--live");
        assert_eq!(live.state_attr, "live");
        assert_eq!(live.role_source_class, "ui-status-light--role-custom");
        assert_eq!(live.role_source_attr, "custom");
        assert!(live.is_live);
        assert!(live.has_custom_class_name);
        assert_eq!(live.class_source_attr, "custom");

        let static_state = resolve_state(StatusLightStateInput {
            variant: StatusLightVariant::Default,
            role: None,
            has_custom_class_name: false,
        });

        assert!(!static_state.is_live);
        assert_eq!(static_state.state_class, "ui-status-light--static");
        assert_eq!(static_state.state_attr, "static");
        assert_eq!(static_state.role_source_class, "ui-status-light--role-none");
        assert_eq!(static_state.role_source_attr, "none");
        assert_eq!(static_state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(StatusLightStateInput {
                variant: StatusLightVariant::Accent,
                role: Some(StatusLightRole::Status),
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-status-light",
            "ui-status-light--variant-accent",
            "ui-status-light--live",
            "ui-status-light--role-custom",
            "ui-status-light--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn normalize_root_state_keeps_single_default_source_and_centralized_derivation() {
        let root = normalize_root_state(StatusLightRootInput {
            variant: None,
            role: None,
            class_name: Some("  docs-status  ".to_string()),
        });

        assert_eq!(root.state.variant, StatusLightVariant::Default);
        assert_eq!(root.state.variant_attr, "default");
        assert_eq!(root.state.state_attr, "static");
        assert!(root.class_name.contains("ui-status-light"));
        assert!(root.class_name.contains("ui-status-light--variant-default"));
        assert!(root.class_name.contains("docs-status"));
    }
}
