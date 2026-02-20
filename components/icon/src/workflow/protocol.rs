use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/icon/src/workflow`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowComponentSchemaVersion {
    #[default]
    V1,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkflowComponentSpec {
    #[serde(default)]
    pub schema_version: WorkflowComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../../test/workflow/protocol.rs"]
mod tests;
