pub use crate::motion::DropZoneMotion;
pub use crate::DroppedFile;
pub use ui_headless::A11yDirection;

pub struct DroppedFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

pub struct DropZoneMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub drop_scale: f64,
    pub hover_highlight: f64,
}

pub const DROP_ZONE_AGENT_SCHEMA: &str;

pub enum DropZoneAgentSchemaVersion {
    V1,
}

pub enum DropZoneAgentIntent {
    FileIngestion,
}

pub enum DropZoneAgentAction {
    AwaitInput,
    CaptureDrop,
    Blocked,
}

pub enum DropZoneAgentState {
    Idle,
    Dragging,
    Disabled,
}

pub enum DropZoneAgentSource {
    IsDisabled,
    DisabledAlias,
    Default,
}

pub enum DropZoneAgentConfigPolicy {
    Whitelist,
}

pub enum DropZoneAgentOutputStatus {
    Verified,
}

pub struct DropZoneAgentCapabilities {
    pub can_drop: bool,
    pub can_paste: bool,
    pub has_drop_callback: bool,
}

pub struct DropZoneAgentContractInput {
    pub drag_phase: crate::logic::DragLifecyclePhase,
    pub is_disabled: bool,
    pub disabled_source: crate::logic::DisabledSource,
    pub motion_source: crate::logic::MotionSource,
    pub aria_source: crate::logic::AriaLabelSource,
    pub has_drop_callback: bool,
}

pub struct DropZoneAgentContract {
    pub schema_name: &'static str,
    pub schema_version: crate::logic::DropZoneAgentSchemaVersion,
    pub intent: crate::logic::DropZoneAgentIntent,
    pub action: crate::logic::DropZoneAgentAction,
    pub state: crate::logic::DropZoneAgentState,
    pub source: crate::logic::DropZoneAgentSource,
    pub config_policy: crate::logic::DropZoneAgentConfigPolicy,
    pub output_status: crate::logic::DropZoneAgentOutputStatus,
    pub capabilities: crate::logic::DropZoneAgentCapabilities,
    pub motion_source: crate::logic::MotionSource,
    pub aria_source: crate::logic::AriaLabelSource,
}

pub fn resolve_agent_contract(
    input: crate::logic::DropZoneAgentContractInput,
) -> crate::logic::DropZoneAgentContract;

pub fn sanitize_motion(motion: crate::motion::DropZoneMotion) -> crate::motion::DropZoneMotion;

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_drop_target: leptos::prelude::ReadSignal<bool>,
    is_focused: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: crate::motion::DropZoneMotion,
);

pub fn DropZone(
    label: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    motion: Option<crate::motion::DropZoneMotion>,
    on_drop_files: Option<leptos::prelude::Callback<Vec<crate::DroppedFile>>>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
