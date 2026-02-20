use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/progress/src/bar`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BarComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BarComponentSpec {
    #[serde(default)]
    pub schema_version: BarComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/bar/protocol.rs"]
mod tests;
