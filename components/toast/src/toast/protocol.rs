use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/toast/src/toast`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ToastComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToastComponentSpec {
    #[serde(default)]
    pub schema_version: ToastComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/toast/protocol.rs"]
mod tests;
