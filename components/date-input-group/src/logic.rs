use std::borrow::Cow;

use crate::DateInputGroupMotion;

pub use ui_state_primitives::date_input_group::{
    DEFAULT_ARIA_LABEL, DateInputGroupState, DateInputGroupStateInput, DateInputGroupStatus,
    DateInputGroupVariant, DateInputGroupWidth, normalize_aria_label, normalize_optional_text,
    resolve_state, resolve_status, resolve_width,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupStateDeriveInput {
    pub variant: DateInputGroupVariant,
    pub width: DateInputGroupWidth,
    pub status: DateInputGroupStatus,
    pub is_segmented: bool,
    pub has_prefix: bool,
    pub has_suffix: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn derive_state(input: DateInputGroupStateDeriveInput) -> DateInputGroupState {
    resolve_state(DateInputGroupStateInput {
        variant: input.variant,
        width: input.width,
        status: input.status,
        segmented: input.is_segmented,
        has_prefix: input.has_prefix,
        has_suffix: input.has_suffix,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    })
}

pub fn resolve_motion_source_attrs(
    motion: DateInputGroupMotion,
) -> (&'static str, Option<&'static str>) {
    let motion_source_attr = if motion == DateInputGroupMotion::default() {
        "default"
    } else {
        "custom"
    };

    let custom_motion_attr = if motion_source_attr == "custom" {
        Some("true")
    } else {
        None
    };

    (motion_source_attr, custom_motion_attr)
}

pub fn compose_class_name(base_class_name: Option<String>, state: DateInputGroupState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-date-input-group"),
        Cow::Borrowed(state.variant_class),
        Cow::Borrowed(state.width_class),
    ];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-date-input-group--disabled"));
    }

    if state.is_invalid {
        classes.push(Cow::Borrowed("ui-date-input-group--invalid"));
    }

    if state.is_segmented {
        classes.push(Cow::Borrowed("ui-date-input-group--segmented"));
    }

    if state.has_prefix {
        classes.push(Cow::Borrowed("ui-date-input-group--has-prefix"));
    }

    if state.has_suffix {
        classes.push(Cow::Borrowed("ui-date-input-group--has-suffix"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-date-input-group--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    let mut composed = String::new();
    for (index, class_name) in classes.iter().enumerate() {
        if index > 0 {
            composed.push(' ');
        }
        composed.push_str(class_name.as_ref());
    }

    composed
}

pub const DATE_INPUT_GROUP_AGENT_SCHEMA: &str = "ui.date-input-group.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentSchemaVersion {
    V1,
}

impl DateInputGroupAgentSchemaVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentIntent {
    DateInputGroup,
}

impl DateInputGroupAgentIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentIntent::DateInputGroup => "date.input-group",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentAction {
    SnapshotRender,
}

impl DateInputGroupAgentAction {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentAction::SnapshotRender => "snapshot-render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentState {
    Default,
    Disabled,
    Invalid,
    Segmented,
}

impl DateInputGroupAgentState {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentState::Default => "default",
            DateInputGroupAgentState::Disabled => "disabled",
            DateInputGroupAgentState::Invalid => "invalid",
            DateInputGroupAgentState::Segmented => "segmented",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentSource {
    StatePrimitives,
}

impl DateInputGroupAgentSource {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentSource::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentOutputStatus {
    Verified,
}

impl DateInputGroupAgentOutputStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentStreamSupport {
    Unsupported,
}

impl DateInputGroupAgentStreamSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentStreamSupport::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentStreamFallback {
    Snapshot,
}

impl DateInputGroupAgentStreamFallback {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateInputGroupAgentStreamMode {
    Streaming,
    Snapshot,
}

impl DateInputGroupAgentStreamMode {
    pub fn as_str(self) -> &'static str {
        match self {
            DateInputGroupAgentStreamMode::Streaming => "streaming",
            DateInputGroupAgentStreamMode::Snapshot => "snapshot",
        }
    }
}
const _: [DateInputGroupAgentStreamMode; 2] = [
    DateInputGroupAgentStreamMode::Streaming,
    DateInputGroupAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupAgentContract {
    pub schema_name: &'static str,
    pub schema_version: DateInputGroupAgentSchemaVersion,
    pub intent: DateInputGroupAgentIntent,
    pub action: DateInputGroupAgentAction,
    pub state: DateInputGroupAgentState,
    pub source: DateInputGroupAgentSource,
    pub output_status: DateInputGroupAgentOutputStatus,
    pub stream_support: DateInputGroupAgentStreamSupport,
    pub stream_fallback: DateInputGroupAgentStreamFallback,
    pub stream_mode: DateInputGroupAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub config_policy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateInputGroupAgentContractInput {
    pub render_state: DateInputGroupState,
    pub is_custom_motion: bool,
}

fn resolve_agent_state(render_state: DateInputGroupState) -> DateInputGroupAgentState {
    if render_state.is_disabled {
        return DateInputGroupAgentState::Disabled;
    }
    if render_state.is_invalid {
        return DateInputGroupAgentState::Invalid;
    }
    if render_state.is_segmented {
        return DateInputGroupAgentState::Segmented;
    }
    DateInputGroupAgentState::Default
}

fn resolve_motion_source(is_custom_motion: bool) -> &'static str {
    if is_custom_motion {
        "custom"
    } else {
        "default"
    }
}

pub fn resolve_agent_contract(
    input: DateInputGroupAgentContractInput,
) -> DateInputGroupAgentContract {
    DateInputGroupAgentContract {
        schema_name: DATE_INPUT_GROUP_AGENT_SCHEMA,
        schema_version: DateInputGroupAgentSchemaVersion::V1,
        intent: DateInputGroupAgentIntent::DateInputGroup,
        action: DateInputGroupAgentAction::SnapshotRender,
        state: resolve_agent_state(input.render_state),
        source: DateInputGroupAgentSource::StatePrimitives,
        output_status: DateInputGroupAgentOutputStatus::Verified,
        stream_support: DateInputGroupAgentStreamSupport::Unsupported,
        stream_fallback: DateInputGroupAgentStreamFallback::Snapshot,
        stream_mode: DateInputGroupAgentStreamMode::Snapshot,
        state_source: "state-primitives",
        motion_source: resolve_motion_source(input.is_custom_motion),
        aria_source: input.render_state.aria_source_attr,
        class_source: input.render_state.class_source_attr,
        config_policy: "whitelist",
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
