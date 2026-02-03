#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Accent,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonVariant::Default => "ui-button--variant-default",
            ButtonVariant::Accent => "ui-button--variant-accent",
            ButtonVariant::Destructive => "ui-button--variant-destructive",
            ButtonVariant::Outline => "ui-button--variant-outline",
            ButtonVariant::Secondary => "ui-button--variant-secondary",
            ButtonVariant::Ghost => "ui-button--variant-ghost",
            ButtonVariant::Link => "ui-button--variant-link",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ButtonSize::Default => "ui-button--size-default",
            ButtonSize::Sm => "ui-button--size-sm",
            ButtonSize::Lg => "ui-button--size-lg",
            ButtonSize::Icon => "ui-button--size-icon",
            ButtonSize::IconSm => "ui-button--size-icon-sm",
            ButtonSize::IconLg => "ui-button--size-icon-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonLoadingPlacement {
    #[default]
    Start,
    End,
    Center,
}

impl ButtonLoadingPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ButtonLoadingPlacement::Start => "start",
            ButtonLoadingPlacement::End => "end",
            ButtonLoadingPlacement::Center => "center",
        }
    }
}
