pub const COMPONENT_ID: &str;

pub enum ContextualHelpVariant {
    Help,
    Info,
}

impl ContextualHelpVariant {
    pub fn default_label(self) -> &'static str;
    pub fn class_name(self) -> &'static str;
    pub fn as_attr(self) -> &'static str;
}

pub struct ContextualHelpMotion {
    pub popover: crate::popover::PopoverMotion,
}

pub const CONTEXTUAL_HELP_AGENT_SCHEMA: &str;

pub enum ContextualHelpLlmOutputMode {
    Streaming,
    Snapshot,
}

pub const CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE: ContextualHelpLlmOutputMode;

pub enum ContextualHelpStreamingRequirement {
    Required,
    Optional,
}

pub struct ContextualHelpStreamingPolicy {
    pub requirement: ContextualHelpStreamingRequirement,
    pub fallback_mode: ContextualHelpLlmOutputMode,
}

pub enum ContextualHelpLlmOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub enum ContextualHelpAgentIntent {
    Help,
    Info,
}

pub enum ContextualHelpAgentAction {
    Idle,
    ToggleOpen,
    Dismiss,
    ExternalSync,
}

pub enum ContextualHelpAgentState {
    Open,
    Closed,
}

pub struct ContextualHelpAgentContract {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub state: &'static str,
    pub source: &'static str,
}

pub fn resolve_agent_contract(
    variant: ContextualHelpVariant,
    source: ui_state_primitives::contextual_help::ContextualHelpOpenInteractionSource,
    is_open: bool,
) -> ContextualHelpAgentContract;

pub fn resolve_llm_output_mode(is_streaming: bool) -> ContextualHelpLlmOutputMode;
pub fn resolve_streaming_policy(is_reader_surface: bool) -> ContextualHelpStreamingPolicy;
pub fn resolve_llm_output_status(output_mode: ContextualHelpLlmOutputMode)
    -> ContextualHelpLlmOutputStatus;

pub fn ContextualHelp(
    children: leptos::children::ChildrenFn,
    variant: ContextualHelpVariant,
    aria_label: Option<String>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    placement: ui_headless::PopoverPlacement,
    motion: ContextualHelpMotion,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    heading: Option<String>,
    footer: Option<leptos::children::ViewFn>,
    class_name: Option<String>,
    id: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
