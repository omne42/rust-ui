use super::*;

#[test]
fn resolve_props_normalizes_text_and_tracks_sources() {
    let resolved = resolve_props(
        "  Name  ".to_string(),
        Some("  description  ".to_string()),
        Some("  error  ".to_string()),
        Some("  placeholder  ".to_string()),
        Some("email"),
        Some("  docs-class  ".to_string()),
    );

    assert_eq!(resolved.label, "Name");
    assert_eq!(resolved.label_source_attr, "custom");
    assert_eq!(resolved.description.as_deref(), Some("description"));
    assert_eq!(resolved.error.as_deref(), Some("error"));
    assert_eq!(resolved.placeholder.as_deref(), Some("placeholder"));
    assert_eq!(resolved.input_type, TextFieldInputType::Email);
    assert_eq!(resolved.type_source_attr, "custom");
    assert!(resolved.has_custom_class_name);
    assert_eq!(resolved.class, "ui-text-field docs-class");
    assert_eq!(resolved.description_source_attr, "custom");
    assert_eq!(resolved.error_source_attr, "custom");
    assert_eq!(resolved.placeholder_source_attr, "custom");
    assert_eq!(resolved.class_source_attr, "custom");
}

#[test]
fn resolve_props_applies_defaults_for_blank_inputs() {
    let resolved = resolve_props(
        "   ".to_string(),
        Some("   ".to_string()),
        Some("\n\t".to_string()),
        None,
        None,
        Some("   ".to_string()),
    );

    assert_eq!(resolved.label, DEFAULT_LABEL);
    assert_eq!(resolved.label_source_attr, "default");
    assert_eq!(resolved.description, None);
    assert_eq!(resolved.error, None);
    assert_eq!(resolved.placeholder, None);
    assert_eq!(resolved.input_type, TextFieldInputType::Text);
    assert_eq!(resolved.type_source_attr, "default");
    assert!(!resolved.has_custom_class_name);
    assert_eq!(resolved.class, "ui-text-field");
    assert_eq!(resolved.description_source_attr, "default");
    assert_eq!(resolved.error_source_attr, "default");
    assert_eq!(resolved.placeholder_source_attr, "default");
    assert_eq!(resolved.class_source_attr, "default");
}

#[test]
fn normalize_default_value_uses_empty_string_when_absent() {
    assert_eq!(normalize_default_value(None), String::new());
    assert_eq!(
        normalize_default_value(Some("prefilled".to_string())),
        "prefilled".to_string()
    );
}

#[test]
fn normalize_value_axis_tracks_mode_and_source_markers() {
    let state = normalize_value_axis(ValueAxisInput {
        has_controlled_value: true,
        default_value: Some("fallback".to_string()),
        has_on_value_change: false,
    });

    assert!(state.is_controlled);
    assert_eq!(state.control_mode_attr, "controlled");
    assert_eq!(state.default_value, "fallback");
    assert_eq!(state.default_value_source_attr, "custom");
    assert_eq!(state.value_change_source_attr, "none");
    assert!(!state.has_value_change_handler);
}

#[test]
fn normalize_value_axis_tracks_on_value_change_source() {
    let state = normalize_value_axis(ValueAxisInput {
        has_controlled_value: false,
        default_value: None,
        has_on_value_change: true,
    });

    assert_eq!(state.control_mode_attr, "uncontrolled");
    assert_eq!(state.default_value_source_attr, "default");
    assert_eq!(state.value_change_source_attr, "on_value_change");
    assert!(state.has_value_change_handler);
}

#[test]
fn value_axis_enum_attrs_are_closed_machine_readable_values() {
    assert_eq!(ValueControlMode::Controlled.as_attr(), "controlled");
    assert_eq!(ValueControlMode::Uncontrolled.as_attr(), "uncontrolled");
    assert_eq!(
        ValueChangeSource::OnValueChange.as_attr(),
        "on_value_change"
    );
    assert_eq!(ValueChangeSource::None.as_attr(), "none");
}

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(true),
        is_read_only: Some(true),
    });

    assert!(state.is_disabled);
    assert!(state.is_read_only);
}

#[test]
fn normalize_accessibility_state_uses_defaults_when_values_are_absent() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        is_read_only: None,
    });

    assert!(!state.is_disabled);
    assert!(!state.is_read_only);
}

#[test]
fn text_field_agent_contract_is_typed_and_stable() {
    let contract = text_field_agent_contract();

    assert_eq!(contract.schema_attr, "ui.text-field");
    assert_eq!(
        contract.schema_version_attr,
        TextFieldAgentSchemaVersion::V1.as_attr()
    );
    assert_eq!(
        contract.intent_attr,
        TextFieldAgentIntent::FormTextInput.as_attr()
    );
    assert_eq!(
        contract.action_model_attr,
        TextFieldAgentActionModel::InputFocusBlurValidate.as_attr()
    );
    assert_eq!(
        contract.state_axis_attr,
        "state|value|requirement|disabled|readonly|focus-visible"
    );
    assert_eq!(
        contract.source_axis_attr,
        "label|description|error|placeholder|type|class|motion|value-axis"
    );
}

#[test]
fn normalize_input_type_maps_compat_strings_to_enum() {
    let default_type = normalize_input_type(None);
    assert_eq!(default_type.input_type, TextFieldInputType::Text);
    assert_eq!(default_type.type_source_attr, "default");

    let email_type = normalize_input_type(Some("email"));
    assert_eq!(email_type.input_type, TextFieldInputType::Email);
    assert_eq!(email_type.type_source_attr, "custom");

    let custom_type = normalize_input_type(Some("datetime-local"));
    assert_eq!(
        custom_type.input_type,
        TextFieldInputType::Custom("datetime-local")
    );
    assert_eq!(custom_type.type_source_attr, "custom");
}
