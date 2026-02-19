use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/status_light`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status LightComponentSchemaVersion {
    V1,
}

impl Default for Status LightComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Status LightComponentSpec {
    #[serde(default)]
    pub schema_version: Status LightComponentSchemaVersion,
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
        assert_serde::<Status LightComponentSchemaVersion>();
        assert_serde::<Status LightComponentSpec>();
    }
}
