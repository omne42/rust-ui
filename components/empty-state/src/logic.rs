use std::borrow::Cow;

use crate::EmptyStateMotion;

pub use ui_state_primitives::empty_state::{
    DEFAULT_ARIA_LABEL, DEFAULT_DESCRIPTION, DEFAULT_TITLE, EmptyStateAlign, EmptyStateState,
    EmptyStateStateInput, EmptyStateTone, compose_class_name, normalize_aria_label,
    normalize_description, normalize_optional_text, normalize_title, resolve_state,
};

#[derive(Clone, Debug)]
pub struct EmptyStateStrings {
    pub default_title: Cow<'static, str>,
    pub default_description: Cow<'static, str>,
    pub default_aria_label: Cow<'static, str>,
}

impl Default for EmptyStateStrings {
    fn default() -> Self {
        Self {
            default_title: Cow::Borrowed(DEFAULT_TITLE),
            default_description: Cow::Borrowed(DEFAULT_DESCRIPTION),
            default_aria_label: Cow::Borrowed(DEFAULT_ARIA_LABEL),
        }
    }
}

pub const EMPTY_STATE_AGENT_SCHEMA_NAME: &str = "ui-empty-state-agent-contract";
pub const EMPTY_STATE_AGENT_SCHEMA_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateAgentIntent {
    Informative,
    Actionable,
}

impl EmptyStateAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Informative => "informative",
            Self::Actionable => "actionable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateAgentAction {
    Render,
}

impl EmptyStateAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateAgentSource {
    Default,
    Custom,
}

impl EmptyStateAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateStreamingSupport {
    Optional,
}

impl EmptyStateStreamingSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateRenderMode {
    Snapshot,
}

impl EmptyStateRenderMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyStateOutputStatus {
    Validated,
}

impl EmptyStateOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Validated => "validated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: EmptyStateAgentIntent,
    pub action: EmptyStateAgentAction,
    pub state: &'static str,
    pub source: EmptyStateAgentSource,
    pub streaming_support: EmptyStateStreamingSupport,
    pub render_mode: EmptyStateRenderMode,
    pub fallback_mode: EmptyStateRenderMode,
    pub output_status: EmptyStateOutputStatus,
}

pub fn resolve_agent_contract(
    state: EmptyStateState,
    motion_source_attr: &'static str,
) -> EmptyStateAgentContract {
    let intent = if state.has_actions {
        EmptyStateAgentIntent::Actionable
    } else {
        EmptyStateAgentIntent::Informative
    };

    let has_custom_source = state.title_source_attr == "custom"
        || state.description_source_attr == "custom"
        || state.aria_source_attr == "custom"
        || state.class_source_attr == "custom"
        || motion_source_attr == "custom";
    let source = if has_custom_source {
        EmptyStateAgentSource::Custom
    } else {
        EmptyStateAgentSource::Default
    };

    EmptyStateAgentContract {
        schema_name: EMPTY_STATE_AGENT_SCHEMA_NAME,
        schema_version: EMPTY_STATE_AGENT_SCHEMA_VERSION,
        intent,
        action: EmptyStateAgentAction::Render,
        state: state.data_state_attr,
        source,
        streaming_support: EmptyStateStreamingSupport::Optional,
        render_mode: EmptyStateRenderMode::Snapshot,
        fallback_mode: EmptyStateRenderMode::Snapshot,
        output_status: EmptyStateOutputStatus::Validated,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmptyStateResolvedDefaults {
    pub title: String,
    pub description: String,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn resolve_defaults(
    title: Option<String>,
    description: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
    default_title: &str,
    default_description: &str,
    default_aria_label: &str,
) -> EmptyStateResolvedDefaults {
    let (title, has_custom_title) = normalize_title(title, default_title);
    let (description, has_custom_description) =
        normalize_description(description, default_description);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(aria_label, default_aria_label);
    let class_name = normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    EmptyStateResolvedDefaults {
        title,
        description,
        aria_label,
        class_name,
        has_custom_title,
        has_custom_description,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateRenderStateInput {
    pub tone: EmptyStateTone,
    pub align: EmptyStateAlign,
    pub is_compact: bool,
    pub is_bordered: bool,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub motion: EmptyStateMotion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyStateRenderState {
    pub state: EmptyStateState,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
    pub agent_contract: EmptyStateAgentContract,
}

pub fn resolve_render_state(input: EmptyStateRenderStateInput) -> EmptyStateRenderState {
    let state = resolve_state(EmptyStateStateInput {
        tone: input.tone,
        align: input.align,
        compact: input.is_compact,
        bordered: input.is_bordered,
        has_icon: input.has_icon,
        has_actions: input.has_actions,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    });

    let has_custom_motion = input.motion != EmptyStateMotion::default();
    let motion_source_attr = if has_custom_motion {
        "custom"
    } else {
        "default"
    };

    let agent_contract = resolve_agent_contract(state, motion_source_attr);

    EmptyStateRenderState {
        state,
        motion_source_attr,
        has_custom_motion,
        agent_contract,
    }
}
