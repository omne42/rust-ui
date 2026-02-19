use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/text_input/search_field`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Search FieldComponentSchemaVersion {
    V1,
}

impl Default for Search FieldComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Search FieldComponentSpec {
    #[serde(default)]
    pub schema_version: Search FieldComponentSchemaVersion,
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
        assert_serde::<Search FieldComponentSchemaVersion>();
        assert_serde::<Search FieldComponentSpec>();
    }
}
