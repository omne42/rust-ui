use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/icon/src/set`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SetComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SetComponentSpec {
    #[serde(default)]
    pub schema_version: SetComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/set/protocol.rs"]
mod tests;
