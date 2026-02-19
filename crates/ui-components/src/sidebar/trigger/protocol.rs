use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/sidebar/trigger`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerComponentSchemaVersion {
    V1,
}

impl Default for TriggerComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TriggerComponentSpec {
    #[serde(default)]
    pub schema_version: TriggerComponentSchemaVersion,
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
        assert_serde::<TriggerComponentSchemaVersion>();
        assert_serde::<TriggerComponentSpec>();
    }
}
