use super::*;

#[test]
fn ids_are_derived_from_id_base() {
    let ids = DisclosureIds::new("example");
    assert_eq!(ids.trigger_id, "example-trigger");
    assert_eq!(ids.panel_id, "example-panel");
}

#[test]
fn resolve_state_tracks_open_and_closed_flags() {
    let state = resolve_state(true, false);
    assert!(state.is_open);
    assert!(!state.is_closed);
    assert!(!state.is_disabled);
}

#[test]
fn resolve_state_tracks_disabled_state() {
    let state = resolve_state(false, true);
    assert!(!state.is_open);
    assert!(state.is_closed);
    assert!(state.is_disabled);
}
