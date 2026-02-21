pub type IllustratedMessageViewState = ui_state_primitives::illustrated_message::IllustratedMessageViewState;
pub type A11yDirection = ui_headless::a11y::A11yDirection;

pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA: &str;
pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION: &str;

pub enum IllustratedMessageOrientation {
    Vertical,
    Horizontal,
}

pub enum IllustratedMessageAgentIntent {
    EmptyStateDisplay,
}

pub enum IllustratedMessageAgentAction {
    RenderSnapshot,
}

pub enum IllustratedMessageAgentState {
    Empty,
    Populated,
}

pub enum IllustratedMessageAgentSource {
    Default,
    Custom,
}

pub enum IllustratedMessageAgentConfigPolicy {
    Whitelist,
}

pub enum IllustratedMessageAgentStreamingPolicy {
    Optional,
}

pub enum IllustratedMessageAgentStreamingFallback {
    Snapshot,
}

pub enum IllustratedMessageAgentOutputStatus {
    Validated,
}

pub struct IllustratedMessageAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub config_policy_attr: &'static str,
    pub streaming_policy_attr: &'static str,
    pub streaming_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub struct IllustratedMessageMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_y_px: f64,
}

pub fn resolve_agent_contract_attrs(
    resolved_view: &crate::logic::IllustratedMessageResolvedView,
) -> IllustratedMessageAgentContractAttrs;

pub fn sanitize_motion(
    motion: crate::motion::IllustratedMessageMotion,
) -> crate::motion::IllustratedMessageMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: crate::motion::IllustratedMessageMotion,
);

pub fn IllustratedMessage(
    title: Option<String>,
    description: Option<String>,
    illustration: Option<leptos::children::ViewFn>,
    actions: Option<leptos::children::ViewFn>,
    orientation: crate::IllustratedMessageOrientation,
    motion: crate::IllustratedMessageMotion,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;
