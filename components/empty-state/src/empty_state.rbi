pub const DEFAULT_TITLE: &str;
pub const DEFAULT_DESCRIPTION: &str;
pub const DEFAULT_ARIA_LABEL: &str;
pub const EMPTY_STATE_AGENT_SCHEMA_NAME: &str;
pub const EMPTY_STATE_AGENT_SCHEMA_VERSION: &str;

pub enum EmptyStateTone {
    Default,
    Muted,
    Accent,
}

pub enum EmptyStateAlign {
    Start,
    Center,
}

pub struct EmptyStateMotion {
    pub animate_in: bool,
}

pub struct EmptyStateStrings {
    pub default_title: std::borrow::Cow<'static, str>,
    pub default_description: std::borrow::Cow<'static, str>,
    pub default_aria_label: std::borrow::Cow<'static, str>,
}

pub enum EmptyStateAgentIntent {
    Informative,
    Actionable,
}

pub enum EmptyStateAgentAction {
    Render,
}

pub enum EmptyStateAgentSource {
    Default,
    Custom,
}

pub enum EmptyStateStreamingSupport {
    Optional,
}

pub enum EmptyStateRenderMode {
    Snapshot,
}

pub enum EmptyStateOutputStatus {
    Validated,
}

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
    state: ui_state_primitives::empty_state::EmptyStateState,
    motion_source_attr: &'static str,
) -> EmptyStateAgentContract;

pub fn EmptyState(
    title: Option<String>,
    description: Option<String>,
    tone: EmptyStateTone,
    align: EmptyStateAlign,
    is_compact: bool,
    is_bordered: bool,
    aria_label: Option<String>,
    class_name: Option<String>,
    motion: EmptyStateMotion,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    icon: Option<leptos::children::ViewFn>,
    actions: Option<leptos::children::ViewFn>,
) -> impl leptos::prelude::IntoView;
