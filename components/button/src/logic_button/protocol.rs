use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/logic_button`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Logic ButtonComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Logic ButtonComponentSpec {
    #[serde(default)]
    pub schema_version: Logic ButtonComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/logic_button/protocol.rs"]
mod tests;
