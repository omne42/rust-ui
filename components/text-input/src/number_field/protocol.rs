use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/number_field`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NumberFieldComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NumberFieldComponentSpec {
    #[serde(default)]
    pub schema_version: NumberFieldComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/number_field/protocol.rs"]
mod tests;
