use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui/src/menu/section`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SectionComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SectionComponentSpec {
    #[serde(default)]
    pub schema_version: SectionComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/section/protocol.rs"]
mod tests;
