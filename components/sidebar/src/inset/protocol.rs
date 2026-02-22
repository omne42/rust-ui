use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/sidebar/inset`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InsetComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InsetComponentSpec {
    #[serde(default)]
    pub schema_version: InsetComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/inset/protocol.rs"]
mod tests;
