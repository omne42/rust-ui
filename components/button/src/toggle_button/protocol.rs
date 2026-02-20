use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/toggle_button`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Toggle ButtonComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Toggle ButtonComponentSpec {
    #[serde(default)]
    pub schema_version: Toggle ButtonComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/toggle_button/protocol.rs"]
mod tests;
