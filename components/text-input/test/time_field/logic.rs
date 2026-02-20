use super::*;

#[test]
fn normalize_disabled_state_prefers_is_disabled_alias() {
    assert!(normalize_disabled_state(DisabledStateInput {
        is_disabled: Some(true),
        disabled: false,
    }));
    assert!(!normalize_disabled_state(DisabledStateInput {
        is_disabled: None,
        disabled: false,
    }));
    assert!(normalize_disabled_state(DisabledStateInput {
        is_disabled: None,
        disabled: true,
    }));
}

#[test]
fn normalize_value_state_centralizes_default_value_normalization() {
    let normalized = normalize_value_state(ValueStateInput {
        value: None,
        default_value: Some(" 9:17 ".to_string()),
        on_value_change: None,
        minute_step: 5,
    });

    assert_eq!(normalized.default_value, Some("09:15".to_string()));
    assert!(!normalized.is_controlled);
    assert!(normalized.has_default_value);
    assert!(!normalized.has_value_change_handler);
}

#[test]
fn normalize_value_state_marks_controlled_axis_when_value_signal_exists() {
    let (value, _) = signal(Some("09:30".to_string()));
    let normalized = normalize_value_state(ValueStateInput {
        value: Some(value.into()),
        default_value: Some("08:00".to_string()),
        on_value_change: None,
        minute_step: 15,
    });

    assert!(normalized.is_controlled);
    assert_eq!(normalized.default_value, Some("08:00".to_string()));
    assert!(normalized.has_default_value);
    assert!(!normalized.has_value_change_handler);
}

#[test]
fn normalize_value_state_clamps_invalid_step_and_value_into_testable_state() {
    let normalized = normalize_value_state(ValueStateInput {
        value: None,
        default_value: Some("10:59".to_string()),
        on_value_change: None,
        minute_step: 0,
    });

    assert_eq!(normalized.default_value, Some("10:59".to_string()));

    let normalized = normalize_value_state(ValueStateInput {
        value: None,
        default_value: Some("10:59".to_string()),
        on_value_change: None,
        minute_step: 60,
    });
    assert_eq!(normalized.default_value, Some("10:30".to_string()));
}

#[test]
fn resolve_agent_contract_tracks_state_source_and_capabilities() {
    let state = resolve_state(TimeFieldStateInput {
        tone: TimeFieldTone::Default,
        disabled: false,
        is_controlled: true,
        has_default_value: false,
        has_value_change_handler: true,
        has_value: true,
        minute_step: 15,
        has_custom_label: false,
        has_custom_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    let contract = resolve_agent_contract(state, TimeFieldAgentSource::HourInput);
    assert_eq!(contract.schema_name, "ui.time-field.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "time-input");
    assert_eq!(contract.action.as_str(), "edit-hour");
    assert_eq!(contract.state.as_str(), "filled");
    assert_eq!(contract.source.as_str(), "hour-input");
    assert_eq!(contract.output_status.as_str(), "submittable");
    assert_eq!(contract.stream_support.as_str(), "unsupported");
    assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
    assert!(contract.capabilities.can_edit);
    assert!(contract.capabilities.can_clear);
}

#[test]
fn resolve_agent_contract_disabled_maps_to_disabled_state_axis() {
    let state = resolve_state(TimeFieldStateInput {
        tone: TimeFieldTone::Default,
        disabled: true,
        is_controlled: false,
        has_default_value: false,
        has_value_change_handler: false,
        has_value: true,
        minute_step: 5,
        has_custom_label: false,
        has_custom_placeholder: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        has_custom_motion: false,
    });

    let contract = resolve_agent_contract(state, TimeFieldAgentSource::Init);
    assert_eq!(contract.state.as_str(), "disabled");
    assert_eq!(contract.output_status.as_str(), "verified");
    assert!(!contract.capabilities.can_edit);
    assert!(!contract.capabilities.can_clear);
}
