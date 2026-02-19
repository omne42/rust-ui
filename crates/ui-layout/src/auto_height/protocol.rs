use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-layout/src/auto_height`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auto HeightComponentSchemaVersion {
    V1,
}

impl Default for Auto HeightComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Auto HeightComponentSpec {
    #[serde(default)]
    pub schema_version: Auto HeightComponentSchemaVersion,
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
        assert_serde::<Auto HeightComponentSchemaVersion>();
        assert_serde::<Auto HeightComponentSpec>();
    }
}
