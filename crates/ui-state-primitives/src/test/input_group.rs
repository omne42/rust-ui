use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-input-group ".to_string())),
        Some("docs-input-group".to_string())
    );
}

#[test]
fn normalize_aria_label_uses_trimmed_label_or_fallback() {
    let (label, explicit) = normalize_aria_label(Some("  Query controls  ".to_string()));
    assert_eq!(label, "Query controls");
    assert!(explicit);

    let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!explicit);

    let (label, explicit) = normalize_aria_label(None);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!explicit);
}

#[test]
fn resolve_state_tracks_phase_attachment_and_source_markers() {
    let state = resolve_state(InputGroupStateInput {
        disabled: true,
        invalid: true,
        attached: false,
        has_start_content: true,
        has_end_content: false,
        has_custom_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.phase_class, "ui-input-group--state-disabled");
    assert_eq!(state.phase_attr, "disabled");
    assert!(state.is_disabled);
    assert!(!state.is_enabled);

    assert_eq!(state.attachment_class, "ui-input-group--detached");
    assert_eq!(state.attachment_attr, "detached");
    assert!(state.is_detached);
    assert!(!state.is_attached);

    assert!(state.is_invalid);
    assert!(state.has_start_content);
    assert!(!state.has_end_content);
    assert_eq!(state.label_source_class, "ui-input-group--label-custom");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}
