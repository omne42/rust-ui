use std::borrow::Cow;

pub use ui_avatar::AvatarSize;
pub use ui_state_primitives::avatar_group::{AvatarGroupRenderState, AvatarGroupStateInput};

pub const AVATAR_GROUP_AGENT_SCHEMA: &str = "ui.avatar-group.agent.v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupNormalizedInput {
    pub max_visible: usize,
    pub aria_label: String,
    pub has_custom_aria_label: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
    pub lang: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarGroupItemFields {
    pub name: String,
    pub src: String,
    pub alt: String,
    pub has_name: bool,
    pub has_src: bool,
    pub has_alt: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentIntent {
    DisplayIdentityCollection,
}

impl AvatarGroupAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayIdentityCollection => "display-identity-collection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentAction {
    RenderStableRoster,
    RenderOverflowSummary,
}

impl AvatarGroupAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderStableRoster => "render-stable-roster",
            Self::RenderOverflowSummary => "render-overflow-summary",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentStateAxis {
    Stable,
    Overflow,
    Empty,
}

impl AvatarGroupAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Overflow => "overflow",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentSourceAxis {
    DefaultOnly,
    PropOrCustom,
}

impl AvatarGroupAgentSourceAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultOnly => "default-only",
            Self::PropOrCustom => "prop-or-custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentStreamSupport {
    Required,
    Optional,
}

impl AvatarGroupAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}
const _: [AvatarGroupAgentStreamSupport; 2] = [
    AvatarGroupAgentStreamSupport::Required,
    AvatarGroupAgentStreamSupport::Optional,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentStreamFallback {
    Snapshot,
}

impl AvatarGroupAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarGroupAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl AvatarGroupAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupAgentContract {
    pub schema: &'static str,
    pub intent: AvatarGroupAgentIntent,
    pub action: AvatarGroupAgentAction,
    pub state: AvatarGroupAgentStateAxis,
    pub source: AvatarGroupAgentSourceAxis,
    pub stream_support: AvatarGroupAgentStreamSupport,
    pub stream_fallback: AvatarGroupAgentStreamFallback,
    pub output_status: AvatarGroupAgentOutputStatus,
}

pub fn normalize_avatar_group_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::avatar_group::normalize_optional_text(value)
}

pub fn normalize_avatar_group_max_visible(value: Option<usize>) -> usize {
    ui_state_primitives::avatar_group::normalize_max_visible(value)
}

pub fn resolve_avatar_group_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::avatar_group::resolve_aria_label(value)
}

pub fn resolve_avatar_group_render_state(input: AvatarGroupStateInput) -> AvatarGroupRenderState {
    ui_state_primitives::avatar_group::resolve_render_state(input)
}

pub fn normalize_avatar_group_input(
    max: Option<usize>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    default_aria_label: &str,
) -> AvatarGroupNormalizedInput {
    let max_visible = normalize_avatar_group_max_visible(max);
    let (aria_label, has_custom_aria_label) =
        resolve_avatar_group_aria_label_with_fallback(aria_label, default_aria_label);
    let class_name = normalize_avatar_group_optional_text(class_name);

    AvatarGroupNormalizedInput {
        max_visible,
        aria_label,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        class_name,
        lang: normalize_avatar_group_optional_text(lang),
    }
}

pub fn resolve_avatar_group_aria_label_with_fallback(
    value: Option<String>,
    fallback: &str,
) -> (String, bool) {
    let (label, explicit) = resolve_avatar_group_aria_label(value);
    if explicit {
        (label, true)
    } else {
        (fallback.into(), false)
    }
}

pub fn normalize_avatar_group_item_fields(
    name: Option<String>,
    src: Option<String>,
    alt: Option<String>,
) -> AvatarGroupItemFields {
    let name = normalize_avatar_group_optional_text(name);
    let src = normalize_avatar_group_optional_text(src);
    let alt = normalize_avatar_group_optional_text(alt);

    AvatarGroupItemFields {
        has_name: name.is_some(),
        has_src: src.is_some(),
        has_alt: alt.is_some(),
        name: name.unwrap_or_default(),
        src: src.unwrap_or_default(),
        alt: alt.unwrap_or_default(),
    }
}

pub fn compose_avatar_group_class_name(
    base_class_name: Option<String>,
    state: AvatarGroupRenderState,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-avatar-group"),
        Cow::Owned(format!("ui-avatar-group--size-{}", state.size_attr)),
        Cow::Borrowed(visual_state_class_name(state.visual_state)),
        Cow::Borrowed(aria_label_source_class_name(state.aria_label_source)),
    ];

    if state.class_source.is_custom() {
        classes.push(Cow::Borrowed("ui-avatar-group--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .into_iter()
        .map(Cow::into_owned)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn resolve_avatar_group_agent_state_axis(
    state: AvatarGroupRenderState,
) -> AvatarGroupAgentStateAxis {
    match state.visual_state {
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Stable => {
            AvatarGroupAgentStateAxis::Stable
        }
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Overflow => {
            AvatarGroupAgentStateAxis::Overflow
        }
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Empty => {
            AvatarGroupAgentStateAxis::Empty
        }
    }
}

pub fn resolve_avatar_group_agent_source_axis(
    state: AvatarGroupRenderState,
) -> AvatarGroupAgentSourceAxis {
    if state.aria_label_source.is_custom() || state.class_source.is_custom() {
        AvatarGroupAgentSourceAxis::PropOrCustom
    } else {
        AvatarGroupAgentSourceAxis::DefaultOnly
    }
}

pub fn resolve_avatar_group_agent_output_status(
    state: AvatarGroupRenderState,
) -> AvatarGroupAgentOutputStatus {
    if state.visual_state.is_empty() {
        AvatarGroupAgentOutputStatus::Draft
    } else if state.aria_label_source.is_custom() || state.class_source.is_custom() {
        AvatarGroupAgentOutputStatus::Submittable
    } else {
        AvatarGroupAgentOutputStatus::Verified
    }
}

pub fn resolve_avatar_group_agent_contract(
    state: AvatarGroupRenderState,
) -> AvatarGroupAgentContract {
    let action = if state.visual_state.has_overflow() {
        AvatarGroupAgentAction::RenderOverflowSummary
    } else {
        AvatarGroupAgentAction::RenderStableRoster
    };

    AvatarGroupAgentContract {
        schema: AVATAR_GROUP_AGENT_SCHEMA,
        intent: AvatarGroupAgentIntent::DisplayIdentityCollection,
        action,
        state: resolve_avatar_group_agent_state_axis(state),
        source: resolve_avatar_group_agent_source_axis(state),
        stream_support: AvatarGroupAgentStreamSupport::Optional,
        stream_fallback: AvatarGroupAgentStreamFallback::Snapshot,
        output_status: resolve_avatar_group_agent_output_status(state),
    }
}

fn visual_state_class_name(
    state: ui_state_primitives::avatar_group::AvatarGroupVisualState,
) -> &'static str {
    match state {
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Stable => {
            "ui-avatar-group--stable"
        }
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Overflow => {
            "ui-avatar-group--overflow"
        }
        ui_state_primitives::avatar_group::AvatarGroupVisualState::Empty => {
            "ui-avatar-group--empty"
        }
    }
}

fn aria_label_source_class_name(
    source: ui_state_primitives::avatar_group::AvatarGroupAriaLabelSource,
) -> &'static str {
    match source {
        ui_state_primitives::avatar_group::AvatarGroupAriaLabelSource::Default => {
            "ui-avatar-group--label-source-default"
        }
        ui_state_primitives::avatar_group::AvatarGroupAriaLabelSource::Custom => {
            "ui-avatar-group--label-source-custom"
        }
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
