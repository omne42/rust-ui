use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/code-block`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeBlockComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeBlockComponentSpec {
    #[serde(default)]
    pub schema_version: CodeBlockComponentSchemaVersion,
}

pub const CODE_BLOCK_AGENT_SCHEMA: &str = "ui.code-block.contract.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentSchema {
    V1,
}

impl CodeBlockAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => CODE_BLOCK_AGENT_SCHEMA,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentIntent {
    DisplayCode,
}

impl CodeBlockAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::DisplayCode => "display-code",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentAction {
    CopyCode,
}

impl CodeBlockAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::CopyCode => "copy-code",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentState {
    Idle,
    Copied,
    CopyLoading,
    CopyError,
}

impl CodeBlockAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Copied => "copied",
            Self::CopyLoading => "copy-loading",
            Self::CopyError => "copy-error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentSource {
    Controlled,
    Uncontrolled,
}

impl CodeBlockAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentCopyableSource {
    Default,
    IsCopyable,
    CopyableLegacy,
}

impl CodeBlockAgentCopyableSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::IsCopyable => "is_copyable",
            Self::CopyableLegacy => "copyable_legacy",
        }
    }

    pub fn from_attr(attr: &str) -> Self {
        match attr {
            "is_copyable" => Self::IsCopyable,
            "copyable_legacy" => Self::CopyableLegacy,
            _ => Self::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentCopiedSource {
    Controlled,
    Uncontrolled,
}

impl CodeBlockAgentCopiedSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }

    pub fn from_attr(attr: &str) -> Self {
        match attr {
            "controlled" => Self::Controlled,
            _ => Self::Uncontrolled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentMotionSource {
    Default,
    Custom,
}

impl CodeBlockAgentMotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub fn from_attr(attr: &str) -> Self {
        match attr {
            "custom" => Self::Custom,
            _ => Self::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentOutputMode {
    Streaming,
    Snapshot,
}

impl CodeBlockAgentOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeBlockAgentOutputStatus {
    Draft,
    Validated,
    ReadyToSubmit,
}

impl CodeBlockAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Validated => "validated",
            Self::ReadyToSubmit => "ready-to-submit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockRenderPolicy {
    pub allow_inner_html: bool,
    pub allow_script_injection: bool,
    pub output_status: CodeBlockAgentOutputStatus,
}

pub const fn render_policy() -> CodeBlockRenderPolicy {
    CodeBlockRenderPolicy {
        allow_inner_html: false,
        allow_script_injection: false,
        output_status: CodeBlockAgentOutputStatus::Validated,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockAgentInput {
    pub copied: bool,
    pub is_loading: bool,
    pub has_error: bool,
    pub output_mode: CodeBlockAgentOutputMode,
    pub output_status: CodeBlockAgentOutputStatus,
    pub copyable_source: CodeBlockAgentCopyableSource,
    pub copied_source: CodeBlockAgentCopiedSource,
    pub motion_source: CodeBlockAgentMotionSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockAgentDataAttrs {
    pub schema: CodeBlockAgentSchema,
    pub intent: CodeBlockAgentIntent,
    pub action: CodeBlockAgentAction,
    pub state: CodeBlockAgentState,
    pub source: CodeBlockAgentSource,
    pub source_copyable: CodeBlockAgentCopyableSource,
    pub source_copied: CodeBlockAgentCopiedSource,
    pub source_motion: CodeBlockAgentMotionSource,
    pub output_mode: CodeBlockAgentOutputMode,
    pub output_status: CodeBlockAgentOutputStatus,
}

pub fn resolve_agent_data_attrs(input: CodeBlockAgentInput) -> CodeBlockAgentDataAttrs {
    let state = if input.has_error {
        CodeBlockAgentState::CopyError
    } else if input.is_loading {
        CodeBlockAgentState::CopyLoading
    } else if input.copied {
        CodeBlockAgentState::Copied
    } else {
        CodeBlockAgentState::Idle
    };

    let source = match input.copied_source {
        CodeBlockAgentCopiedSource::Controlled => CodeBlockAgentSource::Controlled,
        CodeBlockAgentCopiedSource::Uncontrolled => CodeBlockAgentSource::Uncontrolled,
    };

    CodeBlockAgentDataAttrs {
        schema: CodeBlockAgentSchema::V1,
        intent: CodeBlockAgentIntent::DisplayCode,
        action: CodeBlockAgentAction::CopyCode,
        state,
        source,
        source_copyable: input.copyable_source,
        source_copied: input.copied_source,
        source_motion: input.motion_source,
        output_mode: input.output_mode,
        output_status: input.output_status,
    }
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
