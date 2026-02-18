pub use ui_state_primitives::tag::{
    DEFAULT_REMOVE_ARIA_LABEL, TagSize, TagState, TagStateInput, TagVariant,
    normalize_optional_text, normalize_remove_aria_label, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagNormalizedInput {
    pub class_name: Option<String>,
    pub remove_aria_label: String,
    pub state: TagState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentSchemaVersion {
    V1,
}

impl TagAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentIntent {
    Token,
}

impl TagAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Token => "token",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentAction {
    Initialize,
    RemovePointer,
}

impl TagAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::RemovePointer => "remove-pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentStateAxis {
    Static,
    Removable,
    Disabled,
}

impl TagAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Removable => "removable",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentSource {
    Init,
    RemovePointer,
}

impl TagAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::RemovePointer => "remove-pointer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentOutputStatus {
    Verified,
    Submittable,
}

impl TagAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentStreamSupport {
    Unsupported,
}

impl TagAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagAgentStreamFallback {
    FullSnapshot,
}

impl TagAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagAgentCapabilities {
    pub can_remove: bool,
    pub can_disable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagAgentContract {
    pub schema_name: &'static str,
    pub schema_version: TagAgentSchemaVersion,
    pub intent: TagAgentIntent,
    pub action: TagAgentAction,
    pub state: TagAgentStateAxis,
    pub source: TagAgentSource,
    pub output_status: TagAgentOutputStatus,
    pub stream_support: TagAgentStreamSupport,
    pub stream_fallback: TagAgentStreamFallback,
    pub capabilities: TagAgentCapabilities,
}

pub fn normalize_tag_input(
    variant: TagVariant,
    size: TagSize,
    disabled: bool,
    removable: bool,
    has_remove_handler: bool,
    remove_aria_label: Option<String>,
    class_name: Option<String>,
) -> TagNormalizedInput {
    let class_name = normalize_optional_text(class_name);
    let (remove_aria_label, has_custom_remove_aria_label) =
        normalize_remove_aria_label(remove_aria_label);

    let state = resolve_state(TagStateInput {
        variant,
        size,
        disabled,
        removable,
        has_remove_handler,
        has_custom_remove_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    TagNormalizedInput {
        class_name,
        remove_aria_label,
        state,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TagState) -> String {
    let mut classes = vec![
        "ui-tag".to_string(),
        state.variant_class.to_string(),
        state.size_class.to_string(),
        state.state_class.to_string(),
    ];

    if state.is_enabled {
        classes.push("ui-tag--enabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-tag--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_agent_action(source: TagAgentSource) -> TagAgentAction {
    match source {
        TagAgentSource::Init => TagAgentAction::Initialize,
        TagAgentSource::RemovePointer => TagAgentAction::RemovePointer,
    }
}

pub fn resolve_agent_state_axis(state: TagState) -> TagAgentStateAxis {
    if state.is_disabled {
        TagAgentStateAxis::Disabled
    } else if state.is_removable {
        TagAgentStateAxis::Removable
    } else {
        TagAgentStateAxis::Static
    }
}

pub fn resolve_agent_output_status(source: TagAgentSource) -> TagAgentOutputStatus {
    match source {
        TagAgentSource::Init => TagAgentOutputStatus::Verified,
        TagAgentSource::RemovePointer => TagAgentOutputStatus::Submittable,
    }
}

pub fn resolve_agent_contract(state: TagState, source: TagAgentSource) -> TagAgentContract {
    TagAgentContract {
        schema_name: "ui.tag.agent-contract",
        schema_version: TagAgentSchemaVersion::V1,
        intent: TagAgentIntent::Token,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: TagAgentStreamSupport::Unsupported,
        stream_fallback: TagAgentStreamFallback::FullSnapshot,
        capabilities: TagAgentCapabilities {
            can_remove: state.is_removable,
            can_disable: true,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_state_markers() {
        let normalized = normalize_tag_input(
            TagVariant::Default,
            TagSize::Sm,
            false,
            false,
            false,
            None,
            Some("docs-tag-custom".to_string()),
        );

        let class_name = compose_class_name(normalized.class_name, normalized.state);

        for token in [
            "ui-tag",
            "ui-tag--variant-default",
            "ui-tag--size-sm",
            "ui-tag--static",
            "ui-tag--enabled",
            "ui-tag--custom-class",
            "docs-tag-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn normalize_tag_input_centralizes_trim_and_state_resolution() {
        let normalized = normalize_tag_input(
            TagVariant::Surface,
            TagSize::Lg,
            false,
            true,
            true,
            Some("  Remove framework  ".to_string()),
            Some("  docs-tag-custom  ".to_string()),
        );

        assert_eq!(normalized.class_name, Some("docs-tag-custom".to_string()));
        assert_eq!(normalized.remove_aria_label, "Remove framework");
        assert!(normalized.state.is_removable);
        assert_eq!(normalized.state.remove_label_source_attr, "custom");
    }

    #[test]
    fn agent_contract_is_schema_typed_and_snapshot_fallback_is_explicit() {
        let state = resolve_state(TagStateInput {
            variant: TagVariant::Default,
            size: TagSize::Md,
            disabled: false,
            removable: true,
            has_remove_handler: true,
            has_custom_remove_aria_label: false,
            has_custom_class_name: false,
        });
        let contract = resolve_agent_contract(state, TagAgentSource::RemovePointer);

        assert_eq!(contract.schema_name, "ui.tag.agent-contract");
        assert_eq!(contract.schema_version.as_str(), "1");
        assert_eq!(contract.intent.as_str(), "token");
        assert_eq!(contract.action.as_str(), "remove-pointer");
        assert_eq!(contract.state.as_str(), "removable");
        assert_eq!(contract.source.as_str(), "remove-pointer");
        assert_eq!(contract.output_status.as_str(), "submittable");
        assert_eq!(contract.stream_support.as_str(), "unsupported");
        assert_eq!(contract.stream_fallback.as_str(), "full-snapshot");
        assert!(contract.capabilities.can_remove);
        assert!(contract.capabilities.can_disable);
    }
}
