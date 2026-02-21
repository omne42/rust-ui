pub type FlipCardMotion = crate::FlipCardMotion;
pub type FlipCardFlipMode = crate::FlipCardFlipMode;
pub type FlipCardPartState = ui_state_primitives::flip_card::FlipCardPartState;
pub type FlipCardPartStateInput = ui_state_primitives::flip_card::FlipCardPartStateInput;
pub type FlipCardSlot = ui_state_primitives::flip_card::FlipCardSlot;

pub const DEFAULT_DISABLED: bool;
pub const DEFAULT_FLIPPED: bool;
pub const DEFAULT_HOVER_FLIP: bool;
pub const DEFAULT_ID_PREFIX: &str;
pub const FLIP_CARD_AGENT_SCHEMA: &str;

pub enum FlipCardAgentSchemaVersion {
    V1,
}

pub enum FlipCardAgentIntent {
    FlipInteraction,
}

pub enum FlipCardAgentAction {
    SnapshotRender,
    Toggle,
    HoverEnter,
    HoverLeave,
    Focus,
    Blur,
}

pub enum FlipCardAgentState {
    Disabled,
    Flipped,
    Hovered,
    Default,
}

pub enum FlipCardAgentSource {
    StatePrimitives,
}

pub enum FlipCardAgentConfigPolicy {
    Whitelist,
}

pub struct FlipCardAgentContractInput {
    pub action: FlipCardAgentAction,
    pub state: FlipCardAgentState,
    pub flipped_source: &'static str,
    pub mode_source: &'static str,
    pub motion_source: &'static str,
    pub class_source: &'static str,
    pub id_source: &'static str,
}

pub struct FlipCardAgentContract {
    pub schema_name: &'static str,
    pub schema_version: FlipCardAgentSchemaVersion,
    pub intent: FlipCardAgentIntent,
    pub action: FlipCardAgentAction,
    pub state: FlipCardAgentState,
    pub source: FlipCardAgentSource,
    pub flipped_source: &'static str,
    pub mode_source: &'static str,
    pub motion_source: &'static str,
    pub class_source: &'static str,
    pub id_source: &'static str,
    pub config_policy: FlipCardAgentConfigPolicy,
}

pub fn sanitize_motion(motion: crate::FlipCardMotion) -> crate::FlipCardMotion;
pub fn resolve_agent_contract(input: FlipCardAgentContractInput) -> FlipCardAgentContract;

pub fn FlipCard(
    front: leptos::children::ViewFn,
    back: leptos::children::ViewFn,
    is_flipped: Option<leptos::prelude::Signal<bool>>,
    default_is_flipped: Option<bool>,
    default_flipped: Option<bool>,
    on_is_flipped_change: Option<leptos::prelude::Callback<bool>>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    flip_mode: Option<crate::FlipCardFlipMode>,
    is_flip_on_hover: Option<bool>,
    flip_on_hover: Option<bool>,
    motion: crate::FlipCardMotion,
    class_name: Option<String>,
    id: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
