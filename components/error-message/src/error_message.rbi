pub type ErrorMessageTone = ui_state_primitives::error_message::ErrorMessageTone;
pub type ErrorMessageElement = ui_state_primitives::error_message::ErrorMessageElement;

pub struct ErrorMessageMotion {
    pub transition_ms: u16,
}

pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = "ui.error-message.agent-contract.v1";

pub enum ErrorMessageAgentIntent {
    FormValidationFeedback,
}

pub enum ErrorMessageAgentAction {
    AnnounceError,
    ReadOnly,
}

pub enum ErrorMessageAgentOutputStatus {
    Draft,
    Verified,
}

pub enum ErrorMessageAgentStreamSupport {
    Optional,
}

pub enum ErrorMessageAgentStreamMode {
    Streaming,
    Snapshot,
}

pub fn ErrorMessage(
    text: String,
    tone: ErrorMessageTone,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    is_truncated: Option<bool>,
    truncate: Option<bool>,
    element: ErrorMessageElement,
    motion: ErrorMessageMotion,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
