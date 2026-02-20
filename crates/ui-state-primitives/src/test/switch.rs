use super::*;

#[test]
fn resolve_state_tracks_checked_enabled_interactions() {
    let state = resolve_state(SwitchStateInput {
        is_checked: true,
        is_disabled: false,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
    });

    assert!(state.is_checked);
    assert!(!state.is_unchecked);
    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(state.is_pressed);
    assert!(state.is_hovered);
    assert!(state.is_focused);
    assert!(state.is_focus_visible);
    assert_eq!(state.data_state(), "checked");
}

#[test]
fn resolve_state_clears_interaction_flags_when_disabled() {
    let state = resolve_state(SwitchStateInput {
        is_checked: false,
        is_disabled: true,
        is_pressed: true,
        is_hovered: true,
        is_focused: true,
        is_focus_visible: true,
    });

    assert!(!state.is_checked);
    assert!(state.is_unchecked);
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(!state.is_pressed);
    assert!(!state.is_hovered);
    assert!(!state.is_focused);
    assert!(!state.is_focus_visible);
    assert_eq!(state.data_state(), "unchecked");
}
