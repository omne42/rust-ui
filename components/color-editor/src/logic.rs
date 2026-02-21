use std::borrow::Cow;

pub use ui_state_primitives::color_editor::{
    ColorEditorFormat, ColorEditorState, ColorEditorStateInput, DEFAULT_ALPHA, DEFAULT_AREA,
    DEFAULT_ARIA_LABEL, DEFAULT_HUE, DEFAULT_LABEL, compose_class_name, compose_color_from_hsb,
    format_channel_preview, normalize_aria_label, normalize_label, normalize_optional_text,
    resolve_state, sanitize_alpha, sanitize_area, sanitize_color, sanitize_hue,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ColorEditorDefaultInput {
    pub default_selected_color: Option<String>,
    pub default_format: Option<ColorEditorFormat>,
    pub default_hue: Option<f64>,
    pub default_alpha: Option<f64>,
    pub default_area: Option<(f32, f32)>,
    pub area_label: Option<String>,
    pub area_aria_label: Option<String>,
    pub hue_label: Option<String>,
    pub alpha_label: Option<String>,
    pub value_label: Option<String>,
    pub format_aria_label: Option<String>,
    pub preview_color: Option<String>,
    pub class_name: Option<String>,
    pub lang: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorEditorDefaultState {
    pub default_selected_color: Option<String>,
    pub default_format: ColorEditorFormat,
    pub default_hue: f64,
    pub default_alpha: f64,
    pub default_area: (f32, f32),
    pub area_label: String,
    pub area_aria_label: String,
    pub hue_label: String,
    pub alpha_label: String,
    pub value_label: String,
    pub format_aria_label: String,
    pub preview_color: String,
    pub class_name: Option<String>,
    pub normalized_lang: Option<String>,
}

fn normalize_text_with_fallback(value: Option<String>, fallback: &'static str) -> String {
    let normalized: Cow<'static, str> = normalize_optional_text(value)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(fallback));
    normalized.into_owned()
}

pub fn normalize_default_inputs(input: ColorEditorDefaultInput) -> ColorEditorDefaultState {
    let default_selected_color = sanitize_color(input.default_selected_color);
    let default_format = input.default_format.unwrap_or_default();
    let default_hue = sanitize_hue(input.default_hue.unwrap_or(DEFAULT_HUE));
    let default_alpha = sanitize_alpha(input.default_alpha.unwrap_or(DEFAULT_ALPHA));
    let default_area = sanitize_area(input.default_area.unwrap_or(DEFAULT_AREA));

    let preview_color = sanitize_color(input.preview_color).unwrap_or_else(|| {
        compose_color_from_hsb(
            default_hue,
            f64::from(default_area.0 * 100.0),
            f64::from(default_area.1 * 100.0),
            default_alpha,
            true,
        )
    });

    ColorEditorDefaultState {
        default_selected_color,
        default_format,
        default_hue,
        default_alpha,
        default_area,
        area_label: normalize_text_with_fallback(input.area_label, "Saturation / Brightness"),
        area_aria_label: normalize_text_with_fallback(input.area_aria_label, "Color area"),
        hue_label: normalize_text_with_fallback(input.hue_label, "Hue"),
        alpha_label: normalize_text_with_fallback(input.alpha_label, "Alpha"),
        value_label: normalize_text_with_fallback(input.value_label, "Value"),
        format_aria_label: normalize_text_with_fallback(input.format_aria_label, "Color format"),
        preview_color,
        class_name: normalize_optional_text(input.class_name),
        normalized_lang: normalize_optional_text(input.lang),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorSelectionInput {
    pub hue: f64,
    pub area: (f32, f32),
    pub alpha: f64,
    pub hide_alpha_channel: bool,
}

pub fn resolve_selected_color(input: ColorEditorSelectionInput) -> String {
    let hue = sanitize_hue(input.hue);
    let area = sanitize_area(input.area);
    let alpha = sanitize_alpha(input.alpha);

    compose_color_from_hsb(
        hue,
        f64::from(area.0 * 100.0),
        f64::from(area.1 * 100.0),
        alpha,
        input.hide_alpha_channel,
    )
}

pub fn resolve_field_change(next: Option<String>) -> Option<String> {
    sanitize_color(next)
}

pub fn resolve_area_change(
    next_area: (f32, f32),
    current_hue: f64,
    current_alpha: f64,
    hide_alpha_channel: bool,
) -> ((f32, f32), String) {
    let area = sanitize_area(next_area);
    let selected_color = resolve_selected_color(ColorEditorSelectionInput {
        hue: current_hue,
        area,
        alpha: current_alpha,
        hide_alpha_channel,
    });

    (area, selected_color)
}

pub fn resolve_hue_change(
    next_hue: f64,
    current_area: (f32, f32),
    current_alpha: f64,
    hide_alpha_channel: bool,
) -> (f64, String) {
    let hue = sanitize_hue(next_hue);
    let selected_color = resolve_selected_color(ColorEditorSelectionInput {
        hue,
        area: current_area,
        alpha: current_alpha,
        hide_alpha_channel,
    });

    (hue, selected_color)
}

pub fn resolve_alpha_change(
    next_alpha: f64,
    current_hue: f64,
    current_area: (f32, f32),
    hide_alpha_channel: bool,
) -> (f64, String) {
    let alpha = sanitize_alpha(next_alpha);
    let selected_color = resolve_selected_color(ColorEditorSelectionInput {
        hue: current_hue,
        area: current_area,
        alpha,
        hide_alpha_channel,
    });

    (alpha, selected_color)
}

pub const COLOR_EDITOR_AGENT_SCHEMA: &str = "ui.color-editor.agent-contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentSchemaVersion {
    V1,
}

impl ColorEditorAgentSchemaVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentIntent {
    ColorEditing,
}

impl ColorEditorAgentIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentIntent::ColorEditing => "color.editing",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentAction {
    SnapshotRender,
    FieldInput,
    AreaDragUpdate,
    HueDragUpdate,
    AlphaDragUpdate,
    FormatChange,
}

impl ColorEditorAgentAction {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentAction::SnapshotRender => "snapshot-render",
            ColorEditorAgentAction::FieldInput => "field-input",
            ColorEditorAgentAction::AreaDragUpdate => "area-drag-update",
            ColorEditorAgentAction::HueDragUpdate => "hue-drag-update",
            ColorEditorAgentAction::AlphaDragUpdate => "alpha-drag-update",
            ColorEditorAgentAction::FormatChange => "format-change",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentState {
    Disabled,
    Empty,
    Ready,
}

impl ColorEditorAgentState {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentState::Disabled => "disabled",
            ColorEditorAgentState::Empty => "empty",
            ColorEditorAgentState::Ready => "ready",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentSource {
    StatePrimitives,
}

impl ColorEditorAgentSource {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentSource::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentOutputStatus {
    Verified,
    Submittable,
}

impl ColorEditorAgentOutputStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentOutputStatus::Verified => "verified",
            ColorEditorAgentOutputStatus::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentStreamSupport {
    Unsupported,
}

impl ColorEditorAgentStreamSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentStreamSupport::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorEditorAgentStreamFallback {
    FullSnapshot,
}

impl ColorEditorAgentStreamFallback {
    pub fn as_str(self) -> &'static str {
        match self {
            ColorEditorAgentStreamFallback::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorEditorAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorEditorAgentSchemaVersion,
    pub intent: ColorEditorAgentIntent,
    pub action: ColorEditorAgentAction,
    pub state: ColorEditorAgentState,
    pub source: ColorEditorAgentSource,
    pub output_status: ColorEditorAgentOutputStatus,
    pub stream_support: ColorEditorAgentStreamSupport,
    pub stream_fallback: ColorEditorAgentStreamFallback,
    pub selection_source: &'static str,
    pub format_source: &'static str,
    pub motion_source: &'static str,
    pub label_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorEditorAgentContractInput {
    pub render_state: ColorEditorState,
    pub action: ColorEditorAgentAction,
    pub is_selected_controlled: bool,
    pub is_format_controlled: bool,
}

fn resolve_agent_state(render_state: ColorEditorState) -> ColorEditorAgentState {
    if render_state.is_disabled {
        return ColorEditorAgentState::Disabled;
    }
    if render_state.data_state_attr == "ready" {
        return ColorEditorAgentState::Ready;
    }
    ColorEditorAgentState::Empty
}

fn resolve_control_source(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

fn resolve_output_status(action: ColorEditorAgentAction) -> ColorEditorAgentOutputStatus {
    match action {
        ColorEditorAgentAction::SnapshotRender => ColorEditorAgentOutputStatus::Verified,
        ColorEditorAgentAction::FieldInput
        | ColorEditorAgentAction::AreaDragUpdate
        | ColorEditorAgentAction::HueDragUpdate
        | ColorEditorAgentAction::AlphaDragUpdate
        | ColorEditorAgentAction::FormatChange => ColorEditorAgentOutputStatus::Submittable,
    }
}

pub fn resolve_agent_contract(input: ColorEditorAgentContractInput) -> ColorEditorAgentContract {
    ColorEditorAgentContract {
        schema_name: COLOR_EDITOR_AGENT_SCHEMA,
        schema_version: ColorEditorAgentSchemaVersion::V1,
        intent: ColorEditorAgentIntent::ColorEditing,
        action: input.action,
        state: resolve_agent_state(input.render_state),
        source: ColorEditorAgentSource::StatePrimitives,
        output_status: resolve_output_status(input.action),
        stream_support: ColorEditorAgentStreamSupport::Unsupported,
        stream_fallback: ColorEditorAgentStreamFallback::FullSnapshot,
        selection_source: resolve_control_source(input.is_selected_controlled),
        format_source: resolve_control_source(input.is_format_controlled),
        motion_source: input.render_state.motion_source_attr,
        label_source: input.render_state.label_source_attr,
        aria_source: input.render_state.aria_source_attr,
        class_source: input.render_state.class_source_attr,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
