use crate::{
    CommandCollectionAttr, CommandDisabledAttr, CommandFilterState, CommandGroup, CommandPartState,
    CommandPartStateInput, CommandQueryAttr, CommandQueryChangeSourceAttr, CommandQueryControlAttr,
    CommandQueryDefaultSourceAttr, CommandRootStateAttr, CommandSlot, CommandSourceAttr,
};
use std::borrow::Cow;
use ui_state_primitives::command as command_primitives;

pub const DEFAULT_ID_BASE: &str = "command";
pub const DEFAULT_PLACEHOLDER: &str = "Type a command or search...";
pub const DEFAULT_EMPTY_LABEL: &str = "No results found.";
pub const DEFAULT_ARIA_LABEL: &str = "Command menu";
pub const DEFAULT_DISABLED: bool = false;
pub const DEFAULT_QUERY: &str = "";
pub const COMMAND_AGENT_SCHEMA: &str = "ui.command.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentSchemaVersion {
    V1,
}

impl CommandAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentIntent {
    CommandDiscovery,
}

impl CommandAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentIntent::CommandDiscovery => "command.discovery",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentAction {
    FilterNavigateSelect,
}

impl CommandAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentAction::FilterNavigateSelect => "filter.navigate.select",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentState {
    Idle,
    QueryResults,
    QueryEmpty,
    Empty,
    Disabled,
}

impl CommandAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentState::Idle => "idle",
            CommandAgentState::QueryResults => "query-results",
            CommandAgentState::QueryEmpty => "query-empty",
            CommandAgentState::Empty => "empty",
            CommandAgentState::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentSource {
    Controlled,
    Uncontrolled,
}

impl CommandAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentSource::Controlled => "controlled",
            CommandAgentSource::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAgentConfigPolicy {
    Whitelist,
}

impl CommandAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            CommandAgentConfigPolicy::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandAgentContractInput {
    pub state_attr: CommandRootStateAttr,
    pub query_control_attr: CommandQueryControlAttr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CommandAgentSchemaVersion,
    pub intent: CommandAgentIntent,
    pub action: CommandAgentAction,
    pub state: CommandAgentState,
    pub source: CommandAgentSource,
    pub config_policy: CommandAgentConfigPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandRootStateInput<'a> {
    pub item_count: usize,
    pub group_count: usize,
    pub is_disabled: bool,
    pub query: &'a str,
    pub has_custom_id_base: bool,
    pub has_custom_placeholder: bool,
    pub has_i18n_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_i18n_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_i18n_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_on_action: bool,
    pub has_custom_motion: bool,
    pub is_query_controlled: bool,
    pub has_custom_default_query: bool,
    pub has_custom_query_change_handler: bool,
}

pub fn state_attr(item_count: usize, is_disabled: bool, has_query: bool) -> CommandRootStateAttr {
    let is_empty = item_count == 0;

    if is_disabled && is_empty {
        CommandRootStateAttr::DisabledEmpty
    } else if is_disabled {
        CommandRootStateAttr::Disabled
    } else if is_empty && has_query {
        CommandRootStateAttr::QueryEmpty
    } else if is_empty {
        CommandRootStateAttr::Empty
    } else if has_query {
        CommandRootStateAttr::QueryResults
    } else {
        CommandRootStateAttr::Default
    }
}

pub fn item_attr(item_count: usize) -> CommandCollectionAttr {
    if item_count == 0 {
        CommandCollectionAttr::Empty
    } else {
        CommandCollectionAttr::Populated
    }
}

pub fn group_attr(group_count: usize) -> CommandCollectionAttr {
    if group_count == 0 {
        CommandCollectionAttr::Empty
    } else {
        CommandCollectionAttr::Populated
    }
}

pub fn query_attr(has_query: bool) -> CommandQueryAttr {
    if has_query {
        CommandQueryAttr::Present
    } else {
        CommandQueryAttr::Absent
    }
}

pub fn disabled_attr(is_disabled: bool) -> CommandDisabledAttr {
    if is_disabled {
        CommandDisabledAttr::Disabled
    } else {
        CommandDisabledAttr::Enabled
    }
}

pub fn query_control_attr(is_controlled: bool) -> CommandQueryControlAttr {
    if is_controlled {
        CommandQueryControlAttr::Controlled
    } else {
        CommandQueryControlAttr::Uncontrolled
    }
}

pub fn query_default_source_attr(has_custom_default_query: bool) -> CommandQueryDefaultSourceAttr {
    if has_custom_default_query {
        CommandQueryDefaultSourceAttr::Provided
    } else {
        CommandQueryDefaultSourceAttr::Empty
    }
}

pub fn query_change_source_attr(
    has_custom_query_change_handler: bool,
) -> CommandQueryChangeSourceAttr {
    if has_custom_query_change_handler {
        CommandQueryChangeSourceAttr::Provided
    } else {
        CommandQueryChangeSourceAttr::None
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn resolve_placeholder(
    value: Option<String>,
    i18n_fallback: Option<&str>,
) -> (String, CommandSourceAttr) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, CommandSourceAttr::Custom);
    }

    if let Some(fallback) =
        i18n_fallback.and_then(|fallback| normalize_optional_text(Some(fallback.into())))
    {
        return (fallback, CommandSourceAttr::I18n);
    }

    (DEFAULT_PLACEHOLDER.into(), CommandSourceAttr::Default)
}

pub fn resolve_empty_label(
    value: Option<String>,
    i18n_fallback: Option<&str>,
) -> (String, CommandSourceAttr) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, CommandSourceAttr::Custom);
    }

    if let Some(fallback) =
        i18n_fallback.and_then(|fallback| normalize_optional_text(Some(fallback.into())))
    {
        return (fallback, CommandSourceAttr::I18n);
    }

    (DEFAULT_EMPTY_LABEL.into(), CommandSourceAttr::Default)
}

pub fn resolve_aria_label(
    value: Option<String>,
    i18n_fallback: Option<&str>,
) -> (String, CommandSourceAttr) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, CommandSourceAttr::Custom);
    }

    if let Some(fallback) =
        i18n_fallback.and_then(|fallback| normalize_optional_text(Some(fallback.into())))
    {
        return (fallback, CommandSourceAttr::I18n);
    }

    (DEFAULT_ARIA_LABEL.into(), CommandSourceAttr::Default)
}

pub fn resolve_default_query(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_QUERY.into())
}

pub fn has_query_text(query: &str) -> bool {
    !query.trim().is_empty()
}

pub fn resolve_agent_contract(input: CommandAgentContractInput) -> CommandAgentContract {
    let state = match input.state_attr {
        CommandRootStateAttr::DisabledEmpty | CommandRootStateAttr::Disabled => {
            CommandAgentState::Disabled
        }
        CommandRootStateAttr::QueryResults => CommandAgentState::QueryResults,
        CommandRootStateAttr::QueryEmpty => CommandAgentState::QueryEmpty,
        CommandRootStateAttr::Empty => CommandAgentState::Empty,
        CommandRootStateAttr::Default => CommandAgentState::Idle,
    };

    let source = match input.query_control_attr {
        CommandQueryControlAttr::Controlled => CommandAgentSource::Controlled,
        CommandQueryControlAttr::Uncontrolled => CommandAgentSource::Uncontrolled,
    };

    CommandAgentContract {
        schema_name: COMMAND_AGENT_SCHEMA,
        schema_version: CommandAgentSchemaVersion::V1,
        intent: CommandAgentIntent::CommandDiscovery,
        action: CommandAgentAction::FilterNavigateSelect,
        state,
        source,
        config_policy: CommandAgentConfigPolicy::Whitelist,
    }
}

pub fn normalize_selected_index(selected_index: Option<usize>, item_count: usize) -> Option<usize> {
    command_primitives::normalize_selected_index(selected_index, item_count)
}

pub fn filter_groups(groups: &[CommandGroup], query: &str) -> CommandFilterState {
    command_primitives::filter_groups(groups, query)
}

fn source_attr(is_custom: bool, has_i18n: bool) -> CommandSourceAttr {
    if is_custom {
        return CommandSourceAttr::Custom;
    }

    if has_i18n {
        return CommandSourceAttr::I18n;
    }

    CommandSourceAttr::Default
}

pub fn resolve_state(input: CommandPartStateInput) -> CommandPartState {
    let has_items = input.item_count > 0;
    let is_empty = !has_items;

    CommandPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.item_count, input.is_disabled, input.has_query),
        item_attr: item_attr(input.item_count),
        group_attr: group_attr(input.group_count),
        query_attr: query_attr(input.has_query),
        disabled_attr: disabled_attr(input.is_disabled),
        item_count: input.item_count,
        group_count: input.group_count,
        is_empty,
        has_items,
        is_disabled: input.is_disabled,
        is_enabled: !input.is_disabled,
        has_query: input.has_query,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_placeholder: input.has_custom_placeholder,
        has_i18n_placeholder: input.has_i18n_placeholder,
        has_custom_empty_label: input.has_custom_empty_label,
        has_i18n_empty_label: input.has_i18n_empty_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_i18n_aria_label: input.has_i18n_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_on_action: input.has_custom_on_action,
        has_custom_motion: input.has_custom_motion,
        is_query_controlled: input.is_query_controlled,
        has_custom_default_query: input.has_custom_default_query,
        has_custom_query_change_handler: input.has_custom_query_change_handler,
        id_source_attr: source_attr(input.has_custom_id_base, false),
        placeholder_source_attr: source_attr(
            input.has_custom_placeholder,
            input.has_i18n_placeholder,
        ),
        empty_label_source_attr: source_attr(
            input.has_custom_empty_label,
            input.has_i18n_empty_label,
        ),
        aria_label_source_attr: source_attr(input.has_custom_aria_label, input.has_i18n_aria_label),
        class_source_attr: source_attr(input.has_custom_class_name, false),
        disabled_source_attr: source_attr(input.has_custom_disabled, false),
        action_source_attr: source_attr(input.has_custom_on_action, false),
        motion_source_attr: source_attr(input.has_custom_motion, false),
        query_control_attr: query_control_attr(input.is_query_controlled),
        query_default_source_attr: query_default_source_attr(input.has_custom_default_query),
        query_change_source_attr: query_change_source_attr(input.has_custom_query_change_handler),
    }
}

pub fn resolve_root_state(input: CommandRootStateInput<'_>) -> CommandPartState {
    resolve_state(CommandPartStateInput {
        slot: CommandSlot::Root,
        item_count: input.item_count,
        group_count: input.group_count,
        is_disabled: input.is_disabled,
        has_query: has_query_text(input.query),
        has_custom_id_base: input.has_custom_id_base,
        has_custom_placeholder: input.has_custom_placeholder,
        has_i18n_placeholder: input.has_i18n_placeholder,
        has_custom_empty_label: input.has_custom_empty_label,
        has_i18n_empty_label: input.has_i18n_empty_label,
        has_custom_aria_label: input.has_custom_aria_label,
        has_i18n_aria_label: input.has_i18n_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_disabled: input.has_custom_disabled,
        has_custom_on_action: input.has_custom_on_action,
        has_custom_motion: input.has_custom_motion,
        is_query_controlled: input.is_query_controlled,
        has_custom_default_query: input.has_custom_default_query,
        has_custom_query_change_handler: input.has_custom_query_change_handler,
    })
}

pub fn compose_class_name(class_name: Option<String>, state: CommandPartState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if matches!(state.slot, CommandSlot::Root) {
        if state.is_empty {
            classes.push(Cow::Borrowed("ui-command--empty"));
        } else {
            classes.push(Cow::Borrowed("ui-command--has-items"));
        }

        if state.is_disabled {
            classes.push(Cow::Borrowed("ui-command--disabled"));
        } else {
            classes.push(Cow::Borrowed("ui-command--enabled"));
        }

        if state.has_query {
            classes.push(Cow::Borrowed("ui-command--querying"));
        } else {
            classes.push(Cow::Borrowed("ui-command--idle"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-command--custom-id"));
        }

        if state.has_custom_placeholder {
            classes.push(Cow::Borrowed("ui-command--custom-placeholder"));
        }

        if state.has_custom_empty_label {
            classes.push(Cow::Borrowed("ui-command--custom-empty-label"));
        }

        if state.has_custom_aria_label {
            classes.push(Cow::Borrowed("ui-command--custom-aria-label"));
        }

        if state.has_custom_disabled {
            classes.push(Cow::Borrowed("ui-command--custom-disabled"));
        }

        if state.has_custom_on_action {
            classes.push(Cow::Borrowed("ui-command--custom-action"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-command--custom-motion"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-command--custom-class"));
            if let Some(class_name) = normalize_optional_text(class_name) {
                classes.push(Cow::Owned(class_name));
            }
        }
    } else if let Some(class_name) = normalize_optional_text(class_name) {
        classes.push(Cow::Owned(class_name));
    }

    classes
        .into_iter()
        .map(Cow::into_owned)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
