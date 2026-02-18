pub use ui_state_primitives::tag_group::{
    Tag, TagGroupItemStateInput, TagGroupState, merge_describedby_ids, normalize_optional_text,
    resolve_item_state, resolve_state,
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
    let id_base = normalized_id_base.unwrap_or_else(|| DEFAULT_ID_BASE.to_string());

    let label = normalize_optional_text(label);
    let description = normalize_optional_text(description);
    let error = normalize_optional_text(error);
    let normalized_aria_label = normalize_optional_text(aria_label);
    let aria_label_source = if normalized_aria_label.is_some() {
        TagGroupValueSource::Custom
    } else {
        TagGroupValueSource::Default
    };
    let aria_label = normalized_aria_label.unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string());

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
mod tests {
    use super::*;

    #[test]
    fn normalize_group_input_centralizes_default_sources() {
        let normalized = normalize_group_input(None, None, None, None, None, None, None);

        assert_eq!(normalized.id_base, DEFAULT_ID_BASE);
        assert_eq!(normalized.aria_label, DEFAULT_ARIA_LABEL);
        assert_eq!(normalized.class_name, "ui-tag-group");
        assert_eq!(normalized.label, None);
        assert_eq!(normalized.description, None);
        assert_eq!(normalized.error, None);
        assert_eq!(normalized.lang, None);
        assert_eq!(normalized.id_base_source.as_attr(), "default");
        assert_eq!(normalized.aria_label_source.as_attr(), "default");
        assert_eq!(normalized.class_name_source.as_attr(), "default");
        assert_eq!(normalized.lang_source.as_attr(), "missing");
    }

    #[test]
    fn normalize_group_input_trims_user_values() {
        let normalized = normalize_group_input(
            Some("  group-id  ".to_string()),
            Some("  Label  ".to_string()),
            Some("  Desc  ".to_string()),
            Some("  Error  ".to_string()),
            Some("  Custom tags  ".to_string()),
            Some("  custom-class  ".to_string()),
            Some("  zh-CN  ".to_string()),
        );

        assert_eq!(normalized.id_base, "group-id");
        assert_eq!(normalized.label, Some("Label".to_string()));
        assert_eq!(normalized.description, Some("Desc".to_string()));
        assert_eq!(normalized.error, Some("Error".to_string()));
        assert_eq!(normalized.aria_label, "Custom tags");
        assert_eq!(normalized.class_name, "ui-tag-group custom-class");
        assert_eq!(normalized.lang, Some("zh-CN".to_string()));
        assert_eq!(normalized.id_base_source.as_attr(), "custom");
        assert_eq!(normalized.aria_label_source.as_attr(), "custom");
        assert_eq!(normalized.class_name_source.as_attr(), "custom");
        assert_eq!(normalized.lang_source.as_attr(), "provided");
    }

    #[test]
    fn agent_contract_is_schema_typed_and_snapshot_fallback_is_explicit() {
        let state = resolve_state(
            &[
                Tag::new("tag-rust", "Rust"),
                Tag::new("tag-leptos", "Leptos"),
            ],
            false,
            true,
            false,
            false,
        );
        let contract = resolve_agent_contract(state, TagGroupAgentSource::RemovePointer, true);

        assert_eq!(contract.schema_name, "ui.tag-group.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "collection");
        assert_eq!(contract.action.as_str(), "remove-pointer");
        assert_eq!(contract.state.as_str(), "ready");
        assert_eq!(contract.source.as_str(), "remove-pointer");
        assert_eq!(contract.output_status.as_str(), "submittable");
        assert_eq!(contract.stream_support.as_str(), "unsupported");
        assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
        assert!(contract.capabilities.can_remove);
        assert!(contract.capabilities.can_validate);
        assert!(contract.capabilities.can_disable);
    }
}
