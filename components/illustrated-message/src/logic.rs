pub use ui_state_primitives::illustrated_message::{
    IllustratedMessageViewState, resolve_view_state,
};

pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA: &str = "ui.illustrated-message.agent-contract";
pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION: &str = "v1";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IllustratedMessageStateMarker {
    Shown,
    #[default]
    Hidden,
}

impl IllustratedMessageStateMarker {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageStateMarker::Shown => "shown",
            IllustratedMessageStateMarker::Hidden => "hidden",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IllustratedMessageRenderMarker {
    #[default]
    Empty,
    Populated,
}

impl IllustratedMessageRenderMarker {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageRenderMarker::Empty => "empty",
            IllustratedMessageRenderMarker::Populated => "populated",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IllustratedMessageTextSource {
    Provided,
    #[default]
    Missing,
    Blank,
}

impl IllustratedMessageTextSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageTextSource::Provided => "provided",
            IllustratedMessageTextSource::Missing => "missing",
            IllustratedMessageTextSource::Blank => "blank",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IllustratedMessageSlotSource {
    Provided,
    #[default]
    Missing,
}

impl IllustratedMessageSlotSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageSlotSource::Provided => "provided",
            IllustratedMessageSlotSource::Missing => "missing",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IllustratedMessageResolvedView {
    pub state: IllustratedMessageViewState,
    pub title: String,
    pub description: String,
    pub view_state: IllustratedMessageRenderMarker,
    pub title_state: IllustratedMessageStateMarker,
    pub description_state: IllustratedMessageStateMarker,
    pub illustration_state: IllustratedMessageStateMarker,
    pub actions_state: IllustratedMessageStateMarker,
    pub content_state: IllustratedMessageStateMarker,
    pub title_source: IllustratedMessageTextSource,
    pub description_source: IllustratedMessageTextSource,
    pub illustration_source: IllustratedMessageSlotSource,
    pub actions_source: IllustratedMessageSlotSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentIntent {
    EmptyStateDisplay,
}

impl IllustratedMessageAgentIntent {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentIntent::EmptyStateDisplay => "empty-state-display",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentAction {
    RenderSnapshot,
}

impl IllustratedMessageAgentAction {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentAction::RenderSnapshot => "render-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentState {
    Empty,
    Populated,
}

impl IllustratedMessageAgentState {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentState::Empty => "empty",
            IllustratedMessageAgentState::Populated => "populated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentSource {
    Default,
    Custom,
}

impl IllustratedMessageAgentSource {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentSource::Default => "default",
            IllustratedMessageAgentSource::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentConfigPolicy {
    Whitelist,
}

impl IllustratedMessageAgentConfigPolicy {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentConfigPolicy::Whitelist => "whitelist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentStreamingPolicy {
    Optional,
}

impl IllustratedMessageAgentStreamingPolicy {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentStreamingPolicy::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentStreamingFallback {
    Snapshot,
}

impl IllustratedMessageAgentStreamingFallback {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentStreamingFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllustratedMessageAgentOutputStatus {
    Validated,
}

impl IllustratedMessageAgentOutputStatus {
    pub const fn as_data_attr(self) -> &'static str {
        match self {
            IllustratedMessageAgentOutputStatus::Validated => "validated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IllustratedMessageAgentContractAttrs {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub config_policy_attr: &'static str,
    pub streaming_policy_attr: &'static str,
    pub streaming_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract_attrs(
    resolved_view: &IllustratedMessageResolvedView,
) -> IllustratedMessageAgentContractAttrs {
    let state = normalize_agent_state(resolved_view.view_state);
    let source = normalize_agent_source(resolved_view);

    IllustratedMessageAgentContractAttrs {
        schema_attr: ILLUSTRATED_MESSAGE_AGENT_SCHEMA,
        schema_version_attr: ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION,
        intent_attr: IllustratedMessageAgentIntent::EmptyStateDisplay.as_data_attr(),
        action_attr: IllustratedMessageAgentAction::RenderSnapshot.as_data_attr(),
        state_attr: state.as_data_attr(),
        source_attr: source.as_data_attr(),
        config_policy_attr: IllustratedMessageAgentConfigPolicy::Whitelist.as_data_attr(),
        streaming_policy_attr: IllustratedMessageAgentStreamingPolicy::Optional.as_data_attr(),
        streaming_fallback_attr: IllustratedMessageAgentStreamingFallback::Snapshot.as_data_attr(),
        output_status_attr: IllustratedMessageAgentOutputStatus::Validated.as_data_attr(),
    }
}

pub fn resolve_view_model<TIllustration, TActions>(
    title: Option<String>,
    description: Option<String>,
    illustration: Option<&TIllustration>,
    actions: Option<&TActions>,
) -> IllustratedMessageResolvedView {
    let (normalized_title, title_source) = normalize_display_text(title);
    let (normalized_description, description_source) = normalize_display_text(description);
    let illustration_source = normalize_slot_source(illustration);
    let actions_source = normalize_slot_source(actions);
    let state = resolve_view_state(
        matches!(illustration_source, IllustratedMessageSlotSource::Provided),
        normalized_title.as_deref(),
        normalized_description.as_deref(),
        matches!(actions_source, IllustratedMessageSlotSource::Provided),
    );
    let title_state = normalize_state_marker(state.show_title);
    let description_state = normalize_state_marker(state.show_description);
    let illustration_state = normalize_state_marker(state.show_illustration);
    let actions_state = normalize_state_marker(state.show_actions);
    let content_state =
        normalize_state_marker(state.show_title || state.show_description || state.show_actions);
    let view_state = normalize_render_marker(
        state.show_title || state.show_description || state.show_illustration || state.show_actions,
    );

    IllustratedMessageResolvedView {
        state,
        title: normalized_title.unwrap_or_default(),
        description: normalized_description.unwrap_or_default(),
        view_state,
        title_state,
        description_state,
        illustration_state,
        actions_state,
        content_state,
        title_source,
        description_source,
        illustration_source,
        actions_source,
    }
}

pub fn resolve_root_class(
    orientation: crate::IllustratedMessageOrientation,
    class_name: Option<String>,
) -> String {
    let base_class = format!("ui-illustrated-message {}", orientation.class_name());
    let normalized_class = class_name.as_deref().and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then_some(trimmed)
    });

    match normalized_class {
        Some(user_class) => format!("{base_class} {user_class}"),
        None => base_class,
    }
}

fn normalize_display_text(value: Option<String>) -> (Option<String>, IllustratedMessageTextSource) {
    match value {
        None => (None, IllustratedMessageTextSource::Missing),
        Some(text) if text.trim().is_empty() => (None, IllustratedMessageTextSource::Blank),
        Some(text) => (Some(text), IllustratedMessageTextSource::Provided),
    }
}

fn normalize_slot_source<T>(value: Option<&T>) -> IllustratedMessageSlotSource {
    if value.is_some() {
        IllustratedMessageSlotSource::Provided
    } else {
        IllustratedMessageSlotSource::Missing
    }
}

fn normalize_state_marker(is_visible: bool) -> IllustratedMessageStateMarker {
    if is_visible {
        IllustratedMessageStateMarker::Shown
    } else {
        IllustratedMessageStateMarker::Hidden
    }
}

fn normalize_render_marker(has_any_content: bool) -> IllustratedMessageRenderMarker {
    if has_any_content {
        IllustratedMessageRenderMarker::Populated
    } else {
        IllustratedMessageRenderMarker::Empty
    }
}

fn normalize_agent_state(
    view_state: IllustratedMessageRenderMarker,
) -> IllustratedMessageAgentState {
    match view_state {
        IllustratedMessageRenderMarker::Empty => IllustratedMessageAgentState::Empty,
        IllustratedMessageRenderMarker::Populated => IllustratedMessageAgentState::Populated,
    }
}

fn normalize_agent_source(
    resolved_view: &IllustratedMessageResolvedView,
) -> IllustratedMessageAgentSource {
    let has_custom_text = !matches!(
        resolved_view.title_source,
        IllustratedMessageTextSource::Missing
    ) || !matches!(
        resolved_view.description_source,
        IllustratedMessageTextSource::Missing
    );
    let has_custom_slot = matches!(
        resolved_view.illustration_source,
        IllustratedMessageSlotSource::Provided
    ) || matches!(
        resolved_view.actions_source,
        IllustratedMessageSlotSource::Provided
    );

    if has_custom_text || has_custom_slot {
        IllustratedMessageAgentSource::Custom
    } else {
        IllustratedMessageAgentSource::Default
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
