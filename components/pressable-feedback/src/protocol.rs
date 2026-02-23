use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/pressable-feedback/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PressableFeedbackComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PressableFeedbackProtocol {
    #[serde(default)]
    pub schema_version: PressableFeedbackComponentSchemaVersion,
}

pub type PressableFeedbackComponentSpec = PressableFeedbackProtocol;

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
