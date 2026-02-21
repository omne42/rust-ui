use std::borrow::Cow;

pub use ui_state_primitives::direction::{DirectionMode, normalize_optional_text};

pub const DIRECTION_AGENT_SCHEMA_NAME: &str = "ui-direction";
pub const DIRECTION_AGENT_SCHEMA_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionPropSource {
    Direction,
    DirAlias,
    Default,
}

impl DirectionPropSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Direction => "direction",
            Self::DirAlias => "dir-alias",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentIntent {
    ProvideDirectionContext,
}

impl DirectionAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ProvideDirectionContext => "provide-direction-context",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentAction {
    RenderSnapshot,
}

impl DirectionAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentSource {
    Direction,
    DirAlias,
    Default,
}

impl DirectionAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Direction => "direction",
            Self::DirAlias => "dir-alias",
            Self::Default => "default",
        }
    }
}

impl From<DirectionPropSource> for DirectionAgentSource {
    fn from(value: DirectionPropSource) -> Self {
        match value {
            DirectionPropSource::Direction => Self::Direction,
            DirectionPropSource::DirAlias => Self::DirAlias,
            DirectionPropSource::Default => Self::Default,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentStreamSupport {
    Optional,
}

impl DirectionAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentStreamFallback {
    Snapshot,
}

impl DirectionAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionAgentOutputStatus {
    Verified,
}

impl DirectionAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectionAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: DirectionAgentIntent,
    pub action: DirectionAgentAction,
    pub state: DirectionMode,
    pub source: DirectionAgentSource,
    pub stream_support: DirectionAgentStreamSupport,
    pub stream_fallback: DirectionAgentStreamFallback,
    pub output_status: DirectionAgentOutputStatus,
}

pub fn resolve_agent_contract(
    state: DirectionMode,
    source: DirectionPropSource,
) -> DirectionAgentContract {
    DirectionAgentContract {
        schema_name: DIRECTION_AGENT_SCHEMA_NAME,
        schema_version: DIRECTION_AGENT_SCHEMA_VERSION,
        intent: DirectionAgentIntent::ProvideDirectionContext,
        action: DirectionAgentAction::RenderSnapshot,
        state,
        source: source.into(),
        stream_support: DirectionAgentStreamSupport::Optional,
        stream_fallback: DirectionAgentStreamFallback::Snapshot,
        output_status: DirectionAgentOutputStatus::Verified,
    }
}

pub fn resolve_direction(
    direction: Option<DirectionMode>,
    dir: Option<DirectionMode>,
) -> (DirectionMode, DirectionPropSource) {
    if let Some(direction) = direction {
        (direction, DirectionPropSource::Direction)
    } else if let Some(direction) = dir {
        (direction, DirectionPropSource::DirAlias)
    } else {
        (DirectionMode::default(), DirectionPropSource::Default)
    }
}

pub fn compose_class_name(class_name: Option<String>) -> Cow<'static, str> {
    normalize_optional_text(class_name)
        .map(|class_name| Cow::Owned(format!("ui-direction-provider {class_name}")))
        .unwrap_or(Cow::Borrowed("ui-direction-provider"))
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
