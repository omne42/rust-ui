use super::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Newsletter  ".to_string())),
        Some("Newsletter".to_string())
    );

    assert_eq!(normalize_id_base(None), "ui-checkbox-field");
    assert_eq!(
        normalize_id_base(Some("  docs-checkbox-field  ".to_string())),
        "docs-checkbox-field"
    );

    assert_eq!(
        normalize_label(Some("  Accept terms  ".to_string())),
        ("Accept terms".to_string(), true)
    );
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));

    assert_eq!(
        normalize_aria_label(Some("  Custom aria  ".to_string()), "Ignored"),
        ("Custom aria".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None, "Fallback label"),
        ("Fallback label".to_string(), false)
    );
}

#[test]
fn resolve_state_tracks_state_markers() {
    let state = resolve_state(CheckboxFieldStateInput {
        checked: true,
        disabled: false,
        invalid: true,
        tone: CheckboxFieldTone::Quiet,
        indicator_placement: CheckboxFieldIndicatorPlacement::End,
        has_description: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    assert!(state.is_checked);
    assert!(!state.is_unchecked);
    assert!(state.is_invalid);
    assert!(!state.is_disabled);
    assert_eq!(state.tone_class, "ui-checkbox-field--tone-quiet");
    assert_eq!(state.tone_attr, "quiet");
    assert_eq!(
        state.indicator_placement_class,
        "ui-checkbox-field--indicator-end"
    );
    assert_eq!(state.indicator_placement_attr, "end");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.label_source_attr, "default");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.state_attr, "checked-invalid");
}

#[test]
fn compose_class_name_includes_state_classes() {
    let state = resolve_state(CheckboxFieldStateInput {
        checked: false,
        disabled: true,
        invalid: false,
        tone: CheckboxFieldTone::Default,
        indicator_placement: CheckboxFieldIndicatorPlacement::Start,
        has_description: false,
        has_custom_label: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-checkbox-field".to_string()), state);

    for expected in [
        "ui-checkbox-field",
        "ui-checkbox-field--tone-default",
        "ui-checkbox-field--indicator-start",
        "ui-checkbox-field--unchecked",
        "ui-checkbox-field--disabled",
        "ui-checkbox-field--no-description",
        "ui-checkbox-field--custom-class",
        "docs-checkbox-field",
    ] {
        assert!(class_name.contains(expected));
    }
}
