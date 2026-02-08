mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, StepListItem, StepListOrientation, StepListSize};
pub use view::StepList;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListStateInput {
    pub orientation: StepListOrientation,
    pub size: StepListSize,
    pub emphasized: bool,
    pub disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub completed_count: usize,
    pub disabled_count: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListState {
    pub orientation: StepListOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub size: StepListSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub is_emphasized: bool,
    pub is_disabled: bool,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub completed_count: usize,
    pub disabled_count: usize,
    pub has_selection: bool,
    pub has_completed_steps: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub emphasis_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemStateInput {
    pub index: usize,
    pub selected_index: Option<usize>,
    pub completed: bool,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepListItemState {
    pub index: usize,
    pub marker_number: usize,
    pub is_current: bool,
    pub is_completed: bool,
    pub is_disabled: bool,
    pub is_pending: bool,
    pub is_selectable: bool,
    pub status_attr: &'static str,
    pub status_class: &'static str,
}
