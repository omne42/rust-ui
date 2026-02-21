pub type HelpTextTone = ui_state_primitives::help_text::HelpTextTone;
pub type HelpTextMessageKind = ui_state_primitives::help_text::HelpTextMessageKind;
pub type HelpTextDataState = ui_state_primitives::help_text::HelpTextDataState;
pub type HelpTextSourceAttr = ui_state_primitives::help_text::HelpTextSourceAttr;
pub type HelpTextErrorSourceAttr = ui_state_primitives::help_text::HelpTextErrorSourceAttr;

pub const HELP_TEXT_AGENT_SCHEMA: &str = "ui.help-text.agent-contract.v1";

pub enum HelpTextAgentIntent {
    FormAssistance,
}

pub enum HelpTextAgentAction {
    AnnounceError,
    ReadOnly,
}

pub enum HelpTextAgentOutputStatus {
    Verified,
}

pub enum HelpTextAgentStreamSupport {
    Optional,
}

pub enum HelpTextAgentStreamMode {
    Snapshot,
}

pub struct HelpTextMotion {
    pub enabled: bool,
    pub duration_ms: u32,
}

impl HelpTextMotion {
    pub fn disabled() -> Self;
}

pub fn HelpText(
    tone: HelpTextTone,
    is_invalid: bool,
    is_disabled: bool,
    is_error_icon_visible: bool,
    description: Option<String>,
    error_message: Option<String>,
    aria_label: Option<String>,
    motion: HelpTextMotion,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
