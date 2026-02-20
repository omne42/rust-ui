use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-layout/src/auto_height`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Auto HeightComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Auto HeightComponentSpec {
    #[serde(default)]
    pub schema_version: Auto HeightComponentSchemaVersion,
}

#[cfg(test)]
#[path = "test/protocol.rs"]
mod tests;
