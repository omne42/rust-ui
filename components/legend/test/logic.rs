use super::*;

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(LegendTone::Default.class_name(), "ui-legend--tone-default");
    assert_eq!(LegendTone::Muted.class_name(), "ui-legend--tone-muted");
    assert_eq!(LegendTone::Strong.class_name(), "ui-legend--tone-strong");

    assert_eq!(LegendTone::Default.as_attr(), "default");
    assert_eq!(LegendTone::Muted.as_attr(), "muted");
    assert_eq!(LegendTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_helpers_fallback_to_defaults() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Preferences  ".to_string())),
        Some("Preferences".to_string())
    );

    assert_eq!(
        normalize_text(Some("  Notification settings  ".to_string())),
        ("Notification settings".to_string(), true)
    );
    assert_eq!(normalize_text(None), (DEFAULT_TEXT.into(), false));

    assert_eq!(
        normalize_required_indicator(Some("  (required)  ".to_string())),
        ("(required)".to_string(), true)
    );
    assert_eq!(
        normalize_required_indicator(None),
        (DEFAULT_REQUIRED_INDICATOR.into(), false)
    );
}

#[test]
fn normalize_required_and_disabled_states_track_sources() {
    let required = normalize_required_state(Some(false));
    assert!(!required.is_required);
    assert_eq!(required.required_source_attr, "is_required");

    let required = normalize_required_state(None);
    assert!(!required.is_required);
    assert_eq!(required.required_source_attr, "default");

    let disabled = normalize_accessibility_state(Some(false));
    assert!(!disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "is_disabled");

    let disabled = normalize_accessibility_state(None);
    assert!(!disabled.is_disabled);
    assert_eq!(disabled.disabled_source_attr, "default");
}

#[test]
fn normalize_component_state_centralizes_state_derivation() {
    let model = normalize_component_state(LegendNormalizeInput {
        tone: LegendTone::Muted,
        is_required: Some(true),
        is_disabled: None,
        text: Some(" Billing settings ".to_string()),
        required_indicator: Some(" (required) ".to_string()),
        class_name: Some(" docs-legend ".to_string()),
    });

    assert_eq!(model.state.tone_attr, "muted");
    assert!(model.state.is_required);
    assert!(!model.state.is_disabled);
    assert_eq!(model.required_state.required_source_attr, "is_required");
    assert_eq!(model.accessibility_state.disabled_source_attr, "default");
    assert_eq!(model.text, "Billing settings");
    assert_eq!(model.required_indicator, "(required)");
    assert_eq!(model.class_name.as_deref(), Some("docs-legend"));
}

#[test]
fn resolve_state_tracks_required_disabled_and_sources() {
    let state = resolve_state(LegendStateInput {
        tone: LegendTone::Strong,
        is_required: true,
        is_disabled: true,
        has_custom_text: true,
        has_custom_indicator: false,
        has_custom_class_name: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert!(state.is_required);
    assert!(!state.is_optional);
    assert!(state.is_disabled);
    assert_eq!(state.text_source_attr, "custom");
    assert_eq!(state.indicator_source_attr, "default");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(LegendStateInput {
        tone: LegendTone::Muted,
        is_required: true,
        is_disabled: false,
        has_custom_text: false,
        has_custom_indicator: true,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-legend".to_string()), state);
    for token in [
        "ui-legend",
        "ui-legend--tone-muted",
        "ui-legend--required",
        "ui-legend--indicator-custom",
        "ui-legend--custom-class",
        "docs-legend",
    ] {
        assert!(class_name.contains(token), "class should contain `{token}`");
    }
}

#[test]
fn agent_contract_is_stable() {
    let contract = resolve_agent_contract();
    assert_eq!(contract.schema_attr, "ui.legend.agent-contract.v1");
    assert_eq!(contract.schema_version_attr, "1");
    assert_eq!(contract.stream_support_attr, "unsupported");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.stream_mode_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
    assert_eq!(contract.intent_attr, "describe-fieldset");
    assert_eq!(LegendUiAction::Idle.as_attr(), "idle");
}
