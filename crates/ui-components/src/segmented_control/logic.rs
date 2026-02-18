#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedControlOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SegmentedControlOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            SegmentedControlOrientation::Horizontal => "ui-segmented-control--horizontal",
            SegmentedControlOrientation::Vertical => "ui-segmented-control--vertical",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        match self {
            SegmentedControlOrientation::Horizontal => "horizontal",
            SegmentedControlOrientation::Vertical => "vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        self.aria_orientation()
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, SegmentedControlOrientation::Vertical)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedControlSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl SegmentedControlSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SegmentedControlSize::Default => "ui-segmented-control--size-default",
            SegmentedControlSize::Sm => "ui-segmented-control--size-sm",
            SegmentedControlSize::Lg => "ui-segmented-control--size-lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentedControlControlMode {
    Controlled,
}

impl SegmentedControlControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlControlMode::Controlled => "controlled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentedControlSelectionSource {
    None,
    Selected,
    OutOfRange,
}

impl SegmentedControlSelectionSource {
    pub fn from_indices(raw_selected: Option<usize>, normalized_selected: Option<usize>) -> Self {
        match (raw_selected, normalized_selected) {
            (None, None) => SegmentedControlSelectionSource::None,
            (Some(_), Some(_)) => SegmentedControlSelectionSource::Selected,
            (Some(_), None) => SegmentedControlSelectionSource::OutOfRange,
            (None, Some(_)) => SegmentedControlSelectionSource::Selected,
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlSelectionSource::None => "external-none",
            SegmentedControlSelectionSource::Selected => "external-selected",
            SegmentedControlSelectionSource::OutOfRange => "external-out-of-range",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedControlSelectionOrigin {
    #[default]
    Programmatic,
    Keyboard,
    Pointer,
}

impl SegmentedControlSelectionOrigin {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlSelectionOrigin::Programmatic => "programmatic",
            SegmentedControlSelectionOrigin::Keyboard => "keyboard",
            SegmentedControlSelectionOrigin::Pointer => "pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentedControlAgentSchemaVersion {
    V1,
}

impl SegmentedControlAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentedControlAgentIntent {
    SingleChoiceSelection,
}

impl SegmentedControlAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlAgentIntent::SingleChoiceSelection => "single-choice-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentedControlAgentActionModel {
    NavigateAndSelect,
}

impl SegmentedControlAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlAgentActionModel::NavigateAndSelect => "navigate|focus|select",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn segmented_control_agent_contract() -> SegmentedControlAgentContract {
    SegmentedControlAgentContract {
        schema_attr: "ui.segmented-control",
        schema_version_attr: SegmentedControlAgentSchemaVersion::V1.as_attr(),
        intent_attr: SegmentedControlAgentIntent::SingleChoiceSelection.as_attr(),
        action_model_attr: SegmentedControlAgentActionModel::NavigateAndSelect.as_attr(),
        state_axis_attr: "selection|availability|orientation|label",
        source_axis_attr: "control-mode|selection-source|selection-origin|disabled-indices",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_class_names_are_stable() {
        assert_eq!(
            SegmentedControlOrientation::Horizontal.class_name(),
            "ui-segmented-control--horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.class_name(),
            "ui-segmented-control--vertical"
        );
    }

    #[test]
    fn aria_and_data_orientation_values_are_stable() {
        assert_eq!(
            SegmentedControlOrientation::Horizontal.aria_orientation(),
            "horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.aria_orientation(),
            "vertical"
        );
        assert_eq!(
            SegmentedControlOrientation::Horizontal.data_orientation(),
            "horizontal"
        );
        assert_eq!(
            SegmentedControlOrientation::Vertical.data_orientation(),
            "vertical"
        );
    }

    #[test]
    fn orientation_axis_flags_are_stable() {
        assert!(!SegmentedControlOrientation::Horizontal.is_vertical());
        assert!(SegmentedControlOrientation::Vertical.is_vertical());
    }

    #[test]
    fn size_class_names_are_stable() {
        assert_eq!(
            SegmentedControlSize::Default.class_name(),
            "ui-segmented-control--size-default"
        );
        assert_eq!(
            SegmentedControlSize::Sm.class_name(),
            "ui-segmented-control--size-sm"
        );
        assert_eq!(
            SegmentedControlSize::Lg.class_name(),
            "ui-segmented-control--size-lg"
        );
    }

    #[test]
    fn control_mode_attr_is_stable() {
        assert_eq!(
            SegmentedControlControlMode::Controlled.as_attr(),
            "controlled"
        );
    }

    #[test]
    fn selection_source_attr_is_closed_set() {
        assert_eq!(
            SegmentedControlSelectionSource::None.as_attr(),
            "external-none"
        );
        assert_eq!(
            SegmentedControlSelectionSource::Selected.as_attr(),
            "external-selected"
        );
        assert_eq!(
            SegmentedControlSelectionSource::OutOfRange.as_attr(),
            "external-out-of-range"
        );
    }

    #[test]
    fn selection_source_resolves_from_raw_and_normalized_selection() {
        assert_eq!(
            SegmentedControlSelectionSource::from_indices(None, None),
            SegmentedControlSelectionSource::None
        );
        assert_eq!(
            SegmentedControlSelectionSource::from_indices(Some(1), Some(1)),
            SegmentedControlSelectionSource::Selected
        );
        assert_eq!(
            SegmentedControlSelectionSource::from_indices(Some(8), None),
            SegmentedControlSelectionSource::OutOfRange
        );
    }

    #[test]
    fn selection_origin_attr_is_stable() {
        assert_eq!(
            SegmentedControlSelectionOrigin::Programmatic.as_attr(),
            "programmatic"
        );
        assert_eq!(
            SegmentedControlSelectionOrigin::Keyboard.as_attr(),
            "keyboard"
        );
        assert_eq!(
            SegmentedControlSelectionOrigin::Pointer.as_attr(),
            "pointer"
        );
    }

    #[test]
    fn agent_contract_is_schema_typed_and_stable() {
        let contract = segmented_control_agent_contract();
        assert_eq!(contract.schema_attr, "ui.segmented-control");
        assert_eq!(contract.schema_version_attr, "v1");
        assert_eq!(contract.intent_attr, "single-choice-selection");
        assert_eq!(contract.action_model_attr, "navigate|focus|select");
        assert_eq!(
            contract.state_axis_attr,
            "selection|availability|orientation|label"
        );
        assert_eq!(
            contract.source_axis_attr,
            "control-mode|selection-source|selection-origin|disabled-indices"
        );
    }
}
