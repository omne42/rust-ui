pub type CircularProgressState = ui_state_primitives::circular_progress::CircularProgressState;

pub const DEFAULT_ARIA_LABEL: &str;
pub const CIRCULAR_PROGRESS_AGENT_SCHEMA: &str;

pub enum CircularProgressAgentSchemaVersion {
    V1,
}

impl CircularProgressAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum CircularProgressAgentIntent {
    ProgressIndeterminate,
}

impl CircularProgressAgentIntent {
    pub const fn as_str(self) -> &'static str;
}

pub enum CircularProgressAgentAction {
    Render,
}

impl CircularProgressAgentAction {
    pub const fn as_str(self) -> &'static str;
}

pub enum CircularProgressAgentState {
    Indeterminate,
}

impl CircularProgressAgentState {
    pub const fn as_str(self) -> &'static str;
}

pub enum CircularProgressAgentSource {
    StatePrimitives,
}

impl CircularProgressAgentSource {
    pub const fn as_str(self) -> &'static str;
}

pub struct CircularProgressAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CircularProgressAgentSchemaVersion,
    pub intent: CircularProgressAgentIntent,
    pub action: CircularProgressAgentAction,
    pub state: CircularProgressAgentState,
    pub source: CircularProgressAgentSource,
    pub size_source: &'static str,
    pub thickness_source: &'static str,
    pub label_source: &'static str,
    pub class_source: &'static str,
}

pub fn resolve_agent_contract(state: &CircularProgressState) -> CircularProgressAgentContract;

pub fn CircularProgress(
    aria_label: Option<String>,
    size_px: Option<f64>,
    thickness_px: Option<f64>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
