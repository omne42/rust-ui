use crate::color::handle::motion::ColorHandleMotion;

pub use ui_state_primitives::color_handle::{ColorHandleState, ColorHandleStateInput};

pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::color_handle::DEFAULT_ARIA_LABEL;
pub const DEFAULT_IS_DISABLED: bool = false;
pub const DEFAULT_IS_FOCUSED: bool = false;
pub const DEFAULT_IS_DRAGGING: bool = false;
pub const DEFAULT_IS_LOUPE_VISIBLE: bool = true;
pub const DEFAULT_X_PERCENT: f32 = 50.0;
pub const DEFAULT_Y_PERCENT: f32 = 50.0;

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::color_handle::normalize_optional_text(value)
}

pub fn sanitize_color(value: Option<String>) -> Option<String> {
    ui_state_primitives::color_handle::sanitize_color(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::color_handle::normalize_aria_label(value)
}

pub fn resolve_state(input: ColorHandleStateInput) -> ColorHandleState {
    ui_state_primitives::color_handle::resolve_state(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleInteractionState {
    Disabled,
    Dragging,
    Focused,
    Idle,
}

impl ColorHandleInteractionState {
    pub const fn from_flags(is_disabled: bool, is_focused: bool, is_dragging: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else if is_dragging {
            Self::Dragging
        } else if is_focused {
            Self::Focused
        } else {
            Self::Idle
        }
    }

    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub const fn is_focused(self) -> bool {
        matches!(self, Self::Focused)
    }

    pub const fn is_dragging(self) -> bool {
        matches!(self, Self::Dragging)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorHandlePropsInput {
    pub is_disabled: Option<bool>,
    pub is_focused: Option<bool>,
    pub is_dragging: Option<bool>,
    pub is_loupe_visible: Option<bool>,
    pub x_percent: Option<f32>,
    pub y_percent: Option<f32>,
    pub motion: Option<ColorHandleMotion>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorHandleResolvedProps {
    pub interaction_state: ColorHandleInteractionState,
    pub is_loupe_visible: bool,
    pub x_percent: f32,
    pub y_percent: f32,
    pub motion: ColorHandleMotion,
}

pub fn resolve_props(input: ColorHandlePropsInput) -> ColorHandleResolvedProps {
    let is_disabled = input.is_disabled.unwrap_or(DEFAULT_IS_DISABLED);
    let is_focused = input.is_focused.unwrap_or(DEFAULT_IS_FOCUSED);
    let is_dragging = input.is_dragging.unwrap_or(DEFAULT_IS_DRAGGING);

    ColorHandleResolvedProps {
        interaction_state: ColorHandleInteractionState::from_flags(
            is_disabled,
            is_focused,
            is_dragging,
        ),
        is_loupe_visible: input.is_loupe_visible.unwrap_or(DEFAULT_IS_LOUPE_VISIBLE),
        x_percent: input.x_percent.unwrap_or(DEFAULT_X_PERCENT),
        y_percent: input.y_percent.unwrap_or(DEFAULT_Y_PERCENT),
        motion: input.motion.unwrap_or_default(),
    }
}

pub const COLOR_HANDLE_AGENT_SCHEMA_NAME: &str = "ui.color-handle.agent-contract";
const SOURCE_CUSTOM_ATTR: &str = "custom";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentSchemaVersion {
    V1,
}

impl ColorHandleAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentIntent {
    ColorSelection,
}

impl ColorHandleAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorSelection => "color-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentAction {
    Initialize,
    Focus,
    DragUpdate,
}

impl ColorHandleAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Focus => "focus",
            Self::DragUpdate => "drag-update",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentStateAxis {
    Disabled,
    Dragging,
    Focused,
    Color,
    Idle,
}

impl ColorHandleAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Dragging => "dragging",
            Self::Focused => "focused",
            Self::Color => "color",
            Self::Idle => "idle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentSource {
    DefaultConfig,
    CustomizedProps,
    DragInteraction,
}

impl ColorHandleAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultConfig => "default-config",
            Self::CustomizedProps => "customized-props",
            Self::DragInteraction => "drag-interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentStreamSupport {
    Optional,
}

impl ColorHandleAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentStreamFallback {
    Snapshot,
}

impl ColorHandleAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorHandleAgentOutputStatus {
    Verified,
    Submittable,
}

impl ColorHandleAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleAgentCapabilities {
    pub can_drag: bool,
    pub can_focus: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorHandleAgentSchemaVersion,
    pub intent: ColorHandleAgentIntent,
    pub action: ColorHandleAgentAction,
    pub state: ColorHandleAgentStateAxis,
    pub source: ColorHandleAgentSource,
    pub stream_support: ColorHandleAgentStreamSupport,
    pub stream_fallback: ColorHandleAgentStreamFallback,
    pub output_status: ColorHandleAgentOutputStatus,
    pub capabilities: ColorHandleAgentCapabilities,
}

pub fn resolve_agent_action(state: ColorHandleState) -> ColorHandleAgentAction {
    if state.is_dragging {
        ColorHandleAgentAction::DragUpdate
    } else if state.is_focused {
        ColorHandleAgentAction::Focus
    } else {
        ColorHandleAgentAction::Initialize
    }
}

pub fn resolve_agent_state_axis(state: ColorHandleState) -> ColorHandleAgentStateAxis {
    if state.is_disabled {
        ColorHandleAgentStateAxis::Disabled
    } else if state.is_dragging {
        ColorHandleAgentStateAxis::Dragging
    } else if state.is_focused {
        ColorHandleAgentStateAxis::Focused
    } else if state.has_color {
        ColorHandleAgentStateAxis::Color
    } else {
        ColorHandleAgentStateAxis::Idle
    }
}

pub fn resolve_agent_source(
    state: ColorHandleState,
    motion_source_attr: &'static str,
) -> ColorHandleAgentSource {
    if state.is_dragging {
        ColorHandleAgentSource::DragInteraction
    } else if state.aria_source_attr == SOURCE_CUSTOM_ATTR
        || state.class_source_attr == SOURCE_CUSTOM_ATTR
        || motion_source_attr == SOURCE_CUSTOM_ATTR
    {
        ColorHandleAgentSource::CustomizedProps
    } else {
        ColorHandleAgentSource::DefaultConfig
    }
}

pub fn resolve_agent_output_status(state: ColorHandleState) -> ColorHandleAgentOutputStatus {
    if state.is_dragging {
        ColorHandleAgentOutputStatus::Submittable
    } else {
        ColorHandleAgentOutputStatus::Verified
    }
}

pub fn resolve_agent_contract(
    state: ColorHandleState,
    motion_source_attr: &'static str,
) -> ColorHandleAgentContract {
    ColorHandleAgentContract {
        schema_name: COLOR_HANDLE_AGENT_SCHEMA_NAME,
        schema_version: ColorHandleAgentSchemaVersion::V1,
        intent: ColorHandleAgentIntent::ColorSelection,
        action: resolve_agent_action(state),
        state: resolve_agent_state_axis(state),
        source: resolve_agent_source(state, motion_source_attr),
        stream_support: ColorHandleAgentStreamSupport::Optional,
        stream_fallback: ColorHandleAgentStreamFallback::Snapshot,
        output_status: resolve_agent_output_status(state),
        capabilities: ColorHandleAgentCapabilities {
            can_drag: !state.is_disabled,
            can_focus: !state.is_disabled,
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorHandleState) -> String {
    let mut class_name = String::new();
    class_name.push_str("ui-color-handle");

    if state.is_disabled {
        class_name.push_str(" ui-color-handle--disabled");
    }

    if state.is_focused {
        class_name.push_str(" ui-color-handle--focused");
    }

    if state.is_dragging {
        class_name.push_str(" ui-color-handle--dragging");
    }

    if state.has_custom_class_name {
        class_name.push_str(" ui-color-handle--custom-class");
        if let Some(base_class_name) = base_class_name {
            class_name.push(' ');
            class_name.push_str(&base_class_name);
        }
    }

    class_name
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
