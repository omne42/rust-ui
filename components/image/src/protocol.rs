use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/image/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
pub const IMAGE_AGENT_SCHEMA: &str = "ui.image.agent-contract/v1";
const _: ImageComponentSchemaVersion = ImageComponentSchemaVersion::V1;
const _: ImageComponentSpec = ImageComponentSpec {
    schema_version: ImageComponentSchemaVersion::V1,
};
const _: ImageLlmRenderMode = ImageLlmRenderMode::Streaming;
const _: ImageOutputStatus = ImageOutputStatus::Draft;
const _: ImageOutputStatus = ImageOutputStatus::Submittable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ImageComponentSpec {
    #[serde(default)]
    pub schema_version: ImageComponentSchemaVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageAgentIntent {
    Display,
}

impl ImageAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Display => "display",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageAgentAction {
    InitialRender,
    ResourceEvent,
}

impl ImageAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::InitialRender => "initial-render",
            Self::ResourceEvent => "resource-event",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageAgentPropSource {
    ExternalProp,
}

impl ImageAgentPropSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ExternalProp => "external-prop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageContentSource {
    Primary,
    Fallback,
    Empty,
}

impl ImageContentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Fallback => "fallback",
            Self::Empty => "empty",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageStreamSupport {
    Optional,
}

impl ImageStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageStreamFallback {
    Snapshot,
}

impl ImageStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

/// LLM output render mode axis is intentionally closed to two modes only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageLlmRenderMode {
    Streaming,
    Snapshot,
}

impl ImageLlmRenderMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl ImageOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

pub const fn action_from_status_source(
    source: crate::logic::ImageStatusSource,
) -> ImageAgentAction {
    match source {
        crate::logic::ImageStatusSource::Initial => ImageAgentAction::InitialRender,
        crate::logic::ImageStatusSource::Event => ImageAgentAction::ResourceEvent,
    }
}

pub const fn content_source_from_view_state(state: crate::ImageViewState) -> ImageContentSource {
    if state.show_image {
        ImageContentSource::Primary
    } else if state.show_fallback {
        ImageContentSource::Fallback
    } else {
        ImageContentSource::Empty
    }
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
