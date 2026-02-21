pub use crate::{
    CollapsibleAgentAction, CollapsibleAgentContract, CollapsibleAgentContractInput,
    CollapsibleAgentIntent, CollapsibleAgentOutputStatus, CollapsibleAgentSchemaVersion,
    CollapsibleAgentSource, CollapsibleAgentState, CollapsibleAgentStreamFallback,
    CollapsibleAgentStreamMode, CollapsibleAgentStreamSupport, CollapsibleMotion, CollapsibleState,
    CollapsibleStateInput,
};

pub const COLLAPSIBLE_AGENT_SCHEMA: &str;

pub enum CollapsibleAgentSchemaVersion {
    V1,
}

impl CollapsibleAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentIntent {
    CollapsibleInteraction,
}

impl CollapsibleAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentAction {
    Toggle,
}

impl CollapsibleAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentState {
    Open,
    Closed,
    Disabled,
}

impl CollapsibleAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentSource {
    StatePrimitives,
}

impl CollapsibleAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentOutputStatus {
    Verified,
}

impl CollapsibleAgentOutputStatus {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentStreamSupport {
    Unsupported,
}

impl CollapsibleAgentStreamSupport {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentStreamFallback {
    Snapshot,
}

impl CollapsibleAgentStreamFallback {
    pub const fn as_str(self) -> &'static str;
}

pub enum CollapsibleAgentStreamMode {
    Streaming,
    Snapshot,
}

impl CollapsibleAgentStreamMode {
    pub const fn as_str(self) -> &'static str;
}

pub struct CollapsibleAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CollapsibleAgentSchemaVersion,
    pub intent: CollapsibleAgentIntent,
    pub action: CollapsibleAgentAction,
    pub state: CollapsibleAgentState,
    pub source: CollapsibleAgentSource,
    pub output_status: CollapsibleAgentOutputStatus,
    pub stream_support: CollapsibleAgentStreamSupport,
    pub stream_fallback: CollapsibleAgentStreamFallback,
    pub stream_mode: CollapsibleAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub open_value_source: &'static str,
    pub open_change_source: &'static str,
    pub config_policy: &'static str,
}

pub struct CollapsibleAgentContractInput {
    pub render_state: CollapsibleState,
}

pub fn resolve_agent_contract(input: CollapsibleAgentContractInput) -> CollapsibleAgentContract;

pub fn Collapsible(
    id_base: String,
    title: String,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    is_disabled: Option<bool>,
    disabled: bool,
    motion: CollapsibleMotion,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<String>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView;
