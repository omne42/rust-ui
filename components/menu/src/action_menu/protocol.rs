use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/menu/action_menu`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ActionMenuComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionMenuComponentSpec {
    #[serde(default)]
    pub schema_version: ActionMenuComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/action_menu/protocol.rs"]
mod tests;
