use super::*;

#[test]
fn keyboard_activation_defaults_to_automatic() {
    assert_eq!(
        TabsKeyboardActivation::default(),
        TabsKeyboardActivation::Automatic
    );
    assert!(TabsKeyboardActivation::Automatic.selects_on_focus());
    assert!(!TabsKeyboardActivation::Manual.selects_on_focus());
}

#[test]
fn normalize_index_clamps_to_bounds() {
    assert_eq!(normalize_index_skipping_disabled(0, 0, |_| false), 0);
    assert_eq!(normalize_index_skipping_disabled(1, 1, |_| false), 0);
    assert_eq!(normalize_index_skipping_disabled(2, 2, |_| false), 1);
}

#[test]
fn normalize_index_skips_disabled_when_possible() {
    assert_eq!(normalize_index_skipping_disabled(0, 3, |idx| idx == 0), 1);
    assert_eq!(normalize_index_skipping_disabled(2, 3, |idx| idx == 2), 0);
}

#[test]
fn resolve_next_selected_respects_keyboard_activation() {
    let is_disabled = |idx: usize| idx == 2;

    let current = 0;
    let candidate = 1;
    assert_eq!(
        resolve_next_selected_index(
            current,
            candidate,
            3,
            is_disabled,
            TabsKeyboardActivation::Manual,
            TabsSelectionTrigger::Focus
        ),
        current
    );
    assert_eq!(
        resolve_next_selected_index(
            current,
            candidate,
            3,
            is_disabled,
            TabsKeyboardActivation::Automatic,
            TabsSelectionTrigger::Focus
        ),
        candidate
    );

    assert_eq!(
        resolve_next_selected_index(
            current,
            candidate,
            3,
            is_disabled,
            TabsKeyboardActivation::Manual,
            TabsSelectionTrigger::Press
        ),
        candidate
    );
}

#[test]
fn resolve_next_selected_ignores_disabled_candidates() {
    let is_disabled = |idx: usize| idx == 1;
    assert_eq!(
        resolve_next_selected_index(
            0,
            1,
            3,
            is_disabled,
            TabsKeyboardActivation::Automatic,
            TabsSelectionTrigger::Press
        ),
        0
    );
}

#[test]
fn resolve_tabs_state_tracks_selected_and_disabled_flags() {
    let state = resolve_tabs_state(3, 1, true);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert_eq!(state.selected_index, Some(1));
    assert!(state.has_disabled_tabs);
}

#[test]
fn resolve_tabs_state_clamps_selected_index() {
    let state = resolve_tabs_state(2, 99, false);
    assert!(!state.is_empty);
    assert_eq!(state.selected_index, Some(1));
    assert!(!state.has_disabled_tabs);
}

#[test]
fn resolve_tabs_state_handles_empty_tabs() {
    let state = resolve_tabs_state(0, 0, false);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert_eq!(state.selected_index, None);
    assert!(!state.has_disabled_tabs);
}
