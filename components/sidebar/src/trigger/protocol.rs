use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/sidebar/trigger`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TriggerComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TriggerComponentSpec {
    #[serde(default)]
    pub schema_version: TriggerComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/trigger/protocol.rs"]
mod tests;
