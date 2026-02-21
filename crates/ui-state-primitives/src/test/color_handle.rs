use super::*;

#[test]
fn sanitize_color_rejects_unsafe_values() {
    assert_eq!(
        sanitize_color(Some(" #09f ".to_string())),
        Some("#09f".to_string())
    );
    assert_eq!(
        sanitize_color(Some("javascript:alert(1)".to_string())),
        None
    );
}

#[test]
fn normalize_aria_label_uses_default_or_custom_values() {
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some("  Accent handle  ".to_string())),
        ("Accent handle".to_string(), true)
    );
}

#[test]
fn resolve_state_tracks_flags_and_sources() {
    let state = resolve_state(ColorHandleStateInput {
        is_disabled: false,
        is_focused: true,
        is_dragging: true,
        is_loupe_visible: true,
        has_color: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.data_state_attr, "dragging");
    assert!(state.loupe_visible);
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}
