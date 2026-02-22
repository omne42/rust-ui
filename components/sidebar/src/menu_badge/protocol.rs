use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/sidebar/menu_badge`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Menu BadgeComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Menu BadgeComponentSpec {
    #[serde(default)]
    pub schema_version: Menu BadgeComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/menu_badge/protocol.rs"]
mod tests;
