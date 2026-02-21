use crate::ActionBarState;
use serde::{Deserialize, Serialize};

/// Component protocol contract for `components/action-bar/src`.
///
/// This schema is intentionally minimal and versioned so component-specific
/// protocol fields can evolve without breaking deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ActionBarComponentSchemaVersion {
    #[default]
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActionBarRenderCapability {
    SelectionSummary,
    ClearAction,
    ChildrenSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionBarRenderPolicy {
    pub allow_selection_summary: bool,
    pub allow_clear_action: bool,
    pub allow_children_slot: bool,
}

impl ActionBarRenderPolicy {
    fn from_capabilities(capabilities: &[ActionBarRenderCapability]) -> Self {
        Self {
            allow_selection_summary: capabilities
                .contains(&ActionBarRenderCapability::SelectionSummary),
            allow_clear_action: capabilities.contains(&ActionBarRenderCapability::ClearAction),
            allow_children_slot: capabilities.contains(&ActionBarRenderCapability::ChildrenSlot),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionBarComponentSpec {
    #[serde(default)]
    pub schema_version: ActionBarComponentSchemaVersion,
    #[serde(default = "default_render_capabilities")]
    pub render_capabilities: Vec<ActionBarRenderCapability>,
}

impl Default for ActionBarComponentSpec {
    fn default() -> Self {
        Self {
            schema_version: ActionBarComponentSchemaVersion::default(),
            render_capabilities: default_render_capabilities(),
        }
    }
}

impl ActionBarComponentSpec {
    pub fn render_policy(&self) -> ActionBarRenderPolicy {
        ActionBarRenderPolicy::from_capabilities(&self.render_capabilities)
    }
}

fn default_render_capabilities() -> Vec<ActionBarRenderCapability> {
    vec![
        ActionBarRenderCapability::SelectionSummary,
        ActionBarRenderCapability::ClearAction,
        ActionBarRenderCapability::ChildrenSlot,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarAgentIntent {
    BulkSelection,
}

impl ActionBarAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarAgentIntent::BulkSelection => "bulk-selection",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarAgentAction {
    ClearSelection,
}

impl ActionBarAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarAgentAction::ClearSelection => "clear-selection",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarStreamingPolicy {
    Optional,
}

impl ActionBarStreamingPolicy {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarStreamingPolicy::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarStreamingFallback {
    Snapshot,
}

impl ActionBarStreamingFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarStreamingFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarOutputMode {
    Snapshot,
}

impl ActionBarOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarOutputMode::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionBarOutputStatus {
    Validated,
}

impl ActionBarOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            ActionBarOutputStatus::Validated => "validated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionBarAgentDataAttrs {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub streaming_policy: &'static str,
    pub streaming_fallback: &'static str,
    pub output_mode: &'static str,
    pub output_status: &'static str,
    pub state_phase: &'static str,
    pub state_position: &'static str,
    pub state_selection: &'static str,
    pub source_selected_count: &'static str,
    pub source_clear_action: &'static str,
    pub source_motion: &'static str,
}

pub const ACTION_BAR_AGENT_SCHEMA: &str = "ui.action-bar.contract.v1";

pub fn agent_data_attrs(state: ActionBarState) -> ActionBarAgentDataAttrs {
    ActionBarAgentDataAttrs {
        schema: ACTION_BAR_AGENT_SCHEMA,
        intent: ActionBarAgentIntent::BulkSelection.as_attr(),
        action: ActionBarAgentAction::ClearSelection.as_attr(),
        streaming_policy: ActionBarStreamingPolicy::Optional.as_attr(),
        streaming_fallback: ActionBarStreamingFallback::Snapshot.as_attr(),
        output_mode: ActionBarOutputMode::Snapshot.as_attr(),
        output_status: ActionBarOutputStatus::Validated.as_attr(),
        state_phase: state.phase_attr,
        state_position: state.position_attr,
        state_selection: state.selection_attr,
        source_selected_count: state.selected_count_source_attr,
        source_clear_action: state.clear_action_source_attr,
        source_motion: state.motion_source_attr,
    }
}

#[cfg(test)]
#[path = "../test/protocol.rs"]
mod tests;
