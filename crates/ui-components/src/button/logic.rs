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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonState {
    pub is_disabled: bool,
    pub is_loading: bool,
}

pub fn resolve_state(disabled: bool, is_loading: bool) -> ButtonState {
    ButtonState {
        is_disabled: disabled || is_loading,
        is_loading,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_names_are_stable() {
        assert_eq!(
            ButtonVariant::Default.class_name(),
            "ui-button--variant-default"
        );
        assert_eq!(
            ButtonVariant::Accent.class_name(),
            "ui-button--variant-accent"
        );
        assert_eq!(
            ButtonVariant::Destructive.class_name(),
            "ui-button--variant-destructive"
        );
        assert_eq!(
            ButtonVariant::Outline.class_name(),
            "ui-button--variant-outline"
        );
        assert_eq!(
            ButtonVariant::Secondary.class_name(),
            "ui-button--variant-secondary"
        );
        assert_eq!(
            ButtonVariant::Ghost.class_name(),
            "ui-button--variant-ghost"
        );
        assert_eq!(ButtonVariant::Link.class_name(), "ui-button--variant-link");
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(ButtonSize::Default.class_name(), "ui-button--size-default");
        assert_eq!(ButtonSize::Sm.class_name(), "ui-button--size-sm");
        assert_eq!(ButtonSize::Lg.class_name(), "ui-button--size-lg");
        assert_eq!(ButtonSize::Icon.class_name(), "ui-button--size-icon");
        assert_eq!(ButtonSize::IconSm.class_name(), "ui-button--size-icon-sm");
        assert_eq!(ButtonSize::IconLg.class_name(), "ui-button--size-icon-lg");
    }

    #[test]
    fn loading_placement_attrs_match_variants() {
        assert_eq!(ButtonLoadingPlacement::Start.as_attr(), "start");
        assert_eq!(ButtonLoadingPlacement::End.as_attr(), "end");
        assert_eq!(ButtonLoadingPlacement::Center.as_attr(), "center");
    }

    #[test]
    fn loading_forces_disabled() {
        assert!(!resolve_state(false, false).is_disabled);
        assert!(resolve_state(false, true).is_disabled);
        assert!(resolve_state(true, false).is_disabled);
    }
}
