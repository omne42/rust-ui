use super::*;

#[test]
fn normalize_props_centralizes_default_priority_and_source_flags() {
    let normalized = normalize_props(
        Some("  Team  ".to_string()),
        Some("  ".to_string()),
        None,
        Some("  field-id  ".to_string()),
        Some("  docs-field-label  ".to_string()),
    );

    assert_eq!(normalized.text, "Team");
    assert_eq!(normalized.required_indicator, DEFAULT_REQUIRED_INDICATOR);
    assert_eq!(normalized.aria_label, DEFAULT_ARIA_LABEL);
    assert_eq!(normalized.for_id.as_deref(), Some("field-id"));
    assert_eq!(normalized.class_name.as_deref(), Some("docs-field-label"));
    assert!(normalized.has_for_id);
    assert!(normalized.has_custom_text);
    assert!(!normalized.has_custom_indicator);
    assert!(!normalized.has_custom_aria_label);
    assert!(normalized.has_custom_class_name);
}

#[test]
fn derive_view_model_concentrates_state_derivation_in_logic() {
    let view_model = derive_view_model(
        FieldLabelLogicInput {
            tone: FieldLabelTone::Strong,
            is_required: true,
            is_disabled: true,
        },
        Some("  Assignee  ".to_string()),
        None,
        Some("  Assignee field label  ".to_string()),
        Some("  assignee-id  ".to_string()),
        Some("  docs-field-label  ".to_string()),
    );

    assert_eq!(view_model.text, "Assignee");
    assert_eq!(view_model.required_indicator, DEFAULT_REQUIRED_INDICATOR);
    assert_eq!(view_model.aria_label, "Assignee field label");
    assert_eq!(view_model.for_id.as_deref(), Some("assignee-id"));
    assert_eq!(view_model.class_name.as_deref(), Some("docs-field-label"));
    assert_eq!(view_model.state.tone, FieldLabelTone::Strong);
    assert!(view_model.state.is_required);
    assert!(view_model.state.is_disabled);
    assert_eq!(view_model.state.text_source_attr, "custom");
    assert_eq!(view_model.state.indicator_source_attr, "default");
    assert_eq!(view_model.state.aria_source_attr, "custom");
    assert_eq!(view_model.state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let state = resolve_state(FieldLabelStateInput {
        tone: FieldLabelTone::Muted,
        required: true,
        disabled: true,
        has_for_id: true,
        has_custom_text: true,
        has_custom_indicator: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-field-label-custom".to_string()), state);

    for token in [
        "ui-field-label",
        "ui-field-label--tone-muted",
        "ui-field-label--required",
        "ui-field-label--disabled",
        "ui-field-label--for",
        "ui-field-label--text-custom",
        "ui-field-label--indicator-custom",
        "ui-field-label--aria-custom",
        "ui-field-label--custom-class",
        "docs-field-label-custom",
    ] {
        assert!(class_name.contains(token), "class should contain `{token}`");
    }
}
