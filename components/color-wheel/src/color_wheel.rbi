pub type ColorWheelSource = ui_state_primitives::color_wheel::ColorWheelSource;
pub type ColorWheelState = ui_state_primitives::color_wheel::ColorWheelState;
pub type ColorWheelStateInput = ui_state_primitives::color_wheel::ColorWheelStateInput;
pub type ColorWheelStatus = ui_state_primitives::color_wheel::ColorWheelStatus;
pub type ColorWheelValueLabelMode = ui_state_primitives::color_wheel::ColorWheelValueLabelMode;
pub type ColorWheelMotion = crate::ColorWheelMotion;

pub const DEFAULT_ARIA_LABEL: &str;

pub enum ColorWheelAgentSchema {
    V1,
}

pub enum ColorWheelAgentSchemaVersion {
    V1,
}

pub enum ColorWheelStreamSupport {
    Unsupported,
}

pub enum ColorWheelStreamFallback {
    Snapshot,
}

pub enum ColorWheelStreamMode {
    Snapshot,
}

pub enum ColorWheelOutputStatus {
    Verified,
    Submittable,
}

pub enum ColorWheelIntent {
    SelectHueAngle,
}

pub enum ColorWheelInteractionSource {
    None,
    Pointer,
    Keyboard,
    Input,
}

pub enum ColorWheelUiAction {
    Idle,
    Drag,
    Pointer,
    Keyboard,
    Input,
}

pub enum ColorWheelUiState {
    Active,
    Disabled,
}

pub enum ColorWheelUiSource {
    OnValueChange,
    None,
}

pub struct ColorWheelAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub source_attr: &'static str,
}

pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorWheelAgentContract;
pub fn resolve_ui_action(
    is_dragging: bool,
    interaction_source: ColorWheelInteractionSource,
) -> ColorWheelUiAction;
pub fn resolve_ui_state(is_disabled: bool) -> ColorWheelUiState;

pub fn ColorWheel(
    id_base: String,
    label: Option<String>,
    aria_label: Option<String>,
    value: Option<leptos::prelude::Signal<f64>>,
    default_value: Option<f64>,
    on_value_change: Option<leptos::prelude::Callback<f64>>,
    step: f64,
    is_disabled: Option<bool>,
    disabled: bool,
    motion: ColorWheelMotion,
    is_value_label_visible: Option<bool>,
    show_value_label: bool,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
