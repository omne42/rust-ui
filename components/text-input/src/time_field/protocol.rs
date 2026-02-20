use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/time_field`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TimeFieldComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TimeFieldComponentSpec {
    #[serde(default)]
    pub schema_version: TimeFieldComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/time_field/protocol.rs"]
mod tests;
