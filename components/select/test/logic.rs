use super::{SelectStateInput, normalize_is_disabled, resolve_agent_contract, resolve_state};

#[test]
fn normalize_is_disabled_prefers_is_prefix_with_legacy_alias_fallback() {
    assert!(normalize_is_disabled(Some(true), Some(false)));
    assert!(!normalize_is_disabled(Some(false), Some(true)));
    assert!(normalize_is_disabled(None, Some(true)));
    assert!(!normalize_is_disabled(None, Some(false)));
    assert!(!normalize_is_disabled(None, None));
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_snapshot_based() {
    let open = resolve_state(SelectStateInput {
        disabled: false,
        item_count: 3,
        selected_index: Some(1),
        disabled_option_count: 0,
        is_open: true,
        has_custom_class_name: false,
        has_custom_motion: false,
    });
    let disabled = resolve_state(SelectStateInput {
        disabled: true,
        item_count: 1,
        selected_index: Some(0),
        disabled_option_count: 0,
        is_open: false,
        has_custom_class_name: true,
        has_custom_motion: false,
    });

    let open_contract = resolve_agent_contract(open);
    let disabled_contract = resolve_agent_contract(disabled);

    assert_eq!(open_contract.schema_attr, "ui.select.agent-contract.v1");
    assert_eq!(open_contract.schema_version_attr, "v1");
    assert_eq!(open_contract.intent_attr, "choose-option");
    assert_eq!(open_contract.action_attr, "open");
    assert_eq!(open_contract.state_attr, "open");
    assert_eq!(open_contract.stream_support_attr, "optional");
    assert_eq!(open_contract.stream_fallback_attr, "snapshot");
    assert_eq!(open_contract.stream_mode_attr, "snapshot");
    assert_eq!(open_contract.output_status_attr, "verified");
    assert_eq!(open_contract.source_attr, "default");

    assert_eq!(disabled_contract.action_attr, "disabled");
    assert_eq!(disabled_contract.state_attr, "disabled");
    assert_eq!(disabled_contract.source_attr, "custom");
}
