use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/text_field`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextFieldComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TextFieldComponentSpec {
    #[serde(default)]
    pub schema_version: TextFieldComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/text_field/protocol.rs"]
mod tests;
