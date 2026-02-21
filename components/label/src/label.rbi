pub type LabelEmphasis = ui_state_primitives::label::LabelEmphasis;
pub type LabelState = ui_state_primitives::label::LabelState;
pub type LabelStateInput = ui_state_primitives::label::LabelStateInput;
pub type A11yDirection = ui_headless::A11yDirection;

pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::label::DEFAULT_ARIA_LABEL;
pub const DEFAULT_REQUIRED_INDICATOR: &str = ui_state_primitives::label::DEFAULT_REQUIRED_INDICATOR;
pub const LABEL_AGENT_SCHEMA: &str;
pub const LABEL_AGENT_SCHEMA_VERSION: &str;

pub enum LabelAgentIntent {
    FormLabel,
}

pub enum LabelAgentAction {
    RenderSnapshot,
}

pub enum LabelAgentState {
    Required,
    Optional,
}

pub enum LabelAgentSource {
    Default,
    Custom,
}

pub enum LabelAgentStreamSupport {
    Optional,
}

pub enum LabelAgentStreamFallback {
    Snapshot,
}

pub enum LabelAgentOutputStatus {
    Verified,
}

pub struct LabelAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub label_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(
    state: LabelState,
    motion_source_attr: &'static str,
) -> LabelAgentContractAttrs;

pub struct LabelMotion {
    pub color_transition_ms: u16,
    pub weight_transition_ms: u16,
}

pub fn Label(
    text: Option<String>,
    for_id: Option<String>,
    is_required: bool,
    is_disabled: bool,
    emphasis: LabelEmphasis,
    required_indicator: Option<String>,
    class_name: Option<String>,
    motion: LabelMotion,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;
