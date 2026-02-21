use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<ActionBarComponentSchemaVersion>();
    assert_serde::<ActionBarComponentSpec>();
    assert_serde::<ActionBarRenderCapability>();
}

#[test]
fn default_protocol_spec_enforces_render_capability_whitelist() {
    let policy = ActionBarComponentSpec::default().render_policy();

    assert!(policy.allow_selection_summary);
    assert!(policy.allow_clear_action);
    assert!(policy.allow_children_slot);
}

#[test]
fn agent_contract_defaults_to_streaming_optional_snapshot_output() {
    assert_eq!(ActionBarStreamingPolicy::Optional.as_attr(), "optional");
    assert_eq!(ActionBarStreamingFallback::Snapshot.as_attr(), "snapshot");
    assert_eq!(ActionBarOutputMode::Snapshot.as_attr(), "snapshot");
    assert_eq!(ActionBarOutputStatus::Validated.as_attr(), "validated");
}
