use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/text-input/src/date_picker`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DatePickerComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DatePickerComponentSpec {
    #[serde(default)]
    pub schema_version: DatePickerComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/date_picker/protocol.rs"]
mod tests;
