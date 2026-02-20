use super::*;

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

    assert_eq!(contract.schema_attr, "ui.badge.agent-contract");
    assert_eq!(contract.schema_version_attr, "1");
    assert_eq!(contract.intent_attr, "status-display");
    assert_eq!(contract.action_attr, "initialize");
    assert_eq!(contract.state_attr, "outline");
    assert_eq!(contract.source_attr, "custom");
    assert_eq!(contract.stream_support_attr, "unsupported");
    assert_eq!(contract.stream_fallback_attr, "snapshot");
    assert_eq!(contract.stream_mode_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
    assert_eq!(contract.class_source_attr, "custom");
}
