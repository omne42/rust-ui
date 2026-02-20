use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/menu/dropdown`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DropdownComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DropdownComponentSpec {
    #[serde(default)]
    pub schema_version: DropdownComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/dropdown/protocol.rs"]
mod tests;
