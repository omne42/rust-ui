use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/illustrated_message`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Illustrated MessageComponentSchemaVersion {
    V1,
}

impl Default for Illustrated MessageComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Illustrated MessageComponentSpec {
    #[serde(default)]
    pub schema_version: Illustrated MessageComponentSchemaVersion,
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
        assert_serde::<Illustrated MessageComponentSchemaVersion>();
        assert_serde::<Illustrated MessageComponentSpec>();
    }
}
