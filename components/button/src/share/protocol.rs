use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/button/share`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShareComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShareComponentSpec {
    #[serde(default)]
    pub schema_version: ShareComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/share/protocol.rs"]
mod tests;
