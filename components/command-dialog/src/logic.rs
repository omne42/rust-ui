use std::borrow::Cow;

use crate::command_dialog::{
    CommandDialogPartState, CommandDialogPartStateInput, CommandDialogSlot,
};
pub use ui_state_primitives::overlay_trigger::{
    OverlayTriggerState, OverlayTriggerStateOptions, use_overlay_trigger_state,
};

pub const DEFAULT_ID_BASE: &str = "ui-command-dialog";
pub const DEFAULT_TITLE: &str = "Command Menu";
pub const DEFAULT_CLOSE_ON_ACTION: bool = true;
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_DEFAULT_OPEN: bool = false;
pub const COMMAND_DIALOG_AGENT_SCHEMA: &str = "ui.command-dialog.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentSchemaVersion {
    V1,
}

impl CommandDialogAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentIntent {
    CommandDiscovery,
}

impl CommandDialogAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandDiscovery => "command.discovery",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentAction {
    CloseOnAction,
    KeepOpen,
}

impl CommandDialogAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CloseOnAction => "close-on-action",
            Self::KeepOpen => "keep-open",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentState {
    Open,
    Closed,
}

impl CommandDialogAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentSource {
    Controlled,
    Uncontrolled,
}

impl CommandDialogAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentStreamMode {
    Streaming,
    Snapshot,
}

impl CommandDialogAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}
const _: [CommandDialogAgentStreamMode; 2] = [
    CommandDialogAgentStreamMode::Streaming,
    CommandDialogAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentStreamSupport {
    Required,
    Optional,
}

impl CommandDialogAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}
const _: [CommandDialogAgentStreamSupport; 2] = [
    CommandDialogAgentStreamSupport::Required,
    CommandDialogAgentStreamSupport::Optional,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl CommandDialogAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::CommitReady => "commit-ready",
        }
    }
}
const _: [CommandDialogAgentOutputStatus; 3] = [
    CommandDialogAgentOutputStatus::Draft,
    CommandDialogAgentOutputStatus::Verified,
    CommandDialogAgentOutputStatus::CommitReady,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDialogAgentConfigPolicy {
    Whitelist,
}

impl CommandDialogAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandDialogAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CommandDialogAgentSchemaVersion,
    pub intent: CommandDialogAgentIntent,
    pub action: CommandDialogAgentAction,
    pub state: CommandDialogAgentState,
    pub source: CommandDialogAgentSource,
    pub stream_support: CommandDialogAgentStreamSupport,
    pub stream_mode: CommandDialogAgentStreamMode,
    pub stream_fallback: CommandDialogAgentStreamMode,
    pub output_status: CommandDialogAgentOutputStatus,
    pub config_policy: CommandDialogAgentConfigPolicy,
    pub action_source: &'static str,
    pub open_change_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandDialogNormalizationInput {
    pub open_input: Option<bool>,
    pub default_open: Option<bool>,
    pub has_open_prop: bool,
    pub has_on_action: bool,
    pub has_on_open_change: bool,
    pub close_on_action: bool,
    pub id_base: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub placeholder: Option<String>,
    pub empty_label: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub has_custom_command_motion: bool,
    pub has_custom_overlay_motion: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandDialogNormalized {
    pub open_input: Option<bool>,
    pub default_open: Option<bool>,
    pub id_base: String,
    pub title: String,
    pub description_text: String,
    pub placeholder_text: String,
    pub empty_label_text: String,
    pub aria_label_text: String,
    pub class_name: Option<String>,
    pub close_on_action: bool,
    pub disabled: bool,
    pub is_controlled: bool,
    pub has_description: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_on_action: bool,
    pub has_custom_on_open_change: bool,
    pub has_custom_default_open: bool,
    pub has_custom_close_on_action: bool,
    pub has_custom_disabled: bool,
    pub has_custom_command_motion: bool,
    pub has_custom_overlay_motion: bool,
}

pub fn normalize_props(input: CommandDialogNormalizationInput) -> CommandDialogNormalized {
    let id_base = normalize_id_base(input.id_base);
    let has_custom_id_base = id_base != DEFAULT_ID_BASE;

    let title = normalize_title(input.title);
    let has_custom_title = title != DEFAULT_TITLE;

    let description = normalize_optional_text(input.description);
    let has_custom_description = description.is_some();
    let description_text = resolve_text_with_empty_default(description.as_deref());

    let placeholder = normalize_optional_text(input.placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let placeholder_text = resolve_text_with_empty_default(placeholder.as_deref());

    let empty_label = normalize_optional_text(input.empty_label);
    let has_custom_empty_label = empty_label.is_some();
    let empty_label_text = resolve_text_with_empty_default(empty_label.as_deref());

    let aria_label = normalize_optional_text(input.aria_label);
    let has_custom_aria_label = aria_label.is_some();
    let aria_label_text = resolve_text_with_empty_default(aria_label.as_deref());

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let is_controlled = input.has_open_prop;
    let has_custom_default_open = input.default_open.is_some();
    let has_custom_close_on_action = input.close_on_action != DEFAULT_CLOSE_ON_ACTION;
    let has_custom_disabled = input.is_disabled.is_some() || input.disabled != DEFAULT_DISABLED;
    let disabled = normalize_is_disabled(input.is_disabled, input.disabled);

    CommandDialogNormalized {
        open_input: input.open_input,
        default_open: input.default_open,
        id_base,
        title,
        description_text,
        placeholder_text,
        empty_label_text,
        aria_label_text,
        class_name,
        close_on_action: input.close_on_action,
        disabled,
        is_controlled,
        has_description: has_custom_description,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_placeholder,
        has_custom_empty_label,
        has_custom_aria_label,
        has_custom_class_name,
        has_custom_on_action: input.has_on_action,
        has_custom_on_open_change: input.has_on_open_change,
        has_custom_default_open,
        has_custom_close_on_action,
        has_custom_disabled,
        has_custom_command_motion: input.has_custom_command_motion,
        has_custom_overlay_motion: input.has_custom_overlay_motion,
    }
}

pub fn normalize_open_state_options(
    open: Option<bool>,
    default_open: Option<bool>,
) -> OverlayTriggerStateOptions {
    OverlayTriggerStateOptions {
        is_open: open,
        default_open,
        on_open_change: None,
    }
}

pub fn should_emit_open_change(current_open: bool, next_open: bool) -> bool {
    current_open != next_open
}

pub fn apply_open_change(
    state: &mut OverlayTriggerState,
    controlled_open: Option<bool>,
    next_open: bool,
) {
    state.sync_controlled(controlled_open);
    state.set_open(next_open);
}

pub fn state_attr(is_open: bool) -> &'static str {
    if is_open { "open" } else { "closed" }
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn close_on_action_attr(close_on_action: bool) -> &'static str {
    if close_on_action { "true" } else { "false" }
}

pub fn disabled_attr(disabled: bool) -> &'static str {
    if disabled { "true" } else { "false" }
}

pub fn open_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn normalize_title(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TITLE.into())
}

pub fn resolve_text_with_empty_default(value: Option<&str>) -> String {
    value.unwrap_or_default().to_string()
}

pub fn resolve_part_state(
    normalized: &CommandDialogNormalized,
    slot: CommandDialogSlot,
    is_open: bool,
) -> CommandDialogPartState {
    resolve_state(CommandDialogPartStateInput {
        slot,
        is_open,
        has_description: normalized.has_description,
        close_on_action: normalized.close_on_action,
        disabled: normalized.disabled,
        is_controlled: normalized.is_controlled,
        has_custom_id_base: normalized.has_custom_id_base,
        has_custom_title: normalized.has_custom_title,
        has_custom_description: normalized.has_custom_description,
        has_custom_placeholder: normalized.has_custom_placeholder,
        has_custom_empty_label: normalized.has_custom_empty_label,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: matches!(slot, CommandDialogSlot::Root)
            && normalized.has_custom_class_name,
        has_custom_on_action: normalized.has_custom_on_action,
        has_custom_on_open_change: normalized.has_custom_on_open_change,
        has_custom_default_open: normalized.has_custom_default_open,
        has_custom_close_on_action: normalized.has_custom_close_on_action,
        has_custom_disabled: normalized.has_custom_disabled,
        has_custom_command_motion: normalized.has_custom_command_motion,
        has_custom_overlay_motion: normalized.has_custom_overlay_motion,
    })
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: CommandDialogPartStateInput) -> CommandDialogPartState {
    let enabled = !input.disabled;
    let is_uncontrolled = !input.is_controlled;

    CommandDialogPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.is_open),
        description_attr: description_attr(input.has_description),
        close_on_action_attr: close_on_action_attr(input.close_on_action),
        disabled_attr: disabled_attr(input.disabled),
        open_mode_attr: open_mode_attr(input.is_controlled),
        open_attr: input.is_open.then_some("true"),
        is_open: input.is_open,
        has_description: input.has_description,
        close_on_action: input.close_on_action,
        disabled: input.disabled,
        enabled,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_placeholder: input.has_custom_placeholder,
        has_custom_empty_label: input.has_custom_empty_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_on_action: input.has_custom_on_action,
        has_custom_on_open_change: input.has_custom_on_open_change,
        has_custom_default_open: input.has_custom_default_open,
        has_custom_close_on_action: input.has_custom_close_on_action,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_command_motion: input.has_custom_command_motion,
        has_custom_overlay_motion: input.has_custom_overlay_motion,
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        description_source_attr: source_attr(input.has_custom_description),
        placeholder_source_attr: source_attr(input.has_custom_placeholder),
        empty_label_source_attr: source_attr(input.has_custom_empty_label),
        aria_label_source_attr: source_attr(input.has_custom_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name),
        action_source_attr: source_attr(input.has_custom_on_action),
        open_change_source_attr: source_attr(input.has_custom_on_open_change),
        default_open_source_attr: source_attr(input.has_custom_default_open),
        close_on_action_source_attr: source_attr(input.has_custom_close_on_action),
        disabled_source_attr: source_attr(input.has_custom_disabled),
        command_motion_source_attr: source_attr(input.has_custom_command_motion),
        overlay_motion_source_attr: source_attr(input.has_custom_overlay_motion),
    }
}

pub fn resolve_agent_contract(state: CommandDialogPartState) -> CommandDialogAgentContract {
    CommandDialogAgentContract {
        schema_name: COMMAND_DIALOG_AGENT_SCHEMA,
        schema_version: CommandDialogAgentSchemaVersion::V1,
        intent: CommandDialogAgentIntent::CommandDiscovery,
        action: if state.close_on_action {
            CommandDialogAgentAction::CloseOnAction
        } else {
            CommandDialogAgentAction::KeepOpen
        },
        state: if state.is_open {
            CommandDialogAgentState::Open
        } else {
            CommandDialogAgentState::Closed
        },
        source: if state.is_controlled {
            CommandDialogAgentSource::Controlled
        } else {
            CommandDialogAgentSource::Uncontrolled
        },
        stream_support: CommandDialogAgentStreamSupport::Optional,
        stream_mode: CommandDialogAgentStreamMode::Snapshot,
        stream_fallback: CommandDialogAgentStreamMode::Snapshot,
        output_status: CommandDialogAgentOutputStatus::Verified,
        config_policy: CommandDialogAgentConfigPolicy::Whitelist,
        action_source: state.action_source_attr,
        open_change_source: state.open_change_source_attr,
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: CommandDialogPartState,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if matches!(state.slot, CommandDialogSlot::Root) {
        if state.is_open {
            classes.push(Cow::Borrowed("ui-command-dialog--open"));
        } else {
            classes.push(Cow::Borrowed("ui-command-dialog--closed"));
        }

        if state.has_description {
            classes.push(Cow::Borrowed("ui-command-dialog--with-description"));
        } else {
            classes.push(Cow::Borrowed("ui-command-dialog--title-only"));
        }

        if state.close_on_action {
            classes.push(Cow::Borrowed("ui-command-dialog--close-on-action"));
        } else {
            classes.push(Cow::Borrowed("ui-command-dialog--persistent"));
        }

        if state.disabled {
            classes.push(Cow::Borrowed("ui-command-dialog--disabled"));
        }

        if state.is_controlled {
            classes.push(Cow::Borrowed("ui-command-dialog--controlled"));
        } else {
            classes.push(Cow::Borrowed("ui-command-dialog--uncontrolled"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-id"));
        }

        if state.has_custom_title {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-title"));
        }

        if state.has_custom_description {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-description"));
        }

        if state.has_custom_placeholder {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-placeholder"));
        }

        if state.has_custom_empty_label {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-empty-label"));
        }

        if state.has_custom_aria_label {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-aria-label"));
        }

        if state.has_custom_on_action {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-action"));
        }

        if state.has_custom_on_open_change {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-open-change"));
        }

        if state.has_custom_default_open {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-default-open"));
        }

        if state.has_custom_close_on_action {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-close-on-action"));
        }

        if state.has_custom_disabled {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-disabled"));
        }

        if state.has_custom_command_motion {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-command-motion"));
        }

        if state.has_custom_overlay_motion {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-overlay-motion"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-command-dialog--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    } else if let Some(base_class_name) = normalize_optional_text(base_class_name) {
        classes.push(Cow::Owned(base_class_name));
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
