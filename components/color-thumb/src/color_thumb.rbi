pub type ColorThumbState = ui_state_primitives::color_thumb::ColorThumbState;
pub type ColorThumbStateInput = ui_state_primitives::color_thumb::ColorThumbStateInput;
pub type ColorThumbInteractionState = ui_state_primitives::color_thumb::ColorThumbInteractionState;
pub type ColorThumbInputSource = ui_state_primitives::color_thumb::ColorThumbInputSource;
pub type ColorThumbAriaValueTextSource =
    ui_state_primitives::color_thumb::ColorThumbAriaValueTextSource;
pub type ColorThumbMotion = crate::ColorThumbMotion;

pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_COLOR: &str;

pub enum ColorThumbAgentSchema {
    V1,
}

pub enum ColorThumbAgentSchemaVersion {
    V1,
}

pub enum ColorThumbStreamSupport {
    Optional,
}

pub enum ColorThumbStreamFallback {
    Snapshot,
}

pub enum ColorThumbOutputStatus {
    Verified,
}

pub enum ColorThumbIntent {
    PickColorPoint,
}

pub enum ColorThumbUiAction {
    Idle,
    Focus,
    Drag,
}

pub struct ColorThumbAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract() -> ColorThumbAgentContract;
pub fn resolve_ui_action(state: ColorThumbState) -> ColorThumbUiAction;

pub fn ColorThumb(
    id_base: String,
    color: Option<String>,
    is_disabled: bool,
    is_focused: bool,
    is_dragging: bool,
    x_percent: Option<f32>,
    y_percent: Option<f32>,
    is_loupe_visible: Option<bool>,
    motion: ColorThumbMotion,
    aria_label: Option<String>,
    aria_value_text: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
