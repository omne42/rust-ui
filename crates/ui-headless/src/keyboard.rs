use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::keyboard::KeyboardState;

pub const KEYBOARD_AGENT_SCHEMA: &str = "ui.keyboard.agent-contract/v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAgentSchemaVersion {
    V1,
}

impl KeyboardAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAgentIntent {
    DisplayKeyboardRender,
}

impl KeyboardAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardAgentIntent::DisplayKeyboardRender => "display.keyboard.render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAgentAction {
    Render,
}

impl KeyboardAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardAgentAction::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAgentState {
    Default,
    Muted,
    Compact,
}

impl KeyboardAgentState {
    pub fn from_state_attr(state_attr: &'static str) -> Self {
        match state_attr {
            "muted" => KeyboardAgentState::Muted,
            "compact" => KeyboardAgentState::Compact,
            _ => KeyboardAgentState::Default,
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardAgentState::Default => "default",
            KeyboardAgentState::Muted => "muted",
            KeyboardAgentState::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAgentSource {
    Default,
    Custom,
}

impl KeyboardAgentSource {
    pub fn from_sources(aria_source: &'static str, class_source: &'static str) -> Self {
        if aria_source == "custom" || class_source == "custom" {
            KeyboardAgentSource::Custom
        } else {
            KeyboardAgentSource::Default
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardAgentSource::Default => "default",
            KeyboardAgentSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardOutputStatus {
    Draft,
    Verified,
    Committable,
}

impl KeyboardOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardOutputStatus::Draft => "draft",
            KeyboardOutputStatus::Verified => "verified",
            KeyboardOutputStatus::Committable => "committable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardAgentContractAttrs {
    pub data_ui_schema: &'static str,
    pub data_ui_schema_version: &'static str,
    pub data_ui_intent: &'static str,
    pub data_ui_action: &'static str,
    pub data_ui_state: &'static str,
    pub data_ui_source: &'static str,
    pub data_ui_output_status: &'static str,
}

pub fn resolve_agent_contract_attrs(state: KeyboardState) -> KeyboardAgentContractAttrs {
    let schema_version = KeyboardAgentSchemaVersion::V1;
    let intent = KeyboardAgentIntent::DisplayKeyboardRender;
    let action = KeyboardAgentAction::Render;
    let agent_state = KeyboardAgentState::from_state_attr(state.data_state_attr);
    let source = KeyboardAgentSource::from_sources(state.aria_source_attr, state.class_source_attr);
    let output_status = KeyboardOutputStatus::Verified;

    KeyboardAgentContractAttrs {
        data_ui_schema: KEYBOARD_AGENT_SCHEMA,
        data_ui_schema_version: schema_version.as_attr(),
        data_ui_intent: intent.as_attr(),
        data_ui_action: action.as_attr(),
        data_ui_state: agent_state.as_attr(),
        data_ui_source: source.as_attr(),
        data_ui_output_status: output_status.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KeyboardHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardAttrs {
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_slot: &'static str,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_compact: Option<&'static str>,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
    pub data_ui_schema: &'static str,
    pub data_ui_schema_version: &'static str,
    pub data_ui_intent: &'static str,
    pub data_ui_action: &'static str,
    pub data_ui_state: &'static str,
    pub data_ui_source: &'static str,
    pub data_ui_output_status: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardSemanticState {
    pub tone: &'static str,
    pub state: &'static str,
    pub is_compact: bool,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub has_custom_class_name: bool,
    pub intent: &'static str,
    pub action: &'static str,
    pub agent_state: &'static str,
    pub source: &'static str,
    pub output_status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardContract {
    pub attrs: KeyboardAttrs,
    pub handlers: KeyboardHandlers,
    pub state: KeyboardSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardOptions {
    pub state: KeyboardState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_keyboard(options: KeyboardOptions) -> KeyboardContract {
    let locale = locale_attrs(options.lang, options.dir);
    let agent = resolve_agent_contract_attrs(options.state);

    KeyboardContract {
        attrs: KeyboardAttrs {
            aria_label: options.aria_label,
            lang: locale.lang,
            dir: locale.dir,
            data_slot: "keyboard",
            data_tone: options.state.tone_attr,
            data_state: options.state.data_state_attr,
            data_compact: options.state.is_compact.then_some("true"),
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
            data_ui_schema: agent.data_ui_schema,
            data_ui_schema_version: agent.data_ui_schema_version,
            data_ui_intent: agent.data_ui_intent,
            data_ui_action: agent.data_ui_action,
            data_ui_state: agent.data_ui_state,
            data_ui_source: agent.data_ui_source,
            data_ui_output_status: agent.data_ui_output_status,
        },
        handlers: KeyboardHandlers,
        state: KeyboardSemanticState {
            tone: options.state.tone_attr,
            state: options.state.data_state_attr,
            is_compact: options.state.is_compact,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            has_custom_class_name: options.state.has_custom_class_name,
            intent: agent.data_ui_intent,
            action: agent.data_ui_action,
            agent_state: agent.data_ui_state,
            source: agent.data_ui_source,
            output_status: agent.data_ui_output_status,
        },
    }
}

#[cfg(test)]
#[path = "test/keyboard.rs"]
mod tests;
