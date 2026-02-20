use super::*;

#[test]
fn orientation_and_tone_contracts_are_stable() {
    assert_eq!(
        LabeledValueOrientation::Stacked.class_name(),
        "ui-labeled-value--orientation-stacked"
    );
    assert_eq!(LabeledValueOrientation::Inline.as_attr(), "inline");
    assert_eq!(
        LabeledValueTone::Default.class_name(),
        "ui-labeled-value--tone-default"
    );
    assert_eq!(LabeledValueTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(
        normalize_label_text(Some("  Status  ".to_string())),
        ("Status".to_string(), true)
    );
    assert_eq!(
        normalize_value_text(None),
        (DEFAULT_VALUE_TEXT.into(), false)
    );
    assert_eq!(
        normalize_aria_label(Some(" ".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn resolve_state_tracks_sources_and_layout() {
    let state = resolve_state(LabeledValueStateInput {
        orientation: LabeledValueOrientation::Inline,
        tone: LabeledValueTone::Strong,
        has_custom_label: true,
        has_custom_value: false,
        has_description: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.orientation_attr, "inline");
    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.value_source_attr, "default");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert!(state.has_description);
}
