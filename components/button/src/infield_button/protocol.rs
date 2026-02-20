use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/infield_button`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Infield ButtonComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Infield ButtonComponentSpec {
    #[serde(default)]
    pub schema_version: Infield ButtonComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/infield_button/protocol.rs"]
mod tests;
