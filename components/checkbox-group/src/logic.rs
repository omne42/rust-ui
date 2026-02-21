use std::borrow::Cow;

use leptos::prelude::Signal;
use ui_headless::A11yDirection;
#[cfg(test)]
pub use ui_state_primitives::checkbox_group::resolve_checkbox_group_state;
pub use ui_state_primitives::checkbox_group::{
    CheckboxGroupState, normalize_checkbox_group_label, normalize_checkbox_group_optional_text,
};

const BASE_CLASS_NAME: &str = "ui-checkbox-group";
pub const CHECKBOX_GROUP_AGENT_SCHEMA: &str = "ui.checkbox-group.agent-contract";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxGroupIds {
    pub legend_id: String,
}

pub fn resolve_checkbox_group_ids(id: &str) -> CheckboxGroupIds {
    CheckboxGroupIds {
        legend_id: format!("{id}-label"),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckboxGroupContent {
    pub label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub has_description: bool,
    pub has_error: bool,
}

pub fn resolve_checkbox_group_content(
    label: String,
    description: Option<String>,
    error: Option<String>,
) -> CheckboxGroupContent {
    let label = normalize_checkbox_group_label(label);
    let description = normalize_checkbox_group_optional_text(description);
    let error = normalize_checkbox_group_optional_text(error);

    CheckboxGroupContent {
        has_description: description.is_some(),
        has_error: error.is_some(),
        label,
        description,
        error,
    }
}

pub fn resolve_checkbox_group_class_name(class_name: Option<String>) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(BASE_CLASS_NAME)];
    if let Some(class_name) = normalize_checkbox_group_optional_text(class_name) {
        classes.push(Cow::Owned(class_name));
    }
    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupMotionPhase {
    Active,
    Inactive,
}

impl CheckboxGroupMotionPhase {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupStateSource {
    SemanticProps,
}

impl CheckboxGroupStateSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::SemanticProps => "semantic-props",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupViewState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub is_invalid: bool,
    pub is_valid: bool,
    pub is_required: bool,
    pub is_optional: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub shows_error: bool,
    pub has_messages: bool,
    pub state_source: CheckboxGroupStateSource,
    pub motion_phase: CheckboxGroupMotionPhase,
}

pub fn resolve_checkbox_group_motion_phase(shows_error: bool) -> CheckboxGroupMotionPhase {
    if shows_error {
        CheckboxGroupMotionPhase::Active
    } else {
        CheckboxGroupMotionPhase::Inactive
    }
}

pub fn resolve_checkbox_group_state_source() -> CheckboxGroupStateSource {
    CheckboxGroupStateSource::SemanticProps
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentSchemaVersion {
    V1,
}

impl CheckboxGroupAgentSchemaVersion {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentIntent {
    GroupSelection,
}

impl CheckboxGroupAgentIntent {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::GroupSelection => "group-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentAction {
    RenderSemantic,
    RenderSemanticWithError,
}

impl CheckboxGroupAgentAction {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::RenderSemantic => "render-semantic",
            Self::RenderSemanticWithError => "render-semantic-with-error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentState {
    EnabledValid,
    EnabledInvalid,
    DisabledValid,
    DisabledInvalid,
}

impl CheckboxGroupAgentState {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::EnabledValid => "enabled-valid",
            Self::EnabledInvalid => "enabled-invalid",
            Self::DisabledValid => "disabled-valid",
            Self::DisabledInvalid => "disabled-invalid",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentSource {
    SemanticProps,
}

impl CheckboxGroupAgentSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::SemanticProps => "semantic-props",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentConfigPolicy {
    Whitelist,
}

impl CheckboxGroupAgentConfigPolicy {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentStreamSupport {
    Optional,
}

impl CheckboxGroupAgentStreamSupport {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentStreamFallback {
    Snapshot,
}

impl CheckboxGroupAgentStreamFallback {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxGroupAgentOutputStatus {
    Draft,
    Verified,
    CommitReady,
}

impl CheckboxGroupAgentOutputStatus {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::CommitReady => "commit-ready",
        }
    }
}
const _: [CheckboxGroupAgentOutputStatus; 3] = [
    CheckboxGroupAgentOutputStatus::Draft,
    CheckboxGroupAgentOutputStatus::Verified,
    CheckboxGroupAgentOutputStatus::CommitReady,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupAgentContractInput {
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub shows_error: bool,
    pub state_source: CheckboxGroupStateSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupAgentContract {
    pub schema_name: &'static str,
    pub schema_version: CheckboxGroupAgentSchemaVersion,
    pub intent: CheckboxGroupAgentIntent,
    pub action: CheckboxGroupAgentAction,
    pub state: CheckboxGroupAgentState,
    pub source: CheckboxGroupAgentSource,
    pub state_source: CheckboxGroupStateSource,
    pub config_policy: CheckboxGroupAgentConfigPolicy,
    pub stream_support: CheckboxGroupAgentStreamSupport,
    pub stream_fallback: CheckboxGroupAgentStreamFallback,
    pub output_status: CheckboxGroupAgentOutputStatus,
}

pub fn resolve_checkbox_group_agent_action(shows_error: bool) -> CheckboxGroupAgentAction {
    if shows_error {
        CheckboxGroupAgentAction::RenderSemanticWithError
    } else {
        CheckboxGroupAgentAction::RenderSemantic
    }
}

pub fn resolve_checkbox_group_agent_state(
    is_disabled: bool,
    is_invalid: bool,
) -> CheckboxGroupAgentState {
    match (is_disabled, is_invalid) {
        (false, false) => CheckboxGroupAgentState::EnabledValid,
        (false, true) => CheckboxGroupAgentState::EnabledInvalid,
        (true, false) => CheckboxGroupAgentState::DisabledValid,
        (true, true) => CheckboxGroupAgentState::DisabledInvalid,
    }
}

pub fn resolve_checkbox_group_agent_source(
    state_source: CheckboxGroupStateSource,
) -> CheckboxGroupAgentSource {
    match state_source {
        CheckboxGroupStateSource::SemanticProps => CheckboxGroupAgentSource::SemanticProps,
    }
}

pub fn resolve_checkbox_group_agent_contract(
    input: CheckboxGroupAgentContractInput,
) -> CheckboxGroupAgentContract {
    CheckboxGroupAgentContract {
        schema_name: CHECKBOX_GROUP_AGENT_SCHEMA,
        schema_version: CheckboxGroupAgentSchemaVersion::V1,
        intent: CheckboxGroupAgentIntent::GroupSelection,
        action: resolve_checkbox_group_agent_action(input.shows_error),
        state: resolve_checkbox_group_agent_state(input.is_disabled, input.is_invalid),
        source: resolve_checkbox_group_agent_source(input.state_source),
        state_source: input.state_source,
        config_policy: CheckboxGroupAgentConfigPolicy::Whitelist,
        stream_support: CheckboxGroupAgentStreamSupport::Optional,
        stream_fallback: CheckboxGroupAgentStreamFallback::Snapshot,
        output_status: CheckboxGroupAgentOutputStatus::Verified,
    }
}

pub fn resolve_checkbox_group_view_state(state: CheckboxGroupState) -> CheckboxGroupViewState {
    CheckboxGroupViewState {
        is_disabled: state.is_disabled,
        is_enabled: state.is_enabled,
        is_invalid: state.is_invalid,
        is_valid: state.is_valid,
        is_required: state.is_required,
        is_optional: state.is_optional,
        has_description: state.has_description,
        has_error: state.has_error,
        shows_error: state.shows_error,
        has_messages: state.has_messages,
        state_source: resolve_checkbox_group_state_source(),
        motion_phase: resolve_checkbox_group_motion_phase(state.shows_error),
    }
}

#[derive(Clone)]
pub struct CheckboxGroupOptions {
    pub id: String,
    pub is_disabled: bool,
    pub has_description: bool,
    pub has_error: bool,
    pub aria_describedby: Signal<Option<String>>,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub type CheckboxGroupA11y = ui_headless::CheckboxGroupA11y;

pub fn use_checkbox_group(options: CheckboxGroupOptions) -> CheckboxGroupA11y {
    ui_headless::use_checkbox_group(ui_headless::CheckboxGroupOptions {
        id: options.id,
        is_disabled: options.is_disabled,
        has_description: options.has_description,
        has_error: options.has_error,
        aria_describedby: options.aria_describedby,
        is_invalid: options.is_invalid,
        is_required: options.is_required,
        lang: options.lang,
        dir: options.dir,
    })
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
