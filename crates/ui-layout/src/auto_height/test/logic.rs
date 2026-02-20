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
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-auto-height  ".to_string())),
        Some("docs-auto-height".to_string())
    );
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

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_state(AutoHeightStateInput {
            animate_height: false,
            has_custom_class_name: true,
            has_custom_motion: true,
        }),
    );

    for token in [
        "ui-auto-height",
        "ui-auto-height--static",
        "ui-auto-height--custom-motion",
        "ui-auto-height--custom-class",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
