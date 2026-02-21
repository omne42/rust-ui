pub type DateInputGroupVariant = crate::DateInputGroupVariant;
pub type DateInputGroupMotion = crate::DateInputGroupMotion;
pub type DateInputGroupState = ui_state_primitives::date_input_group::DateInputGroupState;
pub type DateInputGroupStateInput = ui_state_primitives::date_input_group::DateInputGroupStateInput;
pub type DateInputGroupWidth = ui_state_primitives::date_input_group::DateInputGroupWidth;
pub type DateInputGroupStatus = ui_state_primitives::date_input_group::DateInputGroupStatus;

pub const DEFAULT_ARIA_LABEL: &str;
pub const DATE_INPUT_GROUP_AGENT_SCHEMA: &str;

pub fn sanitize_motion(motion: crate::DateInputGroupMotion) -> crate::DateInputGroupMotion;

pub enum DateInputGroupAgentSchemaVersion {
    V1,
}

pub enum DateInputGroupAgentIntent {
    DateInputGroup,
}

pub enum DateInputGroupAgentAction {
    SnapshotRender,
}

pub enum DateInputGroupAgentState {
    Default,
    Disabled,
    Invalid,
    Segmented,
}

pub enum DateInputGroupAgentSource {
    StatePrimitives,
}

pub enum DateInputGroupAgentOutputStatus {
    Verified,
}

pub enum DateInputGroupAgentStreamSupport {
    Unsupported,
}

pub enum DateInputGroupAgentStreamFallback {
    Snapshot,
}

pub enum DateInputGroupAgentStreamMode {
    Streaming,
    Snapshot,
}

pub struct DateInputGroupAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DateInputGroupAgentSchemaVersion,
    pub intent: DateInputGroupAgentIntent,
    pub action: DateInputGroupAgentAction,
    pub state: DateInputGroupAgentState,
    pub source: DateInputGroupAgentSource,
    pub output_status: DateInputGroupAgentOutputStatus,
    pub stream_support: DateInputGroupAgentStreamSupport,
    pub stream_fallback: DateInputGroupAgentStreamFallback,
    pub stream_mode: DateInputGroupAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub config_policy: &'static str,
}

pub struct DateInputGroupAgentContractInput {
    pub render_state: DateInputGroupState,
    pub is_custom_motion: bool,
}

pub fn resolve_agent_contract(
    input: DateInputGroupAgentContractInput,
) -> DateInputGroupAgentContract;

pub fn DateInputGroup(
    is_full_width: bool,
    variant: crate::DateInputGroupVariant,
    is_disabled: bool,
    is_invalid: bool,
    is_segmented: bool,
    motion: crate::DateInputGroupMotion,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    prefix: Option<leptos::children::ViewFn>,
    suffix: Option<leptos::children::ViewFn>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
