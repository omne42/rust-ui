use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/button/clear_button`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Clear ButtonComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Clear ButtonComponentSpec {
    #[serde(default)]
    pub schema_version: Clear ButtonComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/clear_button/protocol.rs"]
mod tests;
