use super::*;
use ui_state_primitives::labeled_value::{
    LabeledValueOrientation, LabeledValueStateInput, LabeledValueTone, resolve_state,
};

#[test]
fn use_labeled_value_maps_locale_and_state_markers() {
    let state = resolve_state(LabeledValueStateInput {
        orientation: LabeledValueOrientation::Inline,
        tone: LabeledValueTone::Strong,
        has_custom_label: true,
        has_custom_value: false,
        has_description: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_labeled_value(LabeledValueOptions {
        state,
        aria_label: " Build status ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "group");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_orientation, "inline");
    assert_eq!(contract.attrs.data_tone, "strong");
    assert_eq!(contract.attrs.data_state, "with-description");
    assert_eq!(contract.attrs.data_has_description, Some("true"));
    assert_eq!(contract.attrs.data_label_source, "custom");
    assert_eq!(contract.attrs.data_value_source, "default");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_labeled_value_omits_optional_markers_for_defaults() {
    let state = resolve_state(LabeledValueStateInput {
        orientation: LabeledValueOrientation::Stacked,
        tone: LabeledValueTone::Default,
        has_custom_label: false,
        has_custom_value: false,
        has_description: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_labeled_value(LabeledValueOptions {
        state,
        aria_label: "Status".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.data_state, "default");
    assert_eq!(contract.attrs.data_has_description, None);
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
}
