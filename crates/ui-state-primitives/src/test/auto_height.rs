use super::*;

#[test]
fn default_state_is_overflow_hidden() {
    let state = resolve_state(AutoHeightStateInput {
        animate_height: true,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    assert!(state.overflow_hidden);
    assert!(state.animate_height);
    assert!(!state.is_static);
    assert!(!state.has_custom_class_name);
    assert!(!state.has_custom_motion);
}

#[test]
fn resolve_state_tracks_static_and_custom_flags() {
    let state = resolve_state(AutoHeightStateInput {
        animate_height: false,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert!(state.overflow_hidden);
    assert!(!state.animate_height);
    assert!(state.is_static);
    assert!(state.has_custom_class_name);
    assert!(state.has_custom_motion);
}
