use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/icon/src/icons`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IconsComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IconsComponentSpec {
    #[serde(default)]
    pub schema_version: IconsComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/icons/protocol.rs"]
mod tests;
