pub use ui_state_primitives::select::{
    SelectHorizontalNav, SelectOpenFocusStrategy, SelectState, SelectStateInput,
    compose_class_name, find_typeahead_match, normalize_id_base, normalize_optional_text,
    resolve_disabled_option_count, resolve_horizontal_nav_target, resolve_ids, resolve_placeholder,
    resolve_state, resolve_trigger_disabled, typeahead_char,
};

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectAgentSchema {
    V1,
}

impl SelectAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.select.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectAgentSchemaVersion {
    V1,
}

impl SelectAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectStreamSupport {
    Optional,
}

impl SelectStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectStreamFallback {
    Snapshot,
}

impl SelectStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectStreamMode {
    Snapshot,
}

impl SelectStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectOutputStatus {
    Verified,
}

impl SelectOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectAgentIntent {
    ChooseOption,
}

impl SelectAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ChooseOption => "choose-option",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectAgentAction {
    Idle,
    Open,
    Select,
    Disabled,
}

impl SelectAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Open => "open",
            Self::Select => "select",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectAgentState {
    Closed,
    Open,
    Empty,
    Disabled,
}

impl SelectAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
            Self::Empty => "empty",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
}

pub fn resolve_agent_contract(state: SelectState) -> SelectAgentContract {
    let action = if state.trigger_disabled {
        SelectAgentAction::Disabled
    } else if state.is_open {
        SelectAgentAction::Open
    } else if state.has_selection {
        SelectAgentAction::Select
    } else {
        SelectAgentAction::Idle
    };

    let state_axis = if state.trigger_disabled {
        SelectAgentState::Disabled
    } else if state.is_empty {
        SelectAgentState::Empty
    } else if state.is_open {
        SelectAgentState::Open
    } else {
        SelectAgentState::Closed
    };

    SelectAgentContract {
        schema_attr: SelectAgentSchema::V1.as_attr(),
        schema_version_attr: SelectAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: SelectStreamSupport::Optional.as_attr(),
        stream_fallback_attr: SelectStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: SelectStreamMode::Snapshot.as_attr(),
        output_status_attr: SelectOutputStatus::Verified.as_attr(),
        intent_attr: SelectAgentIntent::ChooseOption.as_attr(),
        action_attr: action.as_attr(),
        state_attr: state_axis.as_attr(),
        source_attr: state.class_source_attr,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
