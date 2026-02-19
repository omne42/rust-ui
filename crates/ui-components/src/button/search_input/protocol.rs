use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/button/search_input`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Search InputComponentSchemaVersion {
    V1,
}

impl Default for Search InputComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Search InputComponentSpec {
    #[serde(default)]
    pub schema_version: Search InputComponentSchemaVersion,
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
        assert_serde::<Search InputComponentSchemaVersion>();
        assert_serde::<Search InputComponentSpec>();
    }
}
