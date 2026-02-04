use crate::action_button::ActionButtonSize;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ActionButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonGroupOrientation::Horizontal => "ui-action-button-group--horizontal",
            ActionButtonGroupOrientation::Vertical => "ui-action-button-group--vertical",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        match self {
            ActionButtonGroupOrientation::Horizontal => "horizontal",
            ActionButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonGroupDensity {
    #[default]
    Regular,
    Compact,
}

impl ActionButtonGroupDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonGroupDensity::Regular => "ui-action-button-group--density-regular",
            ActionButtonGroupDensity::Compact => "ui-action-button-group--density-compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonGroupContextValue {
    pub size: ActionButtonSize,
    pub density: ActionButtonGroupDensity,
    pub orientation: ActionButtonGroupOrientation,
    pub is_justified: bool,
    pub is_quiet: bool,
    pub is_disabled: bool,
}

pub(crate) fn use_action_button_group_context() -> Option<ActionButtonGroupContextValue> {
    use_context::<ActionButtonGroupContextValue>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn density_class_names_are_stable() {
        assert_eq!(
            ActionButtonGroupDensity::Regular.class_name(),
            "ui-action-button-group--density-regular"
        );
        assert_eq!(
            ActionButtonGroupDensity::Compact.class_name(),
            "ui-action-button-group--density-compact"
        );
    }

    #[test]
    fn orientation_attributes_match_variants() {
        assert_eq!(
            ActionButtonGroupOrientation::Horizontal.aria_orientation(),
            "horizontal"
        );
        assert_eq!(
            ActionButtonGroupOrientation::Vertical.aria_orientation(),
            "vertical"
        );
    }
}
