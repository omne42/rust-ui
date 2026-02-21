use crate::modal::{ModalDescriptionState, ModalPartState, ModalPartStateInput, ModalSlot};
use leptos::prelude::*;
use std::borrow::Cow;

pub const DEFAULT_ID_BASE: &str = "ui-modal";
pub const DEFAULT_TITLE: &str = "Modal";
pub const DEFAULT_OPEN: bool = false;
pub const MODAL_AGENT_SCHEMA: &str = "ui.modal.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentSchemaVersion {
    V1,
}

impl ModalAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentIntent {
    OverlayDialog,
}

impl ModalAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayDialog => "overlay.dialog",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentAction {
    Open,
    Close,
}

impl ModalAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Close => "close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentState {
    Open,
    Closed,
}

impl ModalAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentSource {
    Controlled,
    Uncontrolled,
}

impl ModalAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentConfigPolicy {
    Whitelist,
}

impl ModalAgentConfigPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalAgentOutputStatus {
    Verified,
}

impl ModalAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalAgentCapabilities {
    pub has_description: bool,
    pub can_open: bool,
    pub can_close: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalAgentContractInput {
    pub is_open: bool,
    pub open_mode: ModalOpenMode,
    pub has_description: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ModalAgentSchemaVersion,
    pub intent: ModalAgentIntent,
    pub action: ModalAgentAction,
    pub state: ModalAgentState,
    pub source: ModalAgentSource,
    pub config_policy: ModalAgentConfigPolicy,
    pub output_status: ModalAgentOutputStatus,
    pub capabilities: ModalAgentCapabilities,
}

pub fn resolve_agent_contract(input: ModalAgentContractInput) -> ModalAgentContract {
    let action = if input.is_open {
        ModalAgentAction::Close
    } else {
        ModalAgentAction::Open
    };
    let state = if input.is_open {
        ModalAgentState::Open
    } else {
        ModalAgentState::Closed
    };
    let source = if input.open_mode == ModalOpenMode::Controlled {
        ModalAgentSource::Controlled
    } else {
        ModalAgentSource::Uncontrolled
    };

    ModalAgentContract {
        schema_name: MODAL_AGENT_SCHEMA,
        schema_version: ModalAgentSchemaVersion::V1,
        intent: ModalAgentIntent::OverlayDialog,
        action,
        state,
        source,
        config_policy: ModalAgentConfigPolicy::Whitelist,
        output_status: ModalAgentOutputStatus::Verified,
        capabilities: ModalAgentCapabilities {
            has_description: input.has_description,
            can_open: !input.is_open,
            can_close: input.is_open,
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalOpenMode {
    Controlled,
    Uncontrolled,
}

impl ModalOpenMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalOpenSource {
    Controlled,
    Default,
    ImplicitDefault,
}

impl ModalOpenSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Default => "default",
            Self::ImplicitDefault => "implicit-default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalOpenChangeSource {
    Custom,
    None,
}

impl ModalOpenChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalOpenPropSource {
    IsOpen,
    None,
}

impl ModalOpenPropSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsOpen => "is_open",
            Self::None => "none",
        }
    }
}

pub struct ModalOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct ModalOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: ModalOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub open_prop_source: ModalOpenPropSource,
}

pub fn normalize_open_state(input: ModalOpenStateInput) -> ModalOpenState {
    let open = input.is_open;
    let mode = if open.is_some() {
        ModalOpenMode::Controlled
    } else {
        ModalOpenMode::Uncontrolled
    };

    ModalOpenState {
        open,
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        on_open_change: input.on_open_change,
        mode,
        has_default_open: input.default_open.is_some(),
        has_open_change_handler: input.on_open_change.is_some(),
        open_prop_source: if input.is_open.is_some() {
            ModalOpenPropSource::IsOpen
        } else {
            ModalOpenPropSource::None
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalOpenContract {
    pub mode: ModalOpenMode,
    pub open_source: ModalOpenSource,
    pub open_change_source: ModalOpenChangeSource,
    pub open_prop_source: ModalOpenPropSource,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
}

pub fn resolve_open_contract(state: &ModalOpenState) -> ModalOpenContract {
    ModalOpenContract {
        mode: state.mode,
        open_source: if state.mode == ModalOpenMode::Controlled {
            ModalOpenSource::Controlled
        } else if state.has_default_open {
            ModalOpenSource::Default
        } else {
            ModalOpenSource::ImplicitDefault
        },
        open_change_source: if state.has_open_change_handler {
            ModalOpenChangeSource::Custom
        } else {
            ModalOpenChangeSource::None
        },
        open_prop_source: state.open_prop_source,
        has_custom_default_open: state.has_default_open,
        has_custom_on_open_change: state.has_open_change_handler,
    }
}

pub struct ModalContentStateInput {
    pub id_base: String,
    pub title: String,
    pub description: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModalContentState {
    pub id_base: String,
    pub title: String,
    pub description: Option<String>,
    pub description_state: ModalDescriptionState,
    pub class_name: Option<String>,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
}

pub fn resolve_content_state(input: ModalContentStateInput) -> ModalContentState {
    let has_custom_id_base = !input.id_base.trim().is_empty();
    let has_custom_title = !input.title.trim().is_empty();
    let id_base = normalize_id_base(input.id_base);
    let title = normalize_required_text(input.title, DEFAULT_TITLE);
    let description = normalize_optional_text(input.description);
    let class_name = normalize_optional_text(input.class_name);

    ModalContentState {
        id_base,
        title,
        description_state: if description.is_some() {
            ModalDescriptionState::WithDescription
        } else {
            ModalDescriptionState::TitleOnly
        },
        has_custom_id_base,
        has_custom_title,
        has_custom_description: description.is_some(),
        has_custom_class_name: class_name.is_some(),
        description,
        class_name,
    }
}

pub fn normalize_on_exit_complete(callback: Option<Callback<()>>) -> Callback<()> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        trimmed.into()
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: ModalPartStateInput) -> ModalPartState {
    ModalPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        description_state: input.description_state,
        state_attr: input.description_state.as_state_attr(),
        description_attr: input.description_state.as_description_attr(),
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        description_source_attr: source_attr(input.has_custom_description),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: ModalPartState,
) -> Cow<'static, str> {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];

    if state.slot == ModalSlot::Root {
        if state.description_state.shows_description() {
            classes.push(Cow::Borrowed("ui-modal--with-description"));
        } else {
            classes.push(Cow::Borrowed("ui-modal--title-only"));
        }

        if state.has_custom_id_base {
            classes.push(Cow::Borrowed("ui-modal--custom-id"));
        }

        if state.has_custom_title {
            classes.push(Cow::Borrowed("ui-modal--custom-title"));
        }

        if state.has_custom_description {
            classes.push(Cow::Borrowed("ui-modal--custom-description"));
        }

        if state.has_custom_motion {
            classes.push(Cow::Borrowed("ui-modal--custom-motion"));
        }

        if state.has_on_exit_complete {
            classes.push(Cow::Borrowed("ui-modal--custom-exit"));
        }

        if state.has_custom_class_name {
            classes.push(Cow::Borrowed("ui-modal--custom-class"));
            if let Some(base_class_name) = base_class_name {
                classes.push(Cow::Owned(base_class_name));
            }
        }
    }

    if classes.len() == 1 {
        return classes.remove(0);
    }

    Cow::Owned(
        classes
            .iter()
            .map(|class_name| class_name.as_ref())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
