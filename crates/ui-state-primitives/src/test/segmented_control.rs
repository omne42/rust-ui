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
