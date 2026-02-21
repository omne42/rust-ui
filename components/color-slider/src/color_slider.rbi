pub type ColorSliderChannel = ui_state_primitives::color_slider::ColorSliderChannel;
pub type ColorSliderState = ui_state_primitives::color_slider::ColorSliderState;
pub type ColorSliderStateInput = ui_state_primitives::color_slider::ColorSliderStateInput;
pub type ColorSliderMotion = crate::ColorSliderMotion;

pub const DEFAULT_ARIA_LABEL: &str;

pub enum ColorSliderAgentSchema {
    V1,
}

pub enum ColorSliderAgentSchemaVersion {
    V1,
}

pub enum ColorSliderStreamSupport {
    Unsupported,
}

pub enum ColorSliderStreamFallback {
    Snapshot,
}

pub enum ColorSliderStreamMode {
    Snapshot,
}

pub enum ColorSliderOutputStatus {
    Verified,
    Submittable,
}

pub enum ColorSliderIntent {
    AdjustColorChannel,
}

pub enum ColorSliderUiAction {
    Idle,
    Focus,
    Press,
}

pub struct ColorSliderAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorSliderAgentContract;
pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> ColorSliderUiAction;

pub fn ColorSlider(
    id_base: String,
    channel: ColorSliderChannel,
    label: Option<String>,
    aria_label: Option<String>,
    value: Option<leptos::prelude::Signal<f64>>,
    default_value: Option<f64>,
    on_value_change: Option<leptos::prelude::Callback<f64>>,
    min: f64,
    max: f64,
    step: f64,
    is_disabled: Option<bool>,
    disabled: bool,
    motion: ColorSliderMotion,
    show_value_label: bool,
    track_start_color: Option<String>,
    track_end_color: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
