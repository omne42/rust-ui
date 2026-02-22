use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/label`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LabelComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LabelComponentSpec {
    #[serde(default)]
    pub schema_version: LabelComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
