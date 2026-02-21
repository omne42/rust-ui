use super::*;

#[test]
fn compose_class_name_appends_state_and_custom_class() {
    let state = resolve_state(FieldsetStateInput {
        orientation: FieldsetOrientation::Vertical,
        tone: FieldsetTone::Default,
        required: true,
        disabled: true,
        invalid: false,
        has_legend: true,
        has_description: true,
        has_error_message: false,
        has_actions: true,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-fieldset-custom".to_string()), state);

    for expected in [
        "ui-fieldset",
        "ui-fieldset--orientation-vertical",
        "ui-fieldset--tone-default",
        "ui-fieldset--required",
        "ui-fieldset--disabled",
        "ui-fieldset--has-legend",
        "ui-fieldset--has-description",
        "ui-fieldset--has-actions",
        "ui-fieldset--custom-class",
        "docs-fieldset-custom",
    ] {
        assert!(
            class_name.contains(expected),
            "expected class `{expected}` in `{class_name}`"
        );
    }
}

#[test]
fn resolve_agent_contract_emits_machine_readable_markers() {
    let state = resolve_state(FieldsetStateInput {
        orientation: FieldsetOrientation::Horizontal,
        tone: FieldsetTone::Muted,
        required: false,
        disabled: false,
        invalid: true,
        has_legend: true,
        has_description: false,
        has_error_message: true,
        has_actions: false,
        has_custom_aria_label: true,
        has_custom_error_message: true,
        has_custom_class_name: true,
    });
    let contract = resolve_agent_contract(state);

    assert_eq!(contract.schema_attr, "ui.fieldset.agent-contract");
    assert_eq!(contract.schema_version_attr, "1");
    assert_eq!(contract.intent_attr, "form-grouping");
    assert_eq!(contract.action_attr, "initialize");
    assert_eq!(contract.state_attr, "invalid");
    assert_eq!(contract.source_attr, "custom");
    assert_eq!(contract.stream_support_attr, "unsupported");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.stream_mode_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
}

#[test]
fn normalize_boolean_axis_prefers_controlled_value_source() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: Some(true),
        default_value: Some(false),
        has_on_change: true,
        value_source_attr: "is_required",
        default_source_attr: "default_is_required",
        change_source_attr: "on_is_required_change",
    });

    assert_eq!(axis.controlled_value, Some(true));
    assert!(!axis.initial_value);
    assert_eq!(axis.value_source_attr, "is_required");
    assert_eq!(axis.control_mode_attr, "controlled");
    assert_eq!(axis.change_source_attr, "on_is_required_change");
}

#[test]
fn normalize_boolean_axis_uses_default_source_for_uncontrolled_axis() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: None,
        default_value: Some(true),
        has_on_change: false,
        value_source_attr: "is_disabled",
        default_source_attr: "default_is_disabled",
        change_source_attr: "on_is_disabled_change",
    });

    assert_eq!(axis.controlled_value, None);
    assert!(axis.initial_value);
    assert_eq!(axis.value_source_attr, "default_is_disabled");
    assert_eq!(axis.control_mode_attr, "uncontrolled");
    assert_eq!(axis.change_source_attr, "none");
}

#[test]
fn normalize_boolean_axis_falls_back_to_builtin_default_false() {
    let axis = normalize_boolean_axis(FieldsetBooleanAxisInput {
        value: None,
        default_value: None,
        has_on_change: false,
        value_source_attr: "is_invalid",
        default_source_attr: "default_is_invalid",
        change_source_attr: "on_is_invalid_change",
    });

    assert_eq!(axis.controlled_value, None);
    assert!(!axis.initial_value);
    assert_eq!(axis.value_source_attr, "default");
    assert_eq!(axis.control_mode_attr, "uncontrolled");
    assert_eq!(axis.change_source_attr, "none");
}

#[test]
fn resolve_view_state_normalizes_inputs_and_derives_state() {
    let resolved = resolve_view_state(FieldsetViewStateInput {
        orientation: FieldsetOrientation::Horizontal,
        tone: FieldsetTone::Muted,
        required: true,
        required_source_attr: "is_required",
        required_control_mode_attr: "controlled",
        required_change_source_attr: "on_is_required_change",
        disabled: false,
        disabled_source_attr: "default",
        disabled_control_mode_attr: "uncontrolled",
        disabled_change_source_attr: "none",
        invalid: true,
        invalid_source_attr: "is_invalid",
        invalid_control_mode_attr: "controlled",
        invalid_change_source_attr: "on_is_invalid_change",
        legend: Some("  Channels  ".to_string()),
        description: Some("   ".to_string()),
        error_message: Some("".to_string()),
        class_name: Some(" docs-fieldset ".to_string()),
        has_actions: true,
        has_custom_aria_label: true,
    });

    assert_eq!(resolved.legend.as_deref(), Some("Channels"));
    assert_eq!(resolved.description, None);
    assert_eq!(
        resolved.error_message.as_deref(),
        Some(DEFAULT_ERROR_MESSAGE)
    );
    assert_eq!(resolved.class_name.as_deref(), Some("docs-fieldset"));
    assert!(resolved.state.is_required);
    assert!(resolved.state.is_invalid);
    assert!(resolved.state.has_actions);
    assert_eq!(resolved.required_source_attr, "is_required");
    assert_eq!(resolved.invalid_control_mode_attr, "controlled");
}
