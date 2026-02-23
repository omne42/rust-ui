use super::*;

#[test]
fn compose_class_name_includes_state_markers() {
    let normalized = normalize_tag_input(TagNormalizeInput {
        variant: TagVariant::Default,
        size: TagSize::Sm,
        mode: None,
        is_disabled: Some(false),
        is_removable: Some(false),
        has_remove_handler: false,
        remove_aria_label: None,
        class_name: Some("docs-tag-custom".to_string()),
    });

    let class_name = compose_class_name(normalized.class_name, normalized.state);

    for token in [
        "ui-tag",
        "ui-tag--variant-default",
        "ui-tag--size-sm",
        "ui-tag--static",
        "ui-tag--enabled",
        "ui-tag--custom-class",
        "docs-tag-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_tag_input_centralizes_trim_and_state_resolution() {
    let normalized = normalize_tag_input(TagNormalizeInput {
        variant: TagVariant::Surface,
        size: TagSize::Lg,
        mode: None,
        is_disabled: Some(false),
        is_removable: Some(true),
        has_remove_handler: true,
        remove_aria_label: Some("  Remove framework  ".to_string()),
        class_name: Some("  docs-tag-custom  ".to_string()),
    });

    assert_eq!(normalized.class_name, Some("docs-tag-custom".to_string()));
    assert_eq!(normalized.remove_aria_label, "Remove framework");
    assert!(normalized.state.is_removable);
    assert_eq!(normalized.state.remove_label_source_attr, "custom");
}

#[test]
fn normalize_tag_bool_input_prefers_is_prefix_over_legacy_alias() {
    let normalized = normalize_tag_bool_input(None, Some(true), Some(false));
    assert!(normalized.is_disabled);
    assert!(!normalized.is_removable);
    assert_eq!(normalized.mode, TagInteractivityMode::Disabled);

    let defaulted = normalize_tag_bool_input(None, None, None);
    assert!(!defaulted.is_disabled);
    assert!(!defaulted.is_removable);
    assert_eq!(defaulted.mode, TagInteractivityMode::Static);
}

#[test]
fn normalize_tag_interactivity_mode_prioritizes_enum_over_boolean_matrix() {
    let normalized = normalize_tag_bool_input(
        Some(TagInteractivityMode::Removable),
        Some(true),
        Some(false),
    );

    assert_eq!(normalized.mode, TagInteractivityMode::Removable);
    assert!(!normalized.is_disabled);
    assert!(normalized.is_removable);
}

#[test]
fn agent_contract_is_schema_typed_and_snapshot_fallback_is_explicit() {
    let state = resolve_state(TagStateInput {
        variant: TagVariant::Default,
        size: TagSize::Md,
        disabled: false,
        removable: true,
        has_remove_handler: true,
        has_custom_remove_aria_label: false,
        has_custom_class_name: false,
    });
    let contract = resolve_agent_contract(state, TagAgentSource::RemovePointer);

    assert_eq!(contract.schema_name, "ui.tag.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "token");
    assert_eq!(contract.action.as_str(), "remove-pointer");
    assert_eq!(contract.state.as_str(), "removable");
    assert_eq!(contract.source.as_str(), "remove-pointer");
    assert_eq!(contract.output_status.as_str(), "submittable");
    assert_eq!(contract.stream_support.as_str(), "unsupported");
    assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
    assert!(contract.capabilities.can_remove);
    assert!(contract.capabilities.can_disable);
}
