use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/breadcrumb`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BreadcrumbComponentSpec {
    #[serde(default)]
    pub schema_version: BreadcrumbComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
