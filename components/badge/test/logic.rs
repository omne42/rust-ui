use super::*;

#[test]
fn resolve_variant_defaults_to_default_when_absent() {
    assert_eq!(resolve_variant(None), BadgeVariant::Default);
    assert_eq!(
        resolve_variant(Some(BadgeVariant::Danger)),
        BadgeVariant::Danger
    );
}

#[test]
fn resolve_render_state_centralizes_state_derivation() {
    let resolved =
        resolve_render_state(Some(BadgeVariant::Outline), Some("docs-badge".to_string()));

    assert_eq!(resolved.state.variant, BadgeVariant::Outline);
    assert_eq!(resolved.state.fill_attr, "outline");
    assert_eq!(
        resolved.agent_contract.source,
        BadgeAgentSource::CustomClassName
    );
    assert!(resolved.class_name.contains("ui-badge"));
    assert!(resolved.class_name.contains("ui-badge--fill-outline"));
    assert!(resolved.class_name.contains("docs-badge"));
}

#[test]
fn compose_class_name_includes_state_markers() {
    let solid = resolve_state(BadgeStateInput {
        variant: BadgeVariant::Accent,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-badge".to_string()), solid);

    for token in [
        "ui-badge",
        "ui-badge--variant-accent",
        "ui-badge--fill-solid",
        "ui-badge--custom-class",
        "docs-badge",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn resolve_agent_contract_emits_machine_readable_markers() {
    let contract = resolve_agent_contract(resolve_state(BadgeStateInput {
        variant: BadgeVariant::Outline,
        has_custom_class_name: true,
    }));

    assert_eq!(contract.schema_name, BADGE_AGENT_SCHEMA_NAME);
    assert_eq!(contract.schema_version, BadgeAgentSchemaVersion::V1);
    assert_eq!(contract.intent, BadgeAgentIntent::StatusDisplay);
    assert_eq!(contract.action, BadgeAgentAction::Initialize);
    assert_eq!(contract.state, BadgeAgentStateAxis::Outline);
    assert_eq!(contract.source, BadgeAgentSource::CustomClassName);
    assert_eq!(
        contract.stream_support,
        BadgeAgentStreamSupport::Unsupported
    );
    assert_eq!(contract.stream_fallback, BadgeAgentStreamFallback::Snapshot);
    assert_eq!(contract.stream_mode, BadgeAgentStreamMode::Snapshot);
    assert_eq!(contract.output_status, BadgeAgentOutputStatus::Verified);
    assert_eq!(contract.source.as_attr(), "custom");
}
