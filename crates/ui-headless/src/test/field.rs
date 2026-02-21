use super::*;
use crate::A11yDirection;
use ui_state_primitives::field::{FieldOrientation, FieldStateInput, FieldTone, resolve_state};

#[test]
fn use_field_maps_locale_aria_and_data_contracts() {
    let state = resolve_state(FieldStateInput {
        orientation: FieldOrientation::Horizontal,
        tone: FieldTone::Muted,
        required: true,
        disabled: true,
        invalid: true,
        has_label: true,
        has_description: true,
        has_error_message: true,
        has_custom_aria_label: true,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    let contract = use_field(FieldOptions {
        state,
        aria_label: "Billing Field".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.aria_label, "Billing Field");
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.aria_invalid, Some("true"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_orientation, "horizontal");
    assert_eq!(contract.attrs.data_tone, "muted");
    assert_eq!(contract.attrs.data_state, "invalid-disabled");
    assert_eq!(contract.attrs.data_message_kind, "error");
    assert_eq!(contract.attrs.data_required, Some("true"));
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_error_source, "default");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_field_omits_optional_markers_for_default_state() {
    let state = resolve_state(FieldStateInput {
        orientation: FieldOrientation::Vertical,
        tone: FieldTone::Default,
        required: false,
        disabled: false,
        invalid: false,
        has_label: false,
        has_description: false,
        has_error_message: false,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    let contract = use_field(FieldOptions {
        state,
        aria_label: "Field".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.aria_invalid, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.attrs.data_state, "default");
    assert_eq!(contract.attrs.data_required, None);
    assert_eq!(contract.attrs.data_has_label, None);
    assert_eq!(contract.attrs.data_has_description, None);
    assert_eq!(contract.attrs.data_has_error, None);
    assert_eq!(contract.attrs.data_custom_class, None);
}
