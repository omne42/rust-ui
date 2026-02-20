use crate::{
    StepListItem, StepListItemState, StepListItemStateInput, StepListState, StepListStateInput,
};
use leptos::prelude::*;
use ui_state_primitives::step_list as primitives;

#[derive(Clone)]
pub struct StepListSelectionAxisInput {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
    pub item_count: usize,
}

#[derive(Clone)]
pub struct StepListSelectionAxis {
    pub selected_index: Option<Signal<Option<usize>>>,
    pub default_selected_index: Option<usize>,
    pub on_selected_index_change: Option<Callback<Option<usize>>>,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    primitives::normalize_optional_text(value)
}

pub fn normalize_items(items: Vec<StepListItem>) -> Vec<StepListItem> {
    primitives::normalize_items(items)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    primitives::normalize_aria_label(value)
}

pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    primitives::sanitize_index(index, item_count)
}

pub fn resolve_selected_index(
    items: &[StepListItem],
    selected_index: Option<usize>,
) -> Option<usize> {
    primitives::resolve_selected_index(items, selected_index)
}

pub fn normalize_completed_indices(
    item_count: usize,
    completed_indices: Vec<usize>,
) -> std::collections::BTreeSet<usize> {
    primitives::normalize_completed_indices(item_count, completed_indices)
}

pub fn first_enabled_index(items: &[StepListItem]) -> Option<usize> {
    primitives::first_enabled_index(items)
}

pub fn resolve_state(input: StepListStateInput) -> StepListState {
    primitives::resolve_state(input)
}

pub fn resolve_item_state(input: StepListItemStateInput) -> StepListItemState {
    primitives::resolve_item_state(input)
}

pub fn normalize_id_base(id_base: Option<String>) -> String {
    normalize_optional_text(id_base).unwrap_or_else(|| "ui-step-list".to_string())
}

pub fn normalize_selection_axis(input: StepListSelectionAxisInput) -> StepListSelectionAxis {
    let default_selected_index = sanitize_index(input.default_selected_index, input.item_count);
    StepListSelectionAxis {
        selected_index: input.selected_index,
        default_selected_index,
        on_selected_index_change: input.on_selected_index_change,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: StepListState) -> String {
    let mut classes = vec![
        "ui-step-list".to_string(),
        state.orientation_class.into(),
        state.size_class.into(),
    ];

    if state.is_emphasized {
        classes.push("ui-step-list--emphasized".to_string());
    }

    if state.is_disabled {
        classes.push("ui-step-list--disabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-step-list--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
