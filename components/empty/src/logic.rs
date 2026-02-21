pub use ui_state_primitives::empty::{
    EmptyMediaVariant, EmptyPartState, EmptyPartStateInput, EmptySlot, compose_class_name,
    normalize_optional_text, resolve_state,
};

pub const EMPTY_COMPONENT_SCHEMA_NAME: &str = "ui-empty";
pub const EMPTY_COMPONENT_SCHEMA_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentIntent {
    EmptyDisplay,
}

impl EmptyAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentIntent::EmptyDisplay => "empty-display",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentAction {
    RenderSnapshot,
}

impl EmptyAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentAction::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentSource {
    Default,
    Custom,
}

impl EmptyAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentSource::Default => "default",
            EmptyAgentSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

pub const EMPTY_AGENT_OUTPUT_STATUS_ALL: [EmptyAgentOutputStatus; 3] = [
    EmptyAgentOutputStatus::Draft,
    EmptyAgentOutputStatus::Verified,
    EmptyAgentOutputStatus::Submittable,
];

impl EmptyAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentOutputStatus::Draft => "draft",
            EmptyAgentOutputStatus::Verified => "verified",
            EmptyAgentOutputStatus::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentStreamSupport {
    Required,
    Optional,
}

pub const EMPTY_AGENT_STREAM_SUPPORT_ALL: [EmptyAgentStreamSupport; 2] = [
    EmptyAgentStreamSupport::Required,
    EmptyAgentStreamSupport::Optional,
];

impl EmptyAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentStreamSupport::Required => "required",
            EmptyAgentStreamSupport::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyAgentStreamFallback {
    Snapshot,
}

impl EmptyAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            EmptyAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyAgentContract {
    pub schema_name: &'static str,
    pub schema_version: &'static str,
    pub intent: EmptyAgentIntent,
    pub action: EmptyAgentAction,
    pub state: &'static str,
    pub source: EmptyAgentSource,
    pub stream_support: EmptyAgentStreamSupport,
    pub stream_fallback: EmptyAgentStreamFallback,
    pub output_status: EmptyAgentOutputStatus,
}

pub fn resolve_agent_contract(state: EmptyPartState) -> EmptyAgentContract {
    let source = if state.class_source_attr == "custom" || state.variant_source_attr == "custom" {
        EmptyAgentSource::Custom
    } else {
        EmptyAgentSource::Default
    };
    let stream_support = EmptyAgentStreamSupport::Optional;
    let output_status = EmptyAgentOutputStatus::Verified;
    debug_assert!(EMPTY_AGENT_STREAM_SUPPORT_ALL.contains(&stream_support));
    debug_assert!(EMPTY_AGENT_OUTPUT_STATUS_ALL.contains(&output_status));

    EmptyAgentContract {
        schema_name: EMPTY_COMPONENT_SCHEMA_NAME,
        schema_version: EMPTY_COMPONENT_SCHEMA_VERSION,
        intent: EmptyAgentIntent::EmptyDisplay,
        action: EmptyAgentAction::RenderSnapshot,
        state: state.state_attr,
        source,
        stream_support,
        stream_fallback: EmptyAgentStreamFallback::Snapshot,
        output_status,
    }
}

pub fn normalize_part(
    slot: EmptySlot,
    class_name: Option<String>,
    media_variant: Option<EmptyMediaVariant>,
) -> (String, EmptyPartState) {
    let class_name = normalize_optional_text(class_name);
    let state = resolve_state(EmptyPartStateInput {
        slot,
        media_variant: media_variant.unwrap_or_default(),
        has_custom_class_name: class_name.is_some(),
    });
    let class_name = compose_class_name(class_name, state);
    (class_name, state)
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
