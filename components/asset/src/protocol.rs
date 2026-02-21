use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/asset/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AssetComponentSchemaVersion {
    #[default]
    V1,
}

pub const ASSET_AGENT_SCHEMA: &str = "ui.asset.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetAgentIntent {
    #[default]
    Display,
}

impl AssetAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Display => "display",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetAgentAction {
    #[default]
    StaticRender,
}

impl AssetAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::StaticRender => "static-render",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetInteractionSource {
    #[default]
    ExternalProp,
}

impl AssetInteractionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ExternalProp => "external-prop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetMotionSource {
    #[default]
    Default,
    Custom,
}

impl AssetMotionSource {
    pub const fn from_is_custom(is_custom: bool) -> Self {
        if is_custom {
            Self::Custom
        } else {
            Self::Default
        }
    }

    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetStreamSupport {
    #[default]
    Optional,
}

impl AssetStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetStreamFallback {
    #[default]
    Snapshot,
}

impl AssetStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AssetOutputStatus {
    Draft,
    #[default]
    Verified,
    Submittable,
}

impl AssetOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AssetComponentSpec {
    #[serde(default)]
    pub schema_version: AssetComponentSchemaVersion,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
