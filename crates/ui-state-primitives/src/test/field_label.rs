use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(
        FieldLabelTone::Default.class_name(),
        "ui-field-label--tone-default"
    );
    assert_eq!(
        FieldLabelTone::Muted.class_name(),
        "ui-field-label--tone-muted"
    );
    assert_eq!(
        FieldLabelTone::Strong.class_name(),
        "ui-field-label--tone-strong"
    );

    assert_eq!(FieldLabelTone::Default.as_attr(), "default");
    assert_eq!(FieldLabelTone::Muted.as_attr(), "muted");
    assert_eq!(FieldLabelTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_helpers_use_trimmed_custom_values_or_defaults() {
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Project owner  ".to_string())),
        Some("Project owner".to_string())
    );

    assert_eq!(
        normalize_text(Some("  Team  ".to_string())),
        ("Team".to_string(), true)
    );
    assert_eq!(normalize_text(None), (DEFAULT_TEXT.into(), false));

    assert_eq!(
        normalize_required_indicator(Some("  (required)  ".to_string())),
        ("(required)".to_string(), true)
    );
    assert_eq!(
        normalize_required_indicator(None),
        (DEFAULT_REQUIRED_INDICATOR.into(), false)
    );

    assert_eq!(
        normalize_aria_label(Some("  Field heading  ".to_string())),
        ("Field heading".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_visibility_and_source_markers() {
    let state = resolve_state(FieldLabelStateInput {
        tone: FieldLabelTone::Strong,
        required: true,
        disabled: true,
        has_for_id: true,
        has_custom_text: true,
        has_custom_indicator: false,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(state.has_for_id);
    assert_eq!(state.text_source_attr, "custom");
    assert_eq!(state.indicator_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}
