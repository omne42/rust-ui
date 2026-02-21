pub type DescriptionTone = ui_state_primitives::description::DescriptionTone;
pub type DescriptionState = ui_state_primitives::description::DescriptionState;
pub type DescriptionStateInput = ui_state_primitives::description::DescriptionStateInput;
pub type A11yDirection = ui_headless::A11yDirection;

pub const DESCRIPTION_AGENT_SCHEMA: &str;
pub const DESCRIPTION_AGENT_SCHEMA_VERSION: &str;

pub enum DescriptionElement {
    Span,
    Paragraph,
    Div,
}

pub enum DescriptionAgentIntent {
    TextAssistance,
}

pub enum DescriptionAgentAction {
    RenderSnapshot,
}

pub enum DescriptionAgentSource {
    Default,
    Custom,
}

pub enum DescriptionAgentStreamSupport {
    Optional,
}

pub enum DescriptionAgentStreamFallback {
    Snapshot,
}

pub enum DescriptionAgentOutputStatus {
    Verified,
}

pub struct DescriptionAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs;

pub fn Description(
    text: String,
    tone: DescriptionTone,
    is_disabled: bool,
    is_truncated: bool,
    element: DescriptionElement,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> impl leptos::prelude::IntoView;
