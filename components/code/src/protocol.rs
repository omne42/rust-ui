use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/code/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeComponentSchemaVersion {
    #[default]
    V1,
    V2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeAgentIntent {
    #[default]
    Display,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeAgentAction {
    #[default]
    SnapshotRender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeAgentStateAxis {
    Variant,
    State,
    CustomClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeAgentSourceAxis {
    PropsVariant,
    PropsClassName,
    PrimitiveResolveState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeAgentContract {
    #[serde(default)]
    pub intent: CodeAgentIntent,
    #[serde(default)]
    pub action: CodeAgentAction,
    #[serde(default)]
    pub state_axes: Vec<CodeAgentStateAxis>,
    #[serde(default)]
    pub source_axes: Vec<CodeAgentSourceAxis>,
}

impl Default for CodeAgentContract {
    fn default() -> Self {
        Self {
            intent: CodeAgentIntent::Display,
            action: CodeAgentAction::SnapshotRender,
            state_axes: vec![
                CodeAgentStateAxis::Variant,
                CodeAgentStateAxis::State,
                CodeAgentStateAxis::CustomClass,
            ],
            source_axes: vec![
                CodeAgentSourceAxis::PropsVariant,
                CodeAgentSourceAxis::PropsClassName,
                CodeAgentSourceAxis::PrimitiveResolveState,
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeComponentSpec {
    #[serde(default)]
    pub schema_version: CodeComponentSchemaVersion,
    #[serde(default)]
    pub agent_contract: CodeAgentContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeRenderMode {
    #[default]
    Snapshot,
    StreamingOptional,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeComponentSpecV2 {
    #[serde(default)]
    pub schema_version: CodeComponentSchemaVersion,
    #[serde(default)]
    pub agent_contract: CodeAgentContract,
    #[serde(default)]
    pub render_mode: CodeRenderMode,
}

impl Default for CodeComponentSpecV2 {
    fn default() -> Self {
        Self {
            schema_version: CodeComponentSchemaVersion::V2,
            agent_contract: CodeAgentContract::default(),
            render_mode: CodeRenderMode::Snapshot,
        }
    }
}

/// Pure migration bridge for breaking protocol upgrades.
/// The function is deterministic and side-effect free so it can be reused
/// by codemod/registry tooling.
pub fn migrate_v1_to_v2(v1: CodeComponentSpec) -> CodeComponentSpecV2 {
    CodeComponentSpecV2 {
        schema_version: CodeComponentSchemaVersion::V2,
        agent_contract: v1.agent_contract,
        render_mode: CodeRenderMode::Snapshot,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeSchemaStatus {
    Active,
    Deprecated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeDeprecationWindow {
    pub starts_on: &'static str,
    pub ends_on: &'static str,
}

pub type CodeSchemaMigration = fn(CodeComponentSpec) -> CodeComponentSpecV2;

#[derive(Debug, Clone, Copy)]
pub struct CodeSchemaRegistryEntry {
    pub schema: CodeComponentSchemaVersion,
    pub schema_name: &'static str,
    pub status: CodeSchemaStatus,
    pub deprecation_window: Option<CodeDeprecationWindow>,
    pub successor: Option<CodeComponentSchemaVersion>,
    pub migration: Option<CodeSchemaMigration>,
}

pub const CODE_SCHEMA_REGISTRY: [CodeSchemaRegistryEntry; 2] = [
    CodeSchemaRegistryEntry {
        schema: CodeComponentSchemaVersion::V1,
        schema_name: "code.v1",
        status: CodeSchemaStatus::Deprecated,
        deprecation_window: Some(CodeDeprecationWindow {
            starts_on: "2026-02-20",
            ends_on: "2026-08-31",
        }),
        successor: Some(CodeComponentSchemaVersion::V2),
        migration: Some(migrate_v1_to_v2),
    },
    CodeSchemaRegistryEntry {
        schema: CodeComponentSchemaVersion::V2,
        schema_name: "code.v2",
        status: CodeSchemaStatus::Active,
        deprecation_window: None,
        successor: None,
        migration: None,
    },
];

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
