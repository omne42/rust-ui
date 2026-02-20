use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-layout/src/content`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContentComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContentComponentSpec {
    #[serde(default)]
    pub schema_version: ContentComponentSchemaVersion,
}

#[cfg(test)]
#[path = "test/protocol.rs"]
mod tests;
