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
