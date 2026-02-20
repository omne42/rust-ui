use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/progress/src/circle`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CircleComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CircleComponentSpec {
    #[serde(default)]
    pub schema_version: CircleComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/circle/protocol.rs"]
mod tests;
