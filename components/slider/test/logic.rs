use super::*;

#[test]
fn normalize_value_axis_tracks_control_mode_and_sources() {
    let (value, _set_value) = signal(30.0_f64);
    let (_legacy_value, set_legacy_value) = signal(0.0_f64);
    let on_value_change = Callback::new(|_: f64| {});

    let normalized = normalize_value_axis(ValueAxisInput {
        value: Some(value.into()),
        default_value: Some(15.0),
        on_value_change: Some(on_value_change),
        set_value: Some(set_legacy_value),
        on_change: Some(Callback::new(|_: f64| {})),
    });

    assert_eq!(normalized.control_mode_attr, "controlled");
    assert_eq!(normalized.value_source_attr, "external");
    assert_eq!(normalized.default_value_source_attr, "custom");
    assert_eq!(normalized.value_change_source_attr, "on_value_change");

    let normalized = normalize_value_axis(ValueAxisInput {
        value: None,
        default_value: None,
        on_value_change: None,
        set_value: None,
        on_change: None,
    });

    assert_eq!(normalized.control_mode_attr, "uncontrolled");
    assert_eq!(normalized.value_source_attr, "default_value");
    assert_eq!(normalized.default_value, DEFAULT_MIN);
    assert_eq!(normalized.default_value_source_attr, "default");
    assert_eq!(normalized.value_change_source_attr, "none");
}

#[test]
fn normalize_value_axis_supports_legacy_setter_fallback() {
    let (value, set_value) = signal(10.0_f64);
    let normalized = normalize_value_axis(ValueAxisInput {
        value: Some(value.into()),
        default_value: None,
        on_value_change: None,
        set_value: Some(set_value),
        on_change: None,
    });

    assert_eq!(normalized.value_change_source_attr, "set_value");
    let handler = normalized
        .on_value_change
        .expect("legacy setter should map to on_value_change");
    handler.run(77.0);
    assert_eq!(value.get_untracked(), 77.0);
}

#[test]
fn normalize_accessibility_state_prefers_is_prefixed_input() {
    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: Some(false),
        disabled: true,
    });
    assert!(!state.is_disabled);
    assert_eq!(state.disabled_source_attr, "is_disabled");

    let state = normalize_accessibility_state(AccessibilityStateInput {
        is_disabled: None,
        disabled: true,
    });
    assert!(state.is_disabled);
    assert_eq!(state.disabled_source_attr, "disabled");
}

#[test]
fn normalize_id_resolves_default_and_custom_sources() {
    let default_id = normalize_id("  ".to_string());
    assert_eq!(default_id.id, DEFAULT_ID);
    assert_eq!(default_id.id_source_attr, "default");
    assert!(!default_id.has_custom_id);

    let custom_id = normalize_id(" docs-slider ".to_string());
    assert_eq!(custom_id.id, "docs-slider");
    assert_eq!(custom_id.id_source_attr, "custom");
    assert!(custom_id.has_custom_id);
}

#[test]
fn resolve_agent_contract_uses_closed_set_markers() {
    let controlled = resolve_agent_contract(true);
    assert_eq!(controlled.schema_attr, "ui.slider.agent-contract.v1");
    assert_eq!(controlled.stream_support_attr, "unsupported");
    assert_eq!(controlled.stream_fallback_attr, "snapshot");
    assert_eq!(controlled.stream_mode_attr, "snapshot");
    assert_eq!(controlled.output_status_attr, "submittable");
    assert_eq!(controlled.intent_attr, "adjust-value");

    let snapshot_only = resolve_agent_contract(false);
    assert_eq!(snapshot_only.output_status_attr, "verified");
}

#[test]
fn resolve_ui_action_is_closed_set_and_priority_ordered() {
    assert_eq!(resolve_ui_action(false, false).as_attr(), "idle");
    assert_eq!(resolve_ui_action(false, true).as_attr(), "focus");
    assert_eq!(resolve_ui_action(true, false).as_attr(), "press");
    assert_eq!(resolve_ui_action(true, true).as_attr(), "press");
}
