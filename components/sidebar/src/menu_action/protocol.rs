use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/sidebar/menu_action`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Menu ActionComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Menu ActionComponentSpec {
    #[serde(default)]
    pub schema_version: Menu ActionComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/menu_action/protocol.rs"]
mod tests;
