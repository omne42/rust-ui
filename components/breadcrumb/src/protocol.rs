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

pub const BREADCRUMB_AGENT_SCHEMA_NAME: &str = "ui.breadcrumb.agent-contract";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentSchemaVersion {
    #[default]
    V1,
}

impl BreadcrumbAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentIntent {
    #[default]
    TrailNavigation,
}

impl BreadcrumbAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrailNavigation => "trail-navigation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentAction {
    #[default]
    Navigate,
}

impl BreadcrumbAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Navigate => "navigate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentState {
    #[default]
    Empty,
    ItemOnly,
    LinksOnly,
    CurrentPageOnly,
    LinkedTrail,
}

impl BreadcrumbAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::ItemOnly => "item-only",
            Self::LinksOnly => "links-only",
            Self::CurrentPageOnly => "current-page-only",
            Self::LinkedTrail => "linked-trail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentSource {
    #[default]
    DefaultOnly,
    I18nFallback,
    Customized,
    Mixed,
}

impl BreadcrumbAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultOnly => "default-only",
            Self::I18nFallback => "i18n-fallback",
            Self::Customized => "customized",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentRenderMode {
    Streaming,
    #[default]
    Snapshot,
}

impl BreadcrumbAgentRenderMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentStreamSupport {
    Required,
    #[default]
    Optional,
}

impl BreadcrumbAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentStreamFallback {
    #[default]
    Snapshot,
}

impl BreadcrumbAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BreadcrumbAgentOutputStatus {
    Draft,
    #[default]
    Verified,
    Submittable,
}

impl BreadcrumbAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BreadcrumbAgentContract {
    pub schema_name: &'static str,
    pub schema_version: BreadcrumbAgentSchemaVersion,
    pub intent: BreadcrumbAgentIntent,
    pub action: BreadcrumbAgentAction,
    pub state: BreadcrumbAgentState,
    pub source: BreadcrumbAgentSource,
    pub render_mode: BreadcrumbAgentRenderMode,
    pub stream_support: BreadcrumbAgentStreamSupport,
    pub stream_fallback: BreadcrumbAgentStreamFallback,
    pub output_status: BreadcrumbAgentOutputStatus,
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
