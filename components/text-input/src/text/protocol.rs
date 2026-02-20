use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/text`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TextComponentSpec {
    #[serde(default)]
    pub schema_version: TextComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/text/protocol.rs"]
mod tests;
