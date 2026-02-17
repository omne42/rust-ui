pub use crate::button::normalize_optional_text;

pub const DEFAULT_DISMISS_ARIA_LABEL: &str = "Remove tag";

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
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_dismiss_action: bool,
    pub is_static: bool,
    pub has_custom_dismiss_aria_label: bool,
    pub dismiss_label_source_class: &'static str,
    pub dismiss_label_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}

pub fn resolve_dismiss_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_DISMISS_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: ChipStateInput) -> ChipState {
    let (state_class, state_attr) = if input.disabled {
        ("ui-chip--disabled", "disabled")
    } else if input.has_dismiss_action {
        ("ui-chip--removable", "removable")
    } else {
        ("ui-chip--static", "static")
    };

    let (dismiss_label_source_class, dismiss_label_source_attr) =
        if input.has_custom_dismiss_aria_label {
            ("ui-chip--dismiss-label-custom", "custom")
        } else {
            ("ui-chip--dismiss-label-default", "default")
        };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ChipState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_str(),
        size_attr: input.size.as_str(),
        state_class,
        state_attr,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_dismiss_action: input.has_dismiss_action,
        is_static: !input.has_dismiss_action,
        has_custom_dismiss_aria_label: input.has_custom_dismiss_aria_label,
        dismiss_label_source_class,
        dismiss_label_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
    }
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
            (DEFAULT_DISMISS_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            resolve_dismiss_aria_label(Some(" Dismiss assignee ".to_string())),
            ("Dismiss assignee".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_variant_size_and_sources() {
        let removable = resolve_state(ChipStateInput {
            variant: ChipVariant::Danger,
            size: ChipSize::Lg,
            disabled: false,
            has_dismiss_action: true,
            has_custom_dismiss_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(removable.variant, ChipVariant::Danger);
        assert_eq!(removable.size, ChipSize::Lg);
        assert_eq!(removable.variant_class, "ui-chip--variant-danger");
        assert_eq!(removable.size_class, "ui-chip--size-lg");
        assert_eq!(removable.variant_attr, "danger");
        assert_eq!(removable.size_attr, "lg");
        assert_eq!(removable.state_class, "ui-chip--removable");
        assert_eq!(removable.state_attr, "removable");
        assert!(!removable.is_disabled);
        assert!(removable.is_enabled);
        assert!(removable.has_dismiss_action);
        assert!(!removable.is_static);
        assert!(removable.has_custom_dismiss_aria_label);
        assert_eq!(
            removable.dismiss_label_source_class,
            "ui-chip--dismiss-label-custom"
        );
        assert_eq!(removable.dismiss_label_source_attr, "custom");
        assert!(removable.has_custom_class_name);
        assert_eq!(removable.class_source_attr, "custom");

        let disabled = resolve_state(ChipStateInput {
            variant: ChipVariant::Default,
            size: ChipSize::Md,
            disabled: true,
            has_dismiss_action: true,
            has_custom_dismiss_aria_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(disabled.state_class, "ui-chip--disabled");
        assert_eq!(disabled.state_attr, "disabled");
        assert_eq!(
            disabled.dismiss_label_source_class,
            "ui-chip--dismiss-label-default"
        );
        assert_eq!(disabled.dismiss_label_source_attr, "default");
        assert_eq!(disabled.class_source_attr, "default");
    }
}
