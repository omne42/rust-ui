use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/number`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NumberComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NumberComponentSpec {
    #[serde(default)]
    pub schema_version: NumberComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/number/protocol.rs"]
mod tests;
