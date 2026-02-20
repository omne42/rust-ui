use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/menu/menubar`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MenubarComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MenubarComponentSpec {
    #[serde(default)]
    pub schema_version: MenubarComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/menubar/protocol.rs"]
mod tests;
