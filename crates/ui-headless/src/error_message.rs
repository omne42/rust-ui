use crate::a11y::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};
use ui_state_primitives::error_message::ErrorMessageState;

pub const ERROR_MESSAGE_AGENT_SCHEMA: &str = "ui.error-message.agent-contract.v1";
pub const ERROR_MESSAGE_AGENT_SCHEMA_VERSION: &str = "1";
pub const ERROR_MESSAGE_AGENT_INTENT: &str = "form-validation-feedback";
pub const ERROR_MESSAGE_AGENT_STREAM_SUPPORT: &str = "optional";
pub const ERROR_MESSAGE_AGENT_STREAM_FALLBACK: &str = "snapshot";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorMessageAgentOutputMode {
    Streaming,
    Snapshot,
}

impl ErrorMessageAgentOutputMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorMessageAgentOutputMode::Streaming => "streaming",
            ErrorMessageAgentOutputMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorMessageAgentAction {
    AnnounceError,
    ReadOnly,
}

impl ErrorMessageAgentAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorMessageAgentAction::AnnounceError => "announce-error",
            ErrorMessageAgentAction::ReadOnly => "read-only",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorMessageAgentOutputStatus {
    Draft,
    Verified,
}

impl ErrorMessageAgentOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            ErrorMessageAgentOutputStatus::Draft => "draft",
            ErrorMessageAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ErrorMessageHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageAttrs {
    pub role: &'static str,
    pub aria_live: &'static str,
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_truncate: Option<&'static str>,
    pub data_message_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
    pub data_ui_schema: &'static str,
    pub data_ui_schema_version: &'static str,
    pub data_ui_intent: &'static str,
    pub data_ui_action: &'static str,
    pub data_ui_stream_support: &'static str,
    pub data_ui_stream_fallback: &'static str,
    pub data_ui_stream_mode: &'static str,
    pub data_ui_output_status: &'static str,
    pub data_stream_mode: &'static str,
    pub data_stream_fallback: &'static str,
    pub data_output_status: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorMessageSemanticState {
    pub tone: &'static str,
    pub state: &'static str,
    pub message_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_disabled: bool,
    pub is_truncated: bool,
    pub has_custom_class_name: bool,
    pub ui_output_mode: &'static str,
    pub ui_action: &'static str,
    pub ui_output_status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageContract {
    pub attrs: ErrorMessageAttrs,
    pub handlers: ErrorMessageHandlers,
    pub state: ErrorMessageSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessageOptions {
    pub state: ErrorMessageState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_error_message(options: ErrorMessageOptions) -> ErrorMessageContract {
    let locale = locale_attrs(options.lang, options.dir);
    let live_region = live_region_attrs(LiveRegionPriority::Assertive);
    let state = options.state;
    let (ui_action, ui_output_status) = if state.is_disabled {
        (
            ErrorMessageAgentAction::ReadOnly,
            ErrorMessageAgentOutputStatus::Draft,
        )
    } else {
        (
            ErrorMessageAgentAction::AnnounceError,
            ErrorMessageAgentOutputStatus::Verified,
        )
    };
    let ui_output_mode = ErrorMessageAgentOutputMode::Snapshot;

    ErrorMessageContract {
        attrs: ErrorMessageAttrs {
            role: live_region.role,
            aria_live: live_region.aria_live,
            aria_label: options.aria_label,
            aria_disabled: state.is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_tone: state.tone_attr,
            data_state: state.data_state_attr,
            data_disabled: state.is_disabled.then_some("true"),
            data_truncate: state.is_truncated.then_some("true"),
            data_message_source: state.message_source_attr,
            data_aria_source: state.aria_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
            data_ui_schema: ERROR_MESSAGE_AGENT_SCHEMA,
            data_ui_schema_version: ERROR_MESSAGE_AGENT_SCHEMA_VERSION,
            data_ui_intent: ERROR_MESSAGE_AGENT_INTENT,
            data_ui_action: ui_action.as_attr(),
            data_ui_stream_support: ERROR_MESSAGE_AGENT_STREAM_SUPPORT,
            data_ui_stream_fallback: ERROR_MESSAGE_AGENT_STREAM_FALLBACK,
            data_ui_stream_mode: ui_output_mode.as_attr(),
            data_ui_output_status: ui_output_status.as_attr(),
            data_stream_mode: ui_output_mode.as_attr(),
            data_stream_fallback: ERROR_MESSAGE_AGENT_STREAM_FALLBACK,
            data_output_status: ui_output_status.as_attr(),
        },
        handlers: ErrorMessageHandlers,
        state: ErrorMessageSemanticState {
            tone: state.tone_attr,
            state: state.data_state_attr,
            message_source: state.message_source_attr,
            aria_source: state.aria_source_attr,
            class_source: state.class_source_attr,
            is_disabled: state.is_disabled,
            is_truncated: state.is_truncated,
            has_custom_class_name: state.has_custom_class_name,
            ui_output_mode: ui_output_mode.as_attr(),
            ui_action: ui_action.as_attr(),
            ui_output_status: ui_output_status.as_attr(),
        },
    }
}

#[cfg(test)]
#[path = "test/error_message.rs"]
mod tests;
