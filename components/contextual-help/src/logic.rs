use leptos::prelude::{Callback, Signal};
use ui_headless::PopoverPlacement;
use ui_state_primitives::contextual_help as contextual_help_state;

pub use contextual_help_state::{
    ContextualHelpOpenInteractionIntent, ContextualHelpOpenInteractionIntentOutput,
    ContextualHelpOpenInteractionSource, ContextualHelpOpenInteractionSyncInput,
    ContextualHelpOpenInteractionSyncOutput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextualHelpVariant {
    #[default]
    Help,
    Info,
}

impl ContextualHelpVariant {
    pub fn default_label(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "Help",
            ContextualHelpVariant::Info => "Info",
        }
    }

    pub fn class_name(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "ui-contextual-help--variant-help",
            ContextualHelpVariant::Info => "ui-contextual-help--variant-info",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ContextualHelpVariant::Help => "help",
            ContextualHelpVariant::Info => "info",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpStateInput {
    pub variant: ContextualHelpVariant,
    pub placement: PopoverPlacement,
    pub is_disabled: bool,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub has_heading: bool,
    pub has_footer: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_id: bool,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
}

pub struct ContextualHelpOpenStateInput {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

pub struct ContextualHelpOpenStateConfig {
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
    pub has_custom_open: bool,
    pub has_custom_default_open: bool,
    pub has_custom_on_open_change: bool,
    pub is_controlled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpState {
    pub variant: ContextualHelpVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub placement: PopoverPlacement,
    pub placement_class: &'static str,
    pub placement_attr: &'static str,
    pub is_disabled: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub has_heading: bool,
    pub heading_class: &'static str,
    pub heading_attr: &'static str,
    pub has_footer: bool,
    pub footer_class: &'static str,
    pub footer_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub has_custom_id: bool,
    pub id_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
    pub is_controlled: bool,
    pub open_mode_class: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub default_open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub const CONTEXTUAL_HELP_AGENT_SCHEMA: &str = "ui.contextual-help.v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpLlmOutputMode {
    Streaming,
    Snapshot,
}

impl ContextualHelpLlmOutputMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::Snapshot => "snapshot",
        }
    }
}

pub const CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE: ContextualHelpLlmOutputMode =
    ContextualHelpLlmOutputMode::Snapshot;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpStreamingRequirement {
    Required,
    Optional,
}

impl ContextualHelpStreamingRequirement {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpStreamingPolicy {
    pub requirement: ContextualHelpStreamingRequirement,
    pub fallback_mode: ContextualHelpLlmOutputMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpLlmOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl ContextualHelpLlmOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}
const _: [ContextualHelpLlmOutputStatus; 3] = [
    ContextualHelpLlmOutputStatus::Draft,
    ContextualHelpLlmOutputStatus::Verified,
    ContextualHelpLlmOutputStatus::Submittable,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpAgentIntent {
    Help,
    Info,
}

impl ContextualHelpAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Help => "help",
            Self::Info => "info",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpAgentAction {
    Idle,
    ToggleOpen,
    Dismiss,
    ExternalSync,
}

impl ContextualHelpAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::ToggleOpen => "toggle-open",
            Self::Dismiss => "dismiss",
            Self::ExternalSync => "external-sync",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualHelpAgentState {
    Open,
    Closed,
}

impl ContextualHelpAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualHelpAgentContract {
    pub schema: &'static str,
    pub intent: &'static str,
    pub action: &'static str,
    pub state: &'static str,
    pub source: &'static str,
}

fn resolve_agent_intent(variant: ContextualHelpVariant) -> ContextualHelpAgentIntent {
    match variant {
        ContextualHelpVariant::Help => ContextualHelpAgentIntent::Help,
        ContextualHelpVariant::Info => ContextualHelpAgentIntent::Info,
    }
}

fn resolve_agent_action(source: ContextualHelpOpenInteractionSource) -> ContextualHelpAgentAction {
    match source {
        ContextualHelpOpenInteractionSource::Initial => ContextualHelpAgentAction::Idle,
        ContextualHelpOpenInteractionSource::TriggerPress => ContextualHelpAgentAction::ToggleOpen,
        ContextualHelpOpenInteractionSource::DismissPress => ContextualHelpAgentAction::Dismiss,
        ContextualHelpOpenInteractionSource::ExternalSync => {
            ContextualHelpAgentAction::ExternalSync
        }
    }
}

fn resolve_agent_state(is_open: bool) -> ContextualHelpAgentState {
    if is_open {
        ContextualHelpAgentState::Open
    } else {
        ContextualHelpAgentState::Closed
    }
}

pub fn resolve_agent_contract(
    variant: ContextualHelpVariant,
    source: ContextualHelpOpenInteractionSource,
    is_open: bool,
) -> ContextualHelpAgentContract {
    let intent = resolve_agent_intent(variant);
    let action = resolve_agent_action(source);
    let state = resolve_agent_state(is_open);

    ContextualHelpAgentContract {
        schema: CONTEXTUAL_HELP_AGENT_SCHEMA,
        intent: intent.as_attr(),
        action: action.as_attr(),
        state: state.as_attr(),
        source: source.as_attr(),
    }
}

pub fn resolve_llm_output_mode(is_streaming: bool) -> ContextualHelpLlmOutputMode {
    if is_streaming {
        ContextualHelpLlmOutputMode::Streaming
    } else {
        CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE
    }
}

pub fn resolve_streaming_policy(is_reader_surface: bool) -> ContextualHelpStreamingPolicy {
    if is_reader_surface {
        ContextualHelpStreamingPolicy {
            requirement: ContextualHelpStreamingRequirement::Required,
            fallback_mode: CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE,
        }
    } else {
        ContextualHelpStreamingPolicy {
            requirement: ContextualHelpStreamingRequirement::Optional,
            fallback_mode: CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE,
        }
    }
}

pub fn resolve_llm_output_status(
    output_mode: ContextualHelpLlmOutputMode,
) -> ContextualHelpLlmOutputStatus {
    match output_mode {
        ContextualHelpLlmOutputMode::Streaming => ContextualHelpLlmOutputStatus::Draft,
        ContextualHelpLlmOutputMode::Snapshot => ContextualHelpLlmOutputStatus::Verified,
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_trigger_aria_label(
    variant: ContextualHelpVariant,
    aria_label: Option<String>,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        (label, true)
    } else {
        (variant.default_label().into(), false)
    }
}

pub fn resolve_id(id: Option<String>, fallback: String) -> (String, bool) {
    if let Some(id) = normalize_optional_text(id) {
        (id, true)
    } else {
        (fallback, false)
    }
}

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

pub fn resolve_open_state_config(
    input: ContextualHelpOpenStateInput,
) -> ContextualHelpOpenStateConfig {
    let ContextualHelpOpenStateInput {
        open,
        default_open,
        on_open_change,
    } = input;
    let primitive = contextual_help_state::resolve_open_config(
        contextual_help_state::ContextualHelpOpenConfigInput {
            has_custom_open: open.is_some(),
            default_open,
            has_custom_on_open_change: on_open_change.is_some(),
        },
    );

    ContextualHelpOpenStateConfig {
        open,
        default_open: primitive.default_open,
        on_open_change,
        has_custom_open: primitive.has_custom_open,
        has_custom_default_open: primitive.has_custom_default_open,
        has_custom_on_open_change: primitive.has_custom_on_open_change,
        is_controlled: primitive.is_controlled,
    }
}

pub fn resolve_generated_id(provider_generated_id: Option<String>) -> String {
    provider_generated_id.unwrap_or_else(|| "ui-contextual-help-0".into())
}

pub fn resolve_open_interaction_intent(
    intent: ContextualHelpOpenInteractionIntent,
) -> ContextualHelpOpenInteractionIntentOutput {
    contextual_help_state::resolve_open_interaction_intent(intent)
}

pub fn resolve_open_interaction_sync(
    input: ContextualHelpOpenInteractionSyncInput,
) -> ContextualHelpOpenInteractionSyncOutput {
    contextual_help_state::resolve_open_interaction_sync(input)
}

fn placement_class(placement: PopoverPlacement) -> &'static str {
    match placement {
        PopoverPlacement::BottomStart => "ui-contextual-help--placement-bottom-start",
        PopoverPlacement::BottomEnd => "ui-contextual-help--placement-bottom-end",
        PopoverPlacement::TopStart => "ui-contextual-help--placement-top-start",
        PopoverPlacement::TopEnd => "ui-contextual-help--placement-top-end",
    }
}

pub fn resolve_state(input: ContextualHelpStateInput) -> ContextualHelpState {
    ContextualHelpState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        placement: input.placement,
        placement_class: placement_class(input.placement),
        placement_attr: input.placement.as_str(),
        is_disabled: input.is_disabled,
        state_class: if input.is_disabled {
            "ui-contextual-help--disabled"
        } else {
            "ui-contextual-help--enabled"
        },
        state_attr: if input.is_disabled {
            "disabled"
        } else {
            "enabled"
        },
        has_heading: input.has_heading,
        heading_class: if input.has_heading {
            "ui-contextual-help--with-heading"
        } else {
            "ui-contextual-help--no-heading"
        },
        heading_attr: if input.has_heading {
            "present"
        } else {
            "absent"
        },
        has_footer: input.has_footer,
        footer_class: if input.has_footer {
            "ui-contextual-help--with-footer"
        } else {
            "ui-contextual-help--no-footer"
        },
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        has_custom_aria_label: input.has_custom_aria_label,
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        has_custom_id: input.has_custom_id,
        id_source_attr: if input.has_custom_id {
            "custom"
        } else {
            "auto"
        },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_motion: input.has_custom_motion,
        is_controlled: input.is_controlled,
        open_mode_class: if input.is_controlled {
            "ui-contextual-help--controlled"
        } else {
            "ui-contextual-help--uncontrolled"
        },
        open_mode_attr: if input.is_controlled {
            "controlled"
        } else {
            "uncontrolled"
        },
        open_source_attr: if input.has_custom_open {
            "custom"
        } else {
            "default"
        },
        default_open_source_attr: if input.has_custom_default_open {
            "provided"
        } else {
            "implicit"
        },
        open_change_source_attr: if input.has_custom_on_open_change {
            "provided"
        } else {
            "none"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ContextualHelpState) -> String {
    let mut classes = vec![
        "ui-contextual-help".into(),
        state.variant_class.into(),
        state.placement_class.into(),
        state.state_class.into(),
        state.heading_class.into(),
        state.footer_class.into(),
        state.open_mode_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-contextual-help--custom-class".into());
    }

    if state.has_custom_motion {
        classes.push("ui-contextual-help--custom-motion".into());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
