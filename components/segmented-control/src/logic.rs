use leptos::prelude::{ReadSignal, WriteSignal};
use ui_state_primitives::segmented_control::SegmentedControlState;

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
    Uncontrolled,
}

impl SegmentedControlControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            SegmentedControlControlMode::Controlled => "controlled",
            SegmentedControlControlMode::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone)]
pub struct SegmentedControlSelectionAxisInput {
    pub selected_index: Option<ReadSignal<Option<usize>>>,
    pub on_selected_index_change: Option<WriteSignal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub item_count: usize,
}

#[derive(Clone)]
pub struct SegmentedControlSelectionAxis {
    pub selected_index: Option<ReadSignal<Option<usize>>>,
    pub on_selected_index_change: Option<WriteSignal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub control_mode: SegmentedControlControlMode,
}

pub fn normalize_selection_axis(
    input: SegmentedControlSelectionAxisInput,
) -> SegmentedControlSelectionAxis {
    let has_controlled_selected_index = input.selected_index.is_some();
    let has_on_selected_index_change = input.on_selected_index_change.is_some();
    assert!(
        has_controlled_selected_index == has_on_selected_index_change,
        "SegmentedControl: `selected_index` and `on_selected_index_change` must be provided together for controlled mode, or omitted together for uncontrolled mode."
    );

    let control_mode = if has_controlled_selected_index {
        SegmentedControlControlMode::Controlled
    } else {
        SegmentedControlControlMode::Uncontrolled
    };
    let default_selected_index = input
        .default_selected_index
        .filter(|index| *index < input.item_count);

    SegmentedControlSelectionAxis {
        selected_index: input.selected_index,
        on_selected_index_change: input.on_selected_index_change,
        default_selected_index,
        control_mode,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlSemanticStateInput {
    pub control_mode: SegmentedControlControlMode,
    pub raw_selected_index: Option<usize>,
    pub normalized_state: SegmentedControlState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlSemanticState {
    pub control_mode: SegmentedControlControlMode,
    pub selection_source: SegmentedControlSelectionSource,
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub has_disabled_options: bool,
    pub disabled_option_count: usize,
    pub selected_index: Option<usize>,
    pub has_selection: bool,
    pub selection_empty: bool,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub has_label: bool,
}

pub fn normalize_semantic_state(
    input: SegmentedControlSemanticStateInput,
) -> SegmentedControlSemanticState {
    let selection_source = SegmentedControlSelectionSource::from_indices(
        input.raw_selected_index,
        input.normalized_state.selected_index,
    );

    SegmentedControlSemanticState {
        control_mode: input.control_mode,
        selection_source,
        item_count: input.normalized_state.item_count,
        is_empty: input.normalized_state.is_empty,
        has_items: input.normalized_state.has_items,
        is_disabled: input.normalized_state.is_disabled,
        has_disabled_options: input.normalized_state.has_disabled_options,
        disabled_option_count: input.normalized_state.disabled_option_count,
        selected_index: input.normalized_state.selected_index,
        has_selection: input.normalized_state.has_selection,
        selection_empty: input.normalized_state.selection_empty,
        is_horizontal: input.normalized_state.is_horizontal,
        is_vertical: input.normalized_state.is_vertical,
        has_label: input.normalized_state.has_label,
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
