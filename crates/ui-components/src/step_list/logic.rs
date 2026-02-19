use crate::step_list::{
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
mod tests {
    use super::*;

    #[test]
    fn normalize_selection_axis_keeps_controlled_pair_and_sanitizes_default() {
        let (selected_index, _set_selected_index) = signal(Some(1_usize));
        let callback = Callback::new(|_value: Option<usize>| {});
        let axis = normalize_selection_axis(StepListSelectionAxisInput {
            selected_index: Some(selected_index.into()),
            default_selected_index: Some(9),
            on_selected_index_change: Some(callback),
            item_count: 3,
        });

        assert!(axis.selected_index.is_some());
        assert_eq!(axis.default_selected_index, None);
        assert!(axis.on_selected_index_change.is_some());
    }

    #[test]
    fn compose_class_name_includes_sources() {
        let state = resolve_state(StepListStateInput {
            orientation: crate::step_list::StepListOrientation::Horizontal,
            size: crate::step_list::StepListSize::M,
            emphasized: true,
            disabled: true,
            item_count: 1,
            selected_index: Some(0),
            completed_count: 0,
            disabled_count: 0,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-step-list".to_string()), state);
        assert!(class_name.contains("ui-step-list"));
        assert!(class_name.contains("ui-step-list--orientation-horizontal"));
        assert!(class_name.contains("ui-step-list--size-m"));
        assert!(class_name.contains("ui-step-list--emphasized"));
        assert!(class_name.contains("ui-step-list--disabled"));
        assert!(class_name.contains("ui-step-list--custom-class"));
        assert!(class_name.contains("docs-step-list"));
    }
}
