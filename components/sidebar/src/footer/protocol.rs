use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/sidebar/footer`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FooterComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FooterComponentSpec {
    #[serde(default)]
    pub schema_version: FooterComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/footer/protocol.rs"]
mod tests;
