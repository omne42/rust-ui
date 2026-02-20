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
#[path = "../test/logic.rs"]
mod tests;
