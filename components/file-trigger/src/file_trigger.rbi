pub enum FileTriggerSelectionMode {
    SingleFile,
    MultipleFiles,
    Directory,
}

pub struct FileTriggerFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

pub struct FileTriggerMotion {
    pub trigger: ui_components::ButtonMotion,
}

pub const FILE_TRIGGER_COMPONENT_SCHEMA_NAME: &str;
pub const FILE_TRIGGER_COMPONENT_SCHEMA_VERSION: &str;

pub enum FileTriggerAgentIntent {
    FilePick,
}

pub enum FileTriggerAgentAction {
    RenderSnapshot,
}

pub enum FileTriggerAgentSource {
    Default,
    Custom,
}

pub enum FileTriggerAgentStreamSupport {
    Optional,
}

pub enum FileTriggerAgentStreamFallback {
    Snapshot,
}

pub enum FileTriggerAgentOutputStatus {
    Verified,
}

pub struct FileTriggerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: FileTriggerAgentIntent,
    pub action: FileTriggerAgentAction,
    pub state: &'static str,
    pub source: FileTriggerAgentSource,
    pub stream_support: FileTriggerAgentStreamSupport,
    pub stream_fallback: FileTriggerAgentStreamFallback,
    pub output_status: FileTriggerAgentOutputStatus,
}

pub fn resolve_agent_contract(
    state: ui_state_primitives::file_trigger::FileTriggerState,
) -> FileTriggerAgentContract;

pub fn FileTrigger(
    id: Option<String>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    is_multiple: Option<bool>,
    multiple: Option<bool>,
    accept: Option<String>,
    is_accept_directory: Option<bool>,
    accept_directory: Option<bool>,
    capture: Option<String>,
    motion: FileTriggerMotion,
    on_files: Option<leptos::prelude::Callback<Vec<FileTriggerFile>>>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
