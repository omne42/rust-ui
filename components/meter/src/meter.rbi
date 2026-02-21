pub type MeterPhase = ui_state_primitives::meter::MeterPhase;
pub type MeterRange = ui_state_primitives::meter::MeterRange;
pub type MeterSize = ui_state_primitives::meter::MeterSize;
pub type MeterState = ui_state_primitives::meter::MeterState;
pub type MeterStateInput = ui_state_primitives::meter::MeterStateInput;
pub type MeterVariant = ui_state_primitives::meter::MeterVariant;
pub type MeterMotion = crate::motion::MeterMotion;
pub type MeterComponentSchemaVersion = crate::MeterComponentSchemaVersion;
pub type MeterComponentSpec = crate::MeterComponentSpec;

pub const METER_AGENT_SCHEMA: &str = "ui.meter.agent-contract.v1";

pub enum MeterAgentIntent {
    ProgressMeter,
}

pub enum MeterAgentAction {
    Render,
}

pub enum MeterAgentStatePhase {
    Determinate,
    Indeterminate,
}

pub enum MeterAgentStreamMode {
    Snapshot,
}

pub enum MeterAgentOutputMode {
    Snapshot,
}

pub enum MeterAgentOutputStatus {
    Validated,
}

pub struct MeterAgentDataAttrs {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub stream_mode: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
    pub state_phase: &'static str,
    pub state_variant: &'static str,
    pub state_size: &'static str,
    pub source_label: &'static str,
    pub source_value_label: &'static str,
    pub source_motion: &'static str,
    pub source_class: &'static str,
}

pub fn agent_data_attrs(state: MeterState, phase: MeterPhase) -> MeterAgentDataAttrs;

pub fn clamp_to_range(value: f64, range: MeterRange) -> f64;
pub fn normalize_progress(value: f64, range: MeterRange) -> f64;

pub mod styles {
    pub const CSS: &str;
}

pub fn Meter(
    id: String,
    label: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    value: leptos::prelude::Signal<Option<f64>>,
    min: Option<f64>,
    max: Option<f64>,
    size: MeterSize,
    variant: MeterVariant,
    motion: MeterMotion,
    is_value_label_visible: Option<bool>,
    show_value_label: Option<bool>,
    value_label: Option<String>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
