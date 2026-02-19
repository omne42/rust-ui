use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/disclosure`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureComponentSchemaVersion {
    V1,
}

impl Default for DisclosureComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DisclosureComponentSpec {
    #[serde(default)]
    pub schema_version: DisclosureComponentSchemaVersion,
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
        assert_serde::<DisclosureComponentSchemaVersion>();
        assert_serde::<DisclosureComponentSpec>();
    }
}
