use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/textarea`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextareaComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TextareaComponentSpec {
    #[serde(default)]
    pub schema_version: TextareaComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/textarea/protocol.rs"]
mod tests;
