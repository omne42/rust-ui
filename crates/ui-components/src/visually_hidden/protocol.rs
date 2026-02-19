use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/visually_hidden`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visually HiddenComponentSchemaVersion {
    V1,
}

impl Default for Visually HiddenComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Visually HiddenComponentSpec {
    #[serde(default)]
    pub schema_version: Visually HiddenComponentSchemaVersion,
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
        assert_serde::<Visually HiddenComponentSchemaVersion>();
        assert_serde::<Visually HiddenComponentSpec>();
    }
}
