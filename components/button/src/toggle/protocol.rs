use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/button/toggle`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ToggleComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToggleComponentSpec {
    #[serde(default)]
    pub schema_version: ToggleComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/toggle/protocol.rs"]
mod tests;
