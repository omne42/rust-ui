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
    let mut classes = vec!["ui-thumbnail".to_string(), state.size_class.to_string()];

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
mod tests {
    use super::*;
    use ui_state_primitives::thumbnail::ThumbnailDataState;

    #[test]
    fn normalize_input_filters_background_and_class_name() {
        let normalized = normalize_input(
            Some("  #111827 ".to_string()),
            Some(" docs-thumbnail ".to_string()),
        );
        assert_eq!(normalized.background.as_deref(), Some("#111827"));
        assert_eq!(normalized.class_name.as_deref(), Some("docs-thumbnail"));

        let normalized = normalize_input(
            Some("javascript:alert(1)".to_string()),
            Some("   ".to_string()),
        );
        assert_eq!(normalized.background, None);
        assert_eq!(normalized.class_name, None);
    }

    #[test]
    fn normalize_lang_filters_blank_values() {
        assert_eq!(
            normalize_lang(Some("  zh-CN ".to_string())),
            Some("zh-CN".to_string())
        );
        assert_eq!(normalize_lang(Some("   ".to_string())), None);
        assert_eq!(normalize_lang(None), None);
    }

    #[test]
    fn compose_class_name_tracks_state_markers() {
        let state = resolve_state(ThumbnailStateInput {
            size: ThumbnailSize::Size600,
            cover: true,
            layer: true,
            selected: true,
            focused: false,
            has_background: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.data_state, ThumbnailDataState::Selected);
        assert_eq!(state.data_state.as_attr(), "selected");

        let class_name = compose_class_name(Some("docs-thumbnail".to_string()), state);
        for token in [
            "ui-thumbnail",
            "ui-thumbnail--size-600",
            "ui-thumbnail--cover",
            "ui-thumbnail--layer",
            "ui-thumbnail--selected",
            "ui-thumbnail--background",
            "ui-thumbnail--custom-class",
            "docs-thumbnail",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn compose_inline_style_maps_background_to_css_variable() {
        assert_eq!(
            compose_inline_style(Some("#111827")),
            Some("--ui-thumbnail-background: #111827;".to_string())
        );
    }

    #[test]
    fn resolve_motion_source_tracks_default_vs_custom_motion() {
        assert_eq!(
            resolve_motion_source(ThumbnailMotion::default()),
            ThumbnailMotionSource::Default
        );
        assert_eq!(
            resolve_motion_source(ThumbnailMotion {
                active_scale: 1.08,
                ..ThumbnailMotion::default()
            }),
            ThumbnailMotionSource::Custom
        );
    }

    #[test]
    fn boolean_source_resolves_default_and_prop_inputs() {
        assert_eq!(
            ThumbnailBooleanSource::resolve(None),
            (false, ThumbnailBooleanSource::Default)
        );
        assert_eq!(
            ThumbnailBooleanSource::resolve(Some(true)),
            (true, ThumbnailBooleanSource::Prop)
        );
        assert_eq!(
            ThumbnailBooleanSource::resolve(Some(false)),
            (false, ThumbnailBooleanSource::Prop)
        );
        assert_eq!(ThumbnailBooleanSource::Default.as_attr(), "default");
        assert_eq!(ThumbnailBooleanSource::Prop.as_attr(), "prop");
    }

    #[test]
    fn value_source_uses_closed_default_custom_set() {
        assert_eq!(
            ThumbnailValueSource::from_has_custom_value(false),
            ThumbnailValueSource::Default
        );
        assert_eq!(
            ThumbnailValueSource::from_has_custom_value(true),
            ThumbnailValueSource::Custom
        );
        assert_eq!(ThumbnailValueSource::Default.as_attr(), "default");
        assert_eq!(ThumbnailValueSource::Custom.as_attr(), "custom");
    }

    #[test]
    fn resolve_view_state_centralizes_defaults_and_markers() {
        let view_state = resolve_view_state(
            ThumbnailViewStateInput {
                size: ThumbnailSize::Size600,
                cover: Some(true),
                layer: Some(false),
                selected: Some(true),
                focused: Some(false),
                motion_source: ThumbnailMotionSource::Custom,
            },
            normalize_input(
                Some("#0f172a".to_string()),
                Some("docs-thumbnail-custom".to_string()),
            ),
        );

        assert_eq!(view_state.state.data_state, ThumbnailDataState::Selected);
        assert_eq!(view_state.state.data_state.as_attr(), "selected");
        assert_eq!(view_state.motion_source, ThumbnailMotionSource::Custom);
        assert_eq!(view_state.motion_source.as_attr(), "custom");
        assert_eq!(view_state.motion_source.custom_motion_attr(), Some("true"));
        assert!(view_state.motion_active);
        assert_eq!(view_state.cover_source, ThumbnailBooleanSource::Prop);
        assert_eq!(view_state.layer_source, ThumbnailBooleanSource::Prop);
        assert_eq!(view_state.selected_source, ThumbnailBooleanSource::Prop);
        assert_eq!(view_state.focused_source, ThumbnailBooleanSource::Prop);
        assert_eq!(view_state.background_source, ThumbnailValueSource::Custom);
        assert_eq!(view_state.class_name_source, ThumbnailValueSource::Custom);
        assert_eq!(
            view_state.inline_css_vars,
            "--ui-thumbnail-background: #0f172a;"
        );
        assert!(view_state.class_name.contains("docs-thumbnail-custom"));

        let defaults = resolve_view_state(
            ThumbnailViewStateInput {
                size: ThumbnailSize::Size500,
                cover: None,
                layer: None,
                selected: None,
                focused: None,
                motion_source: ThumbnailMotionSource::Default,
            },
            normalize_input(None, None),
        );
        assert_eq!(defaults.inline_css_vars, "");
        assert_eq!(defaults.motion_source, ThumbnailMotionSource::Default);
        assert_eq!(defaults.motion_source.as_attr(), "default");
        assert_eq!(defaults.motion_source.custom_motion_attr(), None);
        assert!(!defaults.motion_active);
        assert_eq!(defaults.cover_source, ThumbnailBooleanSource::Default);
        assert_eq!(defaults.layer_source, ThumbnailBooleanSource::Default);
        assert_eq!(defaults.selected_source, ThumbnailBooleanSource::Default);
        assert_eq!(defaults.focused_source, ThumbnailBooleanSource::Default);
        assert_eq!(defaults.background_source, ThumbnailValueSource::Default);
        assert_eq!(defaults.class_name_source, ThumbnailValueSource::Default);
    }

    #[test]
    fn resolve_agent_contract_is_schema_typed_and_traceable() {
        let defaults = resolve_view_state(
            ThumbnailViewStateInput {
                size: ThumbnailSize::Size500,
                cover: None,
                layer: None,
                selected: None,
                focused: None,
                motion_source: ThumbnailMotionSource::Default,
            },
            normalize_input(None, None),
        );
        let default_contract = resolve_agent_contract(&defaults);
        assert_eq!(default_contract.schema_name, "ui.thumbnail.agent-contract");
        assert_eq!(default_contract.schema_version.as_str(), "1");
        assert_eq!(default_contract.intent.as_str(), "media-preview");
        assert_eq!(default_contract.action.as_str(), "inspect");
        assert_eq!(default_contract.state.as_str(), "default");
        assert_eq!(default_contract.source.as_str(), "default-only");

        let customized = resolve_view_state(
            ThumbnailViewStateInput {
                size: ThumbnailSize::Size600,
                cover: Some(true),
                layer: Some(true),
                selected: Some(true),
                focused: Some(false),
                motion_source: ThumbnailMotionSource::Custom,
            },
            normalize_input(
                Some("#0f172a".to_string()),
                Some("docs-thumbnail-custom".to_string()),
            ),
        );
        let customized_contract = resolve_agent_contract(&customized);
        assert_eq!(customized_contract.state.as_str(), "selected");
        assert_eq!(customized_contract.source.as_str(), "prop-or-custom");
    }
}
