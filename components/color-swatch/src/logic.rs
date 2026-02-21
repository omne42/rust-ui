use std::borrow::Cow;

pub use ui_state_primitives::color_swatch::{
    ColorSwatchAlpha, ColorSwatchBoolSource, ColorSwatchRounding, ColorSwatchShape,
    ColorSwatchSize, ColorSwatchState, ColorSwatchStateInput, DEFAULT_ARIA_LABEL,
    normalize_aria_label, normalize_is_bordered, normalize_is_decorative, normalize_optional_text,
    resolve_alpha, resolve_state, sanitize_color_value,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorSwatchRenderInput {
    pub color: Option<String>,
    pub color_name: Option<String>,
    pub size: ColorSwatchSize,
    pub rounding: ColorSwatchRounding,
    pub shape: ColorSwatchShape,
    pub is_bordered: Option<bool>,
    pub is_decorative: Option<bool>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorSwatchRenderState {
    pub color: Option<String>,
    pub aria_label: String,
    pub is_decorative: bool,
    pub bordered_source: ColorSwatchBoolSource,
    pub decorative_source: ColorSwatchBoolSource,
    pub state: ColorSwatchState,
    pub class_name: String,
    pub inline_style: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchAgentSchema {
    V1,
}

impl ColorSwatchAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-swatch.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchAgentSchemaVersion {
    V1,
}

impl ColorSwatchAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchStreamSupport {
    Optional,
}

impl ColorSwatchStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchStreamFallback {
    Snapshot,
}

impl ColorSwatchStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchOutputStatus {
    Verified,
}

impl ColorSwatchOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchIntent {
    ColorPreview,
}

impl ColorSwatchIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::ColorPreview => "color-preview",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchUiAction {
    Render,
}

impl ColorSwatchUiAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
}

pub fn resolve_agent_contract() -> ColorSwatchAgentContract {
    ColorSwatchAgentContract {
        schema_attr: ColorSwatchAgentSchema::V1.as_attr(),
        schema_version_attr: ColorSwatchAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: ColorSwatchStreamSupport::Optional.as_attr(),
        stream_fallback_attr: ColorSwatchStreamFallback::Snapshot.as_attr(),
        output_status_attr: ColorSwatchOutputStatus::Verified.as_attr(),
        intent_attr: ColorSwatchIntent::ColorPreview.as_attr(),
        action_attr: ColorSwatchUiAction::Render.as_attr(),
    }
}

pub fn resolve_render_state(input: ColorSwatchRenderInput) -> ColorSwatchRenderState {
    let color = sanitize_color_value(input.color);
    let alpha = resolve_alpha(color.as_deref());
    let (aria_label, has_custom_aria_label) =
        normalize_aria_label(input.aria_label, input.color_name, color.as_deref(), alpha);
    let (is_bordered, bordered_source) = normalize_is_bordered(input.is_bordered);
    let (is_decorative, decorative_source) = normalize_is_decorative(input.is_decorative);

    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let state = resolve_state(ColorSwatchStateInput {
        size: input.size,
        rounding: input.rounding,
        shape: input.shape,
        bordered: is_bordered,
        alpha,
        has_color: color.is_some(),
        has_custom_aria_label,
        has_custom_class_name,
    });

    let class_name = compose_class_name(class_name, state);
    let inline_style = resolve_inline_style(color.as_deref());

    ColorSwatchRenderState {
        color,
        aria_label,
        is_decorative,
        bordered_source,
        decorative_source,
        state,
        class_name,
        inline_style,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorSwatchState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-color-swatch"),
        Cow::Borrowed(state.size_class),
        Cow::Borrowed(state.rounding_class),
        Cow::Borrowed(state.shape_class),
        Cow::Borrowed(state.alpha_class),
    ];

    if state.is_bordered {
        classes.push(Cow::Borrowed("ui-color-swatch--bordered"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-color-swatch--custom-class"));
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

pub fn compose_inline_style(color: Option<&str>) -> Option<String> {
    color.map(|color| format!("--ui-color-swatch-color: {color};"))
}

pub fn resolve_inline_style(color: Option<&str>) -> String {
    compose_inline_style(color).unwrap_or_default()
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
