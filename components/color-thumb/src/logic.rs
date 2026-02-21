use std::borrow::Cow;

#[cfg(test)]
pub use ui_state_primitives::color_thumb::DEFAULT_ARIA_VALUE_TEXT;
pub use ui_state_primitives::color_thumb::{
    ColorThumbAriaValueTextSource, ColorThumbInputSource, ColorThumbInteractionState,
    ColorThumbState, ColorThumbStateInput, DEFAULT_ARIA_LABEL, DEFAULT_COLOR,
    DEFAULT_POSITION_PERCENT, normalize_aria_label, normalize_aria_value_text,
    normalize_optional_text, resolve_state, sanitize_color, sanitize_percent,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorThumbLogicInput {
    pub interaction_state: ColorThumbInteractionState,
    pub is_loupe_visible: Option<bool>,
    pub has_color: bool,
    pub x_percent: Option<f32>,
    pub y_percent: Option<f32>,
    pub has_custom_aria_label: bool,
    pub aria_value_text_source: ColorThumbAriaValueTextSource,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbAgentSchema {
    V1,
}

impl ColorThumbAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-thumb.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbAgentSchemaVersion {
    V1,
}

impl ColorThumbAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbStreamSupport {
    Optional,
}

impl ColorThumbStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbStreamFallback {
    Snapshot,
}

impl ColorThumbStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbOutputStatus {
    Verified,
}

impl ColorThumbOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbIntent {
    PickColorPoint,
}

impl ColorThumbIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::PickColorPoint => "pick-color-point",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorThumbUiAction {
    Idle,
    Focus,
    Drag,
}

impl ColorThumbUiAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Focus => "focus",
            Self::Drag => "drag",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorThumbAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_ui_action(state: ColorThumbState) -> ColorThumbUiAction {
    if state.is_dragging {
        ColorThumbUiAction::Drag
    } else if state.is_focused {
        ColorThumbUiAction::Focus
    } else {
        ColorThumbUiAction::Idle
    }
}

pub fn resolve_agent_contract() -> ColorThumbAgentContract {
    ColorThumbAgentContract {
        schema_attr: ColorThumbAgentSchema::V1.as_attr(),
        schema_version_attr: ColorThumbAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: ColorThumbStreamSupport::Optional.as_attr(),
        stream_fallback_attr: ColorThumbStreamFallback::Snapshot.as_attr(),
        output_status_attr: ColorThumbOutputStatus::Verified.as_attr(),
        intent_attr: ColorThumbIntent::PickColorPoint.as_attr(),
    }
}

pub fn source_from_option<T>(value: Option<T>) -> ColorThumbInputSource {
    if value.is_some() {
        ColorThumbInputSource::External
    } else {
        ColorThumbInputSource::Default
    }
}

pub fn normalize_position_percent(value: Option<f32>) -> f32 {
    match value {
        Some(value) => sanitize_percent(value),
        None => DEFAULT_POSITION_PERCENT,
    }
}

pub fn resolve_component_state(input: ColorThumbLogicInput) -> ColorThumbState {
    resolve_state(ColorThumbStateInput {
        interaction_state: input.interaction_state,
        show_loupe: input.is_loupe_visible.unwrap_or(true),
        loupe_source: source_from_option(input.is_loupe_visible),
        has_color: input.has_color,
        x_percent: normalize_position_percent(input.x_percent),
        y_percent: normalize_position_percent(input.y_percent),
        x_source: source_from_option(input.x_percent),
        y_source: source_from_option(input.y_percent),
        has_custom_aria_label: input.has_custom_aria_label,
        aria_value_text_source: input.aria_value_text_source,
        has_custom_class_name: input.has_custom_class_name,
    })
}

pub fn interaction_state_from_flags(
    is_disabled: bool,
    is_focused: bool,
    is_dragging: bool,
) -> ColorThumbInteractionState {
    ColorThumbInteractionState::from_flags(is_disabled, is_focused, is_dragging)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorThumbState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-color-thumb"),
        Cow::Borrowed(state.x_bucket_class),
        Cow::Borrowed(state.y_bucket_class),
    ];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-color-thumb--disabled"));
    }

    if state.is_focused {
        classes.push(Cow::Borrowed("ui-color-thumb--focused"));
    }

    if state.is_dragging {
        classes.push(Cow::Borrowed("ui-color-thumb--dragging"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-color-thumb--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
