pub type ColorSwatchPickerItem = ui_state_primitives::swatch_picker::SwatchPickerItem;
pub type ColorSwatchPickerState = ui_state_primitives::swatch_picker::SwatchPickerState;
pub type ColorSwatchPickerStateInput = ui_state_primitives::swatch_picker::SwatchPickerStateInput;
pub type ColorSwatchPickerMotion = crate::ColorSwatchPickerMotion;
pub type ColorSwatchPickerAgentContract = crate::ColorSwatchPickerAgentContract;
pub type ColorSwatchPickerAgentSchema = crate::ColorSwatchPickerAgentSchema;
pub type ColorSwatchPickerAgentSchemaVersion = crate::ColorSwatchPickerAgentSchemaVersion;
pub type ColorSwatchPickerIntent = crate::ColorSwatchPickerIntent;
pub type ColorSwatchPickerUiAction = crate::ColorSwatchPickerUiAction;
pub type ColorSwatchPickerUiState = crate::ColorSwatchPickerUiState;
pub type ColorSwatchPickerUiSource = crate::ColorSwatchPickerUiSource;

pub const DEFAULT_ARIA_LABEL: &str;
pub fn resolve_agent_contract() -> ColorSwatchPickerAgentContract;
pub fn resolve_ui_action(selection_source_attr: &'static str) -> ColorSwatchPickerUiAction;
pub fn resolve_ui_state(is_disabled: bool, is_empty: bool) -> ColorSwatchPickerUiState;
pub fn resolve_ui_source(selection_source_attr: &'static str) -> ColorSwatchPickerUiSource;

pub fn ColorSwatchPicker(
    swatches: leptos::prelude::ReadSignal<Vec<ColorSwatchPickerItem>>,
    is_disabled: bool,
    size: ColorSwatchSize,
    rounding: ColorSwatchRounding,
    shape: ColorSwatchShape,
    is_bordered: bool,
    selected_color: Option<leptos::prelude::Signal<Option<String>>>,
    default_selected_color: Option<String>,
    on_selected_change: Option<leptos::prelude::Callback<Option<String>>>,
    id_base: Option<String>,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: ColorSwatchPickerMotion,
) -> impl leptos::prelude::IntoView;
