use super::*;
use leptos::prelude::{Callback, signal};

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
fn normalize_selected_axis_keeps_controlled_uncontrolled_triplet_contract() {
    let (controlled, _set_controlled) = signal(true);
    let controlled_axis = normalize_selected_axis(FormFieldSelectedAxisInput {
        is_selected: Some(controlled.into()),
        default_selected: Some(false),
        on_selected_change: Some(Callback::new(|_| {})),
    });

    assert!(controlled_axis.is_controlled);
    assert_eq!(controlled_axis.control_mode_attr, "controlled");
    assert!(!controlled_axis.default_selected);
    assert_eq!(controlled_axis.default_selected_source_attr, "provided");
    assert_eq!(
        controlled_axis.selected_change_source_attr,
        "on_selected_change"
    );

    let uncontrolled_axis = normalize_selected_axis(FormFieldSelectedAxisInput {
        is_selected: None,
        default_selected: None,
        on_selected_change: None,
    });

    assert!(!uncontrolled_axis.is_controlled);
    assert_eq!(uncontrolled_axis.control_mode_attr, "uncontrolled");
    assert_eq!(uncontrolled_axis.default_selected, DEFAULT_SELECTED);
    assert_eq!(uncontrolled_axis.default_selected_source_attr, "default");
    assert_eq!(uncontrolled_axis.selected_change_source_attr, "none");
}

#[test]
fn logic_helpers_keep_state_derivation_in_single_place() {
    assert!(matches!(
        resolve_checkbox_variant(true),
        ui_checkbox::CheckboxVariant::Accent
    ));
    assert!(matches!(
        resolve_checkbox_variant(false),
        ui_checkbox::CheckboxVariant::Default
    ));

    assert_eq!(
        compose_describedby(
            true,
            true,
            "field-description".to_string(),
            "field-error".to_string()
        ),
        Some("field-description field-error".to_string())
    );
    assert_eq!(
        compose_describedby(
            true,
            false,
            "field-description".to_string(),
            "field-error".to_string()
        ),
        Some("field-description".to_string())
    );
    assert_eq!(
        compose_describedby(
            false,
            true,
            "field-description".to_string(),
            "field-error".to_string()
        ),
        Some("field-error".to_string())
    );
    assert_eq!(
        compose_describedby(
            false,
            false,
            "field-description".to_string(),
            "field-error".to_string()
        ),
        None
    );
}

#[test]
fn resolve_state_attr_stays_in_closed_discrete_set() {
    let mut seen = std::collections::BTreeSet::new();

    for is_selected in [false, true] {
        for is_disabled in [false, true] {
            for is_invalid in [false, true] {
                let state = resolve_state(FormFieldStateInput {
                    is_selected,
                    is_disabled,
                    is_invalid,
                    tone: FormFieldTone::Default,
                    indicator_variant: FormFieldIndicatorVariant::Switch,
                    indicator_placement: FormFieldIndicatorPlacement::End,
                    has_description: false,
                    has_error_message: false,
                    has_custom_label: false,
                    has_custom_aria_label: false,
                    has_custom_error_message: false,
                    has_custom_class_name: false,
                });
                seen.insert(state.state_attr);
            }
        }
    }

    let expected = std::collections::BTreeSet::from([
        "unselected",
        "selected",
        "disabled",
        "selected-disabled",
        "invalid",
        "selected-invalid",
        "invalid-disabled",
    ]);

    assert_eq!(seen, expected);
}

#[test]
fn resolve_state_tracks_variant_placement_and_messages() {
    let state = resolve_state(FormFieldStateInput {
        is_selected: true,
        is_disabled: false,
        is_invalid: true,
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
        is_selected: false,
        is_disabled: true,
        is_invalid: false,
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
