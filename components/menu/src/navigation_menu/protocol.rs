use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/menu/navigation_menu`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NavigationMenuComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NavigationMenuComponentSpec {
    #[serde(default)]
    pub schema_version: NavigationMenuComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/navigation_menu/protocol.rs"]
mod tests;
