pub type ColorHandleStateInput = ui_state_primitives::color_handle::ColorHandleStateInput;
pub type ColorHandleState = ui_state_primitives::color_handle::ColorHandleState;

pub struct ColorHandleMotion {
    pub duration_ms: u16,
}

pub enum ColorHandleAgentSchemaVersion {
    V1,
}

pub enum ColorHandleAgentIntent {
    ColorSelection,
}

pub enum ColorHandleAgentAction {
    Initialize,
    Focus,
    DragUpdate,
}

pub enum ColorHandleAgentStateAxis {
    Disabled,
    Dragging,
    Focused,
    Color,
    Idle,
}

pub enum ColorHandleAgentSource {
    DefaultConfig,
    CustomizedProps,
    DragInteraction,
}

pub struct ColorHandleAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorHandleAgentSchemaVersion,
    pub intent: ColorHandleAgentIntent,
    pub action: ColorHandleAgentAction,
    pub state: ColorHandleAgentStateAxis,
    pub source: ColorHandleAgentSource,
}

pub fn resolve_agent_contract(
    state: ColorHandleState,
    motion_source_attr: &'static str,
) -> ColorHandleAgentContract;

pub fn ColorHandle(
    id_base: String,
    color: Option<String>,
    is_disabled: bool,
    is_focused: bool,
    is_dragging: bool,
    is_loupe_visible: bool,
    x_percent: f32,
    y_percent: f32,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    class_name: Option<String>,
    motion: ColorHandleMotion,
) -> impl leptos::prelude::IntoView;
