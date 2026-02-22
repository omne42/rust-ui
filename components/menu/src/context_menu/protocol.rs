use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/menu/context_menu`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContextMenuComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContextMenuComponentSpec {
    #[serde(default)]
    pub schema_version: ContextMenuComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/context_menu/protocol.rs"]
mod tests;
