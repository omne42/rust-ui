use crate::snippet::logic as snippet_logic;
use ui_state_primitives::button_copy::{
    ButtonCopyStateInput, normalize_optional_text as normalize_state_text,
    resolve_state as resolve_copy_state,
};

pub const DEFAULT_COPY_LABEL: &str = snippet_logic::DEFAULT_COPY_LABEL;
pub const DEFAULT_COPIED_LABEL: &str = snippet_logic::DEFAULT_COPIED_LABEL;

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
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Copy now  ".to_string())),
            Some("Copy now".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn resolve_text_contract_uses_defaults_when_values_missing() {
        let contract = resolve_text_contract(None, None, None);

        assert_eq!(contract.label, DEFAULT_COPY_LABEL);
        assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
        assert_eq!(contract.aria_label, DEFAULT_COPY_LABEL);
    }

    #[test]
    fn resolve_text_contract_prefers_custom_values_when_present() {
        let contract = resolve_text_contract(
            Some("  Copy URL  ".to_string()),
            Some("  URL copied  ".to_string()),
            Some("  Copy URL to clipboard  ".to_string()),
        );

        assert_eq!(contract.label, "Copy URL");
        assert_eq!(contract.copied_label, "URL copied");
        assert_eq!(contract.aria_label, "Copy URL to clipboard");
    }

    #[test]
    fn resolve_text_contract_falls_back_aria_to_resolved_label() {
        let contract = resolve_text_contract(Some("  Install  ".to_string()), None, None);

        assert_eq!(contract.label, "Install");
        assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
        assert_eq!(contract.aria_label, "Install");
    }

    #[test]
    fn button_copy_mode_contract_exposes_expected_flags() {
        let text_only = resolve_view_state(
            "",
            false,
            ButtonCopyMode::TextOnly,
            false,
            false,
            false,
            false,
        );
        assert_eq!(text_only.mode_attr, "text-only");
        assert!(text_only.shows_text);
        assert!(!text_only.shows_icon);
        assert!(!text_only.is_icon_only);

        let icon_only = resolve_view_state(
            "",
            false,
            ButtonCopyMode::IconOnly,
            false,
            false,
            false,
            false,
        );
        assert_eq!(icon_only.mode_attr, "icon-only");
        assert!(!icon_only.shows_text);
        assert!(icon_only.shows_icon);
        assert!(icon_only.is_icon_only);
    }

    #[test]
    fn empty_text_is_not_copyable() {
        assert!(
            !resolve_view_state(
                "",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
        assert!(
            !resolve_view_state(
                "   ",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn disabled_is_not_copyable_even_when_text_present() {
        assert!(
            !resolve_view_state(
                "hello",
                true,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn enabled_with_text_is_copyable() {
        assert!(
            resolve_view_state(
                "hello",
                false,
                ButtonCopyMode::IconAndText,
                false,
                false,
                false,
                false
            )
            .is_copyable
        );
    }

    #[test]
    fn resolve_view_state_tracks_metadata_flags() {
        let state = resolve_view_state(
            "hello",
            false,
            ButtonCopyMode::IconAndText,
            true,
            true,
            true,
            true,
        );
        assert!(state.is_copyable);
        assert!(!state.is_disabled);
        assert!(state.is_enabled);
        assert!(state.has_text);
        assert_eq!(state.state_attr, "copyable");
        assert_eq!(state.mode_attr, "icon-and-text");
        assert!(state.shows_text);
        assert!(state.shows_icon);
        assert!(!state.is_icon_only);
        assert!(state.has_custom_label);
        assert!(state.has_custom_copied_label);
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_view_state(
                "hello",
                false,
                ButtonCopyMode::IconAndText,
                true,
                true,
                false,
                true,
            ),
        );

        for token in [
            "ui-button-copy",
            "ui-button-copy--copyable",
            "ui-button-copy--custom-label",
            "ui-button-copy--custom-copied-label",
            "ui-button-copy--icon-and-text",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn button_copy_agent_contract_is_schema_typed_and_stateful() {
        let ready_state = resolve_view_state(
            "copy me",
            false,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false,
        );
        let contract = resolve_agent_contract(ready_state);

        assert_eq!(contract.schema_name, "ui.button-copy.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "clipboard-copy");
        assert_eq!(contract.action.as_str(), "copy");
        assert_eq!(contract.state.as_str(), "ready");
        assert!(contract.capabilities.can_copy);
        assert!(contract.capabilities.can_visual_feedback);
        assert!(contract.capabilities.can_announce_feedback);

        let disabled_state = resolve_view_state(
            "copy me",
            true,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false,
        );
        assert_eq!(
            resolve_agent_contract(disabled_state).state.as_str(),
            "disabled"
        );

        let empty_state = resolve_view_state(
            "   ",
            false,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false,
        );
        assert_eq!(resolve_agent_contract(empty_state).state.as_str(), "empty");
    }

    #[test]
    fn button_copy_agent_output_status_prioritizes_loading_then_error_then_copied() {
        assert_eq!(
            resolve_agent_output_status(true, true, true).as_str(),
            "loading"
        );
        assert_eq!(
            resolve_agent_output_status(false, true, true).as_str(),
            "error"
        );
        assert_eq!(
            resolve_agent_output_status(false, false, true).as_str(),
            "copied"
        );
        assert_eq!(
            resolve_agent_output_status(false, false, false).as_str(),
            "idle"
        );
        assert_eq!(
            resolve_agent_output_status_attr(true, false, false),
            "loading"
        );
        assert_eq!(
            resolve_agent_output_status_attr(false, true, false),
            "error"
        );
        assert_eq!(
            resolve_agent_output_status_attr(false, false, true),
            "copied"
        );
        assert_eq!(
            resolve_agent_output_status_attr(false, false, false),
            "idle"
        );
    }
}
