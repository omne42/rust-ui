use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/text_input/text_area`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Text AreaComponentSchemaVersion {
    V1,
}

impl Default for Text AreaComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Text AreaComponentSpec {
    #[serde(default)]
    pub schema_version: Text AreaComponentSchemaVersion,
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
        assert_serde::<Text AreaComponentSchemaVersion>();
        assert_serde::<Text AreaComponentSpec>();
    }
}
