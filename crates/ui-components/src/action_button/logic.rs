#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonSize {
    XS,
    S,
    #[default]
    M,
    L,
    XL,
}

impl ActionButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonSize::XS => "ui-action-button--size-xs",
            ActionButtonSize::S => "ui-action-button--size-s",
            ActionButtonSize::M => "ui-action-button--size-m",
            ActionButtonSize::L => "ui-action-button--size-l",
            ActionButtonSize::XL => "ui-action-button--size-xl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonLoadingPlacement {
    #[default]
    Center,
    Start,
    End,
}

impl ActionButtonLoadingPlacement {
    pub fn as_attr(self) -> &'static str {
        match self {
            ActionButtonLoadingPlacement::Center => "center",
            ActionButtonLoadingPlacement::Start => "start",
            ActionButtonLoadingPlacement::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonState {
    pub is_disabled: bool,
    pub is_loading: bool,
}

pub fn resolve_state(disabled: bool, is_loading: bool) -> ActionButtonState {
    ActionButtonState {
        is_disabled: disabled || is_loading,
        is_loading,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            ActionButtonSize::XS.class_name(),
            "ui-action-button--size-xs"
        );
        assert_eq!(ActionButtonSize::M.class_name(), "ui-action-button--size-m");
        assert_eq!(
            ActionButtonSize::XL.class_name(),
            "ui-action-button--size-xl"
        );
    }

    #[test]
    fn loading_placement_attrs_match_variants() {
        assert_eq!(ActionButtonLoadingPlacement::Center.as_attr(), "center");
        assert_eq!(ActionButtonLoadingPlacement::Start.as_attr(), "start");
        assert_eq!(ActionButtonLoadingPlacement::End.as_attr(), "end");
    }

    #[test]
    fn loading_forces_disabled() {
        assert!(!resolve_state(false, false).is_disabled);
        assert!(resolve_state(false, true).is_disabled);
        assert!(resolve_state(true, false).is_disabled);
    }
}
