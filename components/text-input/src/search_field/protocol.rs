use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/search_field`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchFieldComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchFieldComponentSpec {
    #[serde(default)]
    pub schema_version: SearchFieldComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/search_field/protocol.rs"]
mod tests;
