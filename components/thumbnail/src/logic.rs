use super::ThumbnailMotion;

pub use ui_state_primitives::thumbnail::{
    ThumbnailDataState, ThumbnailSize, ThumbnailState, ThumbnailStateInput,
    normalize_optional_text, resolve_state, sanitize_background,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThumbnailNormalizedInput {
    pub background: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThumbnailViewStateInput {
    pub size: ThumbnailSize,
    pub cover: Option<bool>,
    pub layer: Option<bool>,
    pub selected: Option<bool>,
    pub focused: Option<bool>,
    pub motion_source: ThumbnailMotionSource,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThumbnailViewState {
    pub state: ThumbnailState,
    pub class_name: String,
    pub inline_css_vars: String,
    pub motion_source: ThumbnailMotionSource,
    pub motion_active: bool,
    pub cover_source: ThumbnailBooleanSource,
    pub layer_source: ThumbnailBooleanSource,
    pub selected_source: ThumbnailBooleanSource,
    pub focused_source: ThumbnailBooleanSource,
    pub background_source: ThumbnailValueSource,
    pub class_name_source: ThumbnailValueSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailMotionSource {
    Default,
    Custom,
}

impl ThumbnailMotionSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub fn custom_motion_attr(self) -> Option<&'static str> {
        matches!(self, Self::Custom).then_some("true")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailBooleanSource {
    Default,
    Prop,
}

impl ThumbnailBooleanSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Prop => "prop",
        }
    }

    pub fn resolve(value: Option<bool>) -> (bool, Self) {
        if let Some(value) = value {
            (value, Self::Prop)
        } else {
            (false, Self::Default)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailValueSource {
    Default,
    Custom,
}

impl ThumbnailValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Custom => "custom",
        }
    }

    pub fn from_has_custom_value(value: bool) -> Self {
        if value { Self::Custom } else { Self::Default }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailAgentSchemaVersion {
    V1,
}

impl ThumbnailAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailAgentIntent {
    MediaPreview,
}

impl ThumbnailAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MediaPreview => "media-preview",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailAgentAction {
    Inspect,
}

impl ThumbnailAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailAgentStateAxis {
    Default,
    Layer,
    Focused,
    Selected,
}

impl ThumbnailAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Layer => "layer",
            Self::Focused => "focused",
            Self::Selected => "selected",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailAgentSource {
    DefaultOnly,
    PropOrCustom,
}

impl ThumbnailAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultOnly => "default-only",
            Self::PropOrCustom => "prop-or-custom",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThumbnailAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ThumbnailAgentSchemaVersion,
    pub intent: ThumbnailAgentIntent,
    pub action: ThumbnailAgentAction,
    pub state: ThumbnailAgentStateAxis,
    pub source: ThumbnailAgentSource,
}

pub fn compose_class_name(base_class_name: Option<String>, state: ThumbnailState) -> String {
    let mut classes = vec!["ui-thumbnail".to_string(), state.size_class.into()];

    if state.cover {
        classes.push("ui-thumbnail--cover".to_string());
    }

    if state.layer {
        classes.push("ui-thumbnail--layer".to_string());
    }

    if state.selected {
        classes.push("ui-thumbnail--selected".to_string());
    }

    if state.focused {
        classes.push("ui-thumbnail--focused".to_string());
    }

    if state.has_background {
        classes.push("ui-thumbnail--background".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-thumbnail--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_inline_style(background: Option<&str>) -> Option<String> {
    background.map(|background| format!("--ui-thumbnail-background: {background};"))
}

pub fn resolve_motion_source(motion: ThumbnailMotion) -> ThumbnailMotionSource {
    if motion == ThumbnailMotion::default() {
        ThumbnailMotionSource::Default
    } else {
        ThumbnailMotionSource::Custom
    }
}

pub fn normalize_lang(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn normalize_input(
    background: Option<String>,
    class_name: Option<String>,
) -> ThumbnailNormalizedInput {
    ThumbnailNormalizedInput {
        background: sanitize_background(background),
        class_name: normalize_optional_text(class_name),
    }
}

pub fn resolve_view_state(
    input: ThumbnailViewStateInput,
    normalized: ThumbnailNormalizedInput,
) -> ThumbnailViewState {
    let (cover, cover_source) = ThumbnailBooleanSource::resolve(input.cover);
    let (layer, layer_source) = ThumbnailBooleanSource::resolve(input.layer);
    let (selected, selected_source) = ThumbnailBooleanSource::resolve(input.selected);
    let (focused, focused_source) = ThumbnailBooleanSource::resolve(input.focused);

    let state = resolve_state(ThumbnailStateInput {
        size: input.size,
        cover,
        layer,
        selected,
        focused,
        has_background: normalized.background.is_some(),
        has_custom_class_name: normalized.class_name.is_some(),
    });

    ThumbnailViewState {
        class_name: compose_class_name(normalized.class_name, state),
        inline_css_vars: compose_inline_style(normalized.background.as_deref()).unwrap_or_default(),
        motion_source: input.motion_source,
        motion_active: state.selected || state.focused,
        cover_source,
        layer_source,
        selected_source,
        focused_source,
        background_source: ThumbnailValueSource::from_has_custom_value(state.has_background),
        class_name_source: ThumbnailValueSource::from_has_custom_value(state.has_custom_class_name),
        state,
    }
}

pub fn resolve_agent_state_axis(state: ThumbnailState) -> ThumbnailAgentStateAxis {
    match state.data_state {
        ThumbnailDataState::Selected => ThumbnailAgentStateAxis::Selected,
        ThumbnailDataState::Focused => ThumbnailAgentStateAxis::Focused,
        ThumbnailDataState::Layer => ThumbnailAgentStateAxis::Layer,
        ThumbnailDataState::Default => ThumbnailAgentStateAxis::Default,
    }
}

pub fn resolve_agent_source(state: &ThumbnailViewState) -> ThumbnailAgentSource {
    let has_prop_or_custom = matches!(state.motion_source, ThumbnailMotionSource::Custom)
        || matches!(state.cover_source, ThumbnailBooleanSource::Prop)
        || matches!(state.layer_source, ThumbnailBooleanSource::Prop)
        || matches!(state.selected_source, ThumbnailBooleanSource::Prop)
        || matches!(state.focused_source, ThumbnailBooleanSource::Prop)
        || matches!(state.background_source, ThumbnailValueSource::Custom)
        || matches!(state.class_name_source, ThumbnailValueSource::Custom);

    if has_prop_or_custom {
        ThumbnailAgentSource::PropOrCustom
    } else {
        ThumbnailAgentSource::DefaultOnly
    }
}

pub fn resolve_agent_contract(state: &ThumbnailViewState) -> ThumbnailAgentContract {
    ThumbnailAgentContract {
        schema_name: "ui.thumbnail.agent-contract",
        schema_version: ThumbnailAgentSchemaVersion::V1,
        intent: ThumbnailAgentIntent::MediaPreview,
        action: ThumbnailAgentAction::Inspect,
        state: resolve_agent_state_axis(state.state),
        source: resolve_agent_source(state),
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
