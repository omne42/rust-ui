use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/icon/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IconComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IconComponentSpec {
    #[serde(default)]
    pub schema_version: IconComponentSchemaVersion,
}

pub const ICON_AGENT_SCHEMA: &str = "ui.icon.agent-contract.v1";
const _: IconComponentSchemaVersion = IconComponentSchemaVersion::V1;
const _: IconComponentSpec = IconComponentSpec {
    schema_version: IconComponentSchemaVersion::V1,
};
const _: IconStreamingRequirement = IconStreamingRequirement::Required;
const _: IconOutputMode = IconOutputMode::Streaming;
const _: IconOutputStatus = IconOutputStatus::Draft;
const _: IconOutputStatus = IconOutputStatus::Submittable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAgentSchemaVersion {
    V1,
}

impl IconAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAgentIntent {
    IconRender,
    IconsResolve,
    IconsetResolve,
    IconsUiResolve,
    IconsWorkflowResolve,
}

impl IconAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IconRender => "display.icon.render",
            Self::IconsResolve => "display.icons.resolve",
            Self::IconsetResolve => "display.iconset.resolve",
            Self::IconsUiResolve => "display.icons-ui.resolve",
            Self::IconsWorkflowResolve => "display.icons-workflow.resolve",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAgentAction {
    Render,
}

impl IconAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Render => "render",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAgentState {
    Default,
    Labeled,
    Disabled,
    Decorative,
    Fallback,
}

impl IconAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Labeled => "labeled",
            Self::Disabled => "disabled",
            Self::Decorative => "decorative",
            Self::Fallback => "fallback",
        }
    }

    pub fn from_state_attr(state_attr: &str) -> Self {
        match state_attr {
            "labeled" => Self::Labeled,
            "disabled" => Self::Disabled,
            "decorative" => Self::Decorative,
            "fallback" => Self::Fallback,
            _ => Self::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAgentSource {
    Default,
    Custom,
    Prop,
    Name,
    Icon,
    Registry,
    Fallback,
    Explicit,
    Prefixed,
}

impl IconAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
            Self::Prop => "prop",
            Self::Name => "name",
            Self::Icon => "icon",
            Self::Registry => "registry",
            Self::Fallback => "fallback",
            Self::Explicit => "explicit",
            Self::Prefixed => "prefixed",
        }
    }

    pub fn from_source_attr(source_attr: &str) -> Self {
        match source_attr {
            "custom" => Self::Custom,
            "prop" => Self::Prop,
            "name" => Self::Name,
            "icon" => Self::Icon,
            "registry" => Self::Registry,
            "fallback" => Self::Fallback,
            "explicit" => Self::Explicit,
            "prefixed" => Self::Prefixed,
            _ => Self::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconAgentInput {
    pub intent: IconAgentIntent,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconAgentDataAttrs {
    pub schema_name: &'static str,
    pub schema_version: IconAgentSchemaVersion,
    pub intent: IconAgentIntent,
    pub action: IconAgentAction,
    pub state: IconAgentState,
    pub source: IconAgentSource,
}

pub fn resolve_agent_data_attrs(input: IconAgentInput) -> IconAgentDataAttrs {
    IconAgentDataAttrs {
        schema_name: ICON_AGENT_SCHEMA,
        schema_version: IconAgentSchemaVersion::V1,
        intent: input.intent,
        action: IconAgentAction::Render,
        state: IconAgentState::from_state_attr(input.state_attr),
        source: IconAgentSource::from_source_attr(input.source_attr),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconStreamingRequirement {
    Required,
    Optional,
}

impl IconStreamingRequirement {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconOutputMode {
    Snapshot,
    Streaming,
}

impl IconOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Streaming => "streaming",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl IconOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconOutputDataAttrs {
    pub streaming: IconStreamingRequirement,
    pub fallback: IconOutputMode,
    pub mode: IconOutputMode,
    pub status: IconOutputStatus,
}

pub const fn resolve_output_data_attrs() -> IconOutputDataAttrs {
    IconOutputDataAttrs {
        streaming: IconStreamingRequirement::Optional,
        fallback: IconOutputMode::Snapshot,
        mode: IconOutputMode::Snapshot,
        status: IconOutputStatus::Verified,
    }
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
