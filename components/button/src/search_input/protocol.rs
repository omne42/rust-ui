use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/button/search_input`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Search InputComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Search InputComponentSpec {
    #[serde(default)]
    pub schema_version: Search InputComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/search_input/protocol.rs"]
mod tests;
