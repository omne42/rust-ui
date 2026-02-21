use crate::DropZoneMotion;
use ui_state_primitives::drop_zone::DragDepth;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DroppedFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledSource {
    IsDisabled,
    DisabledAlias,
    Default,
}

impl DisabledSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::DisabledAlias => "disabled",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionSource {
    Default,
    Custom,
}

impl MotionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledInput {
    IsDisabled(bool),
    DisabledAlias(bool),
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AriaLabelSource {
    Default,
    Custom,
}

impl AriaLabelSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropZonePropsInput {
    pub disabled_input: DisabledInput,
    pub motion: Option<DropZoneMotion>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropZoneResolvedProps {
    pub is_disabled: bool,
    pub disabled_source: DisabledSource,
    pub motion: DropZoneMotion,
    pub motion_source: MotionSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragInteractionAction {
    Enter,
    Leave,
    Drop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragLifecyclePhase {
    Idle,
    Dragging,
}

impl DragLifecyclePhase {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Dragging => "dragging",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragLifecycleAction {
    DragStart,
    DragEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DragInteractionState {
    pub depth: DragDepth,
    pub is_drop_target: bool,
}

pub const DROP_ZONE_AGENT_SCHEMA: &str = "ui.drop_zone.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentSchemaVersion {
    V1,
}

impl DropZoneAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentIntent {
    FileIngestion,
}

impl DropZoneAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileIngestion => "file-ingestion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentAction {
    AwaitInput,
    CaptureDrop,
    Blocked,
}

impl DropZoneAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitInput => "await-input",
            Self::CaptureDrop => "capture-drop",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentState {
    Idle,
    Dragging,
    Disabled,
}

impl DropZoneAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Dragging => "dragging",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentSource {
    IsDisabled,
    DisabledAlias,
    Default,
}

impl DropZoneAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::DisabledAlias => "disabled",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentConfigPolicy {
    Whitelist,
}

impl DropZoneAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropZoneAgentOutputStatus {
    Verified,
}

impl DropZoneAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropZoneAgentCapabilities {
    pub can_drop: bool,
    pub can_paste: bool,
    pub has_drop_callback: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropZoneAgentContractInput {
    pub drag_phase: DragLifecyclePhase,
    pub is_disabled: bool,
    pub disabled_source: DisabledSource,
    pub motion_source: MotionSource,
    pub aria_source: AriaLabelSource,
    pub has_drop_callback: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropZoneAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DropZoneAgentSchemaVersion,
    pub intent: DropZoneAgentIntent,
    pub action: DropZoneAgentAction,
    pub state: DropZoneAgentState,
    pub source: DropZoneAgentSource,
    pub config_policy: DropZoneAgentConfigPolicy,
    pub output_status: DropZoneAgentOutputStatus,
    pub capabilities: DropZoneAgentCapabilities,
    pub motion_source: MotionSource,
    pub aria_source: AriaLabelSource,
}

const fn resolve_agent_source(source: DisabledSource) -> DropZoneAgentSource {
    match source {
        DisabledSource::IsDisabled => DropZoneAgentSource::IsDisabled,
        DisabledSource::DisabledAlias => DropZoneAgentSource::DisabledAlias,
        DisabledSource::Default => DropZoneAgentSource::Default,
    }
}

pub fn resolve_agent_contract(input: DropZoneAgentContractInput) -> DropZoneAgentContract {
    let state = if input.is_disabled {
        DropZoneAgentState::Disabled
    } else {
        match input.drag_phase {
            DragLifecyclePhase::Idle => DropZoneAgentState::Idle,
            DragLifecyclePhase::Dragging => DropZoneAgentState::Dragging,
        }
    };

    let action = if input.is_disabled {
        DropZoneAgentAction::Blocked
    } else {
        match input.drag_phase {
            DragLifecyclePhase::Idle => DropZoneAgentAction::AwaitInput,
            DragLifecyclePhase::Dragging => DropZoneAgentAction::CaptureDrop,
        }
    };

    DropZoneAgentContract {
        schema_name: DROP_ZONE_AGENT_SCHEMA,
        schema_version: DropZoneAgentSchemaVersion::V1,
        intent: DropZoneAgentIntent::FileIngestion,
        action,
        state,
        source: resolve_agent_source(input.disabled_source),
        config_policy: DropZoneAgentConfigPolicy::Whitelist,
        output_status: DropZoneAgentOutputStatus::Verified,
        capabilities: DropZoneAgentCapabilities {
            can_drop: !input.is_disabled,
            can_paste: !input.is_disabled,
            has_drop_callback: input.has_drop_callback,
        },
        motion_source: input.motion_source,
        aria_source: input.aria_source,
    }
}

pub const fn classify_disabled_input(
    is_disabled: Option<bool>,
    disabled: Option<bool>,
) -> DisabledInput {
    if let Some(value) = is_disabled {
        return DisabledInput::IsDisabled(value);
    }

    if let Some(value) = disabled {
        return DisabledInput::DisabledAlias(value);
    }

    DisabledInput::Default
}

pub const fn resolve_is_disabled(input: DisabledInput) -> (bool, DisabledSource) {
    match input {
        DisabledInput::IsDisabled(value) => (value, DisabledSource::IsDisabled),
        DisabledInput::DisabledAlias(value) => (value, DisabledSource::DisabledAlias),
        DisabledInput::Default => (false, DisabledSource::Default),
    }
}

pub const fn resolve_motion_source(is_default: bool) -> MotionSource {
    if is_default {
        MotionSource::Default
    } else {
        MotionSource::Custom
    }
}

pub const fn resolve_aria_label_source(has_custom_aria_label: bool) -> AriaLabelSource {
    if has_custom_aria_label {
        AriaLabelSource::Custom
    } else {
        AriaLabelSource::Default
    }
}

pub(crate) fn resolve_props(input: DropZonePropsInput) -> DropZoneResolvedProps {
    let (is_disabled, disabled_source) = resolve_is_disabled(input.disabled_input);
    let motion = crate::motion::sanitize_motion(input.motion.unwrap_or_default());
    let motion_source = resolve_motion_source(motion == DropZoneMotion::default());

    DropZoneResolvedProps {
        is_disabled,
        disabled_source,
        motion,
        motion_source,
    }
}

pub(crate) fn reduce_drag_interaction(
    depth: DragDepth,
    action: DragInteractionAction,
) -> DragInteractionState {
    let depth = match action {
        DragInteractionAction::Enter => depth.enter(),
        DragInteractionAction::Leave => depth.leave(),
        DragInteractionAction::Drop => depth.reset(),
    };

    DragInteractionState {
        is_drop_target: depth.is_active(),
        depth,
    }
}

pub const fn reduce_drag_lifecycle(
    _phase: DragLifecyclePhase,
    action: DragLifecycleAction,
) -> DragLifecyclePhase {
    match action {
        DragLifecycleAction::DragStart => DragLifecyclePhase::Dragging,
        DragLifecycleAction::DragEnd => DragLifecyclePhase::Idle,
    }
}

pub const fn bool_data_attr(value: bool) -> Option<&'static str> {
    if value { Some("true") } else { None }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
