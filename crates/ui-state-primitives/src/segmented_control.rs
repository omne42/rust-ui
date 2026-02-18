use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlStateInput<'a> {
    pub item_count: usize,
    pub is_disabled: bool,
    pub disabled_indices: &'a HashSet<usize>,
    pub selected_index: Option<usize>,
    pub is_vertical: bool,
    pub has_label: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentedControlState {
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

pub fn resolve_state(input: SegmentedControlStateInput<'_>) -> SegmentedControlState {
    let has_items = input.item_count > 0;
    let selected_index = input
        .selected_index
        .filter(|index| *index < input.item_count);
    let has_selection = selected_index.is_some();
    let disabled_option_count = input
        .disabled_indices
        .iter()
        .filter(|index| **index < input.item_count)
        .count();

    SegmentedControlState {
        item_count: input.item_count,
        is_empty: !has_items,
        has_items,
        is_disabled: input.is_disabled,
        has_disabled_options: disabled_option_count > 0,
        disabled_option_count,
        selected_index,
        has_selection,
        selection_empty: !has_selection,
        is_horizontal: !input.is_vertical,
        is_vertical: input.is_vertical,
        has_label: input.has_label,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_state_tracks_empty_disabled_group() {
        let disabled = HashSet::new();
        let state = resolve_state(SegmentedControlStateInput {
            item_count: 0,
            is_disabled: true,
            disabled_indices: &disabled,
            selected_index: Some(0),
            is_vertical: false,
            has_label: false,
        });

        assert_eq!(state.item_count, 0);
        assert!(state.is_empty);
        assert!(!state.has_items);
        assert!(state.is_disabled);
        assert!(!state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 0);
        assert_eq!(state.selected_index, None);
        assert!(!state.has_selection);
        assert!(state.selection_empty);
        assert!(state.is_horizontal);
        assert!(!state.is_vertical);
        assert!(!state.has_label);
    }

    #[test]
    fn resolve_state_tracks_selection_orientation_and_disabled_options() {
        let disabled = HashSet::from([1_usize, 8_usize]);
        let state = resolve_state(SegmentedControlStateInput {
            item_count: 3,
            is_disabled: false,
            disabled_indices: &disabled,
            selected_index: Some(2),
            is_vertical: true,
            has_label: true,
        });

        assert_eq!(state.item_count, 3);
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(!state.is_disabled);
        assert!(state.has_disabled_options);
        assert_eq!(state.disabled_option_count, 1);
        assert_eq!(state.selected_index, Some(2));
        assert!(state.has_selection);
        assert!(!state.selection_empty);
        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(state.has_label);
    }
}
