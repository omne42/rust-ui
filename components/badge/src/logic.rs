use std::borrow::Cow;

pub use ui_state_primitives::badge::{
    BadgeState, BadgeStateInput, BadgeVariant, normalize_optional_text, resolve_state,
};

pub fn resolve_variant(variant: Option<BadgeVariant>) -> BadgeVariant {
    variant.unwrap_or_default()
}

pub const BADGE_AGENT_SCHEMA_NAME: &str = "ui.badge.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentSchemaVersion {
    V1,
}

impl BadgeAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentIntent {
    StatusDisplay,
}

impl BadgeAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::StatusDisplay => "status-display",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentAction {
    Initialize,
}

impl BadgeAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentStateAxis {
    Solid,
    Outline,
}

impl BadgeAgentStateAxis {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Outline => "outline",
        }
    }

    pub fn from_state(state: BadgeState) -> Self {
        if state.is_outline {
            Self::Outline
        } else {
            Self::Solid
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentSource {
    DefaultClassName,
    CustomClassName,
}

impl BadgeAgentSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::DefaultClassName => "default",
            Self::CustomClassName => "custom",
        }
    }

    pub const fn from_has_custom_class_name(has_custom_class_name: bool) -> Self {
        if has_custom_class_name {
            Self::CustomClassName
        } else {
            Self::DefaultClassName
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentStreamSupport {
    Unsupported,
}

impl BadgeAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentStreamFallback {
    Snapshot,
}

impl BadgeAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentStreamMode {
    Snapshot,
}

impl BadgeAgentStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeAgentOutputStatus {
    Verified,
}

impl BadgeAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadgeAgentContract {
    pub schema_name: &'static str,
    pub schema_version: BadgeAgentSchemaVersion,
    pub intent: BadgeAgentIntent,
    pub action: BadgeAgentAction,
    pub state: BadgeAgentStateAxis,
    pub source: BadgeAgentSource,
    pub stream_support: BadgeAgentStreamSupport,
    pub stream_fallback: BadgeAgentStreamFallback,
    pub stream_mode: BadgeAgentStreamMode,
    pub output_status: BadgeAgentOutputStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BadgeRenderState {
    pub state: BadgeState,
    pub class_name: String,
    pub agent_contract: BadgeAgentContract,
}

pub fn resolve_agent_contract(state: BadgeState) -> BadgeAgentContract {
    let source = BadgeAgentSource::from_has_custom_class_name(state.has_custom_class_name);

    BadgeAgentContract {
        schema_name: BADGE_AGENT_SCHEMA_NAME,
        schema_version: BadgeAgentSchemaVersion::V1,
        intent: BadgeAgentIntent::StatusDisplay,
        action: BadgeAgentAction::Initialize,
        state: BadgeAgentStateAxis::from_state(state),
        source,
        stream_support: BadgeAgentStreamSupport::Unsupported,
        stream_fallback: BadgeAgentStreamFallback::Snapshot,
        stream_mode: BadgeAgentStreamMode::Snapshot,
        output_status: BadgeAgentOutputStatus::Verified,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: BadgeState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-badge"),
        Cow::Borrowed(state.variant_class),
        Cow::Borrowed(state.fill_class),
    ];

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-badge--custom-class"));
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(Cow::Owned(base_class_name));
    }

    let mut class_name = String::new();
    for (index, class) in classes.iter().enumerate() {
        if index > 0 {
            class_name.push(' ');
        }
        class_name.push_str(class.as_ref());
    }

    class_name
}

pub fn resolve_render_state(
    variant: Option<BadgeVariant>,
    class_name: Option<String>,
) -> BadgeRenderState {
    let variant = resolve_variant(variant);
    let class_name = normalize_optional_text(class_name);
    let state = resolve_state(BadgeStateInput {
        variant,
        has_custom_class_name: class_name.is_some(),
    });
    let class_name = compose_class_name(class_name, state);
    let agent_contract = resolve_agent_contract(state);

    BadgeRenderState {
        state,
        class_name,
        agent_contract,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
