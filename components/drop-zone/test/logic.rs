use super::*;

#[test]
fn dropped_file_is_send_sync_friendly() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DroppedFile>();
}

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
    let (label, aria_label, has_custom_aria_label) = resolve_labels(
        Some("  Upload files  ".to_string()),
        Some("  Upload area  ".to_string()),
    );
    assert_eq!(label, Some("Upload files".to_string()));
    assert_eq!(aria_label, "Upload area");
    assert!(has_custom_aria_label);

    let (label, aria_label, has_custom_aria_label) =
        resolve_labels(Some("  Upload files  ".to_string()), Some("  ".to_string()));
    assert_eq!(label, Some("Upload files".to_string()));
    assert_eq!(aria_label, "Upload files");
    assert!(has_custom_aria_label);

    let (label, aria_label, has_custom_aria_label) =
        resolve_labels(Some("  ".to_string()), Some("  ".to_string()));
    assert_eq!(label, None);
    assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
    assert!(!has_custom_aria_label);
}
