use super::*;

#[test]
fn tone_and_indicator_contracts_are_stable() {
    assert_eq!(
        CheckboxFieldTone::Default.class_name(),
        "ui-checkbox-field--tone-default"
    );
    assert_eq!(
        CheckboxFieldTone::Quiet.class_name(),
        "ui-checkbox-field--tone-quiet"
    );
    assert_eq!(CheckboxFieldTone::Default.as_attr(), "default");
    assert_eq!(CheckboxFieldTone::Quiet.as_attr(), "quiet");

    assert_eq!(
        CheckboxFieldIndicatorPlacement::Start.class_name(),
        "ui-checkbox-field--indicator-start"
    );
    assert_eq!(
        CheckboxFieldIndicatorPlacement::End.class_name(),
        "ui-checkbox-field--indicator-end"
    );
    assert_eq!(CheckboxFieldIndicatorPlacement::Start.as_attr(), "start");
    assert_eq!(CheckboxFieldIndicatorPlacement::End.as_attr(), "end");

    assert_eq!(CheckboxFieldStatus::Unchecked.as_attr(), "unchecked");
    assert_eq!(CheckboxFieldStatus::Checked.as_attr(), "checked");
    assert_eq!(CheckboxFieldStatus::Disabled.as_attr(), "disabled");
    assert_eq!(CheckboxFieldStatus::Invalid.as_attr(), "invalid");
    assert_eq!(
        CheckboxFieldStatus::CheckedInvalid.as_attr(),
        "checked-invalid"
    );
}

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

    assert_eq!(state.status, CheckboxFieldStatus::CheckedInvalid);
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
fn resolve_status_models_mutually_exclusive_state_with_enum() {
    assert_eq!(
        resolve_status(false, false, false),
        CheckboxFieldStatus::Unchecked
    );
    assert_eq!(
        resolve_status(true, false, false),
        CheckboxFieldStatus::Checked
    );
    assert_eq!(
        resolve_status(false, true, false),
        CheckboxFieldStatus::Disabled
    );
    assert_eq!(
        resolve_status(false, false, true),
        CheckboxFieldStatus::Invalid
    );
    assert_eq!(
        resolve_status(true, false, true),
        CheckboxFieldStatus::CheckedInvalid
    );
}
