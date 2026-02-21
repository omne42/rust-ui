use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-cp  ".to_string())),
        Some("docs-cp".to_string())
    );
}

#[test]
fn resolve_aria_label_defaults_and_detects_custom_source() {
    assert_eq!(
        resolve_aria_label(None, DEFAULT_ARIA_LABEL),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  ".to_string()), DEFAULT_ARIA_LABEL),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("Loading".to_string()), DEFAULT_ARIA_LABEL),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
    assert_eq!(
        resolve_aria_label(Some("  Syncing mailbox ".to_string()), DEFAULT_ARIA_LABEL),
        ("Syncing mailbox".to_string(), true)
    );
}

#[test]
fn sanitize_dimension_rejects_invalid_values() {
    assert_eq!(sanitize_dimension(None), None);
    assert_eq!(sanitize_dimension(Some(-1.0)), None);
    assert_eq!(sanitize_dimension(Some(0.0)), None);
    assert_eq!(sanitize_dimension(Some(f64::NAN)), None);
    assert_eq!(sanitize_dimension(Some(f64::INFINITY)), None);
    assert_eq!(sanitize_dimension(Some(24.0)), Some(24.0));
}

#[test]
fn resolve_state_tracks_source_flags() {
    let state = resolve_state(CircularProgressStateInput {
        size_px: Some(24.0),
        thickness_px: Some(3.0),
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.size_px, Some(24.0));
    assert_eq!(state.thickness_px, Some(3.0));
    assert!(state.has_custom_size);
    assert!(state.has_custom_thickness);
    assert!(state.has_custom_aria_label);
    assert!(state.has_custom_class_name);
    assert_eq!(state.size_source_attr, "custom");
    assert_eq!(state.thickness_source_attr, "custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_sanitizes_dimension_inputs() {
    let state = resolve_state(CircularProgressStateInput {
        size_px: Some(-4.0),
        thickness_px: Some(f64::NAN),
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.size_px, None);
    assert_eq!(state.thickness_px, None);
    assert!(!state.has_custom_size);
    assert!(!state.has_custom_thickness);
    assert_eq!(state.size_source_attr, "default");
    assert_eq!(state.thickness_source_attr, "default");
}
