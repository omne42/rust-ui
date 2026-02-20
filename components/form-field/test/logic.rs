use super::*;

#[test]
fn enums_expose_stable_class_and_attr_names() {
    assert_eq!(
        FormFieldTone::Default.class_name(),
        "ui-form-field--tone-default"
    );
    assert_eq!(
        FormFieldTone::Quiet.class_name(),
        "ui-form-field--tone-quiet"
    );
    assert_eq!(FormFieldTone::Default.as_attr(), "default");
    assert_eq!(FormFieldTone::Quiet.as_attr(), "quiet");

    assert_eq!(
        FormFieldIndicatorVariant::Switch.class_name(),
        "ui-form-field--indicator-switch"
    );
    assert_eq!(
        FormFieldIndicatorVariant::Checkbox.class_name(),
        "ui-form-field--indicator-checkbox"
    );

    assert_eq!(
        FormFieldIndicatorPlacement::Start.class_name(),
        "ui-form-field--placement-start"
    );
    assert_eq!(
        FormFieldIndicatorPlacement::End.class_name(),
        "ui-form-field--placement-end"
    );
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Enable alerts  ".to_string())),
        Some("Enable alerts".to_string())
    );

    assert_eq!(normalize_id_base(None), "ui-form-field");
    assert_eq!(
        normalize_id_base(Some(" docs-form-field ".to_string())),
        "docs-form-field"
    );

    assert_eq!(
        normalize_label(Some("  Notifications  ".to_string())),
        ("Notifications".to_string(), true)
    );
    assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));

    assert_eq!(
        normalize_aria_label(Some("  Custom aria  ".to_string()), "fallback"),
        ("Custom aria".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(None, "Fallback label"),
        ("Fallback label".to_string(), false)
    );

    assert_eq!(
        normalize_error_message(Some("  Missing choice  ".to_string()), true),
        (Some("Missing choice".to_string()), true)
    );
    assert_eq!(
        normalize_error_message(None, true),
        (Some(DEFAULT_ERROR_MESSAGE.into()), false)
    );
    assert_eq!(normalize_error_message(None, false), (None, false));
}

#[test]
fn resolve_state_tracks_variant_placement_and_messages() {
    let state = resolve_state(FormFieldStateInput {
        selected: true,
        disabled: false,
        invalid: true,
        tone: FormFieldTone::Quiet,
        indicator_variant: FormFieldIndicatorVariant::Checkbox,
        indicator_placement: FormFieldIndicatorPlacement::Start,
        has_description: true,
        has_error_message: true,
        has_custom_label: false,
        has_custom_aria_label: false,
        has_custom_error_message: true,
        has_custom_class_name: true,
    });

    assert!(state.is_selected);
    assert!(!state.is_unselected);
    assert!(state.is_invalid);
    assert_eq!(state.tone_attr, "quiet");
    assert_eq!(state.indicator_variant_attr, "checkbox");
    assert_eq!(state.indicator_placement_attr, "start");
    assert!(state.shows_error);
    assert_eq!(state.message_kind_attr, "error");
    assert_eq!(state.state_attr, "selected-invalid");
    assert_eq!(state.label_source_attr, "default");
    assert_eq!(state.aria_source_attr, "default");
    assert_eq!(state.error_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(FormFieldStateInput {
        selected: false,
        disabled: true,
        invalid: false,
        tone: FormFieldTone::Default,
        indicator_variant: FormFieldIndicatorVariant::Switch,
        indicator_placement: FormFieldIndicatorPlacement::End,
        has_description: false,
        has_error_message: false,
        has_custom_label: true,
        has_custom_aria_label: true,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-form-field".to_string()), state);

    for expected in [
        "ui-form-field",
        "ui-form-field--tone-default",
        "ui-form-field--indicator-switch",
        "ui-form-field--placement-end",
        "ui-form-field--unselected",
        "ui-form-field--disabled",
        "ui-form-field--custom-class",
        "docs-form-field",
    ] {
        assert!(class_name.contains(expected));
    }
}
