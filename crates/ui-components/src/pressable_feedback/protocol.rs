use serde::{Deserialize, Serialize};

/// Component protocol contract for `ui-components/src/pressable_feedback`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pressable FeedbackComponentSchemaVersion {
    V1,
}

impl Default for Pressable FeedbackComponentSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Pressable FeedbackComponentSpec {
    #[serde(default)]
    pub schema_version: Pressable FeedbackComponentSchemaVersion,
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
        assert_serde::<Pressable FeedbackComponentSchemaVersion>();
        assert_serde::<Pressable FeedbackComponentSpec>();
    }
}
