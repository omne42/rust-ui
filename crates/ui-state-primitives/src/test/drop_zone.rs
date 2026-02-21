use super::*;

#[test]
fn drag_depth_tracks_nested_drag_enter_leave_without_flicker() {
    let state = DragDepth::default();
    assert!(!state.is_active());

    let state = state.enter();
    assert!(state.is_active());

    let state = state.enter();
    assert!(state.is_active());

    let state = state.leave();
    assert!(state.is_active());

    let state = state.leave();
    assert!(!state.is_active());

    let state = state.leave();
    assert!(!state.is_active());
}

#[test]
fn drag_depth_reset_always_clears() {
    let state = DragDepth::default().enter().enter();
    assert!(state.is_active());

    let state = state.reset();
    assert!(!state.is_active());
}

#[test]
fn resolve_labels_trims_and_falls_back_to_defaults() {
    let labels = resolve_labels(
        Some("  Upload files  ".to_string()),
        Some("  Upload area  ".to_string()),
    );
    assert_eq!(labels.label, Some("Upload files".to_string()));
    assert_eq!(labels.aria_label, "Upload area");
    assert!(labels.has_custom_aria_label);

    let labels = resolve_labels(Some("  Upload files  ".to_string()), Some("  ".to_string()));
    assert_eq!(labels.label, Some("Upload files".to_string()));
    assert_eq!(labels.aria_label, "Upload files");
    assert!(labels.has_custom_aria_label);

    let labels = resolve_labels(Some("  ".to_string()), Some("  ".to_string()));
    assert_eq!(labels.label, None);
    assert_eq!(labels.aria_label, DEFAULT_ARIA_LABEL);
    assert!(!labels.has_custom_aria_label);
}
