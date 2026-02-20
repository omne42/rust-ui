use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/toast/src/close_button`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CloseButtonComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CloseButtonComponentSpec {
    #[serde(default)]
    pub schema_version: CloseButtonComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/close_button/protocol.rs"]
mod tests;
