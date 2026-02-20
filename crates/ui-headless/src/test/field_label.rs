use super::*;
use ui_state_primitives::field_label::{FieldLabelStateInput, FieldLabelTone, resolve_state};

#[test]
fn use_field_label_maps_locale_and_semantic_attrs() {
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

    let contract = use_field_label(FieldLabelOptions {
        state,
        aria_label: " Assignee field label ".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.aria_label, " Assignee field label ");
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_tone, "strong");
    assert_eq!(contract.attrs.data_state, "required");
    assert_eq!(contract.attrs.data_required, Some("true"));
    assert_eq!(contract.attrs.data_disabled, Some("true"));
    assert_eq!(contract.attrs.data_has_for, Some("true"));
    assert_eq!(contract.attrs.data_text_source, "custom");
    assert_eq!(contract.attrs.data_indicator_source, "default");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_field_label_omits_optional_markers_for_default_optional_case() {
    let state = resolve_state(FieldLabelStateInput {
        tone: FieldLabelTone::Default,
        required: false,
        disabled: false,
        has_for_id: false,
        has_custom_text: false,
        has_custom_indicator: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_field_label(FieldLabelOptions {
        state,
        aria_label: "Field label".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.data_tone, "default");
    assert_eq!(contract.attrs.data_state, "optional");
    assert_eq!(contract.attrs.data_required, None);
    assert_eq!(contract.attrs.data_disabled, None);
    assert_eq!(contract.attrs.data_has_for, None);
    assert_eq!(contract.attrs.data_text_source, "default");
    assert_eq!(contract.attrs.data_indicator_source, "default");
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
}
