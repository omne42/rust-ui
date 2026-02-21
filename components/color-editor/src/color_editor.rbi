pub type ColorEditorFormat = ui_state_primitives::color_editor::ColorEditorFormat;
pub type ColorEditorState = ui_state_primitives::color_editor::ColorEditorState;
pub type ColorEditorStateInput = ui_state_primitives::color_editor::ColorEditorStateInput;
pub type ColorEditorMotion = crate::color_slider::ColorSliderMotion;

pub const COLOR_EDITOR_AGENT_SCHEMA: &str;

pub enum ColorEditorAgentSchemaVersion {
    V1,
}

pub enum ColorEditorAgentIntent {
    ColorEditing,
}

pub enum ColorEditorAgentAction {
    SnapshotRender,
    FieldInput,
    AreaDragUpdate,
    HueDragUpdate,
    AlphaDragUpdate,
    FormatChange,
}

pub enum ColorEditorAgentState {
    Disabled,
    Empty,
    Ready,
}

pub enum ColorEditorAgentSource {
    StatePrimitives,
}

pub enum ColorEditorAgentOutputStatus {
    Verified,
    Submittable,
}

pub enum ColorEditorAgentStreamSupport {
    Unsupported,
}

pub enum ColorEditorAgentStreamFallback {
    FullSnapshot,
}

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

pub struct ColorEditorAgentContractInput {
    pub render_state: ColorEditorState,
    pub action: ColorEditorAgentAction,
    pub is_selected_controlled: bool,
    pub is_format_controlled: bool,
}

pub fn resolve_agent_contract(input: ColorEditorAgentContractInput) -> ColorEditorAgentContract;

pub const DEFAULT_LABEL: &str;
pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_HUE: f64;
pub const DEFAULT_ALPHA: f64;
pub const DEFAULT_AREA: (f32, f32);

pub fn ColorEditor(
    id_base: String,
    label: Option<String>,
    aria_label: Option<String>,
    is_disabled: bool,
    selected_color: Option<leptos::prelude::Signal<Option<String>>>,
    default_selected_color: Option<String>,
    on_selected_change: Option<leptos::prelude::Callback<Option<String>>>,
    format: Option<leptos::prelude::Signal<ColorEditorFormat>>,
    default_format: Option<ColorEditorFormat>,
    on_format_change: Option<leptos::prelude::Callback<ColorEditorFormat>>,
    is_alpha_channel_hidden: bool,
    default_hue: Option<f64>,
    default_alpha: Option<f64>,
    default_area: Option<(f32, f32)>,
    area_label: Option<String>,
    area_aria_label: Option<String>,
    hue_label: Option<String>,
    alpha_label: Option<String>,
    value_label: Option<String>,
    format_aria_label: Option<String>,
    preview_color: Option<String>,
    motion: ColorEditorMotion,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
