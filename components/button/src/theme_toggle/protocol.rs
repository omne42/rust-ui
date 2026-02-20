use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/theme_toggle`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme ToggleComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Theme ToggleComponentSpec {
    #[serde(default)]
    pub schema_version: Theme ToggleComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/theme_toggle/protocol.rs"]
mod tests;
