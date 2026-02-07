#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipVariant {
    #[default]
    Default,
    Accent,
    Danger,
    Outline,
}

impl ChipVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ChipVariant::Default => "ui-chip--variant-default",
            ChipVariant::Accent => "ui-chip--variant-accent",
            ChipVariant::Danger => "ui-chip--variant-danger",
            ChipVariant::Outline => "ui-chip--variant-outline",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ChipVariant::Default => "default",
            ChipVariant::Accent => "accent",
            ChipVariant::Danger => "danger",
            ChipVariant::Outline => "outline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ChipSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ChipSize::Md => "ui-chip--size-md",
            ChipSize::Sm => "ui-chip--size-sm",
            ChipSize::Lg => "ui-chip--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ChipSize::Sm => "sm",
            ChipSize::Md => "md",
            ChipSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipStateInput {
    pub variant: ChipVariant,
    pub size: ChipSize,
    pub disabled: bool,
    pub has_dismiss_action: bool,
    pub has_custom_dismiss_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChipState {
    pub variant: ChipVariant,
    pub size: ChipSize,
    pub variant_class: &'static str,
    pub size_class: &'static str,
    pub variant_attr: &'static str,
    pub size_attr: &'static str,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_dismiss_action: bool,
    pub is_static: bool,
    pub has_custom_dismiss_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_dismiss_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    ("Remove tag".to_string(), false)
}

pub fn resolve_state(input: ChipStateInput) -> ChipState {
    ChipState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_str(),
        size_attr: input.size.as_str(),
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_dismiss_action: input.has_dismiss_action,
        is_static: !input.has_dismiss_action,
        has_custom_dismiss_aria_label: input.has_custom_dismiss_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ChipState) -> String {
    let mut classes = vec![
        "ui-chip".to_string(),
        state.variant_class.to_string(),
        state.size_class.to_string(),
    ];

    if state.is_enabled {
        classes.push("ui-chip--enabled".to_string());
    }
    if state.is_disabled {
        classes.push("ui-chip--disabled".to_string());
    }
    if state.has_dismiss_action {
        classes.push("ui-chip--removable".to_string());
    }
    if state.is_static {
        classes.push("ui-chip--static".to_string());
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
    fn variant_and_size_mappings_are_stable() {
        assert_eq!(
            ChipVariant::Default.class_name(),
            "ui-chip--variant-default"
        );
        assert_eq!(ChipVariant::Accent.class_name(), "ui-chip--variant-accent");
        assert_eq!(ChipVariant::Danger.class_name(), "ui-chip--variant-danger");
        assert_eq!(
            ChipVariant::Outline.class_name(),
            "ui-chip--variant-outline"
        );

        assert_eq!(ChipVariant::Default.as_str(), "default");
        assert_eq!(ChipVariant::Accent.as_str(), "accent");
        assert_eq!(ChipVariant::Danger.as_str(), "danger");
        assert_eq!(ChipVariant::Outline.as_str(), "outline");

        assert_eq!(ChipSize::Sm.class_name(), "ui-chip--size-sm");
        assert_eq!(ChipSize::Md.class_name(), "ui-chip--size-md");
        assert_eq!(ChipSize::Lg.class_name(), "ui-chip--size-lg");

        assert_eq!(ChipSize::Sm.as_str(), "sm");
        assert_eq!(ChipSize::Md.as_str(), "md");
        assert_eq!(ChipSize::Lg.as_str(), "lg");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  custom-chip  ".to_string())),
            Some("custom-chip".to_string())
        );
    }

    #[test]
    fn resolve_dismiss_aria_label_defaults_and_trims() {
        assert_eq!(
            resolve_dismiss_aria_label(None),
            ("Remove tag".to_string(), false)
        );
        assert_eq!(
            resolve_dismiss_aria_label(Some(" Dismiss assignee ".to_string())),
            ("Dismiss assignee".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_variant_size_and_flags() {
        let state = resolve_state(ChipStateInput {
            variant: ChipVariant::Danger,
            size: ChipSize::Lg,
            disabled: true,
            has_dismiss_action: true,
            has_custom_dismiss_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, ChipVariant::Danger);
        assert_eq!(state.size, ChipSize::Lg);
        assert_eq!(state.variant_class, "ui-chip--variant-danger");
        assert_eq!(state.size_class, "ui-chip--size-lg");
        assert_eq!(state.variant_attr, "danger");
        assert_eq!(state.size_attr, "lg");
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(state.has_dismiss_action);
        assert!(!state.is_static);
        assert!(state.has_custom_dismiss_aria_label);
        assert!(state.has_custom_class_name);
    }

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
            "ui-chip--enabled",
            "ui-chip--static",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
