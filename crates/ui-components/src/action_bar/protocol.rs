use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/action_bar`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action BarComponentSchemaVersion {
    V1,
}

impl Default for Action BarComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Action BarComponentSpec {
    #[serde(default)]
    pub schema_version: Action BarComponentSchemaVersion,
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
        assert_serde::<Action BarComponentSchemaVersion>();
        assert_serde::<Action BarComponentSpec>();
    }
}
