pub type ColorPickerState = ui_state_primitives::color_picker::ColorPickerState;
pub type ColorPickerStateInput = ui_state_primitives::color_picker::ColorPickerStateInput;
pub type ColorPickerMotion = crate::ColorPickerMotion;
pub type ColorSwatchSize = crate::color::swatch::ColorSwatchSize;
pub type ColorSwatchRounding = crate::color::swatch::ColorSwatchRounding;
pub type ColorSwatchShape = crate::color::swatch::ColorSwatchShape;

pub const DEFAULT_LABEL: &str;
pub const DEFAULT_ARIA_LABEL: &str;
pub const COLOR_PICKER_AGENT_SCHEMA: &str;

pub enum ColorPickerAgentSchemaVersion {
    V1,
}

pub enum ColorPickerAgentIntent {
    ColorSelection,
}

pub enum ColorPickerAgentAction {
    SnapshotRender,
    ToggleOpen,
    ToggleClose,
}

pub enum ColorPickerAgentState {
    Disabled,
    Open,
    Selected,
    Empty,
}

pub enum ColorPickerAgentSource {
    StatePrimitives,
}

pub enum ColorPickerAgentOutputStatus {
    Verified,
    Submittable,
}

pub enum ColorPickerAgentStreamSupport {
    Unsupported,
}

pub enum ColorPickerAgentStreamFallback {
    Snapshot,
}

pub enum ColorPickerAgentStreamMode {
    Snapshot,
}

pub struct ColorPickerAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ColorPickerAgentSchemaVersion,
    pub intent: ColorPickerAgentIntent,
    pub action: ColorPickerAgentAction,
    pub state: ColorPickerAgentState,
    pub source: ColorPickerAgentSource,
    pub output_status: ColorPickerAgentOutputStatus,
    pub stream_support: ColorPickerAgentStreamSupport,
    pub stream_fallback: ColorPickerAgentStreamFallback,
    pub stream_mode: ColorPickerAgentStreamMode,
    pub selection_source: &'static str,
    pub open_source: &'static str,
    pub motion_source: &'static str,
    pub label_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub config_policy: &'static str,
}

pub struct ColorPickerAgentContractInput {
    pub render_state: ColorPickerState,
    pub action: ColorPickerAgentAction,
    pub is_selection_controlled: bool,
    pub is_custom_motion: bool,
}

pub fn resolve_agent_contract(
    input: ColorPickerAgentContractInput,
) -> ColorPickerAgentContract;

pub fn ColorPicker(
    id_base: String,
    children: leptos::children::ChildrenFn,
    label: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    is_disabled: bool,
    disabled: Option<bool>,
    value: Option<leptos::prelude::Signal<Option<String>>>,
    default_value: Option<String>,
    on_value_change: Option<leptos::prelude::Callback<Option<String>>>,
    selected_color: Option<leptos::prelude::Signal<Option<String>>>,
    default_selected_color: Option<String>,
    on_selected_change: Option<leptos::prelude::Callback<Option<String>>>,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    motion: ColorPickerMotion,
    placement: ui_headless::PopoverPlacement,
    swatch_size: ColorSwatchSize,
    swatch_rounding: ColorSwatchRounding,
    swatch_shape: ColorSwatchShape,
    swatch_bordered: bool,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
