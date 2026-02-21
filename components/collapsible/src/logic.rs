use ui_headless::A11yDirection;

pub use ui_state_primitives::collapsible::{
    CollapsibleClassSource, CollapsibleLabelSource, CollapsibleMotionSource,
    CollapsibleOpenChangeSource, CollapsibleOpenMode, CollapsibleOpenState,
    CollapsibleOpenStateOptions, CollapsibleOpenValueSource, CollapsibleState, CollapsibleStatus,
    DEFAULT_ID_BASE, DEFAULT_TITLE, normalize_id_base, normalize_optional_text, resolve_aria_label,
    resolve_state, resolve_title, use_collapsible_open_state,
};

const _: &str = DEFAULT_ID_BASE;
const _: &str = DEFAULT_TITLE;
pub const COLLAPSIBLE_AGENT_SCHEMA: &str = "ui.collapsible.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentSchemaVersion {
    V1,
}

impl CollapsibleAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentIntent {
    CollapsibleInteraction,
}

impl CollapsibleAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CollapsibleInteraction => "collapsible.interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentAction {
    Toggle,
}

impl CollapsibleAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Toggle => "toggle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentState {
    Open,
    Closed,
    Disabled,
}

impl CollapsibleAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentSource {
    StatePrimitives,
}

impl CollapsibleAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentOutputStatus {
    Verified,
}

impl CollapsibleAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentStreamSupport {
    Unsupported,
}

impl CollapsibleAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentStreamFallback {
    Snapshot,
}

impl CollapsibleAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleAgentStreamMode {
    Streaming,
    Snapshot,
}

impl CollapsibleAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}
const _: [CollapsibleAgentStreamMode; 2] = [
    CollapsibleAgentStreamMode::Streaming,
    CollapsibleAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CollapsibleAgentContractInput {
    pub render_state: CollapsibleState,
}

fn resolve_agent_state(input: CollapsibleAgentContractInput) -> CollapsibleAgentState {
    match input.render_state.status {
        CollapsibleStatus::Open => CollapsibleAgentState::Open,
        CollapsibleStatus::Closed => CollapsibleAgentState::Closed,
        CollapsibleStatus::Disabled => CollapsibleAgentState::Disabled,
    }
}

pub fn resolve_agent_contract(input: CollapsibleAgentContractInput) -> CollapsibleAgentContract {
    CollapsibleAgentContract {
        schema_name: COLLAPSIBLE_AGENT_SCHEMA,
        schema_version: CollapsibleAgentSchemaVersion::V1,
        intent: CollapsibleAgentIntent::CollapsibleInteraction,
        action: CollapsibleAgentAction::Toggle,
        state: resolve_agent_state(input),
        source: CollapsibleAgentSource::StatePrimitives,
        output_status: CollapsibleAgentOutputStatus::Verified,
        stream_support: CollapsibleAgentStreamSupport::Unsupported,
        stream_fallback: CollapsibleAgentStreamFallback::Snapshot,
        stream_mode: CollapsibleAgentStreamMode::Snapshot,
        state_source: input.render_state.open_mode_attr,
        motion_source: input.render_state.motion_source_attr,
        open_value_source: input.render_state.open_value_source_attr,
        open_change_source: input.render_state.open_change_source_attr,
        config_policy: "whitelist",
    }
}

pub fn compose_class_name(class_name: Option<String>, state: CollapsibleState) -> String {
    let mut classes = vec![
        "ui-collapsible".to_string(),
        format!("ui-collapsible--state-{}", state.state_attr),
        format!("ui-collapsible--mode-{}", state.open_mode_attr),
    ];

    if state.motion_source.is_custom() {
        classes.push("ui-collapsible--custom-motion".to_string());
    }

    if state.class_source.is_custom() {
        classes.push("ui-collapsible--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

pub fn normalize_open_state_options(
    open: Option<bool>,
    default_open: Option<bool>,
) -> CollapsibleOpenStateOptions {
    // Precedence is explicit at the component logic boundary:
    // open (controlled) > default_open (uncontrolled seed) > primitive fallback.
    CollapsibleOpenStateOptions { open, default_open }
}

pub fn normalize_status(is_open: bool, is_disabled: bool) -> CollapsibleStatus {
    CollapsibleStatus::from_parts(is_open, is_disabled)
}

pub fn normalize_open_mode(is_controlled: bool) -> CollapsibleOpenMode {
    CollapsibleOpenMode::from_is_controlled(is_controlled)
}

pub fn normalize_label_source(has_custom_aria_label: bool) -> CollapsibleLabelSource {
    CollapsibleLabelSource::from_has_custom(has_custom_aria_label)
}

pub fn normalize_class_source(has_custom_class_name: bool) -> CollapsibleClassSource {
    CollapsibleClassSource::from_has_custom(has_custom_class_name)
}

pub fn normalize_motion_source(has_custom_motion: bool) -> CollapsibleMotionSource {
    CollapsibleMotionSource::from_has_custom(has_custom_motion)
}

pub fn normalize_open_value_source(
    open: Option<bool>,
    default_open: Option<bool>,
) -> CollapsibleOpenValueSource {
    CollapsibleOpenValueSource::from_input(open, default_open)
}

pub fn normalize_open_change_source(is_interaction: bool) -> CollapsibleOpenChangeSource {
    if is_interaction {
        CollapsibleOpenChangeSource::Interaction
    } else {
        CollapsibleOpenChangeSource::ExternalSync
    }
}

pub fn compute_next_open(current_open: bool) -> bool {
    !current_open
}

pub fn should_emit_open_change(current_open: bool, next_open: bool) -> bool {
    current_open != next_open
}

pub fn apply_open_change(
    state: &mut CollapsibleOpenState,
    controlled_open: Option<bool>,
    next_open: bool,
) {
    state.sync_controlled(controlled_open);
    state.set_open(next_open);
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn normalize_dir(dir: Option<String>) -> Option<A11yDirection> {
    dir.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.eq_ignore_ascii_case("ltr") {
            Some(A11yDirection::Ltr)
        } else if trimmed.eq_ignore_ascii_case("rtl") {
            Some(A11yDirection::Rtl)
        } else {
            None
        }
    })
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
