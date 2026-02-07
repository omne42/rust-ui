#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl AlertVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            AlertVariant::Default => "ui-alert--variant-default",
            AlertVariant::Accent => "ui-alert--variant-accent",
            AlertVariant::Danger => "ui-alert--variant-danger",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Accent => "accent",
            AlertVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateInput {
    pub variant: AlertVariant,
    pub has_title: bool,
    pub has_description: bool,
    pub has_actions: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertState {
    pub variant: AlertVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub has_title: bool,
    pub title_class: &'static str,
    pub title_attr: &'static str,
    pub has_description: bool,
    pub description_class: &'static str,
    pub description_attr: &'static str,
    pub has_actions: bool,
    pub actions_class: &'static str,
    pub actions_attr: &'static str,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub role_attr: &'static str,
    pub live_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: AlertStateInput) -> AlertState {
    let title_class = if input.has_title {
        "ui-alert--with-title"
    } else {
        "ui-alert--no-title"
    };
    let title_attr = if input.has_title { "present" } else { "absent" };

    let description_class = if input.has_description {
        "ui-alert--with-description"
    } else {
        "ui-alert--no-description"
    };
    let description_attr = if input.has_description {
        "present"
    } else {
        "absent"
    };

    let actions_class = if input.has_actions {
        "ui-alert--with-actions"
    } else {
        "ui-alert--no-actions"
    };
    let actions_attr = if input.has_actions {
        "present"
    } else {
        "absent"
    };

    let (state_class, state_attr) = if input.has_title && input.has_description {
        ("ui-alert--detailed", "detailed")
    } else {
        ("ui-alert--compact", "compact")
    };

    let (role_attr, live_attr) = if input.variant == AlertVariant::Danger {
        ("alert", "assertive")
    } else {
        ("status", "polite")
    };

    AlertState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        has_title: input.has_title,
        title_class,
        title_attr,
        has_description: input.has_description,
        description_class,
        description_attr,
        has_actions: input.has_actions,
        actions_class,
        actions_attr,
        state_class,
        state_attr,
        role_attr,
        live_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AlertState) -> String {
    let mut classes = vec![
        "ui-alert".to_string(),
        state.variant_class.to_string(),
        state.state_class.to_string(),
        state.title_class.to_string(),
        state.description_class.to_string(),
        state.actions_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-alert--custom-class".to_string());
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
    fn alert_variant_class_and_attr_are_stable() {
        assert_eq!(
            AlertVariant::Default.class_name(),
            "ui-alert--variant-default"
        );
        assert_eq!(
            AlertVariant::Accent.class_name(),
            "ui-alert--variant-accent"
        );
        assert_eq!(
            AlertVariant::Danger.class_name(),
            "ui-alert--variant-danger"
        );

        assert_eq!(AlertVariant::Default.as_attr(), "default");
        assert_eq!(AlertVariant::Accent.as_attr(), "accent");
        assert_eq!(AlertVariant::Danger.as_attr(), "danger");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-alert ".to_string())),
            Some("docs-alert".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_variant_content_and_live_region() {
        let state = resolve_state(AlertStateInput {
            variant: AlertVariant::Danger,
            has_title: true,
            has_description: true,
            has_actions: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, AlertVariant::Danger);
        assert_eq!(state.variant_class, "ui-alert--variant-danger");
        assert_eq!(state.variant_attr, "danger");
        assert!(state.has_title);
        assert_eq!(state.title_attr, "present");
        assert!(state.has_description);
        assert_eq!(state.description_attr, "present");
        assert!(!state.has_actions);
        assert_eq!(state.actions_attr, "absent");
        assert_eq!(state.state_class, "ui-alert--detailed");
        assert_eq!(state.state_attr, "detailed");
        assert_eq!(state.role_attr, "alert");
        assert_eq!(state.live_attr, "assertive");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-alert-custom".to_string()),
            resolve_state(AlertStateInput {
                variant: AlertVariant::Accent,
                has_title: false,
                has_description: true,
                has_actions: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-alert",
            "ui-alert--variant-accent",
            "ui-alert--compact",
            "ui-alert--no-title",
            "ui-alert--with-description",
            "ui-alert--with-actions",
            "ui-alert--custom-class",
            "docs-alert-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
