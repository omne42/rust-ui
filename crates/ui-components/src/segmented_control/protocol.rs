use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/segmented_control`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Segmented ControlComponentSchemaVersion {
    V1,
}

impl Default for Segmented ControlComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Segmented ControlComponentSpec {
    #[serde(default)]
    pub schema_version: Segmented ControlComponentSchemaVersion,
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
        assert_serde::<Segmented ControlComponentSchemaVersion>();
        assert_serde::<Segmented ControlComponentSpec>();
    }
}
