use crate::{HoverCardMotion, HoverCardPartState, HoverCardPartStateInput, HoverCardSlot};
use leptos::prelude::*;
use std::borrow::Cow;
use ui_state_primitives::hover_card as hover_card_state;

pub const DEFAULT_OPEN_DELAY_MS: u64 = hover_card_state::DEFAULT_OPEN_DELAY_MS;
pub const DEFAULT_CLOSE_DELAY_MS: u64 = hover_card_state::DEFAULT_CLOSE_DELAY_MS;

pub fn state_attr_for_open(is_open: bool) -> &'static str {
    hover_card_state::state_attr_for_open(is_open)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    hover_card_state::normalize_optional_text(value)
}

pub fn resolve_id(custom_id: Option<String>, fallback_id: Cow<'static, str>) -> (String, bool) {
    hover_card_state::resolve_id(custom_id, fallback_id)
}

pub fn has_custom_delays(open_delay_ms: u64, close_delay_ms: u64) -> bool {
    hover_card_state::has_custom_delays(open_delay_ms, close_delay_ms)
}

pub struct DelayStateInput {
    pub open_delay_ms: Option<u64>,
    pub close_delay_ms: Option<u64>,
}

pub struct DelayState {
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub has_custom_delays: bool,
}

pub fn normalize_delay_state(input: DelayStateInput) -> DelayState {
    let open_delay_ms = input.open_delay_ms.unwrap_or(DEFAULT_OPEN_DELAY_MS);
    let close_delay_ms = input.close_delay_ms.unwrap_or(DEFAULT_CLOSE_DELAY_MS);
    let has_custom_delays = has_custom_delays(open_delay_ms, close_delay_ms);

    DelayState {
        open_delay_ms,
        close_delay_ms,
        has_custom_delays,
    }
}

pub fn is_custom_motion(motion: HoverCardMotion) -> bool {
    motion != HoverCardMotion::default()
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

pub struct OpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct OpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub is_controlled: bool,
}

pub fn normalize_open_state(input: OpenStateInput) -> OpenState {
    let open = input.is_open.or(input.open);
    OpenState {
        is_controlled: open.is_some(),
        open,
        default_open: input.default_open,
        on_open_change: input.on_open_change,
    }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn open_value_source_attr(is_controlled: bool) -> &'static str {
    if is_controlled { "external" } else { "default" }
}

pub const fn open_intent_source_attr() -> &'static str {
    "interaction"
}

pub struct PartStatesInput {
    pub class_name: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub motion: HoverCardMotion,
    pub has_custom_delays: bool,
    pub has_custom_id: bool,
}

pub struct PartStates {
    pub root_state: HoverCardPartState,
    pub root_class: String,
    pub trigger_state: HoverCardPartState,
    pub trigger_class: String,
    pub panel_state: HoverCardPartState,
    pub panel_class: String,
}

pub fn normalize_part_states(input: PartStatesInput) -> PartStates {
    let has_custom_motion = is_custom_motion(input.motion);
    let has_custom_class_name = input.class_name.is_some();

    let root_state = resolve_part_state(HoverCardPartStateInput {
        slot: HoverCardSlot::Root,
        open: input.is_open,
        disabled: input.is_disabled,
        has_custom_class_name,
        has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
    });
    let trigger_state = resolve_part_state(HoverCardPartStateInput {
        slot: HoverCardSlot::Trigger,
        open: false,
        disabled: input.is_disabled,
        has_custom_class_name: false,
        has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
    });
    let panel_state = resolve_part_state(HoverCardPartStateInput {
        slot: HoverCardSlot::Panel,
        open: false,
        disabled: input.is_disabled,
        has_custom_class_name: false,
        has_custom_motion,
        has_custom_delays: input.has_custom_delays,
        has_custom_id: input.has_custom_id,
    });

    PartStates {
        root_class: compose_class_name(input.class_name, root_state),
        trigger_class: compose_class_name(None, trigger_state),
        panel_class: compose_class_name(None, panel_state),
        root_state,
        trigger_state,
        panel_state,
    }
}

pub fn resolve_part_state(input: HoverCardPartStateInput) -> HoverCardPartState {
    hover_card_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: HoverCardPartState) -> String {
    hover_card_state::compose_class_name(base_class_name, state)
}

pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String {
    hover_card_state::compose_panel_vars(top_px, left_px, anchor_width_px)
}

pub const HOVER_CARD_AGENT_SCHEMA: &str = "ui.hover_card.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentSchemaVersion {
    V1,
}

impl HoverCardAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentIntent {
    OverlayHint,
}

impl HoverCardAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayHint => "overlay-hint",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentAction {
    Open,
    Close,
}

impl HoverCardAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Close => "close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentState {
    Open,
    Closed,
}

impl HoverCardAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentSource {
    Controlled,
    Uncontrolled,
}

impl HoverCardAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentConfigPolicy {
    Whitelist,
}

impl HoverCardAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoverCardAgentOutputStatus {
    Verified,
}

impl HoverCardAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardAgentCapabilities {
    pub can_open: bool,
    pub can_close: bool,
    pub has_panel: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardAgentContractInput {
    pub is_open: bool,
    pub is_controlled: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoverCardAgentContract {
    pub schema_name: &'static str,
    pub schema_version: HoverCardAgentSchemaVersion,
    pub intent: HoverCardAgentIntent,
    pub action: HoverCardAgentAction,
    pub state: HoverCardAgentState,
    pub source: HoverCardAgentSource,
    pub config_policy: HoverCardAgentConfigPolicy,
    pub output_status: HoverCardAgentOutputStatus,
    pub capabilities: HoverCardAgentCapabilities,
}

pub fn resolve_agent_contract(input: HoverCardAgentContractInput) -> HoverCardAgentContract {
    let source = if input.is_controlled {
        HoverCardAgentSource::Controlled
    } else {
        HoverCardAgentSource::Uncontrolled
    };
    let state = if input.is_open {
        HoverCardAgentState::Open
    } else {
        HoverCardAgentState::Closed
    };
    let action = if input.is_open {
        HoverCardAgentAction::Open
    } else {
        HoverCardAgentAction::Close
    };

    HoverCardAgentContract {
        schema_name: HOVER_CARD_AGENT_SCHEMA,
        schema_version: HoverCardAgentSchemaVersion::V1,
        intent: HoverCardAgentIntent::OverlayHint,
        action,
        state,
        source,
        config_policy: HoverCardAgentConfigPolicy::Whitelist,
        output_status: HoverCardAgentOutputStatus::Verified,
        capabilities: HoverCardAgentCapabilities {
            can_open: !input.is_disabled,
            can_close: true,
            has_panel: true,
        },
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
