pub type ColorAreaStateInput = ui_state_primitives::color_area::ColorAreaStateInput;
pub type ColorAreaState = ui_state_primitives::color_area::ColorAreaState;

pub struct ColorAreaMotion {
    pub duration_ms: f64,
}

pub enum ColorAreaAgentSchemaVersion {
    V1,
}

pub enum ColorAreaAgentIntent {
    SelectColorPoint,
}

pub enum ColorAreaAgentAction {
    Select,
    Disabled,
}

pub enum ColorAreaAgentStateAxis {
    Active,
    Disabled,
}

pub enum ColorAreaAgentSource {
    External,
    Default,
}

pub struct ColorAreaAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorAreaAgentSchemaVersion,
    pub intent: ColorAreaAgentIntent,
    pub action: ColorAreaAgentAction,
    pub state: ColorAreaAgentStateAxis,
    pub source: ColorAreaAgentSource,
}

pub fn ColorArea(
    id_base: String,
    label: Option<String>,
    is_disabled: Option<bool>,
    value: Option<leptos::prelude::Signal<(f32, f32)>>,
    default_value: Option<(f32, f32)>,
    on_value_change: Option<leptos::prelude::Callback<(f32, f32)>>,
    step: Option<f32>,
    grid_size: Option<usize>,
    preview_color: Option<String>,
    motion: ColorAreaMotion,
    aria_label: Option<String>,
    x_axis_label: Option<String>,
    y_axis_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
