use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/flip`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FlipComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FlipComponentSpec {
    #[serde(default)]
    pub schema_version: FlipComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/flip/protocol.rs"]
mod tests;
