use ui_state_primitives::error_view as error_view_state;
pub use ui_state_primitives::error_view::{
    DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE, ErrorViewState, ErrorViewStateInput, ErrorViewTone,
    normalize_aria_label, normalize_message, normalize_optional_text,
};
pub const ERROR_VIEW_AGENT_SCHEMA: &str = "ui.error-view.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentSchemaVersion {
    V1,
}

impl ErrorViewAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentIntent {
    ErrorFeedback,
}

impl ErrorViewAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentIntent::ErrorFeedback => "error.feedback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentAction {
    AnnounceOnly,
    AnnounceWithActions,
}

impl ErrorViewAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentAction::AnnounceOnly => "announce-only",
            ErrorViewAgentAction::AnnounceWithActions => "announce-with-actions",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentState {
    Visible,
    Hidden,
}

impl ErrorViewAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentState::Visible => "visible",
            ErrorViewAgentState::Hidden => "hidden",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentSource {
    Default,
    Custom,
}

impl ErrorViewAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentSource::Default => "default",
            ErrorViewAgentSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentStateSource {
    InvalidProp,
}

impl ErrorViewAgentStateSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentStateSource::InvalidProp => "is-invalid-prop",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentActionSource {
    MessageOnly,
    ActionsSlot,
}

impl ErrorViewAgentActionSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentActionSource::MessageOnly => "message-only",
            ErrorViewAgentActionSource::ActionsSlot => "actions-slot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentMotionSource {
    Default,
    Custom,
}

impl ErrorViewAgentMotionSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentMotionSource::Default => "default",
            ErrorViewAgentMotionSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorViewAgentConfigPolicy {
    Whitelist,
}

impl ErrorViewAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorViewAgentConfigPolicy::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewAgentContractInput {
    pub is_visible: bool,
    pub message_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_actions: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorViewAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ErrorViewAgentSchemaVersion,
    pub intent: ErrorViewAgentIntent,
    pub action: ErrorViewAgentAction,
    pub state: ErrorViewAgentState,
    pub source: ErrorViewAgentSource,
    pub state_source: ErrorViewAgentStateSource,
    pub action_source: ErrorViewAgentActionSource,
    pub motion_source: ErrorViewAgentMotionSource,
    pub config_policy: ErrorViewAgentConfigPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorViewNormalizeInput {
    pub tone: Option<ErrorViewTone>,
    pub is_invalid: bool,
    pub is_compact: Option<bool>,
    pub is_bordered: Option<bool>,
    pub message: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub has_icon: bool,
    pub has_actions: bool,
    pub has_children: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorViewNormalizedProps {
    pub state_input: ErrorViewStateInput,
    pub message: String,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub tone_source_attr: &'static str,
    pub compact_source_attr: &'static str,
    pub bordered_source_attr: &'static str,
}

pub fn resolve_state(input: ErrorViewStateInput) -> ErrorViewState {
    error_view_state::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ErrorViewState) -> String {
    error_view_state::compose_class_name(base_class_name, state)
}

pub fn resolve_agent_contract(input: ErrorViewAgentContractInput) -> ErrorViewAgentContract {
    let state = if input.is_visible {
        ErrorViewAgentState::Visible
    } else {
        ErrorViewAgentState::Hidden
    };

    let action = if input.has_actions {
        ErrorViewAgentAction::AnnounceWithActions
    } else {
        ErrorViewAgentAction::AnnounceOnly
    };

    let action_source = if input.has_actions {
        ErrorViewAgentActionSource::ActionsSlot
    } else {
        ErrorViewAgentActionSource::MessageOnly
    };

    let source = if input.message_source_attr != "default"
        || input.aria_source_attr != "default"
        || input.class_source_attr != "default"
    {
        ErrorViewAgentSource::Custom
    } else {
        ErrorViewAgentSource::Default
    };

    let motion_source = if input.motion_source_attr == "custom" {
        ErrorViewAgentMotionSource::Custom
    } else {
        ErrorViewAgentMotionSource::Default
    };

    ErrorViewAgentContract {
        schema_name: ERROR_VIEW_AGENT_SCHEMA,
        schema_version: ErrorViewAgentSchemaVersion::V1,
        intent: ErrorViewAgentIntent::ErrorFeedback,
        action,
        state,
        source,
        state_source: ErrorViewAgentStateSource::InvalidProp,
        action_source,
        motion_source,
        config_policy: ErrorViewAgentConfigPolicy::Whitelist,
    }
}

fn source_attr_from_presence(is_present: bool) -> &'static str {
    if is_present { "prop" } else { "default" }
}

fn resolve_bool_axis(value: Option<bool>, default_value: bool) -> (bool, &'static str) {
    if let Some(value) = value {
        return (value, "is-prop");
    }
    (default_value, "default")
}

pub fn normalize_props(input: ErrorViewNormalizeInput) -> ErrorViewNormalizedProps {
    let tone = input.tone.unwrap_or_default();
    let tone_source_attr = source_attr_from_presence(input.tone.is_some());

    let (compact, compact_source_attr) = resolve_bool_axis(input.is_compact, false);
    let (bordered, bordered_source_attr) = resolve_bool_axis(input.is_bordered, false);

    let (message, has_custom_message) = normalize_message(input.message);
    let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);
    let class_name = normalize_optional_text(input.class_name);

    ErrorViewNormalizedProps {
        state_input: ErrorViewStateInput {
            tone,
            is_invalid: input.is_invalid,
            compact,
            bordered,
            has_icon: input.has_icon,
            has_actions: input.has_actions,
            has_children: input.has_children,
            has_custom_message,
            has_custom_aria_label,
            has_custom_class_name: class_name.is_some(),
            has_custom_motion: input.has_custom_motion,
        },
        message,
        aria_label,
        class_name,
        tone_source_attr,
        compact_source_attr,
        bordered_source_attr,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
