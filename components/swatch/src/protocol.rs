use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/color/swatch_core`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Swatch CoreComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Swatch CoreComponentSpec {
    #[serde(default)]
    pub schema_version: Swatch CoreComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
