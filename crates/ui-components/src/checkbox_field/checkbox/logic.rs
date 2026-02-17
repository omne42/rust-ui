#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxVariant {
    #[default]
    Default,
    Accent,
}

impl CheckboxVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--variant-default",
            Self::Accent => "ui-checkbox--variant-accent",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl CheckboxSize {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Default => "ui-checkbox--size-default",
            Self::Sm => "ui-checkbox--size-sm",
            Self::Lg => "ui-checkbox--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxState {
    pub is_checked: bool,
    pub is_unchecked: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
}

impl CheckboxState {
    pub fn data_state(self) -> &'static str {
        if self.is_checked {
            "checked"
        } else {
            "unchecked"
        }
    }
}

pub fn resolve_state(
    is_checked: bool,
    is_disabled: bool,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
) -> CheckboxState {
    let is_enabled = !is_disabled;

    CheckboxState {
        is_checked,
        is_unchecked: !is_checked,
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
            CheckboxVariant::Default.class_name(),
            "ui-checkbox--variant-default"
        );
        assert_eq!(
            CheckboxVariant::Accent.class_name(),
            "ui-checkbox--variant-accent"
        );
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            CheckboxSize::Default.class_name(),
            "ui-checkbox--size-default"
        );
        assert_eq!(CheckboxSize::Sm.class_name(), "ui-checkbox--size-sm");
        assert_eq!(CheckboxSize::Lg.class_name(), "ui-checkbox--size-lg");
    }

    #[test]
    fn resolve_state_tracks_checked_enabled_interactions() {
        let state = resolve_state(true, false, true, true, true, true);

        assert!(state.is_checked);
        assert!(!state.is_unchecked);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.is_pressed);
        assert!(state.is_hovered);
        assert!(state.is_focused);
        assert!(state.is_focus_visible);
        assert_eq!(state.data_state(), "checked");
    }

    #[test]
    fn resolve_state_clears_interaction_flags_when_disabled() {
        let state = resolve_state(false, true, true, true, true, true);

        assert!(!state.is_checked);
        assert!(state.is_unchecked);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(!state.is_pressed);
        assert!(!state.is_hovered);
        assert!(!state.is_focused);
        assert!(!state.is_focus_visible);
        assert_eq!(state.data_state(), "unchecked");
    }
}
