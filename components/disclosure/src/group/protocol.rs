use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/disclosure/src/group`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GroupComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GroupComponentSpec {
    #[serde(default)]
    pub schema_version: GroupComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/group/protocol.rs"]
mod tests;
