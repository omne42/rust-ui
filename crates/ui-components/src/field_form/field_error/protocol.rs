use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/field_form/field_error`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field ErrorComponentSchemaVersion {
    V1,
}

impl Default for Field ErrorComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Field ErrorComponentSpec {
    #[serde(default)]
    pub schema_version: Field ErrorComponentSchemaVersion,
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
        assert_serde::<Field ErrorComponentSchemaVersion>();
        assert_serde::<Field ErrorComponentSpec>();
    }
}
