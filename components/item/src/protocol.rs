use crate::logic::ItemRenderState;
use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/item/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ItemComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ItemComponentSpec {
    #[serde(default)]
    pub schema_version: ItemComponentSchemaVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAgentIntent {
    CollectionItem,
}

impl ItemAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemAgentIntent::CollectionItem => "collection-item",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAgentAction {
    Render,
}

impl ItemAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemAgentAction::Render => "render",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAgentStreamMode {
    Streaming,
    Snapshot,
}

impl ItemAgentStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemAgentStreamMode::Streaming => "streaming",
            ItemAgentStreamMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemStreamingPolicy {
    Optional,
}

impl ItemStreamingPolicy {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemStreamingPolicy::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemStreamingFallback {
    Snapshot,
}

impl ItemStreamingFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemStreamingFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAgentOutputMode {
    Snapshot,
}

impl ItemAgentOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemAgentOutputMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemOutputStatus {
    Validated,
}

impl ItemOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ItemOutputStatus::Validated => "validated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemAgentDataAttrs {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub streaming_policy: &'static str,
    pub streaming_fallback: &'static str,
    pub stream_mode: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
    pub state_variant: &'static str,
    pub state_size: &'static str,
    pub source_variant: &'static str,
    pub source_size: &'static str,
}

pub const ITEM_AGENT_SCHEMA: &str = "ui.item.agent-contract.v1";
const _: ItemComponentSchemaVersion = ItemComponentSchemaVersion::V1;
const _: ItemComponentSpec = ItemComponentSpec {
    schema_version: ItemComponentSchemaVersion::V1,
};
const _: ItemAgentStreamMode = ItemAgentStreamMode::Streaming;

pub fn agent_data_attrs(state: ItemRenderState) -> ItemAgentDataAttrs {
    ItemAgentDataAttrs {
        schema: ITEM_AGENT_SCHEMA,
        intent: ItemAgentIntent::CollectionItem.as_attr(),
        action: ItemAgentAction::Render.as_attr(),
        streaming_policy: ItemStreamingPolicy::Optional.as_attr(),
        streaming_fallback: ItemStreamingFallback::Snapshot.as_attr(),
        stream_mode: ItemAgentStreamMode::Snapshot.as_attr(),
        output_mode: ItemAgentOutputMode::Snapshot.as_attr(),
        output_status: ItemOutputStatus::Validated.as_attr(),
        state_variant: state.variant_attr,
        state_size: state.size_attr,
        source_variant: state.variant_source_attr,
        source_size: state.size_source_attr,
    }
}

#[cfg(test)]
#[path = "test/protocol.rs"]
mod tests;
