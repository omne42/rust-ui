use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/sidebar/rail`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RailComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RailComponentSpec {
    #[serde(default)]
    pub schema_version: RailComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/rail/protocol.rs"]
mod tests;
