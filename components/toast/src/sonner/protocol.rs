use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/toast/src/sonner`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SonnerComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SonnerComponentSpec {
    #[serde(default)]
    pub schema_version: SonnerComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/sonner/protocol.rs"]
mod tests;
