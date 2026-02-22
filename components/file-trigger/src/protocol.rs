use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/file_trigger`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum File TriggerComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct File TriggerComponentSpec {
    #[serde(default)]
    pub schema_version: File TriggerComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
