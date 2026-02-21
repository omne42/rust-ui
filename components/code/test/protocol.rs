use super::*;
use serde::de::DeserializeOwned;

fn assert_serde<T>()
where
    T: Serialize + DeserializeOwned,
{
}

#[test]
fn protocol_types_implement_serde_contract() {
    assert_serde::<CodeComponentSchemaVersion>();
    assert_serde::<CodeAgentIntent>();
    assert_serde::<CodeAgentAction>();
    assert_serde::<CodeAgentStateAxis>();
    assert_serde::<CodeAgentSourceAxis>();
    assert_serde::<CodeAgentContract>();
    assert_serde::<CodeComponentSpec>();
    assert_serde::<CodeRenderMode>();
    assert_serde::<CodeComponentSpecV2>();
}

#[test]
fn migrate_v1_to_v2_preserves_contract_axes_and_sets_v2_defaults() {
    let v1 = CodeComponentSpec {
        schema_version: CodeComponentSchemaVersion::V1,
        agent_contract: CodeAgentContract::default(),
    };

    let v2 = migrate_v1_to_v2(v1.clone());

    assert_eq!(v2.schema_version, CodeComponentSchemaVersion::V2);
    assert_eq!(v2.agent_contract, v1.agent_contract);
    assert_eq!(v2.render_mode, CodeRenderMode::Snapshot);
}

#[test]
fn schema_registry_tracks_v1_deprecation_window_and_migration_hook() {
    let v1_entry = CODE_SCHEMA_REGISTRY
        .iter()
        .find(|entry| entry.schema == CodeComponentSchemaVersion::V1)
        .expect("schema registry should contain code.v1");
    let v2_entry = CODE_SCHEMA_REGISTRY
        .iter()
        .find(|entry| entry.schema == CodeComponentSchemaVersion::V2)
        .expect("schema registry should contain code.v2");

    assert_eq!(v1_entry.schema_name, "code.v1");
    assert_eq!(v1_entry.status, CodeSchemaStatus::Deprecated);
    assert_eq!(
        v1_entry.deprecation_window,
        Some(CodeDeprecationWindow {
            starts_on: "2026-02-20",
            ends_on: "2026-08-31",
        })
    );
    assert_eq!(v1_entry.successor, Some(CodeComponentSchemaVersion::V2));
    assert!(v1_entry.migration.is_some());

    assert_eq!(v2_entry.schema_name, "code.v2");
    assert_eq!(v2_entry.status, CodeSchemaStatus::Active);
    assert_eq!(v2_entry.deprecation_window, None);
    assert_eq!(v2_entry.successor, None);
    assert_eq!(v2_entry.migration, None);
}
