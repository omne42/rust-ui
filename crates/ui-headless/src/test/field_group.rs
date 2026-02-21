use super::*;
use crate::A11yDirection;
use ui_state_primitives::field_group::{
    FieldGroupDensity, FieldGroupOrientation, FieldGroupStateInput, resolve_state,
};

#[test]
fn use_field_group_prefers_labelledby_when_label_is_present_and_custom_aria_is_absent() {
    let state = resolve_state(FieldGroupStateInput {
        orientation: FieldGroupOrientation::Horizontal,
        density: FieldGroupDensity::Compact,
        disabled: true,
        invalid: true,
        has_label: true,
        has_description: true,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let contract = use_field_group(FieldGroupOptions {
        state,
        aria_label: "Billing controls".to_string(),
        label_id: Some("  billing-label  ".to_string()),
        description_id: Some("billing-description".to_string()),
        lang: Some("  ar ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "group");
    assert_eq!(contract.attrs.aria_label, None);
    assert_eq!(
        contract.attrs.aria_labelledby.as_deref(),
        Some("billing-label")
    );
    assert_eq!(
        contract.attrs.aria_describedby.as_deref(),
        Some("billing-description")
    );
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.aria_invalid, Some("true"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("ar"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_orientation, "horizontal");
    assert_eq!(contract.attrs.data_density, "compact");
    assert_eq!(contract.attrs.data_state, "invalid-disabled");
    assert_eq!(contract.attrs.data_label, "present");
    assert_eq!(contract.attrs.data_description, "present");
    assert_eq!(contract.attrs.data_aria_source, "label");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_field_group_keeps_aria_label_when_custom_aria_is_enabled() {
    let state = resolve_state(FieldGroupStateInput {
        orientation: FieldGroupOrientation::Vertical,
        density: FieldGroupDensity::Comfortable,
        disabled: false,
        invalid: false,
        has_label: true,
        has_description: false,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    let contract = use_field_group(FieldGroupOptions {
        state,
        aria_label: "Custom field cluster".to_string(),
        label_id: Some("account-label".to_string()),
        description_id: Some("  ".to_string()),
        lang: None,
        dir: None,
    });

    assert_eq!(
        contract.attrs.aria_label.as_deref(),
        Some("Custom field cluster")
    );
    assert_eq!(contract.attrs.aria_labelledby, None);
    assert_eq!(contract.attrs.aria_describedby, None);
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.aria_invalid, None);
    assert_eq!(contract.attrs.data_orientation, "vertical");
    assert_eq!(contract.attrs.data_density, "comfortable");
    assert_eq!(contract.attrs.data_state, "default");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
}
