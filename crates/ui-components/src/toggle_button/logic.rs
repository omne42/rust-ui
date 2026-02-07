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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonState {
    pub is_selected: bool,
    pub is_unselected: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl ToggleButtonState {
    pub fn data_state(self) -> &'static str {
        if self.is_selected {
            "selected"
        } else {
            "unselected"
        }
    }
}

pub fn resolve_state(
    is_selected: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> ToggleButtonState {
    let is_enabled = !is_disabled;

    ToggleButtonState {
        is_selected,
        is_unselected: !is_selected,
        is_disabled,
        is_enabled,
        is_pressed: is_pressed && is_enabled,
        is_hovered: is_hovered && is_enabled,
        is_focused: is_focused && is_enabled,
        is_focus_visible: is_focus_visible && is_enabled,
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

    #[test]
    fn resolve_state_tracks_selected_enabled_interactions() {
        let state = resolve_state(true, false, true, true, true, true);

        assert!(state.is_selected);
        assert!(!state.is_unselected);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.is_pressed);
        assert!(state.is_hovered);
        assert!(state.is_focused);
        assert!(state.is_focus_visible);
        assert_eq!(state.data_state(), "selected");
    }

    #[test]
    fn resolve_state_clears_interaction_flags_when_disabled() {
        let state = resolve_state(false, true, true, true, true, true);

        assert!(!state.is_selected);
        assert!(state.is_unselected);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(!state.is_pressed);
        assert!(!state.is_hovered);
        assert!(!state.is_focused);
        assert!(!state.is_focus_visible);
        assert_eq!(state.data_state(), "unselected");
    }
}
