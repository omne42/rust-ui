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
