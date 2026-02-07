#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StatusLightVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl StatusLightVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            StatusLightVariant::Default => "ui-status-light--variant-default",
            StatusLightVariant::Accent => "ui-status-light--variant-accent",
            StatusLightVariant::Danger => "ui-status-light--variant-danger",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            StatusLightVariant::Default => "default",
            StatusLightVariant::Accent => "accent",
            StatusLightVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusLightRole {
    Status,
}

impl StatusLightRole {
    pub fn as_attr(self) -> &'static str {
        match self {
            StatusLightRole::Status => "status",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusLightStateInput {
    pub variant: StatusLightVariant,
    pub role: Option<StatusLightRole>,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusLightState {
    pub variant: StatusLightVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub role: Option<StatusLightRole>,
    pub role_attr: Option<&'static str>,
    pub is_live: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: StatusLightStateInput) -> StatusLightState {
    let role_attr = input.role.map(StatusLightRole::as_attr);

    StatusLightState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_str(),
        role: input.role,
        role_attr,
        is_live: input.role.is_some(),
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: StatusLightState) -> String {
    let mut classes = vec![
        "ui-status-light".to_string(),
        state.variant_class.to_string(),
    ];

    if state.is_live {
        classes.push("ui-status-light--live".to_string());
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

        assert_eq!(StatusLightVariant::Default.as_str(), "default");
        assert_eq!(StatusLightVariant::Accent.as_str(), "accent");
        assert_eq!(StatusLightVariant::Danger.as_str(), "danger");
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
    fn resolve_state_tracks_variant_role_and_class_source() {
        let state = resolve_state(StatusLightStateInput {
            variant: StatusLightVariant::Danger,
            role: Some(StatusLightRole::Status),
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, StatusLightVariant::Danger);
        assert_eq!(state.variant_class, "ui-status-light--variant-danger");
        assert_eq!(state.variant_attr, "danger");
        assert_eq!(state.role, Some(StatusLightRole::Status));
        assert_eq!(state.role_attr, Some("status"));
        assert!(state.is_live);
        assert!(state.has_custom_class_name);
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
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
