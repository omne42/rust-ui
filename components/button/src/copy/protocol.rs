use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/copy`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CopyComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CopyComponentSpec {
    #[serde(default)]
    pub schema_version: CopyComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/copy/protocol.rs"]
mod tests;
