pub use crate::motion::HoverCardMotion;
pub use ui_headless::PopoverPlacement;
pub use ui_state_primitives::hover_card::{HoverCardPartState, HoverCardPartStateInput, HoverCardSlot};

pub const DEFAULT_OPEN_DELAY_MS: u64;
pub const DEFAULT_CLOSE_DELAY_MS: u64;
pub const HOVER_CARD_AGENT_SCHEMA: &str;

pub struct HoverCardMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

pub fn sanitize_motion(motion: crate::motion::HoverCardMotion) -> crate::motion::HoverCardMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<ui_headless::PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: crate::motion::HoverCardMotion,
);

pub enum HoverCardComponentSchemaVersion {
    V1,
}

pub struct HoverCardComponentSpec {
    pub schema_version: crate::protocol::HoverCardComponentSchemaVersion,
}

pub enum HoverCardAgentSchemaVersion {
    V1,
}

pub enum HoverCardAgentIntent {
    OverlayHint,
}

pub enum HoverCardAgentAction {
    Open,
    Close,
}

pub enum HoverCardAgentState {
    Open,
    Closed,
}

pub enum HoverCardAgentSource {
    Controlled,
    Uncontrolled,
}

pub enum HoverCardAgentConfigPolicy {
    Whitelist,
}

pub enum HoverCardAgentOutputStatus {
    Verified,
}

pub struct HoverCardAgentCapabilities {
    pub can_open: bool,
    pub can_close: bool,
    pub has_panel: bool,
}

pub struct HoverCardAgentContractInput {
    pub is_open: bool,
    pub is_controlled: bool,
    pub is_disabled: bool,
}

pub struct HoverCardAgentContract {
    pub schema_name: &'static str,
    pub schema_version: crate::logic::HoverCardAgentSchemaVersion,
    pub intent: crate::logic::HoverCardAgentIntent,
    pub action: crate::logic::HoverCardAgentAction,
    pub state: crate::logic::HoverCardAgentState,
    pub source: crate::logic::HoverCardAgentSource,
    pub config_policy: crate::logic::HoverCardAgentConfigPolicy,
    pub output_status: crate::logic::HoverCardAgentOutputStatus,
    pub capabilities: crate::logic::HoverCardAgentCapabilities,
}

pub fn resolve_agent_contract(
    input: crate::logic::HoverCardAgentContractInput,
) -> crate::logic::HoverCardAgentContract;

pub fn HoverCard(
    content: leptos::children::ViewFn,
    children: leptos::children::Children,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    placement: ui_headless::PopoverPlacement,
    is_open: Option<leptos::prelude::Signal<bool>>,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    open_delay_ms: Option<u64>,
    close_delay_ms: Option<u64>,
    motion: crate::HoverCardMotion,
    class_name: Option<String>,
    id: Option<String>,
    lang: Option<String>,
    dir: Option<String>,
) -> impl leptos::prelude::IntoView;
