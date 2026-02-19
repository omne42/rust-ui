use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/hover_card`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Hover CardComponentSchemaVersion {
    V1,
}

impl Default for Hover CardComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Hover CardComponentSpec {
    #[serde(default)]
    pub schema_version: Hover CardComponentSchemaVersion,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::DeserializeOwned;

    fn assert_serde<T>()
    where
        T: Serialize + DeserializeOwned,
    {
    }

    #[test]
    fn protocol_types_implement_serde_contract() {
        assert_serde::<Hover CardComponentSchemaVersion>();
        assert_serde::<Hover CardComponentSpec>();
    }
}
