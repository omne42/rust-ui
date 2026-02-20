use super::*;

#[test]
fn resolve_state_tracks_open_and_closed_flags() {
    let state = resolve_state(DisclosureStateInput {
        is_open: true,
        is_disabled: false,
    });
    assert!(state.is_open);
    assert!(!state.is_closed);
    assert!(!state.is_disabled);
}

#[test]
fn resolve_state_tracks_disabled_state() {
    let state = resolve_state(DisclosureStateInput {
        is_open: false,
        is_disabled: true,
    });
    assert!(!state.is_open);
    assert!(state.is_closed);
    assert!(state.is_disabled);
}
