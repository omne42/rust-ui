#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonVariant {
    #[default]
    Default,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
}

impl ToggleButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonVariant::Default => "ui-toggle-button--variant-default",
            ToggleButtonVariant::Accent => "ui-toggle-button--variant-accent",
            ToggleButtonVariant::Destructive => "ui-toggle-button--variant-destructive",
            ToggleButtonVariant::Outline => "ui-toggle-button--variant-outline",
            ToggleButtonVariant::Secondary => "ui-toggle-button--variant-secondary",
            ToggleButtonVariant::Ghost => "ui-toggle-button--variant-ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ToggleButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonSize::Default => "ui-toggle-button--size-default",
            ToggleButtonSize::Sm => "ui-toggle-button--size-sm",
            ToggleButtonSize::Lg => "ui-toggle-button--size-lg",
            ToggleButtonSize::Icon => "ui-toggle-button--size-icon",
            ToggleButtonSize::IconSm => "ui-toggle-button--size-icon-sm",
            ToggleButtonSize::IconLg => "ui-toggle-button--size-icon-lg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            ToggleButtonVariant::Default.class_name(),
            "ui-toggle-button--variant-default"
        );
        assert_eq!(
            ToggleButtonVariant::Accent.class_name(),
            "ui-toggle-button--variant-accent"
        );
        assert_eq!(
            ToggleButtonVariant::Destructive.class_name(),
            "ui-toggle-button--variant-destructive"
        );
        assert_eq!(
            ToggleButtonVariant::Outline.class_name(),
            "ui-toggle-button--variant-outline"
        );
        assert_eq!(
            ToggleButtonVariant::Secondary.class_name(),
            "ui-toggle-button--variant-secondary"
        );
        assert_eq!(
            ToggleButtonVariant::Ghost.class_name(),
            "ui-toggle-button--variant-ghost"
        );
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            ToggleButtonSize::Default.class_name(),
            "ui-toggle-button--size-default"
        );
        assert_eq!(
            ToggleButtonSize::Sm.class_name(),
            "ui-toggle-button--size-sm"
        );
        assert_eq!(
            ToggleButtonSize::Lg.class_name(),
            "ui-toggle-button--size-lg"
        );
        assert_eq!(
            ToggleButtonSize::Icon.class_name(),
            "ui-toggle-button--size-icon"
        );
        assert_eq!(
            ToggleButtonSize::IconSm.class_name(),
            "ui-toggle-button--size-icon-sm"
        );
        assert_eq!(
            ToggleButtonSize::IconLg.class_name(),
            "ui-toggle-button--size-icon-lg"
        );
    }
}
