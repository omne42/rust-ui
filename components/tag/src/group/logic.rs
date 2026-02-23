use leptos::prelude::Signal;

pub use ui_state_primitives::tag_group::{
    Tag, TagGroupItemState, TagGroupItemStateInput, TagGroupState, merge_describedby_ids,
    normalize_optional_text, resolve_item_state, resolve_state,
};

pub const DEFAULT_ID_BASE: &str = "tag-group";
pub const DEFAULT_ARIA_LABEL: &str = "Tags";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupValueSource {
    Default,
    Custom,
}

impl TagGroupValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            TagGroupValueSource::Default => "default",
            TagGroupValueSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupPresenceSource {
    Missing,
    Provided,
}

impl TagGroupPresenceSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            TagGroupPresenceSource::Missing => "missing",
            TagGroupPresenceSource::Provided => "provided",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagGroupNormalizedInput {
    pub id_base: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub aria_label: String,
    pub class_name: String,
    pub lang: Option<String>,
    pub id_base_source: TagGroupValueSource,
    pub aria_label_source: TagGroupValueSource,
    pub class_name_source: TagGroupValueSource,
    pub lang_source: TagGroupPresenceSource,
}

#[derive(Clone)]
pub struct TagGroupBooleanInput {
    pub is_disabled: bool,
    pub is_invalid: Signal<bool>,
    pub is_required: Signal<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupRootStateInput {
    pub is_disabled: bool,
    pub has_remove_callback: bool,
    pub is_invalid: bool,
    pub is_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupRenderableItemStateInput {
    pub is_group_disabled: bool,
    pub has_remove_callback: bool,
    pub is_tag_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentSchemaVersion {
    V1,
}

impl TagGroupAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentIntent {
    Collection,
}

impl TagGroupAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Collection => "collection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentAction {
    Initialize,
    RemovePointer,
}

impl TagGroupAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::RemovePointer => "remove-pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentStateAxis {
    Empty,
    Disabled,
    Invalid,
    Ready,
}

impl TagGroupAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Disabled => "disabled",
            Self::Invalid => "invalid",
            Self::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentSource {
    Init,
    RemovePointer,
}

impl TagGroupAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::RemovePointer => "remove-pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentOutputStatus {
    Verified,
    Submittable,
}

impl TagGroupAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentStreamSupport {
    Unsupported,
}

impl TagGroupAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagGroupAgentStreamFallback {
    FullSnapshot,
}

impl TagGroupAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupAgentCapabilities {
    pub can_remove: bool,
    pub can_validate: bool,
    pub can_disable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagGroupAgentContract {
    pub schema_name: &'static str,
    pub schema_version: TagGroupAgentSchemaVersion,
    pub intent: TagGroupAgentIntent,
    pub action: TagGroupAgentAction,
    pub state: TagGroupAgentStateAxis,
    pub source: TagGroupAgentSource,
    pub output_status: TagGroupAgentOutputStatus,
    pub stream_support: TagGroupAgentStreamSupport,
    pub stream_fallback: TagGroupAgentStreamFallback,
    pub capabilities: TagGroupAgentCapabilities,
}

pub fn normalize_group_input(
    id_base: Option<String>,
    label: Option<String>,
    description: Option<String>,
    error: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
) -> TagGroupNormalizedInput {
    let normalized_id_base = normalize_optional_text(id_base);
    let id_base_source = if normalized_id_base.is_some() {
        TagGroupValueSource::Custom
    } else {
        TagGroupValueSource::Default
    };
    let id_base = normalized_id_base.unwrap_or_else(|| DEFAULT_ID_BASE.into());

    let label = normalize_optional_text(label);
    let description = normalize_optional_text(description);
    let error = normalize_optional_text(error);
    let normalized_aria_label = normalize_optional_text(aria_label);
    let aria_label_source = if normalized_aria_label.is_some() {
        TagGroupValueSource::Custom
    } else {
        TagGroupValueSource::Default
    };
    let aria_label = normalized_aria_label.unwrap_or_else(|| DEFAULT_ARIA_LABEL.into());

    let base_class = "ui-tag-group".to_string();
    let normalized_class_name = class_name.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| format!("{base_class} {trimmed}"))
    });
    let class_name_source = if normalized_class_name.is_some() {
        TagGroupValueSource::Custom
    } else {
        TagGroupValueSource::Default
    };
    let class_name = normalized_class_name.unwrap_or(base_class);

    let lang = normalize_optional_text(lang);
    let lang_source = if lang.is_some() {
        TagGroupPresenceSource::Provided
    } else {
        TagGroupPresenceSource::Missing
    };

    TagGroupNormalizedInput {
        id_base,
        label,
        description,
        error,
        aria_label,
        class_name,
        lang,
        id_base_source,
        aria_label_source,
        class_name_source,
        lang_source,
    }
}

pub fn normalize_group_bool_input(
    is_disabled: Option<bool>,
    is_invalid: Option<Signal<bool>>,
    is_required: Option<Signal<bool>>,
) -> TagGroupBooleanInput {
    TagGroupBooleanInput {
        is_disabled: is_disabled.unwrap_or(false),
        is_invalid: is_invalid.unwrap_or_else(|| Signal::derive(|| false)),
        is_required: is_required.unwrap_or_else(|| Signal::derive(|| false)),
    }
}

pub fn resolve_group_state(tags: &[Tag], input: TagGroupRootStateInput) -> TagGroupState {
    resolve_state(
        tags,
        input.is_disabled,
        input.has_remove_callback,
        input.is_invalid,
        input.is_required,
    )
}

pub fn resolve_group_item_state(input: TagGroupRenderableItemStateInput) -> TagGroupItemState {
    resolve_item_state(TagGroupItemStateInput {
        group_disabled: input.is_group_disabled,
        supports_removal: input.has_remove_callback,
        tag_disabled: input.is_tag_disabled,
    })
}

pub fn resolve_agent_action(source: TagGroupAgentSource) -> TagGroupAgentAction {
    match source {
        TagGroupAgentSource::Init => TagGroupAgentAction::Initialize,
        TagGroupAgentSource::RemovePointer => TagGroupAgentAction::RemovePointer,
    }
}

pub fn resolve_agent_state_axis(state: TagGroupState) -> TagGroupAgentStateAxis {
    if state.is_empty {
        TagGroupAgentStateAxis::Empty
    } else if state.is_disabled {
        TagGroupAgentStateAxis::Disabled
    } else if state.is_invalid {
        TagGroupAgentStateAxis::Invalid
    } else {
        TagGroupAgentStateAxis::Ready
    }
}

pub fn resolve_agent_output_status(source: TagGroupAgentSource) -> TagGroupAgentOutputStatus {
    match source {
        TagGroupAgentSource::Init => TagGroupAgentOutputStatus::Verified,
        TagGroupAgentSource::RemovePointer => TagGroupAgentOutputStatus::Submittable,
    }
}

pub fn resolve_agent_contract(
    state: TagGroupState,
    source: TagGroupAgentSource,
    has_remove_callback: bool,
) -> TagGroupAgentContract {
    TagGroupAgentContract {
        schema_name: "ui.tag-group.agent-contract",
        schema_version: TagGroupAgentSchemaVersion::V1,
        intent: TagGroupAgentIntent::Collection,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: TagGroupAgentStreamSupport::Unsupported,
        stream_fallback: TagGroupAgentStreamFallback::FullSnapshot,
        capabilities: TagGroupAgentCapabilities {
            can_remove: has_remove_callback && state.has_items && !state.is_disabled,
            can_validate: true,
            can_disable: true,
        },
    }
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
