pub type DirectionMode = ui_state_primitives::direction::DirectionMode;

pub enum DirectionSource {
    Direction,
    DirAlias,
    Default,
}

pub const DIRECTION_AGENT_SCHEMA: &str;

pub enum DirectionAgentIntent {
    ProvideDirection,
}

pub enum DirectionAgentAction {
    RenderSnapshot,
}

pub enum DirectionAgentState {
    Ltr,
    Rtl,
}

pub enum DirectionAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub enum DirectionAgentStreamSupport {
    Optional,
}

pub enum DirectionAgentStreamFallback {
    Snapshot,
}

pub struct DirectionAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: DirectionAgentIntent,
    pub action: DirectionAgentAction,
    pub state: DirectionMode,
    pub source: DirectionSource,
    pub stream_support: DirectionAgentStreamSupport,
    pub stream_fallback: DirectionAgentStreamFallback,
    pub output_status: DirectionAgentOutputStatus,
}

pub fn resolve_agent_contract(
    state: DirectionMode,
    source: DirectionSource,
) -> DirectionAgentContract;

pub fn DirectionProvider(
    direction: Option<DirectionMode>,
    dir: Option<DirectionMode>,
    lang: Option<String>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
