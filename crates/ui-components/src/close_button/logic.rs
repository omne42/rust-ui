use crate::close_button::{CloseButtonState, CloseButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Close";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CloseButtonVariant {
    #[default]
    Default,
    OverBackground,
}

impl CloseButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            CloseButtonVariant::Default => "ui-close-button--variant-default",
            CloseButtonVariant::OverBackground => "ui-close-button--variant-over-background",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CloseButtonVariant::Default => "default",
            CloseButtonVariant::OverBackground => "over-background",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CloseButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CloseButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            CloseButtonSize::Sm => "ui-close-button--size-sm",
            CloseButtonSize::Md => "ui-close-button--size-md",
            CloseButtonSize::Lg => "ui-close-button--size-lg",
            CloseButtonSize::Xl => "ui-close-button--size-xl",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CloseButtonSize::Sm => "sm",
            CloseButtonSize::Md => "md",
            CloseButtonSize::Lg => "lg",
            CloseButtonSize::Xl => "xl",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: CloseButtonStateInput) -> CloseButtonState {
    CloseButtonState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_attr(),
        size_attr: input.size.as_attr(),
        is_disabled: input.disabled,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        data_state_attr: if input.disabled { "disabled" } else { "ready" },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CloseButtonState) -> String {
    let mut classes = vec![
        "ui-close-button".to_string(),
        state.variant_class.to_string(),
        state.size_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-close-button--disabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-close-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-close-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-close-button--custom-class".to_string());
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
    fn variant_and_size_contracts_are_stable() {
        assert_eq!(
            CloseButtonVariant::Default.class_name(),
            "ui-close-button--variant-default"
        );
        assert_eq!(
            CloseButtonVariant::OverBackground.class_name(),
            "ui-close-button--variant-over-background"
        );
        assert_eq!(CloseButtonVariant::Default.as_attr(), "default");
        assert_eq!(
            CloseButtonVariant::OverBackground.as_attr(),
            "over-background"
        );

        assert_eq!(CloseButtonSize::Sm.class_name(), "ui-close-button--size-sm");
        assert_eq!(CloseButtonSize::Md.class_name(), "ui-close-button--size-md");
        assert_eq!(CloseButtonSize::Lg.class_name(), "ui-close-button--size-lg");
        assert_eq!(CloseButtonSize::Xl.class_name(), "ui-close-button--size-xl");

        assert_eq!(CloseButtonSize::Sm.as_attr(), "sm");
        assert_eq!(CloseButtonSize::Md.as_attr(), "md");
        assert_eq!(CloseButtonSize::Lg.as_attr(), "lg");
        assert_eq!(CloseButtonSize::Xl.as_attr(), "xl");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-close-button  ".to_string())),
            Some("docs-close-button".to_string())
        );

        let (aria_label, custom) = normalize_aria_label(Some("  Dismiss panel  ".to_string()));
        assert_eq!(aria_label, "Dismiss panel");
        assert!(custom);

        let (aria_label, custom) = normalize_aria_label(None);
        assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_variant_size_and_sources() {
        let state = resolve_state(CloseButtonStateInput {
            variant: CloseButtonVariant::OverBackground,
            size: CloseButtonSize::Lg,
            disabled: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
            has_custom_press_handler: true,
        });

        assert_eq!(state.variant_attr, "over-background");
        assert_eq!(state.size_attr, "lg");
        assert!(!state.is_disabled);
        assert_eq!(state.data_state_attr, "ready");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-close-button-custom".to_string()),
            resolve_state(CloseButtonStateInput {
                variant: CloseButtonVariant::Default,
                size: CloseButtonSize::Md,
                disabled: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_press_handler: true,
            }),
        );

        for token in [
            "ui-close-button",
            "ui-close-button--variant-default",
            "ui-close-button--size-md",
            "ui-close-button--disabled",
            "ui-close-button--custom-handler",
            "ui-close-button--custom-class",
            "docs-close-button-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
