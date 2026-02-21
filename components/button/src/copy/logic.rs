use crate::snippet as snippet_logic;
use ui_state_primitives::button_copy::{
    ButtonCopyStateInput, normalize_optional_text as normalize_state_text,
    resolve_state as resolve_copy_state,
};

pub const DEFAULT_COPY_LABEL: &str = snippet_logic::DEFAULT_COPY_LABEL;
pub const DEFAULT_COPIED_LABEL: &str = snippet_logic::DEFAULT_COPIED_LABEL;
pub const DEFAULT_COPY_FAILED_STATUS_TEXT: &str = "Copy failed";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonCopyMode {
    #[default]
    TextOnly,
    IconOnly,
    IconAndText,
}

impl ButtonCopyMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::TextOnly => "text-only",
            Self::IconOnly => "icon-only",
            Self::IconAndText => "icon-and-text",
        }
    }

    pub fn shows_text(self) -> bool {
        !matches!(self, Self::IconOnly)
    }

    pub fn shows_icon(self) -> bool {
        !matches!(self, Self::TextOnly)
    }

    pub fn is_icon_only(self) -> bool {
        matches!(self, Self::IconOnly)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonCopyAgentSchemaVersion {
    V1,
}

impl ButtonCopyAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonCopyAgentIntent {
    ClipboardCopy,
}

impl ButtonCopyAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClipboardCopy => "clipboard-copy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonCopyAgentAction {
    Copy,
}

impl ButtonCopyAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonCopyAgentStateAxis {
    Ready,
    Disabled,
    Empty,
}

impl ButtonCopyAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Disabled => "disabled",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonCopyAgentOutputStatus {
    Idle,
    Loading,
    Copied,
    Error,
}

impl ButtonCopyAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Loading => "loading",
            Self::Copied => "copied",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyAgentCapabilities {
    pub can_copy: bool,
    pub can_visual_feedback: bool,
    pub can_announce_feedback: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ButtonCopyAgentSchemaVersion,
    pub intent: ButtonCopyAgentIntent,
    pub action: ButtonCopyAgentAction,
    pub state: ButtonCopyAgentStateAxis,
    pub capabilities: ButtonCopyAgentCapabilities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonCopyViewState {
    pub is_copyable: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_text: bool,
    pub state_attr: &'static str,
    pub mode: ButtonCopyMode,
    pub mode_attr: &'static str,
    pub shows_text: bool,
    pub shows_icon: bool,
    pub is_icon_only: bool,
    pub has_custom_label: bool,
    pub has_custom_copied_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonCopyTextContract {
    pub label: String,
    pub copied_label: String,
    pub aria_label: String,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    normalize_state_text(value)
}

pub fn resolve_text_contract(
    label: Option<String>,
    copied_label: Option<String>,
    aria_label: Option<String>,
) -> ButtonCopyTextContract {
    let snippet_logic::SnippetTextContract {
        copy_label,
        copied_label,
        ..
    } = snippet_logic::resolve_text_contract(
        label,
        copied_label,
        None,
        None,
        snippet_logic::SnippetTextFallbacks {
            copy_label: Some(DEFAULT_COPY_LABEL.into()),
            copied_label: Some(DEFAULT_COPIED_LABEL.into()),
            copy_aria_label: None,
            copy_error_label: None,
        },
    );
    let aria_label = normalize_optional_text(aria_label).unwrap_or_else(|| copy_label.clone());

    ButtonCopyTextContract {
        label: copy_label,
        copied_label,
        aria_label,
    }
}

pub fn resolve_text_contract_with_defaults(
    label: Option<String>,
    default_label: Option<String>,
    copied_label: Option<String>,
    default_copied_label: Option<String>,
    aria_label: Option<String>,
) -> ButtonCopyTextContract {
    resolve_text_contract(
        label.or(default_label),
        copied_label.or(default_copied_label),
        aria_label,
    )
}

pub fn resolve_copy_failed_status_text(
    copy_failed_status_text: Option<String>,
    fallback_copy_failed_status_text: Option<String>,
) -> String {
    normalize_optional_text(copy_failed_status_text)
        .or_else(|| normalize_optional_text(fallback_copy_failed_status_text))
        .unwrap_or_else(|| DEFAULT_COPY_FAILED_STATUS_TEXT.to_string())
}

pub fn resolve_view_state(
    text: &str,
    disabled: bool,
    mode: ButtonCopyMode,
    has_custom_label: bool,
    has_custom_copied_label: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ButtonCopyViewState {
    let state = resolve_copy_state(ButtonCopyStateInput {
        text,
        is_disabled: disabled,
    });

    ButtonCopyViewState {
        is_copyable: state.is_copyable,
        is_disabled: state.is_disabled,
        is_enabled: state.is_enabled,
        has_text: state.has_text,
        state_attr: state.state_attr,
        mode,
        mode_attr: mode.as_attr(),
        shows_text: mode.shows_text(),
        shows_icon: mode.shows_icon(),
        is_icon_only: mode.is_icon_only(),
        has_custom_label,
        has_custom_copied_label,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

pub fn resolve_agent_state_axis(view_state: ButtonCopyViewState) -> ButtonCopyAgentStateAxis {
    if view_state.is_disabled {
        ButtonCopyAgentStateAxis::Disabled
    } else if !view_state.has_text {
        ButtonCopyAgentStateAxis::Empty
    } else {
        ButtonCopyAgentStateAxis::Ready
    }
}

pub fn resolve_agent_contract(view_state: ButtonCopyViewState) -> ButtonCopyAgentContract {
    ButtonCopyAgentContract {
        schema_name: "ui.button-copy.agent-contract",
        schema_version: ButtonCopyAgentSchemaVersion::V1,
        intent: ButtonCopyAgentIntent::ClipboardCopy,
        action: ButtonCopyAgentAction::Copy,
        state: resolve_agent_state_axis(view_state),
        capabilities: ButtonCopyAgentCapabilities {
            can_copy: view_state.is_copyable,
            can_visual_feedback: view_state.is_copyable,
            can_announce_feedback: view_state.is_copyable,
        },
    }
}

pub fn resolve_agent_output_status(
    is_copying: bool,
    has_copy_error: bool,
    copied: bool,
) -> ButtonCopyAgentOutputStatus {
    if is_copying {
        ButtonCopyAgentOutputStatus::Loading
    } else if has_copy_error {
        ButtonCopyAgentOutputStatus::Error
    } else if copied {
        ButtonCopyAgentOutputStatus::Copied
    } else {
        ButtonCopyAgentOutputStatus::Idle
    }
}

pub fn resolve_agent_output_status_attr(
    is_copying: bool,
    has_copy_error: bool,
    copied: bool,
) -> &'static str {
    resolve_agent_output_status(is_copying, has_copy_error, copied).as_str()
}

pub fn compose_class_name(base_class_name: Option<String>, state: ButtonCopyViewState) -> String {
    let mut classes = vec!["ui-button-copy".to_string()];

    if state.is_copyable {
        classes.push("ui-button-copy--copyable".to_string());
    }
    if state.is_disabled {
        classes.push("ui-button-copy--disabled".to_string());
    }
    if !state.has_text {
        classes.push("ui-button-copy--empty".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-button-copy--custom-label".to_string());
    }
    if state.has_custom_copied_label {
        classes.push("ui-button-copy--custom-copied-label".to_string());
    }
    match state.mode {
        ButtonCopyMode::TextOnly => classes.push("ui-button-copy--text-only".to_string()),
        ButtonCopyMode::IconOnly => classes.push("ui-button-copy--icon-only".to_string()),
        ButtonCopyMode::IconAndText => classes.push("ui-button-copy--icon-and-text".to_string()),
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/copy/logic.rs"]
mod tests;
